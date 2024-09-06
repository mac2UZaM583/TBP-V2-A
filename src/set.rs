use crate::get::g_last_prices;

use ndarray::Array1;
use std::time::Instant;

pub async fn s_point_data_update(
    symbols_old: &mut Array1<String>, 
    prices_old: &mut Array1<f64>,
    start_changes: &mut Instant
) {
    (*symbols_old, *prices_old) = g_last_prices().await.expect("s_point_data_update err");
    *start_changes = Instant::now();
}