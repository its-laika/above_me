use std::collections::HashMap;

use reqwest::IntoUrl;

use super::conversion::convert;
use super::error::HttpError;
use crate::aprs::Aircraft;

const FIELD_ENCLOSURE: char = '\'';
const LINE_BREAK: char = '\n';
const EMPTY: &str = "";

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
