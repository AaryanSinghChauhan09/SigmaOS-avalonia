//! desktop module

use anyhow::Result;
use log::debug;

pub struct DesktopManager;

impl DesktopManager {
    pub fn new(_config: crate::config::CompatibilityConfig) -> Result<Self> {
        debug!("Creating Desktop manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Desktop manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Desktop manager");
        Ok(())
    }
}
