use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;

fn g_(path: &str) -> HashMap<String, String> {
    return fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let filename = entry.file_name();
            if filename.to_string_lossy().ends_with(".txt") {
                let mut file = File::open(entry.path()).ok()?;
                let mut content = String::new();
                file.read_to_string(&mut content).ok()?;
                Some((
                    filename.to_string_lossy().trim_end_matches(".txt").to_string(),
                    content
                ))
            } else {
                None
            }
        })
        .collect()
}

pub struct Settings {
    pub API_EXCHANGE: String,
    pub API_2_EXCHANGE: String,
    pub MODE: String,
    pub ACCOUNT_TYPE: String,
    pub THRESHOLD_PERCENT: f64,
    pub LIMIT_PERCENT: f64,
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    let settings_ = g_("SETTINGS");
    Settings {
        API_EXCHANGE: settings_.get("API_EXCHANGE").unwrap_or(&"default_exchange".to_string()).clone(),
        API_2_EXCHANGE: settings_.get("API_2_EXCHANGE").unwrap_or(&"default_exchange_2".to_string()).clone(),
        MODE: settings_.get("MODE").unwrap_or(&"default_mode".to_string()).clone(),
        ACCOUNT_TYPE: settings_.get("ACCOUNT_TYPE").unwrap_or(&"default_account".to_string()).clone(),
        THRESHOLD_PERCENT: settings_.get("THRESHOLD_PERCENT").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap(),
        LIMIT_PERCENT: settings_.get("LIMIT_PERCENT").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap(),
    }
});