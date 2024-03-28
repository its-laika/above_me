pub use server::init;
pub use state::App;

mod routes {
    pub mod aircraft;
    pub mod overview;
}

mod server;
mod state;
