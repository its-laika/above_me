use crate::{aprs::Status, mutex::get_locked, time::get_current_timestamp};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::MAX_AGE_DIFF;

pub fn create_app_state() -> AppState {
    AppState {
        states: Arc::new(Mutex::new(HashMap::new())),
    }
}

pub async fn update_app_state(status: Status, app_state: &AppState) {
    let current_timestamp = get_current_timestamp();

    let mut mapping = get_locked(&app_state.states);

    let outdated_keys = mapping
        .values()
        .filter(|e| current_timestamp - e.time_stamp <= MAX_AGE_DIFF)
        .map(|e| e.aircraft.call_sign.clone())
        .collect::<Vec<String>>();

    for key in outdated_keys {
        mapping.remove(&key);
    }

    mapping.insert(status.aircraft.call_sign.clone(), status);
}

#[derive(Clone)]
pub struct AppState {
    pub states: Arc<Mutex<HashMap<String, Status>>>,
}
