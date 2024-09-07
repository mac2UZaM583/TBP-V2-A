mod session_;
mod get_;
mod set_;
mod settings_;
use get_::*;
use set_::*;
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
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
        }   
        let (symbol, last_price) = g_percent_changes(
            &smbls_prcs_old,
            *threshold_percent,
            *limit_percent
        ).await.unwrap_or_default();
        
        // START
        if !symbol.is_empty() {
            println!("{}", (&symbol));
            let (balance, round_qty) = tokio::join!(
                g_balance(
                    &settings_["MODE"], 
                    &settings_["ACCOUNT_TYPE"], 
                    &settings_["API_EXCHANGE"], 
                    &settings_["API_2_EXCHANGE"]
                ),
                g_round_qty(&symbol)
            );
            
            
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
        }
    }
}