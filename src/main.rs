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
    let args = (&SETTINGS.API_EXCHANGE, &SETTINGS.API_2_EXCHANGE,);
    let mut smbls_prcs_old = g_last_prices().await.unwrap_or_default(); 
    let mut start_changes = Instant::now();

    loop {
        println!("//");
        if start_changes.elapsed() >= Duration::new(60, 0) {
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
        }   
        let (symbol, side, last_price) = g_percent_changes(
            &smbls_prcs_old,
            SETTINGS.THRESHOLD_PERCENT,
            SETTINGS.LIMIT_PERCENT
        ).await.unwrap_or_default();
        
        // START
        if !symbol.is_empty() {
            println!("{:#?}", (&symbol));
            if let (Ok(balance), Ok(round_qty)) = tokio::join!(
                g_balance(
                    &args,
                    &SETTINGS.MODE, 
                    &SETTINGS.ACCOUNT_TYPE,
                ),
                g_round_qty(&symbol)
            ) {
                s_place_order(
                    &args,
                    &SETTINGS.MODE,
                    &symbol,
                    "Market",
                    &last_price.to_string(),
                    &side,
                    &format!("{:.1$}", balance / last_price, round_qty[1]) // leverage 10x
                ).await;
                println!("{:#?}", (balance, round_qty, last_price));
            };
            s_point_data_update(&mut smbls_prcs_old, &mut start_changes).await;
        }
    }

    // println!("{:#?}", g_balance(
    //     &SETTINGS.MODE, 
    //     &SETTINGS.ACCOUNT_TYPE,
    //     &SETTINGS.API_EXCHANGE,
    //     &SETTINGS.API_2_EXCHANGE
    // ).await);
}