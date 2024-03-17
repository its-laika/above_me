use crate::{aprs::Status, time::get_current_timestamp};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

const MAX_AGE_DIFF: u64 = 60 * 5; /* 5 minutes */

/* approximated. (meaning: copied from the internet.) */
const FACTOR_LATITUDE_KM_TO_DEG: f32 = 0.00905;
const FACTOR_LONGITUDE_KM_TO_DEG: f32 = 0.000905;

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

    pub fn get_filtered_states(&self, latitude: f32, longitude: f32, range: f32) -> Vec<Status> {
        let mut states = self.states.lock().expect("Mutex was poisoned");

        AppState::remove_outdated_states(&mut states);

        self.states
            .lock()
            .expect("Mutex was poisoned")
            .values()
            .filter(|&status| {
                let latitude_diff = FACTOR_LATITUDE_KM_TO_DEG * range;
                if f32::abs(status.position.latitude - latitude) > latitude_diff {
                    return false;
                }

                let longitude_diff = FACTOR_LONGITUDE_KM_TO_DEG * range;
                if f32::abs(status.position.longitude - longitude) > longitude_diff {
                    return false;
                }

                true
            })
            .cloned()
            .collect::<Vec<Status>>()
    }

    pub async fn push_status(&self, status: Status) {
        let mut states = self.states.lock().expect("Mutex was poisoned");

        AppState::remove_outdated_states(&mut states);
        states.insert(status.aircraft.call_sign.clone(), status);
    }

    fn remove_outdated_states(states: &mut MutexGuard<HashMap<String, Status>>) {
        let current_timestamp = get_current_timestamp();

        let outdated_keys = states
            .values()
            .filter(|e| current_timestamp - e.time_stamp <= MAX_AGE_DIFF)
            .map(|e| e.aircraft.call_sign.clone())
            .collect::<Vec<String>>();

        for key in outdated_keys {
            states.remove(&key);
        }
    }
}
