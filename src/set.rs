use crate::get::g_last_prices;

use ndarray::Array1;
use std::time::Instant;

pub async fn s_point_data_update(
    symbols_old: &mut Array1<String>, 
    prices_old: &mut Array1<f64>,
    start_changes: &mut Instant
) {
    let res = g_last_prices().await.unwrap_or_default();
    (*symbols_old, *prices_old) = res;
    *start_changes = Instant::now();
}