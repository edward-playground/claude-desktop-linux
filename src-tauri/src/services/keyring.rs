//! Keyring service for secure credential storage
//!
//! Uses the system keyring (GNOME Keyring, KWallet) via the Secret Service API.

use crate::error::{AppError, Result};

const SERVICE_NAME: &str = "claude-for-linux";
const API_KEY_ENTRY: &str = "anthropic-api-key";
const ENCRYPTION_SALT_ENTRY: &str = "encryption-salt";

/// Service for secure credential storage using the system keyring
pub struct KeyringService {
    service_name: String,
}

impl KeyringService {
    /// Create a new keyring service
    pub fn new() -> Result<Self> {
        // Test that we can access the keyring
        let entry = keyring::Entry::new(SERVICE_NAME, "test-access")?;

        // Try to delete any test entry that might exist
        let _ = entry.delete_credential();

        Ok(Self {
            service_name: SERVICE_NAME.to_string(),
        })
    }

    /// Store the API key securely
    pub fn set_api_key(&self, api_key: &str) -> Result<()> {
        if api_key.is_empty() {
            return Err(AppError::InvalidInput("API key cannot be empty".to_string()));
        }

        let entry = keyring::Entry::new(&self.service_name, API_KEY_ENTRY)?;
        entry.set_password(api_key)?;

        tracing::info!("API key stored in keyring");
        Ok(())
    }

    /// Retrieve the API key
    pub fn get_api_key(&self) -> Result<String> {
        let entry = keyring::Entry::new(&self.service_name, API_KEY_ENTRY)?;

        match entry.get_password() {
            Ok(key) => Ok(key),
            Err(keyring::Error::NoEntry) => {
                Err(AppError::NotFound("API key not found in keyring".to_string()))
            }
            Err(e) => Err(AppError::Keyring(e.to_string())),
        }
    }

    /// Delete the API key
    pub fn delete_api_key(&self) -> Result<()> {
        let entry = keyring::Entry::new(&self.service_name, API_KEY_ENTRY)?;

        match entry.delete_credential() {
            Ok(()) => {
                tracing::info!("API key deleted from keyring");
                Ok(())
            }
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(AppError::Keyring(e.to_string())),
        }
    }

    /// Check if an API key is stored
    pub fn has_api_key(&self) -> Result<bool> {
        let entry = keyring::Entry::new(&self.service_name, API_KEY_ENTRY)?;

        match entry.get_password() {
            Ok(_) => Ok(true),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(e) => Err(AppError::Keyring(e.to_string())),
        }
    }

    /// Get or create an encryption salt for local data encryption
    pub fn get_or_create_encryption_salt(&self) -> Result<String> {
        let entry = keyring::Entry::new(&self.service_name, ENCRYPTION_SALT_ENTRY)?;

        match entry.get_password() {
            Ok(salt) => Ok(salt),
            Err(keyring::Error::NoEntry) => {
                // Generate a new random salt
                let salt = generate_random_salt();
                entry.set_password(&salt)?;
                tracing::info!("Generated new encryption salt");
                Ok(salt)
            }
            Err(e) => Err(AppError::Keyring(e.to_string())),
        }
    }

    /// Delete all stored credentials
    pub fn clear_all(&self) -> Result<()> {
        self.delete_api_key()?;

        let salt_entry = keyring::Entry::new(&self.service_name, ENCRYPTION_SALT_ENTRY)?;
        let _ = salt_entry.delete_credential();

        tracing::info!("All credentials cleared from keyring");
        Ok(())
    }
}

/// Generate a random 32-byte salt encoded as base64
fn generate_random_salt() -> String {
    use ring::rand::{SecureRandom, SystemRandom};

    let rng = SystemRandom::new();
    let mut salt = [0u8; 32];
    rng.fill(&mut salt).expect("Failed to generate random salt");

    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, salt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_salt() {
        let salt1 = generate_random_salt();
        let salt2 = generate_random_salt();

        // Salts should be different
        assert_ne!(salt1, salt2);

        // Salt should be base64 encoded 32 bytes (44 characters with padding)
        assert_eq!(salt1.len(), 44);
    }
}
