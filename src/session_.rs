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
    prmtrs: Option<&str>,
    set: bool
) -> Result<Value, Box<dyn Error>> {
    fn g_hmac(
        args: &(&String, &String,),
        api_secret: &str,
        prmtrs: &str
    ) -> String {
        let (api, timestamp) = args;
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).unwrap();
        mac.update(format!(
            "{}{}5000{}", 
            &timestamp, 
            api, 
            prmtrs
        ).as_bytes());
        return hex::encode(mac.finalize().into_bytes());
    }
    fn g_headers(
        args: &(&String, &String,),
        sign: &str,
    ) -> HeaderMap_ {
        let (api, timestamp) = args;
        let mut headers = HeaderMap_::new();
        for (key, value) in [
            "X-BAPI-SIGN", 
            "X-BAPI-API-KEY", 
            "X-BAPI-TIMESTAMP", 
            "X-BAPI-RECV-WINDOW",
            "Content-type"
        ].iter().zip(vec![
            sign,
            api,
            timestamp,
            "5000",
            "application/json"
        ]) {
            headers.insert(*key, HeaderValue::from_str(value).unwrap());
        }
        return headers;
    }
    if let (Some(api), Some(api_secret), Some(prmtrs)) = (api, api_secret, prmtrs) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();
        let args = (api, &timestamp);
        let hmac = g_hmac(
            &args,
            api_secret, 
            prmtrs,
        );
        let headers = g_headers(&args,&hmac,);
        let client = Client::new();
        let request_build; 
        if set {
            let prmtrs_json: Value = srd_from_str(prmtrs)?;
            request_build = client.post(url).json(&prmtrs_json);
        } else {
            request_build = client.get(url)
        }

        let res_ = request_build
            .headers(headers)
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