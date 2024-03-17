use config::{ConfigError, File, FileFormat};
use serde::Deserialize;

use crate::aprs::ClientConfig;

pub const FILE_NAME: &str = "config";

#[derive(Deserialize)]
pub struct Config {
    pub aprs: ClientConfig<String>,
    pub ddb_url: String,
    pub bind_to: String,
}

pub fn load_config() -> Result<Config, ConfigError> {
    let config_file = File::new(FILE_NAME, FileFormat::Json);

    config::Config::builder()
        .add_source(config_file)
        .build()?
        .try_deserialize::<Config>()
}
