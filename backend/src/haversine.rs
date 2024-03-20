use crate::aprs::Position;

const EARTH_MEAN_RADIUS_KM: f32 = 6371.0;

/// Calculates the distance of two given positions
///
/// # Arguments
///
/// * `pos1` - The first position
/// * `pos2` - The second position
///
/// # Examples
/// ```
/// use aprs::Position
///
/// let pos1 = Position {
///    latitude: 48.858222,
///    longitude: 2.2945,
/// };
///
/// let pos2 = Position {
///    latitude: 48.86055,
///    longitude: 2.3376,
/// };
///
/// let distance = calculate_distance(&pos1, &pos2);
/// assert_eq!(distance, 3.164);
/// ```
///
/// # Resources
///
/// * [www.geeksforgeeks.org](https://www.geeksforgeeks.org/haversine-formula-to-find-distance-between-two-points-on-a-sphere/)
/// * [www.movable-type.co.uk](https://www.movable-type.co.uk/scripts/latlong.html)
pub fn calculate_distance(pos1: &Position, pos2: &Position) -> f32 {
    let delta_latitude = (pos1.latitude - pos2.latitude).to_radians();
    let delta_longitude = (pos1.longitude - pos2.longitude).to_radians();

    let latitude_1 = pos1.latitude.to_radians();
    let latitude_2 = pos2.latitude.to_radians();

    let a = (delta_latitude / 2.0).sin().powi(2)
        + (delta_longitude / 2.0).sin().powi(2) * latitude_1.cos() * latitude_2.cos();

    let c = 2.0 * a.sqrt().asin();

    c * EARTH_MEAN_RADIUS_KM
}
