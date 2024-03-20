use crate::{
    aprs::{Position, Status},
    haversine::calculate_distance,
    time::get_current_timestamp,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

const MAX_AGE_DIFF: u64 = 60 * 5; /* 5 minutes */

/// Our shared application state for the API
#[derive(Clone)]
pub struct AppState {
    /// Reference to all currently stored states
    states: Arc<Mutex<HashMap<String, Status>>>,
}

impl AppState {
    /// Creates a new `AppState`
    ///
    /// # Examples
    ///
    /// ```
    /// use api::AppState;
    ///
    /// let app_state = AppState::create();
    /// ```
    pub fn create() -> AppState {
        AppState {
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns the states in the `AppState` that match given filters
    ///
    /// # Arguments
    /// * `position` - The position that should be searched for
    /// * `range` - Range around given `latitude` and `longitude` that should be searched for.
    ///
    /// # Examples
    ///
    /// * test `state::get_filtered_states_checks_age`
    /// * test `state::get_filtered_states_checks_range`
    pub fn get_filtered_states(&self, position: &Position, range: f32) -> Vec<Status> {
        let mut states = self.states.lock().expect("Mutex was poisoned");

        AppState::remove_outdated_states(&mut states);

        states
            .values()
            .filter(|&status| calculate_distance(position, &status.position) <= range)
            .cloned()
            .collect::<Vec<Status>>()
    }

    /// Stores / updates a new status in the `AppState`
    ///
    /// # Arguments
    ///
    /// * `status` - The status to store / update
    ///
    /// # Examples
    ///
    /// * test `state::get_filtered_states_checks_age`
    /// * test `state::get_filtered_states_checks_range`
    pub fn push_status(&self, status: Status) {
        let mut states = self.states.lock().expect("Mutex was poisoned");

        AppState::remove_outdated_states(&mut states);

        states.insert(status.aircraft.id.clone(), status);
    }

    /// Removes outdated states (by max age)
    ///
    /// # Arguments
    ///
    /// * `states` - `MutexGuard` of states map
    fn remove_outdated_states(states: &mut MutexGuard<HashMap<String, Status>>) {
        let current_timestamp = get_current_timestamp();

        let outdated_keys = states
            .values()
            .filter(|e| current_timestamp - e.time_stamp > MAX_AGE_DIFF)
            .map(|e| e.aircraft.id.clone())
            .collect::<Vec<String>>();

        for key in outdated_keys {
            states.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aprs::Aircraft;

    use super::*;

    #[test]
    fn get_filtered_states_checks_age() {
        let sut = AppState::create();
        let current_timestamp = get_current_timestamp();
        let outdated_timestamp = current_timestamp - MAX_AGE_DIFF - 1;

        let position = Position {
            longitude: 48.858222,
            latitude: 2.2945,
        };

        sut.push_status(create_status(
            String::from("AB1234"),
            position.clone(),
            current_timestamp,
        ));
        sut.push_status(create_status(
            String::from("CD5678"),
            position.clone(),
            outdated_timestamp,
        ));

        let result = sut.get_filtered_states(&position, 1.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].aircraft.id, "AB1234");
    }

    #[test]
    fn get_filtered_states_checks_range() {
        let sut = AppState::create();
        let current_timestamp = get_current_timestamp();

        let position = Position {
            latitude: 48.858222,
            longitude: 2.2945,
        };

        sut.push_status(create_status(
            String::from("AB1234"),
            position.clone(),
            current_timestamp,
        ));

        sut.push_status(create_status(
            String::from("CD5678"),
            Position {
                /* see haversine.rs -> 3.16 km */
                latitude: 48.86055,
                longitude: 2.3376,
            },
            current_timestamp,
        ));

        sut.push_status(create_status(
            String::from("EF9012"),
            Position {
                longitude: 48.84,
                latitude: 2.2,
            },
            current_timestamp,
        ));

        let result = sut.get_filtered_states(&position, 4.0);

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|s| s.aircraft.id == "AB1234"));
        assert!(result.iter().any(|s| s.aircraft.id == "CD5678"));
    }

    fn create_status(aircraft_id: String, position: Position, time_stamp: u64) -> Status {
        Status {
            aircraft: Aircraft {
                id: aircraft_id,
                call_sign: String::from("G1"),
                registration: String::from("D-6507"),
                aircraft_type: String::from("ASK-21"),
                visible: true,
            },
            position,
            speed: 132,
            vertical_speed: 0.32,
            altitude: 3431,
            turn_rate: 3.5,
            course: 152,
            time_stamp,
        }
    }
}
