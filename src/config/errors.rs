use std::io;
use thiserror::Error;

use super::Config;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error while reading config file: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid config: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

pub type ConfigResult = Result<Config, ConfigError>;
