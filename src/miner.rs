use serde::{Deserialize, Serialize};

/// Mobile miner service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerService {
    pub wallet_address: String,
    pub is_mining: bool,
    pub validations_count: u64,
    pub earnings: f64,
    pub device_type: String,
    pub connection_type: String,
    pub last_sync: u64,
    pub pending_validations: Vec<PendingValidation>,
}

/// Pending validation for offline sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingValidation {
    pub timestamp: u64,
    pub tx_hash: String,
    pub reward: f64,
}

/// Miner statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerStats {
    pub total_validations: u64,
    pub total_earnings: f64,
    pub uptime_seconds: u64,
    pub validations_per_second: f64,
    pub mobile_boost_multiplier: f64,
}

impl MinerService {
    /// Auto-detect device type based on OS
    fn detect_device_type() -> String {
        #[cfg(target_os = "android")]
        {
            "android".to_string()
        }
        #[cfg(target_os = "ios")]
        {
            "ios".to_string()
        }
        #[cfg(target_os = "macos")]
        {
            "macos".to_string()
        }
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            "linux".to_string()
        }
        #[cfg(target_os = "windows")]
        {
            "windows".to_string()
        }
        #[cfg(not(any(
            target_os = "android",
            target_os = "ios",
            target_os = "macos",
            target_os = "linux",
            target_os = "freebsd",
            target_os = "windows"
        )))]
        {
            "unknown".to_string()
        }
    }

    /// Create new miner service with auto-detected device type
    pub fn new(wallet_address: String, device_type: String) -> Self {
        // Auto-detect if no device type provided or if "auto" is specified
        let detected_type = if device_type.is_empty() || device_type.to_lowercase() == "auto" {
            Self::detect_device_type()
        } else {
            device_type
        };

        MinerService {
            wallet_address,
            is_mining: false,
            validations_count: 0,
            earnings: 0.0,
            device_type: detected_type,
            connection_type: "unknown".to_string(),
            last_sync: chrono::Utc::now().timestamp() as u64,
            pending_validations: Vec::new(),
        }
    }

    /// Start mining/validation
    pub fn start_mining(&mut self) {
        if self.is_mining {
            return;
        }
        self.is_mining = true;
        tracing::info!("Miner started for {}", self.wallet_address);
    }

    /// Stop mining
    pub fn stop_mining(&mut self) {
        if !self.is_mining {
            return;
        }
        self.is_mining = false;
        tracing::info!("Miner stopped for {}", self.wallet_address);
    }

    /// Perform validation (called periodically)
    pub fn perform_validation(&mut self, tx_hash: &str) {
        if !self.is_mining {
            return;
        }

        let now = chrono::Utc::now().timestamp() as u64;
        let base_reward = 0.01; // 0.01 NOMAD per validation

        // Mobile boost: 1.5x for mobile devices
        let boost = if self.device_type == "android" || self.device_type == "ios" {
            1.5
        } else {
            1.0
        };

        let reward = base_reward * boost;

        self.validations_count += 1;
        self.earnings += reward;

        // Store pending validation for sync
        self.pending_validations.push(PendingValidation {
            timestamp: now,
            tx_hash: tx_hash.to_string(),
            reward,
        });

        tracing::debug!("Validation completed: tx={}, reward={}", tx_hash, reward);
    }

    /// Sync pending validations with network
    pub fn sync_with_network(&mut self) -> usize {
        let count = self.pending_validations.len();
        self.pending_validations.clear();
        self.last_sync = chrono::Utc::now().timestamp() as u64;

        tracing::info!("Synced {} validations with network", count);
        count
    }

    /// Get miner statistics
    pub fn get_stats(&self) -> MinerStats {
        let now = chrono::Utc::now().timestamp() as u64;
        let uptime = now.saturating_sub(self.last_sync);

        let vps = if uptime > 0 {
            self.validations_count as f64 / uptime as f64
        } else {
            0.0
        };

        let boost = if self.device_type == "android" || self.device_type == "ios" {
            1.5
        } else {
            1.0
        };

        MinerStats {
            total_validations: self.validations_count,
            total_earnings: self.earnings,
            uptime_seconds: uptime,
            validations_per_second: vps,
            mobile_boost_multiplier: boost,
        }
    }

    /// Update connection type
    pub fn update_connection(&mut self, connection_type: String) {
        self.connection_type = connection_type;
    }

    /// Check if miner is active
    pub fn is_active(&self) -> bool {
        self.is_mining
    }

    /// Get earnings
    pub fn get_earnings(&self) -> f64 {
        self.earnings
    }

    /// Get validation count
    pub fn get_validation_count(&self) -> u64 {
        self.validations_count
    }
}

impl Default for MinerService {
    fn default() -> Self {
        Self::new("".to_string(), "unknown".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_miner() {
        let miner = MinerService::new("nomad1test".to_string(), "android".to_string());
        assert!(!miner.is_mining);
        assert_eq!(miner.validations_count, 0);
        assert_eq!(miner.earnings, 0.0);
    }

    #[test]
    fn test_start_stop_mining() {
        let mut miner = MinerService::new("nomad1test".to_string(), "android".to_string());

        miner.start_mining();
        assert!(miner.is_mining);

        miner.stop_mining();
        assert!(!miner.is_mining);
    }

    #[test]
    fn test_perform_validation() {
        let mut miner = MinerService::new("nomad1test".to_string(), "android".to_string());
        miner.start_mining();

        miner.perform_validation("tx1");
        miner.perform_validation("tx2");
        miner.perform_validation("tx3");

        assert_eq!(miner.validations_count, 3);
        // Mobile boost: 0.01 * 1.5 * 3 = 0.045
        assert!((miner.earnings - 0.045).abs() < 0.001);
        assert_eq!(miner.pending_validations.len(), 3);
    }

    #[test]
    fn test_no_validation_when_not_mining() {
        let mut miner = MinerService::new("nomad1test".to_string(), "android".to_string());
        // Don't start mining

        miner.perform_validation("tx1");
        assert_eq!(miner.validations_count, 0);
        assert_eq!(miner.earnings, 0.0);
    }

    #[test]
    fn test_sync_with_network() {
        let mut miner = MinerService::new("nomad1test".to_string(), "android".to_string());
        miner.start_mining();
        miner.perform_validation("tx1");
        miner.perform_validation("tx2");

        let synced = miner.sync_with_network();
        assert_eq!(synced, 2);
        assert_eq!(miner.pending_validations.len(), 0);
    }

    #[test]
    fn test_get_stats() {
        let mut miner = MinerService::new("nomad1test".to_string(), "ios".to_string());
        miner.start_mining();
        miner.perform_validation("tx1");

        let stats = miner.get_stats();
        assert_eq!(stats.total_validations, 1);
        assert!((stats.total_earnings - 0.015).abs() < 0.001);
        assert!((stats.mobile_boost_multiplier - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_desktop_no_boost() {
        let mut miner = MinerService::new("nomad1test".to_string(), "desktop".to_string());
        miner.start_mining();
        miner.perform_validation("tx1");

        let stats = miner.get_stats();
        assert!((stats.mobile_boost_multiplier - 1.0).abs() < 0.001);
        assert!((stats.total_earnings - 0.01).abs() < 0.001);
    }
}
