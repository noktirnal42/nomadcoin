# Phase 4: Bidirectional P2P Communication

## Problem Statement

Current P2P layer:
- ✅ QUIC configured for bidirectional streams (max 100)
- ❌ Only accepts unidirectional streams (`accept_uni()`)
- ❌ `connect_to_peer()` establishes connection but doesn't keep it
- ❌ No way to send messages to specific peers
- ❌ Sync can't request blocks from peers

## Solution: Connection Pool + Bidirectional Messaging

### Architecture

```
P2PNetwork
├─ endpoint: Server listening for incoming connections
├─ peer_connections: HashMap<String, Connection>  [NEW]
│  └─ Stores active outbound connections to peers
│
├─ handle_connection()
│  ├─ Accept incoming uni streams (broadcasts)
│  └─ Accept incoming bidi streams (requests) [NEW]
│
├─ connect_to_peer() [REFACTOR]
│  └─ Store connection in peer_connections
│
├─ send_message_to_peer() [NEW]
│  └─ Use connection from pool to send bidi message
│
└─ send_sync_request() [NEW]
   ├─ Send GetBlocks on bidi stream
   └─ Wait for BlocksResponse
```

## Implementation Steps

### Step 1: Add Connection Pool to P2PNetwork

```rust
pub struct P2PNetwork {
    // ... existing fields ...
    peer_connections: HashMap<String, quinn::Connection>,
}

impl P2PNetwork {
    pub fn new(...) -> Self {
        P2PNetwork {
            // ...
            peer_connections: HashMap::new(),
        }
    }
}
```

### Step 2: Refactor connect_to_peer()

Current (discards connection):
```rust
pub async fn connect_to_peer(&mut self, addr: &str) -> Result<...> {
    let connection = endpoint.connect_with(...)?;
    let _conn = connection.await?;  // <- Unused!
    self.connected_peers.push(addr.to_string());
}
```

New (stores connection):
```rust
pub async fn connect_to_peer(&mut self, addr: &str) -> Result<...> {
    let endpoint = Endpoint::client("[::]:0".parse()?)?;
    let connection = endpoint.connect_with(...)?;
    let conn = connection.await?;
    
    self.peer_connections.insert(addr.to_string(), conn);
    self.connected_peers.push(addr.to_string());
}
```

### Step 3: Add send_message_to_peer()

```rust
pub async fn send_message_to_peer(
    &self,
    peer_addr: &str,
    msg: &P2PMessage,
) -> Result<Vec<u8>, String> {
    let connection = self.peer_connections
        .get(peer_addr)
        .ok_or("Peer not connected")?;
    
    // Open bidirectional stream
    let (mut send, mut recv) = connection
        .open_bi()
        .await
        .map_err(|e| e.to_string())?;
    
    // Send request
    let msg_bytes = serde_json::to_vec(msg)
        .map_err(|e| e.to_string())?;
    send.write_all(&msg_bytes)
        .await
        .map_err(|e| e.to_string())?;
    send.finish()
        .await
        .map_err(|e| e.to_string())?;
    
    // Read response
    let mut buf = Vec::new();
    let mut read_buf = [0u8; 65536];
    loop {
        match recv.read(&mut read_buf).await {
            Ok(None) => break,
            Ok(Some(n)) => buf.extend_from_slice(&read_buf[..n]),
            Err(e) => return Err(e.to_string()),
        }
    }
    
    Ok(buf)
}
```

### Step 4: Update handle_connection() for Bidirectional

```rust
async fn handle_connection(
    connecting: quinn::Connecting,
    _tx_sender: mpsc::Sender<Transaction>,
    blockchain: Option<Arc<Mutex<Blockchain>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = connecting.await?;
    
    loop {
        // Handle both uni and bidi streams
        tokio::select! {
            // Unidirectional (broadcasts)
            Some(stream) = async {
                connection.accept_uni().await.ok()
            } => {
                // Existing broadcast handler
            }
            
            // Bidirectional (request-response)
            Some(stream) = async {
                connection.accept_bi().await.ok()
            } => {
                // New request-response handler
                let (send, recv) = stream;
                handle_request_response(send, recv, blockchain.clone()).await?;
            }
        }
    }
}

async fn handle_request_response(
    mut send: quinn::SendStream,
    mut recv: quinn::RecvStream,
    blockchain: Option<Arc<Mutex<Blockchain>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read request
    let mut buf = Vec::new();
    let mut read_buf = [0u8; 65536];
    loop {
        match recv.read(&mut read_buf).await {
            Ok(None) => break,
            Ok(Some(n)) => buf.extend_from_slice(&read_buf[..n]),
            Err(e) => {
                warn!("Failed to read request: {}", e);
                return Ok(());
            }
        }
    }
    
    // Parse and handle
    match serde_json::from_slice::<P2PMessage>(&buf) {
        Ok(P2PMessage::GetBlocks { from_height, limit }) => {
            if let Some(bc) = blockchain {
                let bc_guard = bc.lock().await;
                let blocks = bc_guard.get_blocks(from_height, limit);
                let response = P2PMessage::BlocksResponse { blocks };
                if let Ok(resp_bytes) = serde_json::to_vec(&response) {
                    let _ = send.write_all(&resp_bytes).await;
                }
            }
        }
        _ => {}
    }
    
    send.finish().await?;
    Ok(())
}
```

### Step 5: Implement actual sync_from_peer()

Current (stub):
```rust
pub async fn sync_from_peer(&mut self, peer_addr: &str, network: &P2PNetwork) -> Result<(), String> {
    let _msg = P2PMessage::GetBlocks { from_height: self.height, limit: 1000 };
    // Message created but never sent!
    Ok(())
}
```

New (actually syncs):
```rust
pub async fn sync_from_peer(
    &mut self,
    peer_addr: &str,
    network: &P2PNetwork,
) -> Result<(), String> {
    // Request blocks starting from our current height
    let request = P2PMessage::GetBlocks {
        from_height: self.height,
        limit: 1000,
    };
    
    // Send to peer and wait for response
    let response_bytes = network
        .send_message_to_peer(peer_addr, &request)
        .await?;
    
    // Parse response
    match serde_json::from_slice::<P2PMessage>(&response_bytes) {
        Ok(P2PMessage::BlocksResponse { blocks }) => {
            tracing::info!("Received {} blocks from sync peer", blocks.len());
            
            // Apply each block
            for block_bytes in blocks {
                match serde_json::from_slice::<Block>(&block_bytes) {
                    Ok(block) => {
                        self.apply_synced_block(block)?;
                    }
                    Err(e) => {
                        return Err(format!("Failed to parse synced block: {}", e));
                    }
                }
            }
            
            Ok(())
        }
        _ => Err("Invalid response from peer".to_string()),
    }
}
```

## Testing Strategy

### Phase 4a: Bidirectional Messaging (Local Test)
```bash
# Test request-response works
# Measure round-trip latency
# Verify block data transfer
```

### Phase 4b: Multi-Node Sync (Pi Test)
```bash
# Node 1: Start (bootstrap)
# Node 2: Connect and sync from Node 1
# Verify: Node 2 height == Node 1 height after sync
# Verify: Node 2 sees Node 1's validators
```

### Phase 4c: Consensus (Pi Test)
```bash
# All 3 nodes sync validators
# Check: All nodes see 3 validators
# Check: Blocks produced every 5 seconds
# Check: Block height increments on all nodes
```

## Success Criteria

✅ Phase 4 Complete when:
1. `send_message_to_peer()` successfully sends GetBlocks
2. Peer responds with BlocksResponse over bidi stream
3. Node 2 receives blocks and applies to local state
4. Node 2 height matches Node 1 after sync completes
5. All 3 nodes reach consensus and produce blocks
6. Block height increments every 5 seconds (~25 blocks per 2 minutes)
7. Finality confirmed (blocks require 5 blocks to confirm ~25 seconds)

## Fallback Strategy

If bidirectional QUIC proves problematic:
- Implement HTTP/REST sync endpoint (simpler, less elegant)
- Or: Batch sync requests via unidirectional broadcasts
- Or: Pre-share validators via config file (temporary workaround)

## Effort Estimate

- Connection pool: 1 hour
- Bidirectional messaging: 1-2 hours
- Sync integration: 1 hour
- Testing on Pi: 1-2 hours
- **Total: 4-6 hours**

## Files to Modify

1. `src/network.rs` - Core P2P refactor (connection pool, bidirectional handling)
2. `src/blockchain.rs` - Implement actual sync_from_peer()
3. `src/main.rs` - Call actual sync (already has framework)

No new files needed - pure refactor.
