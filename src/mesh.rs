use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mesh message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeshMessageType {
    TransactionRequest,
    TransactionResponse,
    StateSync,
    PeerDiscovery,
    ValidatorMessage,
}

/// Mesh message for offline transaction relay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshMessage {
    pub msg_type: MeshMessageType,
    pub sender: String,
    pub receiver: Option<String>,
    pub payload: Vec<u8>,
    pub hop_limit: u8,
    pub timestamp: u64,
    pub message_id: String,
}

/// Mesh node in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    pub id: String,
    pub address: String,
    pub last_seen: u64,
    pub connection_type: ConnectionType,
    pub trust_score: f64,
}

/// Connection types supported by mesh network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Bluetooth,
    WiFiDirect,
    LoRa,
    LTE,
}

/// Offline transaction pending mesh delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineTransaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: u64,
    pub signature: String,
    pub valid_after: u64,
    pub hop_limit: u8,
    pub delivered: bool,
}

/// Mesh network manager
pub struct MeshNetwork {
    pub nodes: HashMap<String, MeshNode>,
    pub pending_transactions: HashMap<String, OfflineTransaction>,
    pub message_queue: Vec<MeshMessage>,
    pub max_hops: u8,
}

impl MeshNetwork {
    /// Create new mesh network
    pub fn new(max_hops: u8) -> Self {
        MeshNetwork {
            nodes: HashMap::new(),
            pending_transactions: HashMap::new(),
            message_queue: Vec::new(),
            max_hops,
        }
    }

    /// Register a mesh node
    pub fn register_node(&mut self, node: MeshNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Remove stale nodes (not seen in last N seconds)
    pub fn prune_stale_nodes(&mut self, max_age_seconds: u64) {
        let now = chrono::Utc::now().timestamp() as u64;
        self.nodes
            .retain(|_, node| now.saturating_sub(node.last_seen) < max_age_seconds);
    }

    /// Create offline transaction for mesh delivery
    pub fn create_offline_transaction(
        &mut self,
        from: String,
        to: String,
        amount: f64,
        fee: f64,
        signature: String,
    ) -> OfflineTransaction {
        let now = chrono::Utc::now().timestamp() as u64;
        let tx = OfflineTransaction {
            id: crate::crypto::hash_data(format!("{}:{}:{}", from, to, now).as_bytes()),
            from,
            to,
            amount,
            fee,
            timestamp: now,
            signature,
            valid_after: now,
            hop_limit: self.max_hops,
            delivered: false,
        };

        self.pending_transactions.insert(tx.id.clone(), tx.clone());

        tx
    }

    /// Route message through mesh network
    pub fn route_message(&self, message: &MeshMessage) -> Result<(), String> {
        if message.hop_limit == 0 {
            return Err("Hop limit exceeded".to_string());
        }

        // Simple routing - in production, use proper mesh routing algorithm
        Ok(())
    }

    /// Broadcast message to all known peers
    pub fn broadcast(&mut self, message: MeshMessage) {
        self.message_queue.push(message);
    }

    /// Process pending messages
    pub fn process_messages(&mut self) -> Vec<MeshMessage> {
        let messages = self.message_queue.clone();
        self.message_queue.clear();
        messages
    }

    /// Mark transaction as delivered
    pub fn mark_delivered(&mut self, tx_id: &str) {
        if let Some(tx) = self.pending_transactions.get_mut(tx_id) {
            tx.delivered = true;
        }
    }

    /// Get pending (undelivered) transactions
    pub fn get_pending_transactions(&self) -> Vec<&OfflineTransaction> {
        self.pending_transactions
            .values()
            .filter(|tx| !tx.delivered)
            .collect()
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get pending transaction count
    pub fn pending_transaction_count(&self) -> usize {
        self.pending_transactions
            .values()
            .filter(|tx| !tx.delivered)
            .count()
    }
}

impl Default for MeshNetwork {
    fn default() -> Self {
        Self::new(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mesh_network() {
        let mesh = MeshNetwork::new(5);
        assert_eq!(mesh.node_count(), 0);
        assert_eq!(mesh.pending_transaction_count(), 0);
    }

    #[test]
    fn test_register_node() {
        let mut mesh = MeshNetwork::new(5);
        let node = MeshNode {
            id: "node1".to_string(),
            address: "192.168.1.1".to_string(),
            last_seen: chrono::Utc::now().timestamp() as u64,
            connection_type: ConnectionType::WiFiDirect,
            trust_score: 1.0,
        };

        mesh.register_node(node);
        assert_eq!(mesh.node_count(), 1);
    }

    #[test]
    fn test_create_offline_transaction() {
        let mut mesh = MeshNetwork::new(5);
        let tx = mesh.create_offline_transaction(
            "nomad1alice".to_string(),
            "nomad1bob".to_string(),
            100.0,
            0.001,
            "signature".to_string(),
        );

        assert!(!tx.id.is_empty());
        assert_eq!(tx.from, "nomad1alice");
        assert_eq!(tx.to, "nomad1bob");
        assert_eq!(tx.amount, 100.0);
        assert!(!tx.delivered);
        assert_eq!(mesh.pending_transaction_count(), 1);
    }

    #[test]
    fn test_mark_delivered() {
        let mut mesh = MeshNetwork::new(5);
        let tx = mesh.create_offline_transaction(
            "nomad1alice".to_string(),
            "nomad1bob".to_string(),
            100.0,
            0.001,
            "signature".to_string(),
        );

        mesh.mark_delivered(&tx.id);
        assert_eq!(mesh.pending_transaction_count(), 0);
    }

    #[test]
    fn test_prune_stale_nodes() {
        let mut mesh = MeshNetwork::new(5);
        let now = chrono::Utc::now().timestamp() as u64;

        // Add fresh node
        mesh.register_node(MeshNode {
            id: "fresh".to_string(),
            address: "1.1.1.1".to_string(),
            last_seen: now,
            connection_type: ConnectionType::WiFiDirect,
            trust_score: 1.0,
        });

        // Add stale node
        mesh.register_node(MeshNode {
            id: "stale".to_string(),
            address: "2.2.2.2".to_string(),
            last_seen: now - 3600, // 1 hour ago
            connection_type: ConnectionType::Bluetooth,
            trust_score: 0.5,
        });

        mesh.prune_stale_nodes(1800); // 30 minute max age
        assert_eq!(mesh.node_count(), 1);
        assert!(mesh.nodes.contains_key("fresh"));
        assert!(!mesh.nodes.contains_key("stale"));
    }
}
