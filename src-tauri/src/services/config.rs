//! Configuration service for managing application paths and settings

use std::path::PathBuf;

use crate::error::{AppError, Result};

/// Configuration paths following XDG Base Directory Specification
#[derive(Debug, Clone)]
pub struct ConfigPaths {
    /// Configuration directory (~/.config/claude-for-linux/)
    pub config_dir: PathBuf,

    /// Data directory (~/.local/share/claude-for-linux/)
    pub data_dir: PathBuf,

    /// Cache directory (~/.cache/claude-for-linux/)
    pub cache_dir: PathBuf,
}

impl ConfigPaths {
    /// Get the database file path
    pub fn database_path(&self) -> PathBuf {
        self.data_dir.join("conversations.db")
    }

    /// Get the settings file path
    pub fn settings_path(&self) -> PathBuf {
        self.config_dir.join("settings.json")
    }

    /// Get the log file path
    pub fn log_path(&self) -> PathBuf {
        self.cache_dir.join("app.log")
    }

    /// Get the plugins directory path
    pub fn plugins_dir(&self) -> PathBuf {
        self.data_dir.join("plugins")
    }
}

/// Configuration service
pub struct ConfigService {
    paths: ConfigPaths,
}

impl ConfigService {
    const APP_NAME: &'static str = "claude-for-linux";

    /// Create a new configuration service
    pub fn new() -> Self {
        let paths = ConfigPaths {
            config_dir: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join(Self::APP_NAME),
            data_dir: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                .join(Self::APP_NAME),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("~/.cache"))
                .join(Self::APP_NAME),
        };

        Self { paths }
    }

    /// Get the configuration paths
    pub fn paths(&self) -> &ConfigPaths {
        &self.paths
    }

    /// Ensure all necessary directories exist
    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.paths.config_dir).map_err(|e| {
            AppError::Config(format!("Failed to create config directory: {}", e))
        })?;

        std::fs::create_dir_all(&self.paths.data_dir).map_err(|e| {
            AppError::Config(format!("Failed to create data directory: {}", e))
        })?;

        std::fs::create_dir_all(&self.paths.cache_dir).map_err(|e| {
            AppError::Config(format!("Failed to create cache directory: {}", e))
        })?;

        // Set restrictive permissions on data directory (owner only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o700);
            std::fs::set_permissions(&self.paths.data_dir, permissions).ok();
        }

        Ok(())
    }

    /// Get the version of the application
    pub fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    /// Get the application name
    pub fn app_name(&self) -> &str {
        Self::APP_NAME
    }
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_service_creation() {
        let config = ConfigService::new();
        assert!(config.paths().config_dir.ends_with("claude-for-linux"));
        assert!(config.paths().data_dir.ends_with("claude-for-linux"));
        assert!(config.paths().cache_dir.ends_with("claude-for-linux"));
    }

    #[test]
    fn test_database_path() {
        let config = ConfigService::new();
        assert!(config.paths().database_path().ends_with("conversations.db"));
    }
}
