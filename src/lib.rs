//! SigmaOS Avalonia Compatibility Layer
//!
//! Avalonia compatibility for SigmaOS
//! Provides compatibility and integration

#![forbid(unsafe_code)]

use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod package;
pub mod configuration;
pub mod services;
pub mod desktop;
pub mod error;
pub mod config;

pub use package::*;
pub use configuration::*;
pub use services::*;
pub use desktop::*;
pub use error::*;
pub use config::*;

/// Compatibility layer
pub struct Compatibility {
    pub config: CompatibilityConfig,
    pub package_manager: Arc<RwLock<PackageManager>>,
    pub service_manager: Arc<RwLock<ServiceManager>>,
}

impl Compatibility {
    /// Create new compatibility instance
    pub fn new() -> Result<Self> {
        info!("Initializing SigmaOS Avalonia compatibility");
        let config = CompatibilityConfig::load_default()?;
        debug!("Loaded compatibility configuration");
        
        let package_manager = Arc::new(RwLock::new(PackageManager::new(config.clone())?));
        let service_manager = Arc::new(RwLock::new(ServiceManager::new(config.clone())?));
        
        Ok(Self {
            config,
            package_manager,
            service_manager,
        })
    }

    /// Initialize compatibility
    pub async fn init(&mut self) -> Result<()> {
        info!("Starting compatibility initialization");
        
        let mut package_manager = self.package_manager.write().await;
        package_manager.init().await?;
        debug!("Package manager initialized");
        
        let mut service_manager = self.service_manager.write().await;
        service_manager.init().await?;
        debug!("Service manager initialized");
        
        info!("Compatibility initialization complete");
        Ok(())
    }

    /// Shutdown compatibility
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down compatibility");
        
        let mut package_manager = self.package_manager.write().await;
        package_manager.shutdown().await?;
        
        let mut service_manager = self.service_manager.write().await;
        service_manager.shutdown().await?;
        
        info!("Compatibility shutdown complete");
        Ok(())
    }
}
