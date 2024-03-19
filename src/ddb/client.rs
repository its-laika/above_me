use std::collections::HashMap;

use reqwest::IntoUrl;

use super::error::HttpError;
use crate::aprs::Aircraft;

const VALUE_YES: &str = "Y";
const FIELD_ENCLOSURE: char = '\'';
const FIELD_SEPARATOR: char = ',';
const IDENTIFIER_COMMENT: char = '#';
const LINE_BREAK: char = '\n';
const EMPTY: &str = "";

const INDEX_ID: usize = 1;
const INDEX_TYPE: usize = 2;
const INDEX_REGISTRATION: usize = 3;
const INDEX_CALL_SIGN: usize = 4;
const INDEX_TRACKED: usize = 5;
const INDEX_IDENTIFIED: usize = 6;

/// Alias for `String`, just for readability.
pub type AircraftId = String;

/// Fetches aircraft data from DDB
///
/// # Arguments
///
/// * `url` - The DDB server url
///
/// # Examples
/// ```
/// let url = "https://ddb.example.com/aircraft"
/// let aircrafts = fetch_aircrafts(url)
///     .await
///     .expect("Could not fetch DDB data");
/// ```
pub async fn fetch_aircrafts<A: IntoUrl>(
    url: A,
) -> Result<HashMap<AircraftId, Aircraft>, HttpError> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| HttpError::FetchError)?
        .text()
        .await
        .map_err(|_| HttpError::ResponseError)?;

    Ok(response
        .replace(FIELD_ENCLOSURE, EMPTY)
        .split(LINE_BREAK)
        .filter_map(convert)
        .map(|a| (a.id.clone(), a))
        .collect::<HashMap<AircraftId, Aircraft>>())
}

fn convert(line: &str) -> Option<Aircraft> {
    if line.starts_with(IDENTIFIER_COMMENT) {
        return None;
    }

    let fields = line
        .split(FIELD_SEPARATOR)
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    if fields.len() < 7 {
        return None;
    }

    let id = fields[INDEX_ID].to_string();
    if id.is_empty() {
        return None;
    }

    let call_sign = fields[INDEX_CALL_SIGN].to_string();
    let registration = fields[INDEX_REGISTRATION].to_string();
    let aircraft_type = fields[INDEX_TYPE].to_string();
    let visible = fields[INDEX_IDENTIFIED] == VALUE_YES && fields[INDEX_TRACKED] == VALUE_YES;

    Some(Aircraft {
        id,
        call_sign,
        registration,
        aircraft_type,
        visible,
    })
}
