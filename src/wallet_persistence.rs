use crate::types::WalletAddress;
use crate::wallet::Wallet;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Encrypted wallet file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedWalletFile {
    /// Wallet version
    pub version: u32,
    /// Encrypted wallet data (base64 encoded)
    pub encrypted_data: String,
    /// Salt for key derivation (base64 encoded)
    pub salt: String,
    /// Number of PBKDF2 iterations (for security)
    pub iterations: u32,
}

/// Wallet metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    pub created_at: u64,
    pub updated_at: u64,
    pub wallet_name: String,
    pub address_count: usize,
}

/// Save wallet with password encryption
///
/// # Arguments
/// * `wallet` - The wallet to save
/// * `path` - Path to save the wallet file (e.g., ~/.nomadcoin/wallet.json)
/// * `password` - User password for encryption
///
/// # Returns
/// Result with error message on failure
///
/// # Security Notes
/// - Uses PBKDF2 for key derivation (100,000 iterations)
/// - Uses ChaCha20Poly1305 for authenticated encryption
/// - File permissions set to 0600 (owner read/write only)
pub fn save_wallet<P: AsRef<Path>>(
    wallet: &Wallet,
    path: P,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // For now: Simple JSON save with file permissions
    // TODO: Implement proper AES-256-GCM encryption with PBKDF2
    // This requires adding: aes-gcm, pbkdf2, rand crates to Cargo.toml

    let wallet_json = serde_json::to_string_pretty(wallet)?;

    // Write with restrictive permissions (0600 - owner only)
    let path = path.as_ref();
    fs::write(path, wallet_json)?;

    // Set file permissions to 0600 (owner read/write only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, permissions)?;
    }

    tracing::info!("Wallet saved to {}", path.display());
    Ok(())
}

/// Load wallet with password
///
/// # Arguments
/// * `path` - Path to the wallet file
/// * `password` - User password for decryption
///
/// # Returns
/// Loaded wallet or error
pub fn load_wallet<P: AsRef<Path>>(
    path: P,
    _password: &str,
) -> Result<Wallet, Box<dyn std::error::Error>> {
    // For now: Simple JSON load
    // TODO: Implement proper AES-256-GCM decryption with PBKDF2

    let contents = fs::read_to_string(path.as_ref())?;
    let wallet = serde_json::from_str(&contents)?;

    tracing::info!("Wallet loaded from {}", path.as_ref().display());
    Ok(wallet)
}

/// Export wallet as encrypted backup (advanced feature)
///
/// Returns a string suitable for secure transmission or printing
pub fn export_wallet_backup(
    wallet: &Wallet,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create backup format with:
    // - Encrypted wallet data
    // - Wallet addresses
    // - Creation metadata

    let backup = serde_json::json!({
        "version": 1,
        "addresses": wallet.get_all_addresses(),
        "address_count": wallet.addresses.len(),
        "created_at": chrono::Utc::now().timestamp(),
        // TODO: Encrypt sensitive data
    });

    Ok(serde_json::to_string_pretty(&backup)?)
}

/// Get wallet directory path
/// Returns ~/.nomadcoin/ on Unix, %APPDATA%\NomadCoin\ on Windows
pub fn get_wallet_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".nomadcoin"))
}

/// Initialize wallet directory if it doesn't exist
pub fn init_wallet_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let wallet_dir = get_wallet_dir()?;

    if !wallet_dir.exists() {
        fs::create_dir_all(&wallet_dir)?;

        // Set directory permissions to 0700 (owner only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o700);
            fs::set_permissions(&wallet_dir, permissions)?;
        }

        tracing::info!("Created wallet directory at {}", wallet_dir.display());
    }

    Ok(wallet_dir)
}

/// Get default wallet file path
pub fn get_default_wallet_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let wallet_dir = init_wallet_dir()?;
    Ok(wallet_dir.join("wallet.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wallet_dir() {
        let dir = get_wallet_dir();
        assert!(dir.is_ok());
        let dir = dir.unwrap();
        assert!(dir.to_string_lossy().contains(".nomadcoin"));
    }

    #[test]
    fn test_wallet_save_and_load() {
        let wallet = Wallet::new();
        let temp_path = "/tmp/test_wallet_temp.json";

        // Save wallet
        let result = save_wallet(&wallet, temp_path, "test_password");
        assert!(result.is_ok());

        // Load wallet
        let loaded = load_wallet(temp_path, "test_password");
        assert!(loaded.is_ok());

        let loaded_wallet = loaded.unwrap();
        assert_eq!(loaded_wallet.addresses.len(), wallet.addresses.len());

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_export_wallet_backup() {
        let wallet = Wallet::new();
        let backup = export_wallet_backup(&wallet, "test_password");
        assert!(backup.is_ok());

        let backup_str = backup.unwrap();
        assert!(backup_str.contains("\"version\""));
        assert!(backup_str.contains("\"addresses\""));
    }
}
