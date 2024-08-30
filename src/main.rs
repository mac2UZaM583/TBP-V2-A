mod urls;
mod get;
mod settings_;
use get::{g_last_prices, g_percent_changes, g_round_qty};
use settings_::g_;

use tokio; 
use std::time::{
    Instant,
    Duration
};

#[tokio::main]
async fn main() {
    // loop {
    //     let Ok((mut symbols_old, mut prices_old)) = g_last_prices().await else {todo!()};
    //     let mut start_changes = Instant::now();

    //     loop {
    //         println!("///");
    //         if start_changes.elapsed() >= Duration::new(60, 0) {
    //             let Ok((symbols_old_, prices_old_)) = g_last_prices().await else {todo!()};
    //             (symbols_old, prices_old) = (symbols_old_, prices_old_);
    //             start_changes = Instant::now();
    //         }
    //         let Ok((symbols, percent_change)) = g_percent_changes(&symbols_old, &prices_old).await else {todo!()};
    //         if !(symbols.is_empty() && percent_change.is_empty()) {
    //             println!("{:#?}", (symbols, percent_change));
    //             break;
    //         }
    //     }
    // }

    // println!("{:#?}", g_("SETTINGS").unwrap()["TB_ID"]);
    g_round_qty("SILLYUSDT").await;
}