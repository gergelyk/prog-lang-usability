use std::fmt::Debug;

use crate::config;

#[derive(thiserror::Error, Debug)]
pub enum SetupError {
    #[error("Failed to configure")]
    LoadConfigError(#[from] config::ConfigError),
}

pub fn setup_app() -> Result<(), SetupError> {
    Ok(config::load_config()?)
}
