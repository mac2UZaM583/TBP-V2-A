use crate::get_::g_last_prices;
use crate::session_::*;

use ndarray::Array1;
use serde_json::json;
use std::time::Instant;

pub async fn s_point_data_update(
    smbls_prcs_old: &mut (Array1<String>, Array1<f64>),
    start_changes: &mut Instant
) {
    *smbls_prcs_old = g_last_prices().await.unwrap_or_default();
    *start_changes = Instant::now();
}

pub async fn s_place_order(
    args: (&String, &String),
    mode: &String,
    symbol: &str, 
    order_type: &str,
    price: &str,
    side: &str,
    qty: &str 
) {
    let (api, api_secret) = args;
    let prmtrs = json!({
        "category": "linear",
        "symbol": symbol,
        "side": side,
        "orderType": order_type,
        "price": price,
        "qty": qty
    }).to_string();
    let response = request_(
        &format!("{}{}{}{}", DOMEN, mode, PLACE_ORDER, &prmtrs), 
        Some((api, api_secret, &prmtrs)),
        true,
    ).await;
    println!("{:#?}", response);
}