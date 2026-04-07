use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Validator state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorState {
    pub address: String,
    pub stake: u64,
    pub is_mobile: bool,
    pub last_proposal: u64,
    pub total_proposals: u64,
    pub total_validations: u64,
    pub uptime_percentage: f64,
    pub jailed: bool,
}

/// Consensus round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRound {
    pub height: u64,
    pub round: u32,
    pub proposer: String,
    pub prevotes: HashMap<String, bool>,
    pub precommits: HashMap<String, bool>,
    pub started_at: u64,
}

/// NomadPOS consensus engine
pub struct ConsensusEngine {
    pub validators: HashMap<String, ValidatorState>,
    pub current_round: Option<ConsensusRound>,
    pub height: u64,
    pub min_stake: u64,
    pub mobile_boost: f64,
    pub slash_factor: f64,
    pub block_time_ms: u64,
    /// Number of blocks required for finality (5 blocks = ~25 seconds)
    pub finality_blocks: u32,
}

impl ConsensusEngine {
    /// Create new consensus engine
    pub fn new(min_stake: u64, mobile_boost: f64) -> Self {
        ConsensusEngine {
            validators: HashMap::new(),
            current_round: None,
            height: 0,
            min_stake,
            mobile_boost,
            slash_factor: 0.01, // 1% slash for downtime
            block_time_ms: 5000,
            finality_blocks: 5, // 5 blocks for finality (~25 seconds)
        }
    }

    /// Register a new validator
    pub fn register_validator(
        &mut self,
        address: String,
        stake: u64,
        is_mobile: bool,
    ) -> Result<(), String> {
        if stake < self.min_stake {
            return Err(format!(
                "Insufficient stake: {} < {} NOMAD minimum",
                stake, self.min_stake
            ));
        }

        if self.validators.contains_key(&address) {
            return Err(format!("Validator {} already registered", address));
        }

        let effective_stake = if is_mobile {
            (stake as f64 * self.mobile_boost) as u64
        } else {
            stake
        };

        self.validators.insert(
            address.clone(),
            ValidatorState {
                address: address.clone(),
                stake: effective_stake,
                is_mobile,
                last_proposal: 0,
                total_proposals: 0,
                total_validations: 0,
                uptime_percentage: 100.0,
                jailed: false,
            },
        );

        tracing::info!(
            "Validator registered: {} (stake={}, mobile={}, effective_stake={})",
            address,
            stake,
            is_mobile,
            effective_stake
        );

        Ok(())
    }

    /// Select proposer for next block using weighted random selection
    pub fn select_proposer(&self) -> Option<String> {
        if self.validators.is_empty() {
            return None;
        }

        // Filter out jailed validators
        let active_validators: Vec<&ValidatorState> =
            self.validators.values().filter(|v| !v.jailed).collect();

        if active_validators.is_empty() {
            return None;
        }

        // Weighted random selection based on stake
        let total_stake: u64 = active_validators.iter().map(|v| v.stake).sum();
        let mut random_value = self.get_random_seed() % total_stake;

        for validator in &active_validators {
            if random_value < validator.stake {
                return Some(validator.address.clone());
            }
            random_value -= validator.stake;
        }

        // Fallback to first validator
        active_validators.first().map(|v| v.address.clone())
    }

    /// Start a new consensus round
    pub fn start_round(&mut self, height: u64, proposer: String) -> ConsensusRound {
        let round = ConsensusRound {
            height,
            round: 0,
            proposer: proposer.clone(),
            prevotes: HashMap::new(),
            precommits: HashMap::new(),
            started_at: self.current_timestamp(),
        };

        self.current_round = Some(round.clone());
        round
    }

    /// Record a prevote from a validator
    pub fn record_prevote(&mut self, validator: &str, vote: bool) -> Result<(), String> {
        if let Some(ref mut round) = self.current_round {
            round.prevotes.insert(validator.to_string(), vote);
            Ok(())
        } else {
            Err("No active consensus round".to_string())
        }
    }

    /// Record a precommit from a validator
    pub fn record_precommit(&mut self, validator: &str, vote: bool) -> Result<(), String> {
        if let Some(ref mut round) = self.current_round {
            round.precommits.insert(validator.to_string(), vote);
            Ok(())
        } else {
            Err("No active consensus round".to_string())
        }
    }

    /// Check if consensus is reached (2/3+ majority)
    pub fn is_consensus_reached(&self) -> bool {
        if let Some(ref round) = self.current_round {
            let active_count = self.validators.values().filter(|v| !v.jailed).count();
            let required = (active_count as f64 * 2.0 / 3.0).ceil() as usize;

            let positive_precommits = round.precommits.values().filter(|&&v| v).count();
            positive_precommits >= required
        } else {
            false
        }
    }

    /// Finalize block after consensus
    pub fn finalize_block(&mut self) -> Result<(), String> {
        if !self.is_consensus_reached() {
            return Err("Consensus not reached".to_string());
        }

        if let Some(round) = self.current_round.take() {
            // Update proposer stats
            if let Some(validator) = self.validators.get_mut(&round.proposer) {
                validator.total_proposals += 1;
                validator.last_proposal = round.height;
            }

            // Update validation stats for all voters
            for validator_addr in round.precommits.keys() {
                if let Some(validator) = self.validators.get_mut(validator_addr) {
                    validator.total_validations += 1;
                }
            }

            self.height = round.height;

            tracing::info!("Block {} finalized by consensus", round.height);
            Ok(())
        } else {
            Err("No active consensus round".to_string())
        }
    }

    /// Check if a block at the given height is finalized
    /// A block is finalized when finality_blocks confirmations have passed
    pub fn is_block_finalized(&self, block_height: u64) -> bool {
        let finalized_height = self.height.saturating_sub(self.finality_blocks as u64);
        block_height <= finalized_height
    }

    /// Slash a validator for misbehavior
    pub fn slash_validator(&mut self, address: &str, reason: &str) -> Result<u64, String> {
        if let Some(validator) = self.validators.get_mut(address) {
            let slash_amount = (validator.stake as f64 * self.slash_factor) as u64;
            validator.stake = validator.stake.saturating_sub(slash_amount);
            validator.jailed = true;

            tracing::warn!(
                "Validator {} slashed: {} (reason: {}, new_stake: {})",
                address,
                slash_amount,
                reason,
                validator.stake
            );

            Ok(slash_amount)
        } else {
            Err(format!("Validator {} not found", address))
        }
    }

    /// Unjail a validator
    pub fn unjail_validator(&mut self, address: &str) -> Result<(), String> {
        if let Some(validator) = self.validators.get_mut(address) {
            if validator.stake >= self.min_stake {
                validator.jailed = false;
                tracing::info!("Validator {} unjailed", address);
                Ok(())
            } else {
                Err(format!(
                    "Validator {} has insufficient stake to unjail",
                    address
                ))
            }
        } else {
            Err(format!("Validator {} not found", address))
        }
    }

    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    /// Get active validator count
    pub fn active_validator_count(&self) -> usize {
        self.validators.values().filter(|v| !v.jailed).count()
    }

    /// Get total stake
    pub fn total_stake(&self) -> u64 {
        self.validators.values().map(|v| v.stake).sum()
    }

    /// Calculate rewards for a validator
    pub fn calculate_rewards(&self, address: &str, blocks_validated: u64) -> f64 {
        if let Some(validator) = self.validators.get(address) {
            let base_reward = blocks_validated as f64 * 0.01; // 0.01 NOMAD per block
            let mobile_multiplier = if validator.is_mobile {
                self.mobile_boost
            } else {
                1.0
            };
            base_reward * mobile_multiplier
        } else {
            0.0
        }
    }

    /// Get current timestamp
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Get random seed for proposer selection
    fn get_random_seed(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        let timestamp = self.current_timestamp();
        timestamp.hash(&mut hasher);
        self.height.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_engine() -> ConsensusEngine {
        let mut engine = ConsensusEngine::new(100, 1.5);

        // Register validators
        engine
            .register_validator("validator1".to_string(), 1000, false)
            .unwrap();
        engine
            .register_validator("validator2".to_string(), 500, true)
            .unwrap();
        engine
            .register_validator("validator3".to_string(), 2000, false)
            .unwrap();

        engine
    }

    #[test]
    fn test_register_validator() {
        let mut engine = ConsensusEngine::new(100, 1.5);

        // Valid registration
        assert!(engine
            .register_validator("val1".to_string(), 1000, false)
            .is_ok());

        // Duplicate registration
        assert!(engine
            .register_validator("val1".to_string(), 1000, false)
            .is_err());

        // Insufficient stake
        assert!(engine
            .register_validator("val2".to_string(), 50, false)
            .is_err());

        assert_eq!(engine.validator_count(), 1);
    }

    #[test]
    fn test_mobile_boost() {
        let mut engine = ConsensusEngine::new(100, 1.5);

        engine
            .register_validator("desktop_val".to_string(), 1000, false)
            .unwrap();
        engine
            .register_validator("mobile_val".to_string(), 1000, true)
            .unwrap();

        // Mobile validator should have 1.5x effective stake
        let desktop = engine.validators.get("desktop_val").unwrap();
        let mobile = engine.validators.get("mobile_val").unwrap();

        assert_eq!(desktop.stake, 1000);
        assert_eq!(mobile.stake, 1500); // 1000 * 1.5
    }

    #[test]
    fn test_select_proposer() {
        let engine = create_test_engine();
        let proposer = engine.select_proposer();
        assert!(proposer.is_some());

        // Proposer should be one of the registered validators
        let proposer = proposer.unwrap();
        assert!(engine.validators.contains_key(&proposer));
    }

    #[test]
    fn test_consensus_round() {
        let mut engine = create_test_engine();

        let proposer = engine.select_proposer().unwrap();
        let round = engine.start_round(1, proposer);

        assert_eq!(round.height, 1);
        assert_eq!(round.prevotes.len(), 0);
        assert_eq!(round.precommits.len(), 0);
    }

    #[test]
    fn test_consensus_reached() {
        let mut engine = create_test_engine();

        let proposer = engine.select_proposer().unwrap();
        engine.start_round(1, proposer);

        // Need 2/3+ of 3 validators = 2 precommits
        engine.record_precommit("validator1", true).unwrap();
        assert!(!engine.is_consensus_reached());

        engine.record_precommit("validator2", true).unwrap();
        assert!(engine.is_consensus_reached());
    }

    #[test]
    fn test_finalize_block() {
        let mut engine = create_test_engine();

        let proposer = engine.select_proposer().unwrap();
        engine.start_round(1, proposer);

        engine.record_precommit("validator1", true).unwrap();
        engine.record_precommit("validator2", true).unwrap();

        assert!(engine.finalize_block().is_ok());
        assert_eq!(engine.height, 1);
        assert!(engine.current_round.is_none());
    }

    #[test]
    fn test_slash_validator() {
        let mut engine = create_test_engine();

        let initial_stake = engine.validators.get("validator1").unwrap().stake;
        let slash_amount = engine.slash_validator("validator1", "downtime").unwrap();

        let validator = engine.validators.get("validator1").unwrap();
        assert!(validator.jailed);
        assert_eq!(validator.stake, initial_stake - slash_amount);
        assert_eq!(slash_amount, (initial_stake as f64 * 0.01) as u64);
    }

    #[test]
    fn test_calculate_rewards() {
        let engine = create_test_engine();

        let desktop_rewards = engine.calculate_rewards("validator1", 100);
        let mobile_rewards = engine.calculate_rewards("validator2", 100);

        // Mobile gets 1.5x rewards
        assert!((mobile_rewards - desktop_rewards * 1.5).abs() < 0.001);
    }
}
