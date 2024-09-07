use crate::get::g_last_prices;

use ndarray::Array1;
use std::time::Instant;

pub async fn s_point_data_update(
    smbls_prcs_old: &mut (Array1<String>, Array1<f64>),
    start_changes: &mut Instant
) {
    *smbls_prcs_old = g_last_prices().await.unwrap_or_default();
    *start_changes = Instant::now();
}