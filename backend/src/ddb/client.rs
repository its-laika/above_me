use std::collections::HashMap;

use reqwest::IntoUrl;

use super::conversion::convert;
use super::error;
use crate::ogn::{Aircraft, AircraftId};

const LINE_BREAK: char = '\n';

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
) -> Result<HashMap<AircraftId, Aircraft>, error::Http> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| error::Http::FetchError)?
        .text()
        .await
        .map_err(|_| error::Http::ResponseError)?;

    Ok(response
        .split(LINE_BREAK)
        .filter_map(convert)
        .map(|a| (a.id.clone(), a))
        .collect::<HashMap<AircraftId, Aircraft>>())
}
