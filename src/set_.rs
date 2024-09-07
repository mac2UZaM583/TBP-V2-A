use crate::get_::g_last_prices;
use crate::session_::*;

use ndarray::Array1;
use std::collections::HashMap;
use std::time::Instant;

pub async fn s_point_data_update(
    smbls_prcs_old: &mut (Array1<String>, Array1<f64>),
    start_changes: &mut Instant
) {
    *smbls_prcs_old = g_last_prices().await.unwrap_or_default();
    *start_changes = Instant::now();
}

pub async fn s_place_order(
    settings_: &HashMap<String, String>,
    symbol: &str, 
    side: &str,
    qty: &str 
) {
    let prmtrs = &format!(
        "{{\"category\": \"spot\",
        \"symbol\": {},
        \"side\": {}
        \"orderType\": \"Market\"
        \"qty\": {}", symbol, side, qty
    );
    let _ = request_(
        PLACE_ORDER, 
        Some(&settings_["API_EXCHANGE"]),
        Some(&settings_["API_2_EXCHANGE"]), 
        Some(prmtrs)
    ).await;
}