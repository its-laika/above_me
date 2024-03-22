use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

use crate::aprs;

/// Name of the config file (".json" is added by the `config` crate automatically)
pub const PROJECT_CONFIG_FILE: &str = "../config";
pub const BACKEND_CONFIG_FILE: &str = "config";
pub const ENVIRONMENT_PREFIX: &str = "ABOVE_ME";
const ENVIRONMENT_SEPARATOR: &str = "__";

/// Representation of program configuration
#[derive(Deserialize)]
pub struct Config {
    /// Config for connecting to the APRS server
    pub aprs: aprs::Config<String>,
    /// Url of the DDB server to fetch aircraft information
    pub ddb_url: String,
    /// Url that the API server should bind to
    pub bind_to: String,
}

/// Tries loading configuration from config files or environment
///
/// # Examples
/// ```
/// let config = load_config().expect("Could not load config by file");
/// print!("Server will bind to: {}", config.bind_to);
/// ```
pub fn load() -> Result<Config, ConfigError> {
    config::Config::builder()
        .add_source(File::new(PROJECT_CONFIG_FILE, FileFormat::Json).required(false))
        .add_source(File::new(BACKEND_CONFIG_FILE, FileFormat::Json).required(false))
        .add_source(Environment::with_prefix(ENVIRONMENT_PREFIX).separator(ENVIRONMENT_SEPARATOR))
        .build()?
        .try_deserialize::<Config>()
}
