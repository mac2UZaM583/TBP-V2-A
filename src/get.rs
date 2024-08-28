use std::error::Error;
use crate::urls::TICKERS;
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
            if let Some(symbol) = item.get("symbol").and_then(Value::as_str) {
                if item["curPreListingPhase"] == "" && symbol.contains("USDT") && !symbol.contains("USDC") {
                    symbols.push(symbol.to_string());
                    if let Some(price_str) = item["lastPrice"].as_str() {
                        if let Ok(price) = price_str.parse::<f64>() {
                            prices.push(price);
                        }
                    }
                }
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