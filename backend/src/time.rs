use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current unix timestamp
///
/// # Examples
///
/// ```
/// print!("Current unix timestamp: {}", get_current_timestamp()); // "Current unix timestamp: 670932000"
/// ```
pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Could not get unix timestamp")
        .as_secs()
}
