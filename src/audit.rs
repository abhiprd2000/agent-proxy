use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(action: &str, command: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let line = format!(
        "{} {} {}\n",
        timestamp,
        action,
        command
    );

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("agentproxy.log")
    {
        let _ = file.write_all(line.as_bytes());
    }
}