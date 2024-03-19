use config::{ConfigError, File, FileFormat};
use serde::Deserialize;

use crate::aprs::ClientConfig;

/// Name of the config file (".json" is added by the `config` crate automatically)
pub const FILE_NAME: &str = "../config";

/// Representation of program configuration
#[derive(Deserialize)]
pub struct Config {
    /// Config for connecting to the APRS server
    pub aprs: ClientConfig<String>,
    /// Url of the DDB server to fetch aircraft information
    pub ddb_url: String,
    /// Url that the API server should bind to
    pub bind_to: String,
}

/// Tries loading configuration from config file
///
/// # Examples
/// ```
/// let config = load_config.expect("Could not load config by file");
///
/// print!("Server will bind to: {}", config.bind_to);
/// ```
pub fn load_config() -> Result<Config, ConfigError> {
    let config_file = File::new(FILE_NAME, FileFormat::Json);

    config::Config::builder()
        .add_source(config_file)
        .build()?
        .try_deserialize::<Config>()
}
