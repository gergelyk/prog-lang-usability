use anyhow::{Result, Context};

use crate::config;

pub fn setup_app() -> Result<()> {
    Ok(config::load_config().context("Failed to configure")?)
}
