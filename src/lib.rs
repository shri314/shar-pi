// Copyright (c) 2025 SharPi Contributors
// MIT License

use anyhow::Result;
use log::{debug, info};

pub mod config;
pub mod clients;

pub fn init() -> Result<()> {
    info!("Initializing SharPi agent");
    debug!("SharPi agent initialization complete");
    Ok(())
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn build_info() -> String {
    format!(
        "Version: {}\nBuild date: {}",
        version(),
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}
