use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub chain_id: String,
    pub block_time_ms: u64,
    pub max_tx_per_block: u32,
    pub max_block_size: u32,
    pub total_supply: u64,
    pub community_allocation: u64,
    pub validator_reward: u64,
    pub inflation_rate: f64,
    pub confirmations_for_finality: u32,
    pub stake_minimum: u64,
    pub mesh_max_hops: u8,
    pub mesh_discovery_interval: u64,
    pub offline_tx_validity: u64,
    /// Bootstrap peers for P2P network discovery
    #[serde(default)]
    pub bootstrap_peers: Vec<String>,
}

impl NetworkConfig {
    /// Load configuration from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Get default mainnet configuration
    pub fn default_mainnet() -> Self {
        NetworkConfig {
            chain_id: "nomadcoin-mainnet-1".to_string(),
            block_time_ms: 5000,
            max_tx_per_block: 1000,
            max_block_size: 2_000_000, // 2MB
            total_supply: 100_000_000,
            community_allocation: 10_000_000,
            validator_reward: 1_000_000, // Per year
            inflation_rate: 0.05,        // 5% annual
            confirmations_for_finality: 5,
            stake_minimum: 100,
            mesh_max_hops: 5,
            mesh_discovery_interval: 300, // seconds
            offline_tx_validity: 86400,   // 24 hours
            bootstrap_peers: Vec::new(), // No hardcoded bootstrap peers
        }
    }

    /// Get default testnet configuration
    pub fn default_testnet() -> Self {
        NetworkConfig {
            chain_id: "nomadcoin-testnet-1".to_string(),
            block_time_ms: 5000,
            max_tx_per_block: 1000,
            max_block_size: 2_000_000, // 2MB
            total_supply: 100_000_000,
            community_allocation: 10_000_000,
            validator_reward: 1_000_000, // Per year
            inflation_rate: 0.05,        // 5% annual
            confirmations_for_finality: 5,
            stake_minimum: 100,
            mesh_max_hops: 5,
            mesh_discovery_interval: 300, // seconds
            offline_tx_validity: 86400,   // 24 hours
            bootstrap_peers: vec!["127.0.0.1:9333".to_string()], // Localhost for development
        }
    }

    /// Get default development configuration
    pub fn default_devnet() -> Self {
        NetworkConfig {
            chain_id: "nomadcoin-dev-1".to_string(),
            block_time_ms: 5000,
            max_tx_per_block: 1000,
            max_block_size: 2_000_000, // 2MB
            total_supply: 100_000_000,
            community_allocation: 10_000_000,
            validator_reward: 1_000_000, // Per year
            inflation_rate: 0.05,        // 5% annual
            confirmations_for_finality: 5,
            stake_minimum: 100,
            mesh_max_hops: 5,
            mesh_discovery_interval: 300, // seconds
            offline_tx_validity: 86400,   // 24 hours
            bootstrap_peers: vec!["127.0.0.1:9333".to_string()], // Localhost only
        }
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.chain_id.is_empty() {
            errors.push("chain_id must not be empty".to_string());
        }

        if self.chain_id.len() < 3 || self.chain_id.len() > 64 {
            errors.push("chain_id must be between 3 and 64 characters".to_string());
        }

        if self.block_time_ms < 1000 {
            errors.push("block_time_ms must be at least 1000ms".to_string());
        }

        if self.max_tx_per_block == 0 {
            errors.push("max_tx_per_block must be > 0".to_string());
        }

        if self.total_supply == 0 {
            errors.push("total_supply must be > 0".to_string());
        }

        if self.confirmations_for_finality < 1 {
            errors.push("confirmations_for_finality must be >= 1".to_string());
        }

        if self.stake_minimum == 0 {
            errors.push("stake_minimum must be > 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::default_devnet()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mainnet_config() {
        let config = NetworkConfig::default_mainnet();
        assert_eq!(config.chain_id, "nomadcoin-mainnet-1");
        assert_eq!(config.confirmations_for_finality, 5);
        assert!(config.bootstrap_peers.is_empty());
    }

    #[test]
    fn test_default_testnet_config() {
        let config = NetworkConfig::default_testnet();
        assert_eq!(config.chain_id, "nomadcoin-testnet-1");
        assert_eq!(config.bootstrap_peers, vec!["127.0.0.1:9333"]);
    }

    #[test]
    fn test_config_validation() {
        let mut config = NetworkConfig::default_mainnet();
        assert!(config.validate().is_ok());

        config.chain_id = String::new();
        assert!(config.validate().is_err());

        config.chain_id = "nomadcoin-mainnet-1".to_string();
        config.confirmations_for_finality = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_serialization() {
        let config = NetworkConfig::default_testnet();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: NetworkConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.chain_id, deserialized.chain_id);
    }
}
