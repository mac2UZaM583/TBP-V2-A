use crate::session_::*;

use ndarray::{Array1, Axis};
use std::error::Error;


pub async fn g_last_prices() -> Result<(Array1<String>, Array1<f64>), Box<dyn Error>> {
    let mut symbols: Vec<String> = Vec::new();
    let mut prices: Vec<f64> = Vec::new();
    for item in {
        request_(
            &format!("{}{}", DOMEN, TICKERS), 
            None, 
            None, 
            None, 
            false
        )
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
    smbls_prcs_old: &(Array1<String>, Array1<f64>),
    threshold_percent: f64,
    limit_percent: f64
) -> Result<(String, f64), Box<dyn Error>> {
    let (symbols_new, prices_new) = g_last_prices().await?;
    let (symbols_old, prices_old) = smbls_prcs_old;
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
            symbols_f[0].clone().replace("\"", ""),
            prices_new.select(Axis(0), &indices)[0]
        ));
    }
    Err("g_percent_changes symbols !=".into())
}

pub async fn g_round_qty(symbol: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    Ok(
        request_(
            &format!("{}{}{}", DOMEN, INSTRUMENTS_INFO, symbol), 
            None, 
            None,
            None,
            false,
        )
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
        request_(
            &format!("{}{}{}{}", DOMEN, mode, WALLET_BALANCE, prmtrs), 
            Some(api),
            Some(api_secret),
            Some(prmtrs),
            false,
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
