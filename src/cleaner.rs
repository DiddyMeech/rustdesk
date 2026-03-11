use std::path::PathBuf;

pub fn scan_for_junk() -> Vec<PathBuf> {
    // Stub: Returns a list of paths considered "junk" (e.g., matching cleaner-rules.json)
    vec![
        PathBuf::from("C:\\Windows\\Temp\\old_log.txt"),
        PathBuf::from("C:\\Users\\meech\\AppData\\Local\\Temp\\cache.sqlite"),
    ]
}

pub fn clean_junk(paths: Vec<PathBuf>) -> Result<usize, String> {
    // Stub: Removes junk files, ideally using the quarantine first
    let mut count = 0;
    for _path in paths {
        count += 1;
    }
    Ok(count)
}
