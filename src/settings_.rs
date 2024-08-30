use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{
    Read, 
    Error as io_Error
};

pub fn g_(path: &str) -> Result<HashMap<String, Result<f32, String>>, io_Error> {
    let fls_cntnt: HashMap<_, _> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?; // result -> option
            let filename = entry.file_name();
            if filename.to_string_lossy().ends_with(".txt") {
                let mut file = File::open(entry.path()).ok()?;
                let mut content = String::new();
                file.read_to_string(&mut content).ok()?;
                let key = filename.to_string_lossy().trim_end_matches(".txt").to_string();
                Some((key, match content.parse::<f32>() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(content.to_string()),
                }))
            } else {
                None
            }
        })
        .collect();

    Ok(fls_cntnt)
}

// if pub const fls_cntnt: HashMap<String, String> = g_("SETTINGS").unwrap();