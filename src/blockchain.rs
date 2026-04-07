use crate::crypto::{hash_data, verify_signature};
use crate::types::{Block, BlockHeader, ChainState, Transaction, TxOutput};
use std::collections::HashMap;

/// Blockchain implementation
pub struct Blockchain {
    pub state: ChainState,
    pub height: u64,
    /// Track nonces per address for replay protection
    pub address_nonces: HashMap<String, u64>,
}

impl Blockchain {
    /// Create new blockchain instance
    pub fn new() -> Self {
        Blockchain {
            state: ChainState {
                blocks: Vec::new(),
                balances: HashMap::new(),
                mempool: Vec::new(),
                validators: HashMap::new(),
            },
            height: 0,
            address_nonces: HashMap::new(),
        }
    }

    /// Create genesis block and initialize chain
    pub fn create_genesis(&mut self, community_allocation: f64, community_address: String) {
        let genesis_tx = Transaction {
            version: 1,
            txid: "genesis".to_string(),
            inputs: vec![],
            outputs: vec![TxOutput {
                address: community_address.clone(),
                amount: community_allocation,
                stealth: false,
            }],
            fee: 0.0,
            timestamp: 0,
            memo: Some("Genesis block - community allocation".to_string()),
            nonce: 0,
            chain_id: "nomadcoin".to_string(),
            sequence_number: 0,
        };

        let genesis_block = Block {
            header: BlockHeader {
                version: 1,
                previous_block: "0".repeat(64),
                timestamp: 0,
                validator_set_hash: "0".repeat(64),
                state_merkle_root: hash_data(b"genesis"),
                transaction_count: 1,
                cumulative_difficulty: 1,
            },
            transactions: vec![genesis_tx],
            validator_signatures: vec![],
        };

        // Pre-allocate community funds
        self.state
            .balances
            .insert(community_address, community_allocation);

        self.state.blocks.push(genesis_block);
        self.height = 1;

        tracing::info!("Genesis block created with {} NOMAD", community_allocation);
    }

    /// Validate transaction with replay protection
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<(), String> {
        // Validate chain_id matches (prevent cross-chain replays)
        if tx.chain_id != "nomadcoin" {
            return Err(format!(
                "Invalid chain_id: {} (expected nomadcoin)",
                tx.chain_id
            ));
        }

        // Validate sequence_number (transaction becomes valid at specified block height)
        if tx.sequence_number > self.height {
            return Err(format!(
                "Transaction sequence number {} is in the future (current height: {})",
                tx.sequence_number, self.height
            ));
        }

        // Check inputs exist and have sufficient balance
        for input in &tx.inputs {
            let balance = self.state.balances.get(&input.txid).unwrap_or(&0.0);
            if *balance < input.amount {
                return Err(format!(
                    "Insufficient funds for {}: balance {} < amount {}",
                    input.txid, balance, input.amount
                ));
            }
        }

        // Check fee is sufficient
        if tx.fee < 0.001 {
            return Err("Insufficient fee: minimum 0.001 NOMAD".to_string());
        }

        // Validate nonce is sequential (prevent replay attacks)
        // Get sender from first input (the payer)
        if let Some(first_input) = tx.inputs.first() {
            let sender = &first_input.txid;
            let expected_nonce = self.address_nonces.get(sender).copied().unwrap_or(0);
            if tx.nonce != expected_nonce {
                return Err(format!(
                    "Invalid nonce for {}: {} (expected {})",
                    sender, tx.nonce, expected_nonce
                ));
            }
        }

        // Verify signatures
        for input in &tx.inputs {
            let tx_data = format!("{}:{}:{}", input.txid, input.index, input.amount);
            if !verify_signature(&input.txid, tx_data.as_bytes(), &input.signature) {
                return Err(format!("Invalid signature for input {}", input.txid));
            }
        }

        Ok(())
    }

    /// Add transaction to mempool
    pub fn add_to_mempool(&mut self, tx: Transaction) -> Result<(), String> {
        self.validate_transaction(&tx)?;
        self.state.mempool.push(tx);
        tracing::debug!("Transaction added to mempool");
        Ok(())
    }

    /// Create new block from mempool transactions
    pub fn create_block(&mut self, validator: &str) -> Block {
        let previous_block = self
            .state
            .blocks
            .last()
            .map(|b| hash_data(&serde_json::to_vec(&b.header).unwrap()))
            .unwrap_or_else(|| "0".repeat(64));

        let tx_count = self.state.mempool.len() as u32;
        let merkle_root = self.compute_merkle_root();

        let new_block = Block {
            header: BlockHeader {
                version: 1,
                previous_block,
                timestamp: chrono::Utc::now().timestamp() as u64,
                validator_set_hash: hash_data(validator.as_bytes()),
                state_merkle_root: merkle_root,
                transaction_count: tx_count,
                cumulative_difficulty: (self.height + 1) as u128,
            },
            transactions: self.state.mempool.clone(),
            validator_signatures: vec![],
        };

        // Update balances and increment nonces
        for tx in &new_block.transactions {
            for input in &tx.inputs {
                let balance = self.state.balances.entry(input.txid.clone()).or_insert(0.0);
                *balance -= input.amount;
            }
            for output in &tx.outputs {
                let balance = self
                    .state
                    .balances
                    .entry(output.address.clone())
                    .or_insert(0.0);
                *balance += output.amount;
            }
            // Increment nonce for each transaction's sender
            if !tx.inputs.is_empty() {
                let sender = tx.inputs[0].txid.clone();
                self.address_nonces
                    .entry(sender)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }

        // Reward validator
        let validator_reward = 0.01 * tx_count as f64; // 0.01 NOMAD per tx
        let balance = self
            .state
            .balances
            .entry(validator.to_string())
            .or_insert(0.0);
        *balance += validator_reward;

        // Clear mempool
        self.state.mempool.clear();

        // Add block to chain
        self.state.blocks.push(new_block.clone());
        self.height += 1;

        tracing::info!(
            "Block {} created with {} transactions",
            self.height,
            tx_count
        );

        new_block
    }

    /// Compute Merkle root of mempool transactions
    fn compute_merkle_root(&self) -> String {
        if self.state.mempool.is_empty() {
            return hash_data(b"empty");
        }

        let mut hashes: Vec<String> = self
            .state
            .mempool
            .iter()
            .map(|tx| hash_data(&serde_json::to_vec(tx).unwrap()))
            .collect();

        // Build Merkle tree
        while hashes.len() > 1 {
            if hashes.len() % 2 == 1 {
                hashes.push(hashes.last().unwrap().clone());
            }

            let mut new_level = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let combined = format!("{}{}", hashes[i], hashes[i + 1]);
                new_level.push(hash_data(combined.as_bytes()));
            }
            hashes = new_level;
        }

        hashes[0].clone()
    }

    /// Get balance for address
    pub fn get_balance(&self, address: &str) -> f64 {
        *self.state.balances.get(address).unwrap_or(&0.0)
    }

    /// Get block by height
    pub fn get_block(&self, height: u64) -> Option<&Block> {
        if height == 0 || height > self.height {
            return None;
        }
        self.state.blocks.get((height - 1) as usize)
    }

    /// Get chain height
    pub fn height(&self) -> u64 {
        self.height
    }

    /// Get mempool size
    pub fn mempool_size(&self) -> usize {
        self.state.mempool.len()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_genesis() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis(
            10_000_000.0,
            "nomad1community0000000000000000000000000".to_string(),
        );

        assert_eq!(blockchain.height(), 1);
        assert_eq!(blockchain.state.blocks.len(), 1);
        assert_eq!(
            blockchain.get_balance("nomad1community0000000000000000000000000"),
            10_000_000.0
        );
    }

    #[test]
    fn test_create_block() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis(
            10_000_000.0,
            "nomad1community0000000000000000000000000".to_string(),
        );

        let block = blockchain.create_block("validator1");
        assert_eq!(blockchain.height(), 2);
        assert_eq!(block.header.transaction_count, 0);
    }

    #[test]
    fn test_get_balance() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis(
            10_000_000.0,
            "nomad1community0000000000000000000000000".to_string(),
        );

        assert_eq!(
            blockchain.get_balance("nomad1community0000000000000000000000000"),
            10_000_000.0
        );
        assert_eq!(blockchain.get_balance("nonexistent"), 0.0);
    }

    #[test]
    fn test_merkle_root_empty() {
        let blockchain = Blockchain::new();
        let root = blockchain.compute_merkle_root();
        assert_eq!(root, hash_data(b"empty"));
    }
}
