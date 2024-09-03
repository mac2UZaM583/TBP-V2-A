mod urls;
mod get;
mod settings_;
use base64::encode;
use get::*;
use settings_::g_;

use sha2::Sha256;
use tokio; 
use std::time::{
    Instant,
    Duration
};

use rsa::pkcs1::FromRsaPrivateKey;
use rsa::pkcs8::FromPrivateKey;
use rsa::{
    RsaPrivateKey, 
    PaddingScheme
};
use hmac::{Hmac, Mac};
use hex; 

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


    let settings___ = g_("SETTINGS").unwrap();
    println!("{:#?}", g_balance(
        &settings___["MODE"],
        &settings___["ACCOUNT_TYPE"], 
        &settings___["API_EXCHANGE"], 
        &settings___["API_2_EXCHANGE"]
    ).await);
}