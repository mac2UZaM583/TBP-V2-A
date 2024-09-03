use crate::urls::*;

use std::collections::HashMap;
use std::error::Error;
use reqwest::{
    get as r_get, 
    Client
};
use reqwest::header::{
    HeaderMap as HeaderMap_,
    HeaderValue, Values
};
use serde_json::{
    Value, 
    from_str as srd_from_str
};
use ndarray::{
    Array1, 
    Axis
};
use sha2::Sha256;
use std::time::{
    SystemTime, 
    UNIX_EPOCH
};
use hmac::{Hmac, Mac};
use hex; 

async fn response(
    url: &str, 
    api: Option<&String>, 
    api_secret: Option<&String>,
    prmtrs: Option<&str>
) -> Result<Value, Box<dyn Error>> {
    if let (Some(api), Some(api_secret)) = (api, api_secret) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())?;
        mac.update(format!(
            "{}{}5000{}", 
            &timestamp, 
            api, 
            prmtrs.unwrap()
        ).as_bytes());
        
        let mut headers = HeaderMap_::new();
        for (key, value) in [
            "X-BAPI-SIGN", 
            "X-BAPI-API-KEY", 
            "X-BAPI-TIMESTAMP", 
            "X-BAPI-RECV-WINDOW",
            "Content-type"
        ].iter().zip(vec![
            &hex::encode(mac.finalize().into_bytes()),
            api,
            &timestamp.to_string(),
            "5000",
            "application/json"
        ]) {
            headers.insert(*key, HeaderValue::from_str(value)?);
        }
        let res_ = Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?
            .replace("\\\"", "\"")
            .replace("\\", "");
        let json_rasponse: Value = srd_from_str(&res_)?;
        return Ok(json_rasponse);
    }
    Ok(
        r_get(url)
        .await?
        .json()
        .await?
    )
}

pub async fn g_last_prices() -> Result<(Array1<String>, Array1<f64>), Box<dyn Error>> {
    fn s_unpacking_data(data: &Vec<Value>) -> Result<(Array1<String>, Array1<f64>), Box<dyn Error>> {
        let mut symbols: Vec<String> = Vec::new();
        let mut prices: Vec<f64> = Vec::new();
        
        for item in data {
            let symbol = item["symbol"].as_str().unwrap();
            if item["curPreListingPhase"] == "" && symbol.contains("USDT") && !symbol.contains("USDC") {
                symbols.push(symbol.to_string());
                prices.push(item["lastPrice"].as_str().unwrap().parse::<f64>().unwrap());
            }
        }
        Ok((Array1::from_vec(symbols), Array1::from_vec(prices)))
    }
    let response = response(TICKERS, None, None, None).await?;
    if let Some(tickers) = response
        .get("result")
        .and_then(|v| v.get("list"))
        .and_then(Value::as_array) {
        return s_unpacking_data(tickers);
    }
    Err("tickers not found".into())
}

pub async fn g_percent_changes(
    symbols_old: &Array1<String>, 
    prices_old: &Array1<f64>
) -> Result<(Array1<String>, Array1<f64>), Box<dyn Error>> {
    if let Ok((symbols_new, prices_new)) = g_last_prices().await {
        let threshold_percent: f64 = 0.005; // <-
        let limit_percent: f64 = 0.1; // <-
        
        //SET
        let changes = &prices_new / prices_old - 1.0;
        let indices: Vec<usize> = changes
            .iter()
            .enumerate()
            .filter(|(_, &change)| {
                let change = change.abs();
                change >= threshold_percent && change < limit_percent
            })
            .map(|(index, _)| index)
            .collect();
        let symbols_f = symbols_new.select(Axis(0), &indices);
        if symbols_old.select(Axis(0), &indices) == symbols_f {
            return Ok((
                symbols_f,
                changes.select(Axis(0), &indices)
            ));
        }
    }
    Err("data not found".into())
}

pub async fn g_round_qty(symbol: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    if let Ok(response) = response(&format!("{}{}", INSTRUMENTS_INFO, symbol), None, None, None).await {
        let instruments_info = &response["result"]["list"][0]["lotSizeFilter"];
        let res: Vec<usize> = instruments_info
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(k, v)| {
                if k == "minOrderQty" || k == "qtyStep" {
                    v.as_str().and_then(|v| v.find(".").map_or(
                        Some(0), |index| v.get(index..).and_then(|v_| Some(v_.len() - 1))
                    ))
                } else {None}
            })
            .collect();
        return Ok(res);
    }
    Err("instruments info not found".into())
}

pub async fn g_balance(
    mode: &String, 
    account_type: &String, 
    api: &String, 
    api_secret: &String
) -> Result<Value, Box<dyn Error>> {
    let prmtrs = &format!("accountType={}&coin=USDT", account_type);
    Ok(
        response(
            &format!("{}{}{}?{}", DOMEN, mode, WALLET_BALANCE, prmtrs), 
            Some(api),
            Some(api_secret),
            Some(prmtrs)
        ).await?
    )
}
