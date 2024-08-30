use std::error::Error;
use crate::urls::{INSTRUMENTS_INFO, TICKERS};
use reqwest::{get as r_get, Error as Error__};
use serde_json::Value;
use ndarray::{Array1, Axis};

async fn g_response(url: &str) -> Result<Value, Error__> {
    Ok(r_get(url).await?.json().await?)
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
    let response = g_response(TICKERS).await?;
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

pub async fn g_round_qty(symbol: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if let Ok(response) = g_response(&format!("{}{}", INSTRUMENTS_INFO, symbol)).await {
        let instruments_info = &response["result"]["list"][0]["lotSizeFilter"];
        let res: Vec<usize> = instruments_info
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(k, v)| {
                if k == "minOrderQty" || k == "qtyStep" {
                    v.as_str().and_then(|v| v.find(".").map_or(
                        Some(0), |index| v.get(index..).and_then(|v_| Some(v_.len()))
                    ))
                } else {None}
            })
            .collect();
        println!("{:#?}", instruments_info);
        println!("{:#?}", res);

    }
    Err("instruments info not found".into())

}
