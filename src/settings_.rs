use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{
    Read, 
    Error as io_Error
};

pub fn g_(path: &str) -> Result<HashMap<String, String>, io_Error> {
    let fls_cntnt: HashMap<_, _> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?; // result -> option
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
        .collect();

    Ok(fls_cntnt)
}