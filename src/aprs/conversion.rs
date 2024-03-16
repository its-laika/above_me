use super::status::{Aircraft, Position, Status};

/// Tries converting an APRS line into a `Status`
///
/// # Arguments
///
/// * `_line` - The APRS line of the APRS server
///
/// # Examples
///
/// ```
/// /* see: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs */
/// let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0ADDE626 -019fpm +0.0rot 5.5dB 3e -4.3kHz";
/// let status = convert(line).await;
///
/// print!("Callsign: {}", status.aircraft.callsign); // "Callsign: ABCDE"
/// ```
pub async fn convert(_line: &str) -> Option<Status> {
    // TODO
    let status = Status {
        aircraft: Aircraft {
            id: String::from("SOMEID"),
            call_sign: String::from("G1"),
            registration: String::from("D-6507"),
            aircraft_type: String::from("ASK-21"),
        },
        position: Position {
            latitude: 1.23,
            longitude: 4.56,
        },
        speed: 120,
        vertical_speed: 1.54,
        altitude: 522,
        turn_rate: 0.2,
        course: 282,
        time_stamp: 123,
    };

    Some(status)
}
