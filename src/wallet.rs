use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::crypto::{generate_address, generate_keypair, sign_data};
use crate::types::{Transaction, TxInput, TxOutput, WalletAddress};

/// Wallet implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub addresses: Vec<WalletAddress>,
    pub transactions: Vec<Transaction>,
    /// Nonce tracking per address for replay protection
    pub nonces: HashMap<String, u64>,
}

impl Wallet {
    /// Create new empty wallet
    pub fn new() -> Self {
        Wallet {
            addresses: Vec::new(),
            transactions: Vec::new(),
            nonces: HashMap::new(),
        }
    }

    /// Create new address and add to wallet
    pub fn create_address(&mut self) -> WalletAddress {
        let (private_key, public_key) = generate_keypair();
        let address = generate_address(&public_key);

        let wallet_address = WalletAddress {
            public_key,
            private_key,
            address: address.clone(),
        };

        self.addresses.push(wallet_address.clone());
        wallet_address
    }

    /// Import address from private key
    pub fn import_address(&mut self, private_key: &str) -> Result<WalletAddress, String> {
        let public_key = crate::crypto::derive_public_key(private_key)?;
        let address = crate::crypto::generate_address(&public_key);

        let wallet_address = WalletAddress {
            public_key,
            private_key: private_key.to_string(),
            address,
        };

        self.addresses.push(wallet_address.clone());
        Ok(wallet_address)
    }

    /// Get address by index
    pub fn get_address(&self, index: usize) -> Option<&WalletAddress> {
        self.addresses.get(index)
    }

    /// Get address by string
    pub fn find_address(&self, address: &str) -> Option<&WalletAddress> {
        self.addresses.iter().find(|a| a.address == address)
    }

    /// Create unsigned transaction
    pub fn create_transaction(
        &self,
        from_address: &str,
        to_address: &str,
        amount: f64,
        fee: f64,
        memo: Option<String>,
    ) -> Result<Transaction, String> {
        let _wallet_addr = self
            .find_address(from_address)
            .ok_or("Address not found in wallet")?;

        // Get current nonce for this address (starts at 0)
        let nonce = self.nonces.get(from_address).copied().unwrap_or(0);

        let tx = Transaction {
            version: 1,
            txid: "pending".to_string(),
            inputs: vec![TxInput {
                txid: from_address.to_string(),
                index: 0,
                amount,
                signature: String::new(),
            }],
            outputs: vec![TxOutput {
                address: to_address.to_string(),
                amount,
                stealth: false,
            }],
            fee,
            timestamp: chrono::Utc::now().timestamp() as u64,
            memo,
            nonce,                         // Replay protection: sequential nonce
            chain_id: "nomadcoin".to_string(), // TODO: Load from config
            sequence_number: 0,            // Valid immediately (no timelock)
        };

        Ok(tx)
    }

    /// Sign transaction with wallet's private key
    pub fn sign_transaction(&self, tx: &mut Transaction, from_address: &str) -> Result<(), String> {
        let wallet_addr = self.find_address(from_address).ok_or("Address not found")?;

        // Sign all inputs
        for input in &mut tx.inputs {
            let tx_data = format!("{}:{}:{}", input.txid, input.index, input.amount);
            input.signature = sign_data(&wallet_addr.private_key, tx_data.as_bytes());
        }

        // Generate transaction ID
        tx.txid = crate::crypto::hash_data(&serde_json::to_vec(tx).unwrap());

        Ok(())
    }

    /// Create and sign transaction in one step
    pub fn send_transaction(
        &mut self,
        from_address: &str,
        to_address: &str,
        amount: f64,
        fee: f64,
        memo: Option<String>,
    ) -> Result<Transaction, String> {
        let mut tx = self.create_transaction(from_address, to_address, amount, fee, memo)?;
        self.sign_transaction(&mut tx, from_address)?;
        self.transactions.push(tx.clone());

        // Increment nonce for this address after successful signing
        self.nonces
            .entry(from_address.to_string())
            .and_modify(|n| *n += 1)
            .or_insert(1);

        Ok(tx)
    }

    /// Get all addresses
    pub fn get_all_addresses(&self) -> Vec<String> {
        self.addresses.iter().map(|a| a.address.clone()).collect()
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Get recent transactions
    pub fn get_recent_transactions(&self, limit: usize) -> Vec<&Transaction> {
        self.transactions.iter().rev().take(limit).collect()
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet() {
        let wallet = Wallet::new();
        assert_eq!(wallet.addresses.len(), 0);
        assert_eq!(wallet.transactions.len(), 0);
    }

    #[test]
    fn test_create_address() {
        let mut wallet = Wallet::new();
        let addr = wallet.create_address();

        assert!(addr.address.starts_with("nomad1"));
        assert_eq!(wallet.addresses.len(), 1);
        assert_eq!(wallet.get_all_addresses().len(), 1);
    }

    #[test]
    fn test_create_and_sign_transaction() {
        let mut wallet = Wallet::new();
        let addr = wallet.create_address();
        let recipient = wallet.create_address();

        let tx = wallet.send_transaction(
            &addr.address,
            &recipient.address,
            100.0,
            0.001,
            Some("test payment".to_string()),
        );

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert_eq!(tx.outputs[0].address, recipient.address);
        assert_eq!(tx.outputs[0].amount, 100.0);
        assert_eq!(tx.fee, 0.001);
        assert!(!tx.inputs[0].signature.is_empty());
        assert_ne!(tx.txid, "pending");
    }

    #[test]
    fn test_get_recent_transactions() {
        let mut wallet = Wallet::new();
        let addr1 = wallet.create_address();
        let addr2 = wallet.create_address();
        let addr3 = wallet.create_address();

        wallet
            .send_transaction(&addr1.address, &addr2.address, 10.0, 0.001, None)
            .unwrap();
        wallet
            .send_transaction(&addr2.address, &addr3.address, 20.0, 0.001, None)
            .unwrap();
        wallet
            .send_transaction(&addr3.address, &addr1.address, 30.0, 0.001, None)
            .unwrap();

        let recent = wallet.get_recent_transactions(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].outputs[0].amount, 30.0); // Most recent first
    }
}
