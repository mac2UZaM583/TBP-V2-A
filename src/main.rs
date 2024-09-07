mod session_;
mod get_;
mod set_;
mod settings_;
use get_::*;
use set_::*;
use settings_::*;

use tokio; 
use std::time::{Instant, Duration};

#[tokio::main]
async fn main() {
    let mut smbls_prcs_old = g_last_prices().await.unwrap_or_default(); 
    let mut start_changes = Instant::now();

    loop {
        if start_changes.elapsed() >= Duration::new(60, 0) {
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
        }   
        let (symbol, last_price) = g_percent_changes(
            &smbls_prcs_old,
            SETTINGS.THRESHOLD_PERCENT,
            SETTINGS.LIMIT_PERCENT
        ).await.unwrap_or_default();
        
        // START
        if !symbol.is_empty() {
            println!("{}", (&symbol));
            let (balance, round_qty) = tokio::join!(
                g_balance(
                    &SETTINGS.MODE, 
                    &SETTINGS.ACCOUNT_TYPE,
                    &SETTINGS.API_EXCHANGE,
                    &SETTINGS.API_2_EXCHANGE
                ),
                g_round_qty(&symbol)
            );
            
            
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
        }
    }
}