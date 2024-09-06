use crate::urls::*;

use ndarray::{Array1, Axis};
use std::error::Error;
use serde_json::{Value, from_str as srd_from_str};
use reqwest::{get as r_get, Client};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::{HeaderMap as HeaderMap_, HeaderValue};
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
            .await
            .expect(&format!("{} response err", &url));
        let json_rasponse: Value = srd_from_str(&res_
            .text()
            .await
            .expect(&format!("{} json response err", &url))
            .replace("\\\"", "\"")
            .replace("\\", "")
        )?;
        return Ok(json_rasponse);
    }
    Ok(
        r_get(url)
        .await
        .expect(&format!("{} response err", &url))
        .json()
        .await
        .expect(&format!("{} json response err", &url))
    )
}

pub async fn g_last_prices(mode: &String) -> Result<(Array1<String>, Array1<f64>), Box<dyn Error>> {
    let mut symbols: Vec<String> = Vec::new();
    let mut prices: Vec<f64> = Vec::new();
    for item in {
        response(&format!("{}{}{}", DOMEN, mode, TICKERS), None, None, None)
            .await?
            .as_object()
            .unwrap()
            ["result"]["list"]
            .as_array()
            .unwrap()
    } {
        let symbol = item["symbol"].to_string();
        if item["curPreListingPhase"] == "" && symbol.contains("USDT") && !symbol.contains("USDC") {
            symbols.push(symbol);
            prices.push(item["lastPrice"].as_str().unwrap().parse::<f64>()?);
        }
    }
    Ok((Array1::from_vec(symbols), Array1::from_vec(prices)))
}

pub async fn g_percent_changes(
    mode: &String,
    symbols_old: &Array1<String>, 
    prices_old: &Array1<f64>,
    threshold_percent: f64,
    limit_percent: f64
) -> Result<(Array1<String>, Array1<f64>), Box<dyn Error>> {
    let (symbols_new, prices_new) = g_last_prices(mode).await?;
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
    Err("data not found".into())
}

pub async fn g_round_qty(symbol: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    Ok(
        response(&format!("{}{}", INSTRUMENTS_INFO, symbol), None, None, None)
            .await?
            ["result"]["list"][0]["lotSizeFilter"]
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
            .collect()
    )
}

pub async fn g_balance(
    mode: &String, 
    account_type: &String, 
    api: &String, 
    api_secret: &String
) -> Result<f64, Box<dyn Error>> {
    let prmtrs = &format!("accountType={}&coin=USDT", account_type);
    Ok(
        response(
            &format!("{}{}{}?{}", DOMEN, mode, WALLET_BALANCE, prmtrs), 
            Some(api),
            Some(api_secret),
            Some(prmtrs)
        )
            .await?
            .as_object()
            .unwrap()
            ["result"]["list"][0]["coin"][0]["walletBalance"]
            .as_str()
            .unwrap()
            .parse::<f64>()?
    )
}
