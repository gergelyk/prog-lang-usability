use anyhow::{Result, Context};

use crate::setup;

pub fn launch_app() -> Result<()> {
    Ok(setup::setup_app().context("Failed to setup application")?)
}
