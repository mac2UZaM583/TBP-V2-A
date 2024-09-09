use std::error::Error;
use serde_json::{from_str as srd_from_str, json, Value};
use reqwest::{get as r_get, Client};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::{HeaderMap as HeaderMap_, HeaderValue};
use hmac::{Hmac, Mac};
use hex; 

pub const DOMEN: &str = "https://api";

// GET
pub const TICKERS: &str = ".bybit.com/v5/market/tickers?category=linear";
pub const INSTRUMENTS_INFO: &str = ".bybit.com/v5/market/instruments-info?category=linear&symbol=";
pub const WALLET_BALANCE: &str = ".bybit.com/v5/account/wallet-balance?";

// SET
pub const PLACE_ORDER: &str = ".bybit.com/v5/order/create?";

pub async fn request_(
    url: &str, 
    api: Option<&String>, 
    api_secret: Option<&String>,
    set: bool,
    prmtrs: Option<&str>
) -> Result<Value, Box<dyn Error>> {
    if let (Some(api), Some(api_secret)) = (api, api_secret) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())?;
        mac.update(format!(
            "{}{}5000{}", 
            &timestamp, 
            api, 
            prmtrs.unwrap()
        ).as_bytes());
        
        let mut headers = HeaderMap_::new();
        for (key, value) in [
            "X-BAPI-SIGN", 
            "X-BAPI-API-KEY", 
            "X-BAPI-TIMESTAMP", 
            "X-BAPI-RECV-WINDOW",
            "Content-type"
        ].iter().zip(vec![
            &hex::encode(mac.finalize().into_bytes()),
            api,
            &timestamp.to_string(),
            "5000",
            "application/json"
        ]) {
            headers.insert(*key, HeaderValue::from_str(value)?);
        }
        let client = Client::new();
        let request_build; 
        if set {
            let prmtrs_json: Value = srd_from_str(prmtrs.unwrap())?;
            request_build = client
                .post(url)
                .headers(headers)
                .json(&prmtrs_json);
        } else {
            request_build = client
                .get(url)
                .headers(headers);
        }
        let res_ = request_build
            .send()
            .await
            .expect(&format!("{} request_ err", &url));
        let json_rasponse: Value = srd_from_str(&res_
            .text()
            .await
            .expect(&format!("{} json request_ err", &url))
            .replace("\\\"", "\"")
            .replace("\\", "")
        )?;
        return Ok(json_rasponse);
    }
    Ok(
        r_get(url)
            .await
            .expect(&format!("{} request_ err", &url))
            .json()
            .await
            .expect(&format!("{} json request_ err", &url))
    )
}