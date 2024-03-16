use crate::aprs::Status;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub fn create_app_state() -> AppState {
    AppState {
        states: Arc::new(Mutex::new(HashMap::new())),
    }
}

#[derive(Clone)]
pub struct AppState {
    pub states: Arc<Mutex<HashMap<String, Status>>>,
}
