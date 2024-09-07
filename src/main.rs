mod urls;
mod get;
mod set;
mod settings_;
use get::*;
use set::*;
use settings_::g_;

use tokio; 
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    let settings_ = g_("SETTINGS").unwrap();
    let threshold_percent = &settings_["THRESHOLD_PERCENT"].parse::<f64>().unwrap();
    let limit_percent = &settings_["LIMIT_PERCENT"].parse::<f64>().unwrap();
    let mut smbls_prcs_old = g_last_prices().await.unwrap_or_default(); 
    let mut start_changes = Instant::now();

    loop {
        if start_changes.elapsed() >= Duration::new(60, 0) {
            s_point_data_update(&mut smbls_prcs_old,&mut start_changes).await;
        }   
        let (symbols, percent_change) = g_percent_changes(
            &smbls_prcs_old,
            *threshold_percent,
            *limit_percent
        ).await.unwrap_or_default();
        
        // START
        if !(symbols.is_empty() && percent_change.is_empty()) {
            println!("{:#?}", (symbols, percent_change));
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
            
        }
    }
}