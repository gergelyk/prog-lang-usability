use std::fmt::Debug;
use std::error::Error;

use crate::setup;

#[derive(thiserror::Error)]
pub enum LaunchError {
    #[error("Failed to setup application")]
    SetupAppError(#[from] setup::SetupError),
}

impl Debug for LaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;
        
        let mut err_obj : &dyn Error = self;
        while let Some(source) = err_obj.source() {
            write!(f, "\n  Because: {}", source)?;
            err_obj = source;
        }
        Ok(())
    }
}

pub fn launch_app() -> Result<(), LaunchError> {
    Ok(setup::setup_app()?)
}
