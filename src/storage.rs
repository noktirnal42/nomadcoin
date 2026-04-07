use rocksdb::{Options, DB};
use tracing::info;

use crate::blockchain::Blockchain;
use crate::consensus::ConsensusEngine;
use crate::types::{Block, Transaction};

/// Database storage for blockchain state
pub struct Storage {
    pub db: DB,
}

impl Storage {
    /// Open or create database at given path
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path)?;
        info!("Database opened at: {}", path);

        Ok(Storage { db })
    }

    /// Save block to database
    pub fn save_block(&self, height: u64, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("block:{}", height);
        let value = serde_json::to_vec(block)?;
        self.db.put(key, value)?;
        Ok(())
    }

    /// Load block from database by height
    pub fn load_block(&self, height: u64) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let key = format!("block:{}", height);
        match self.db.get(key)? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    /// Get latest block height
    pub fn get_latest_height(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match self.db.get("meta:latest_height")? {
            Some(data) => {
                let height_str = String::from_utf8(data)?;
                Ok(height_str.parse::<u64>().unwrap_or(0))
            }
            None => Ok(0),
        }
    }

    /// Update latest block height
    pub fn set_latest_height(&self, height: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.db.put("meta:latest_height", height.to_string())?;
        Ok(())
    }

    /// Save transaction to database
    pub fn save_transaction(
        &self,
        txid: &str,
        tx: &Transaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("tx:{}", txid);
        let value = serde_json::to_vec(tx)?;
        self.db.put(key, value)?;
        Ok(())
    }

    /// Load transaction from database
    pub fn load_transaction(
        &self,
        txid: &str,
    ) -> Result<Option<Transaction>, Box<dyn std::error::Error>> {
        let key = format!("tx:{}", txid);
        match self.db.get(key)? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    /// Save balance to database
    pub fn save_balance(
        &self,
        address: &str,
        balance: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("balance:{}", address);
        self.db.put(key, balance.to_string())?;
        Ok(())
    }

    /// Load balance from database
    pub fn load_balance(&self, address: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let key = format!("balance:{}", address);
        match self.db.get(key)? {
            Some(data) => {
                let balance_str = String::from_utf8(data)?;
                Ok(balance_str.parse::<f64>().unwrap_or(0.0))
            }
            None => Ok(0.0),
        }
    }

    /// Save consensus state
    pub fn save_consensus(
        &self,
        engine: &ConsensusEngine,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_vec(&engine.validators)?;
        self.db.put("consensus:validators", data)?;
        self.db.put("consensus:height", engine.height.to_string())?;
        Ok(())
    }

    /// Load consensus state
    pub fn load_consensus(
        &self,
        min_stake: u64,
        mobile_boost: f64,
    ) -> Result<ConsensusEngine, Box<dyn std::error::Error>> {
        let mut engine = ConsensusEngine::new(min_stake, mobile_boost);

        if let Some(data) = self.db.get("consensus:validators")? {
            engine.validators = serde_json::from_slice(&data)?;
        }

        if let Some(data) = self.db.get("consensus:height")? {
            let height_str = String::from_utf8(data)?;
            engine.height = height_str.parse::<u64>().unwrap_or(0);
        }

        Ok(engine)
    }

    /// Save blockchain state
    pub fn save_blockchain(
        &self,
        blockchain: &Blockchain,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Save balances
        for (address, balance) in &blockchain.state.balances {
            self.save_balance(address, *balance)?;
        }

        // Save latest height
        self.set_latest_height(blockchain.height())?;

        // Save latest block
        if let Some(block) = blockchain.state.blocks.last() {
            self.save_block(blockchain.height(), block)?;
        }

        Ok(())
    }

    /// Load blockchain from database
    pub fn load_blockchain(&self) -> Result<Option<Blockchain>, Box<dyn std::error::Error>> {
        let latest_height = self.get_latest_height()?;
        if latest_height == 0 {
            return Ok(None);
        }

        // Load latest block to restore blockchain state
        let mut blockchain = Blockchain::new();

        // Load all blocks sequentially
        for h in 1..=latest_height {
            if let Some(block) = self.load_block(h)? {
                blockchain.state.blocks.push(block);
            }
        }

        blockchain.height = latest_height;

        // Reload balances
        // In production, iterate over all balance keys
        // For now, balances are reconstructed from blocks

        Ok(Some(blockchain))
    }

    /// Get database statistics
    pub fn get_stats(&self) -> Result<DbStats, Box<dyn std::error::Error>> {
        let latest_height = self.get_latest_height()?;

        // Count transactions by scanning keys
        let mut tx_count = 0;
        let mut balance_count = 0;

        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (key, _) = item?;
            let key_str = String::from_utf8_lossy(&key);
            if key_str.starts_with("tx:") {
                tx_count += 1;
            } else if key_str.starts_with("balance:") {
                balance_count += 1;
            }
        }

        Ok(DbStats {
            latest_height,
            tx_count,
            balance_count,
        })
    }
}

/// Database statistics
#[derive(Debug)]
pub struct DbStats {
    pub latest_height: u64,
    pub tx_count: u64,
    pub balance_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_temp_db() -> (Storage, String) {
        let path = format!(
            "/tmp/nomadcoin_test_db_{}_{}",
            std::process::id(),
            rand::random::<u64>()
        );
        let _ = fs::remove_dir_all(&path);
        let storage = Storage::new(&path).unwrap();
        (storage, path)
    }

    #[test]
    fn test_create_and_open_db() {
        let (storage, path) = create_temp_db();
        assert!(storage.db.get("test").is_ok());

        // Cleanup
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn test_save_and_load_block() {
        let (storage, path) = create_temp_db();

        let mut blockchain = Blockchain::new();
        blockchain.create_genesis(
            10_000_000.0,
            "nomad1community0000000000000000000000000".to_string(),
        );

        let block = blockchain.state.blocks.last().unwrap();
        storage.save_block(1, block).unwrap();

        let loaded = storage.load_block(1).unwrap();
        assert!(loaded.is_some());

        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn test_save_and_load_transaction() {
        let (storage, path) = create_temp_db();

        let tx = Transaction {
            version: 1,
            txid: "test_tx_123".to_string(),
            inputs: vec![],
            outputs: vec![],
            fee: 0.001,
            timestamp: 1234567890,
            memo: Some("test".to_string()),
        };

        storage.save_transaction(&tx.txid, &tx).unwrap();
        let loaded = storage.load_transaction("test_tx_123").unwrap();

        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.txid, "test_tx_123");
        assert_eq!(loaded.fee, 0.001);

        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn test_save_and_load_balance() {
        let (storage, path) = create_temp_db();

        storage.save_balance("nomad1test", 1000.0).unwrap();
        let balance = storage.load_balance("nomad1test").unwrap();

        assert_eq!(balance, 1000.0);

        // Non-existent address should return 0
        let balance = storage.load_balance("nomad1nonexistent").unwrap();
        assert_eq!(balance, 0.0);

        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn test_latest_height() {
        let (storage, path) = create_temp_db();

        assert_eq!(storage.get_latest_height().unwrap(), 0);

        storage.set_latest_height(100).unwrap();
        assert_eq!(storage.get_latest_height().unwrap(), 100);

        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn test_db_stats() {
        let (storage, path) = create_temp_db();

        storage.save_balance("nomad1test", 100.0).unwrap();
        storage.save_balance("nomad1test2", 200.0).unwrap();

        let tx = Transaction {
            version: 1,
            txid: "tx1".to_string(),
            inputs: vec![],
            outputs: vec![],
            fee: 0.001,
            timestamp: 1234567890,
            memo: None,
        };
        storage.save_transaction(&tx.txid, &tx).unwrap();

        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.balance_count, 2);
        assert_eq!(stats.tx_count, 1);
        assert_eq!(stats.latest_height, 0);

        let _ = fs::remove_dir_all(&path);
    }
}
