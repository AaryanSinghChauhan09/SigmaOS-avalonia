//! configuration module

use anyhow::Result;
use log::debug;

pub struct ConfigurationManager;

impl ConfigurationManager {
    pub fn new(_config: crate::config::CompatibilityConfig) -> Result<Self> {
        debug!("Creating Configuration manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Configuration manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Configuration manager");
        Ok(())
    }
}
