use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    FileReadError(#[from] std::io::Error),
    
    #[error("Failed to parse config file: {0}")]
    ParseConfigError(String),
}

pub fn load_config() -> Result<(), ConfigError> {
    let _text = std::fs::read_to_string("myconfig")?;
    Err(ConfigError::ParseConfigError("Unknown key 'foo'".to_string()))?;
    return Ok(());
}
