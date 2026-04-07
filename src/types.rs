use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Transaction input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxInput {
    pub txid: String,
    pub index: u32,
    pub amount: f64,
    pub signature: String,
}

/// Transaction output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxOutput {
    pub address: String,
    pub amount: f64,
    pub stealth: bool,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub version: u8,
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub fee: f64,
    pub timestamp: u64,
    pub memo: Option<String>,
    // Replay protection fields
    pub nonce: u64,                    // Sequential counter per sender address (prevents replay)
    pub chain_id: String,              // Chain identifier (prevents cross-chain replay)
    pub sequence_number: u64,          // Block height at which this tx becomes valid (0 = valid immediately)
}

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockHeader {
    pub version: u8,
    pub previous_block: String,
    pub timestamp: u64,
    pub validator_set_hash: String,
    pub state_merkle_root: String,
    pub transaction_count: u32,
    pub cumulative_difficulty: u128,
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub validator_signatures: Vec<String>,
}

/// Wallet address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletAddress {
    pub public_key: String,
    pub private_key: String,
    pub address: String,
}

/// Blockchain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    pub blocks: Vec<Block>,
    pub balances: HashMap<String, f64>,
    pub mempool: Vec<Transaction>,
    pub validators: HashMap<String, f64>,
}

/// Validator info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub stake: f64,
    pub is_mobile: bool,
    pub last_active: u64,
    pub validations: u64,
}

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
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            chain_id: "nomadcoin".to_string(),
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
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {{ txid: {}, fee: {}, timestamp: {} }}",
            self.txid, self.fee, self.timestamp
        )
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block {{ height: {}, txs: {}, timestamp: {} }}",
            self.header.cumulative_difficulty, self.header.transaction_count, self.header.timestamp
        )
    }
}
