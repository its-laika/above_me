use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current unix timestamp
///
/// # Examples
///
/// ```
/// assert!(get_current_timestamp() > 0);
/// ```
pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Could not get unix timestamp")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Possibly dump test
    /// Ensures that function does not fail and returns some value
    fn timestamp_not_empty() {
        assert!(get_current_timestamp() > 0);
    }
}
