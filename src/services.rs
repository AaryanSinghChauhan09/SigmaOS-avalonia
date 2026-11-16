//! services module

use anyhow::Result;
use log::debug;

pub struct ServicesManager;

impl ServicesManager {
    pub fn new(_config: crate::config::CompatibilityConfig) -> Result<Self> {
        debug!("Creating Services manager");
        Ok(Self)
    }
    
    pub async fn init(&mut self) -> Result<()> {
        debug!("Initializing Services manager");
        Ok(())
    }
    
    pub async fn shutdown(&mut self) -> Result<()> {
        debug!("Shutting down Services manager");
        Ok(())
    }
}
