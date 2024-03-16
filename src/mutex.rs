use std::sync::{Mutex, MutexGuard};

/// Locks `Mutex` and returns `MutexGuard`
///
/// # Arguments
///
/// * `mutex` - The `Mutex` to lock
///
/// # Examples
///
/// ```
/// let mutex = std::sync::Mutex::new(0u8);
/// let value = get_locked(&mutex);
///
/// print!("Value is {}", value); // "Value is 0"
/// ```
pub fn get_locked<T>(mutex: &Mutex<T>) -> MutexGuard<T> {
    mutex.lock().expect("Mutex was poisoned")
}
