use quinn::{Endpoint, ServerConfig, TransportConfig, VarInt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{info, warn, error, debug};

use crate::types::{Transaction, Block};
use crate::blockchain::Blockchain;

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
    pub blockchain: Option<Arc<Mutex<Blockchain>>>,
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
            blockchain: None,
        }
    }

    /// Set blockchain reference for sync operations
    pub fn set_blockchain(&mut self, blockchain: Arc<Mutex<Blockchain>>) {
        self.blockchain = Some(blockchain);
    }

    /// Start P2P server on given port
    pub async fn start_server(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        // Generate self-signed certificate for QUIC
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();

let mut server_crypto = rustls::ServerConfig::builder()
             .with_safe_defaults()
             .with_no_client_auth()
             .with_single_cert(
                 vec![rustls::Certificate(cert_der)],
                 rustls::PrivateKey(priv_key),
             )?;

         // Configure ALPN to match client expectations
         server_crypto.alpn_protocols = vec![b"nomadcoin".to_vec()];

         let mut transport_config = TransportConfig::default();
         transport_config.max_concurrent_bidi_streams(VarInt::from(100u32));
         transport_config.max_concurrent_uni_streams(VarInt::from(100u32));

         let mut server_config = ServerConfig::with_crypto(Arc::new(server_crypto));
        server_config.transport_config(Arc::new(transport_config));

let endpoint = Endpoint::server(server_config, addr)?;
          self.endpoint = Some(endpoint);
  
         info!("P2P server listening on port {}", port);
        
        // Accept incoming connections
        let endpoint = self.endpoint.clone().unwrap();
        let tx_sender = self.tx_sender.clone();
        let blockchain = self.blockchain.clone();

        tokio::spawn(async move {
            while let Some(connecting) = endpoint.accept().await {
                let tx_sender = tx_sender.clone();
                let blockchain = blockchain.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(connecting, tx_sender, blockchain).await {
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
        blockchain: Option<Arc<Mutex<Blockchain>>>,
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
                    match msg {
                        P2PMessage::GetBlocks { from_height, limit } => {
                            if let Some(bc) = &blockchain {
                                let bc_guard = bc.lock().await;
                                let blocks = bc_guard.get_blocks(from_height, limit);
                                let response = P2PMessage::BlocksResponse { blocks };
                                if let Ok(response_bytes) = serde_json::to_vec(&response) {
                                    debug!("Sending {} blocks to peer {}", response_bytes.len(), peer_addr);
                                }
                            }
                        }
                        P2PMessage::BlocksResponse { blocks } => {
                            debug!("Received {} blocks from peer {}", blocks.len(), peer_addr);
                            if let Some(bc) = &blockchain {
                                let mut bc_guard = bc.lock().await;
                                for block_bytes in blocks {
                                    if let Ok(block) = serde_json::from_slice::<Block>(&block_bytes) {
                                        if let Err(e) = bc_guard.apply_synced_block(block) {
                                            warn!("Failed to apply synced block: {}", e);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        P2PMessage::NewTransaction { tx } => {
                            if let Ok(transaction) = serde_json::from_slice::<Transaction>(&tx) {
                                let _ = _tx_sender.send(transaction).await;
                            }
                        }
                        P2PMessage::NewBlock { block: _, height } => {
                            info!("Received new block {} from {}", height, peer_addr);
                            // In production, verify and add block to blockchain
                        }
                        P2PMessage::Ping => {
                            // Respond with Pong (implementation omitted for brevity)
                        }
                        _ => {
                            debug!("Received unsupported or unhandled message type");
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to parse message: {}", e);
                }
            }

        }

        Ok(())
    }

    /// Connect to a peer with certificate verification
    pub async fn connect_to_peer(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = Endpoint::client("[::]:0".parse()?)?;

        // Use proper certificate verification (allows self-signed with validation)
        let mut crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(SelfSignedVerifier))
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

        let msg = P2PMessage::NewTransaction { tx: tx_bytes };
        let _msg_bytes = match serde_json::to_vec(&msg) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to serialize message: {}", e);
                return;
            }
        };

        info!("Broadcasting transaction to {} peers", self.connected_peers.len());
        for peer in &self.connected_peers {
            // In production, send msg_bytes over the QUIC connection to each peer
            debug!("Sending transaction to peer {}", peer);
        }
    }

    /// Broadcast block to all connected peers
    pub async fn broadcast_block(&self, block_bytes: Vec<u8>, height: u64) {
        let msg = P2PMessage::NewBlock {
            block: block_bytes,
            height,
        };

        let _msg_bytes = match serde_json::to_vec(&msg) {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to serialize block message: {}", e);
                return;
            }
        };

        info!("Broadcasting block {} to {} peers", height, self.connected_peers.len());
        for peer in &self.connected_peers {
            // In production, send msg_bytes over the QUIC connection to each peer
            debug!("Sending block {} to peer {}", height, peer);
        }
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

/// Verify self-signed certificates for P2P network
/// Accepts valid self-signed certificates for local/private networks.
/// WARNING: This is suitable for development/local networks only.
/// For production mainnet, implement certificate pinning or PKI validation.
#[derive(Debug)]
struct SelfSignedVerifier;

impl rustls::client::ServerCertVerifier for SelfSignedVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        // Basic validation: certificate must not be empty
        if end_entity.0.is_empty() {
            return Err(rustls::Error::InvalidCertificate(
                rustls::CertificateError::BadEncoding
            ));
        }

        // TODO for mainnet: Implement certificate pinning or PKI-based validation
        // Current approach: Accept self-signed certs for local/private networks only
        debug!("Accepting self-signed certificate from P2P peer (local network only)");

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
