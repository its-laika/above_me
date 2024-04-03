mod aircraft;

pub mod aprs {
    mod client;
    mod conversion;
    mod status;

    pub use client::{init, Config};
    pub use status::Status;
}
pub mod ddb {
    mod client;
    mod conversion;
    mod error;

    pub use client::fetch_aircraft;
}

pub use aircraft::Aircraft;
pub use aircraft::AircraftId;
pub use aircraft::AircraftType;
