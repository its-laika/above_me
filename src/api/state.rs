use crate::{aprs::Status, time::get_current_timestamp};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

pub const MAX_AGE_DIFF: u64 = 60 * 5; /* 5 minutes */

#[derive(Clone)]
pub struct AppState {
    states: Arc<Mutex<HashMap<String, Status>>>,
}

impl AppState {
    pub fn create() -> AppState {
        AppState {
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_states(&self) -> MutexGuard<HashMap<String, Status>> {
        self.states.lock().expect("Mutex was poisoned")
    }

    pub async fn push_status(&self, status: Status) {
        let current_timestamp = get_current_timestamp();

        let mut states = self.get_states();

        let outdated_keys = states
            .values()
            .filter(|e| current_timestamp - e.time_stamp <= MAX_AGE_DIFF)
            .map(|e| e.aircraft.call_sign.clone())
            .collect::<Vec<String>>();

        for key in outdated_keys {
            states.remove(&key);
        }

        states.insert(status.aircraft.call_sign.clone(), status);
    }
}
