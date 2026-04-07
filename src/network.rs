use quinn::{Endpoint, ServerConfig, TransportConfig, VarInt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn, error, debug};
use rand::Rng;

use crate::types::Transaction;

/// P2P message types for node communication
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum P2PMessage {
    /// Request blocks from peer
    GetBlocks { from_height: u64, limit: u32 },
    /// Response with blocks
    BlocksResponse { blocks: Vec<Vec<u8>> },
    /// Broadcast new transaction
    NewTransaction { tx: Vec<u8> },
    /// Broadcast new block
    NewBlock { block: Vec<u8>, height: u64 },
    /// Peer discovery request
    PeerDiscovery,
    /// Peer discovery response
    PeerDiscoveryResponse { peers: Vec<String> },
    /// Ping for keepalive
    Ping,
    /// Pong response
    Pong,
    /// Request peer list
    GetPeers,
    /// Peer list response
    PeersResponse { peers: Vec<PeerInfo> },
}

/// Peer information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerInfo {
    pub address: String,
    pub height: u64,
    pub version: String,
}

/// P2P network manager
pub struct P2PNetwork {
    pub endpoint: Option<Endpoint>,
    pub known_peers: Vec<String>,
    pub connected_peers: Vec<String>,
    pub tx_sender: mpsc::Sender<Transaction>,
    pub version: String,
    pub height: u64,
}

impl P2PNetwork {
    /// Create new P2P network instance
    pub fn new(tx_sender: mpsc::Sender<Transaction>) -> Self {
        P2PNetwork {
            endpoint: None,
            known_peers: Vec::new(),
            connected_peers: Vec::new(),
            tx_sender,
            version: env!("CARGO_PKG_VERSION").to_string(),
            height: 0,
        }
    }

    /// Start P2P server on given port
    pub async fn start_server(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        // Generate self-signed certificate for QUIC
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();

let server_crypto = rustls::ServerConfig::builder()
             .with_safe_defaults()
             .with_no_client_auth()
             .with_single_cert(
                 vec![rustls::Certificate(cert_der)],
                 rustls::PrivateKey(priv_key),
             )?;

         let mut transport_config = TransportConfig::default();
         transport_config.max_concurrent_bidi_streams(VarInt::from(100u32));
         transport_config.max_concurrent_uni_streams(VarInt::from(100u32));

         let mut server_config = ServerConfig::with_crypto(Arc::new(server_crypto));
        server_config.transport_config(Arc::new(transport_config));

let endpoint = Endpoint::server(server_config, addr)?;
         self.endpoint = Some(endpoint);
 
 let local_addr = self.endpoint.as_ref().unwrap().local_addr()?;
         let _peer_id = format!("{:x}", rand::random::<u64>());
         info!("P2P server listening on port {}", port);
         info!("Peer ID: {}", _peer_id);
        
        // Accept incoming connections
        let endpoint = self.endpoint.clone().unwrap();
        let tx_sender = self.tx_sender.clone();

        tokio::spawn(async move {
            while let Some(connecting) = endpoint.accept().await {
                let tx_sender = tx_sender.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(connecting, tx_sender).await {
                        error!("Connection error: {}", e);
                    }
                });
            }
        });

        Ok(())
    }

    /// Handle incoming connection
async fn handle_connection(
        connecting: quinn::Connecting,
        _tx_sender: mpsc::Sender<Transaction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let connection = connecting.await?;
        let peer_addr = connection.remote_address();

        info!("New peer connected: {}", peer_addr);

        loop {
            let mut stream: quinn::RecvStream = match connection.accept_uni().await {
                Ok(s) => s,
                Err(_) => break,
            };

            // Read message
let mut buf = Vec::new();
             let mut read_buf = [0u8; 65536];
             loop {
                 match stream.read(&mut read_buf).await {
                     Ok(None) => break,
                     Ok(Some(n)) => {
                         buf.extend_from_slice(&read_buf[..n]);
                     }
                     Err(e) => {
                         warn!("Failed to read from stream: {}", e);
                         break;
                     }
                 }
             }

            if buf.is_empty() {
                continue;
            }

            // Parse and handle message
            match serde_json::from_slice::<P2PMessage>(&buf) {
                Ok(msg) => {
                    debug!("Received message from {}: {:?}", peer_addr, msg);
                    // Handle message based on type
                }
                Err(e) => {
                    warn!("Failed to parse message: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Connect to a peer
    pub async fn connect_to_peer(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = Endpoint::client("[::]:0".parse()?)?;

// Skip TLS verification for development
         let mut crypto = rustls::ClientConfig::builder()
             .with_safe_defaults()
             .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
             .with_no_client_auth();

         crypto.alpn_protocols = vec![b"nomadcoin".to_vec()];

         let client_config = quinn::ClientConfig::new(Arc::new(crypto));

        let peer_addr: SocketAddr = addr.parse()?;
        let connection = endpoint.connect_with(client_config, peer_addr, "localhost")?;
        let _conn = connection.await?;

        info!("Connected to peer: {}", addr);
        self.connected_peers.push(addr.to_string());

        Ok(())
    }

    /// Broadcast transaction to all connected peers
    pub async fn broadcast_transaction(&self, tx: &Transaction) {
        let tx_bytes = match serde_json::to_vec(tx) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to serialize transaction: {}", e);
                return;
            }
        };

        let _msg = P2PMessage::NewTransaction { tx: tx_bytes };
        let _msg_bytes = match serde_json::to_vec(&_msg) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to serialize message: {}", e);
                return;
            }
        };

        info!("Broadcasting transaction to {} peers", self.connected_peers.len());
        // In production, send to all connected peers
    }

    /// Broadcast block to all connected peers
    pub async fn broadcast_block(&self, block_bytes: Vec<u8>, height: u64) {
        let _msg = P2PMessage::NewBlock {
            block: block_bytes,
            height,
        };

        info!("Broadcasting block {} to {} peers", height, self.connected_peers.len());
        // In production, send to all connected peers
    }

    /// Get peer count
    pub fn peer_count(&self) -> usize {
        self.connected_peers.len()
    }

    /// Update chain height
    pub fn update_height(&mut self, height: u64) {
        self.height = height;
    }
}

/// Skip server certificate verification (for development only)
#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2p_message_serialization() {
        let msg = P2PMessage::Ping;
        let bytes = serde_json::to_vec(&msg).unwrap();
        let parsed: P2PMessage = serde_json::from_slice(&bytes).unwrap();
        assert!(matches!(parsed, P2PMessage::Ping));
    }

    #[test]
    fn test_peer_info_serialization() {
        let peer = PeerInfo {
            address: "192.168.1.1:9333".to_string(),
            height: 100,
            version: "0.1.0".to_string(),
        };

        let bytes = serde_json::to_vec(&peer).unwrap();
        let parsed: PeerInfo = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(peer.address, parsed.address);
        assert_eq!(peer.height, parsed.height);
    }

    #[test]
    fn test_new_network() {
        let (tx, _rx) = mpsc::channel(100);
        let network = P2PNetwork::new(tx);
        assert_eq!(network.peer_count(), 0);
        assert_eq!(network.height, 0);
    }
}
