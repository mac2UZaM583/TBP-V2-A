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
    let Ok((mut symbols_old, mut prices_old)) = g_last_prices(&settings_["MODE"]).await else {todo!();};
    let mut start_changes = Instant::now();

    loop {
        if start_changes.elapsed() >= Duration::new(60, 0) {
            s_point_data_update(
                &settings_["MODE"], 
                &mut symbols_old, 
                &mut prices_old, 
                &mut start_changes
            ).await;
        }   
        let Ok((symbols, percent_change)) = g_percent_changes(
            &settings_["MODE"],
            &symbols_old, 
            &prices_old,
            *threshold_percent,
            *limit_percent
        ).await else {todo!()};
        if !(symbols.is_empty() && percent_change.is_empty()) {
            println!("{:#?}", (symbols, percent_change));
            s_point_data_update(
                &settings_["MODE"], 
                &mut symbols_old, 
                &mut prices_old, 
                &mut start_changes
            ).await;
            
        }
    }
}