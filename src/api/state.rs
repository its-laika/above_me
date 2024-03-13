use std::sync::{Arc, Mutex};

pub fn create_app_state() -> AppState {
    AppState {
        api_state: Arc::new(Mutex::new(ApiState {
            message: String::new(),
        })),
    }
}

#[derive(Clone)]
pub struct AppState {
    pub api_state: Arc<Mutex<ApiState>>,
}

#[derive(Clone)]
pub struct ApiState {
    pub message: String,
}
