// AI Agent Library
//
// A command-line AI agent with Anthropic Sonnet support and various terminal integrations
// including tmux, neovim/vim, fzf, and git.

use anyhow::Result;
use log::{debug, info};

/// Initialize the AI agent
pub fn init() -> Result<()> {
    info!("Initializing AI agent");
    debug!("AI agent initialization complete");
    Ok(())
}

/// Version information for the AI agent
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Build information for the AI agent
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
