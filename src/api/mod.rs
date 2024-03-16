pub use router::init_api_server;

pub const MAX_AGE_DIFF: u64 = 60 * 5; /* 5 minutes */
pub const POSITION_RADIUS: f32 = 0.05; /* approx. 5 - 6 km */

mod handler;
mod router;
mod state;
