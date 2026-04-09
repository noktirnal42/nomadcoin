# Phase 3: Blockchain State Synchronization Plan

## Problem Statement

Each node independently creates its own blockchain state:
- Node 1: Genesis → Validators[A]
- Node 2: Genesis → Validators[B]  
- Node 3: Genesis → Validators[C]

Consensus requires all nodes to have identical state and see the same 2/3+ validators. Current implementation simulates votes only with local validators, blocking consensus on multi-node clusters.

## Solution Architecture

### 1. Synchronization Flow

```
Node startup:
  ↓
Load local blockchain & consensus
  ↓
IF peers exist:
  │
  ├→ Request blocks from bootstrap peer
  │  (P2PMessage::GetBlocks)
  │
  ├→ Validate each received block
  │
  ├→ Apply blocks to local state
  │  (update height, balances, validators)
  │
  └→ Load validators from synced state
  │
  ✓ ALL NODES NOW HAVE SAME STATE
  │
  ↓
Enter main consensus loop
  ↓
All validators visible → 2/3 consensus possible
  ↓
Block production starts
```

### 2. Implementation Components

#### A. Network Protocol (network.rs)
**Already exists, needs implementation:**
- `P2PMessage::GetBlocks { from_height: u64, limit: u32 }`
- `P2PMessage::BlocksResponse { blocks: Vec<Vec<u8>> }`

**Handler in `handle_connection()` (line ~155):**
```rust
P2PMessage::GetBlocks { from_height, limit } => {
    // Fetch blocks from blockchain
    let blocks = blockchain.get_blocks(from_height, limit);
    let response = P2PMessage::BlocksResponse { blocks };
    // Send back to peer
}

P2PMessage::BlocksResponse { blocks } => {
    // Queue blocks for sync processor
    sync_queue.send(blocks).await;
}
```

#### B. Blockchain Sync Methods (blockchain.rs)
**New methods to add:**
```rust
impl Blockchain {
    /// Get blocks from local blockchain for peer sync
    pub fn get_blocks(&self, from_height: u64, limit: u32) -> Vec<Vec<u8>> {
        // Return serialized blocks starting at from_height
    }
    
    /// Apply synced block to local state
    pub fn apply_synced_block(&mut self, block: Block) -> Result<(), String> {
        // Verify block hash chain
        // Update validators from block
        // Update balances from transactions
        // Increment height
    }
    
    /// Sync blockchain state from peer
    pub async fn sync_from_peer(
        &mut self,
        peer_addr: &str,
        network: &P2PNetwork,
    ) -> Result<(), String> {
        // Request blocks starting from height 1
        // Validate and apply each block
        // Return on success or error
    }
}
```

#### C. Node Startup Sync (main.rs)
**Modify `run_node()` before main loop:**
```rust
// After loading blockchain and consensus (line ~450)

// Sync blockchain from peers if we're not the first node
if blockchain.height() == 1 && !peers.is_empty() {
    println!("🔄 Synchronizing blockchain from peers...");
    
    match tokio::time::timeout(
        Duration::from_secs(30),
        blockchain.sync_from_peer(&peers[0], &network)
    ).await {
        Ok(Ok(())) => {
            println!("✅ Blockchain synced!");
            println!("  Height:     {}", blockchain.height());
            println!("  Validators: {}", consensus.validator_count());
            
            // Reload validators from synced blockchain
            consensus = storage.load_consensus(100, 1.5)?;
        }
        Ok(Err(e)) => {
            eprintln!("⚠️  Sync failed: {}. Continuing with local state.", e);
        }
        Err(_) => {
            eprintln!("⚠️  Sync timeout. Continuing with local state.");
        }
    }
}

// Now enter main loop (existing code at line ~487)
```

## Implementation Steps

### Step 1: Add Sync Message Handler (network.rs)
- Implement GetBlocks handler in `handle_connection()` 
- Add GetBlocksResponse handler
- Create async task to queue received blocks

### Step 2: Add Blockchain Sync Methods (blockchain.rs)
- Implement `get_blocks(from_height, limit) -> Vec<Vec<u8>>`
- Implement `apply_synced_block(block) -> Result<>`
- Implement `sync_from_peer(peer_addr, network) -> Result<>`

### Step 3: Connect Network to Blockchain (network.rs)
- Pass blockchain reference to `handle_connection()` 
- Enable GetBlocks handler to access blockchain.get_blocks()
- Use Arc<Mutex<Blockchain>> for thread-safe access

### Step 4: Integrate Sync into Node Startup (main.rs)
- Add sync call before main consensus loop
- Handle sync timeout (30 seconds)
- Reload consensus engine after sync
- Fall back to local state if sync fails

### Step 5: Test Sync Flow
- Single node (no peers): Creates genesis, runs consensus solo
- Two nodes (N1 → N2): N2 syncs from N1, then consensus with both validators
- Three nodes (N1 → N2, N3): All sync from N1, consensus with all 3 validators

## Critical Implementation Details

### 1. Thread-Safe Blockchain Access
The `handle_connection()` runs in a tokio task. It needs mutable access to blockchain:

```rust
// In run_node():
let blockchain = Arc::new(Mutex::new(blockchain));
let blockchain_for_network = blockchain.clone();

// Pass to network
network.set_blockchain(blockchain_for_network);
```

### 2. Block Validation During Sync
Each received block must be validated:
```rust
pub fn apply_synced_block(&mut self, block: Block) -> Result<(), String> {
    // 1. Verify previous_block matches our current tip
    let expected_prev = self.state.blocks.last()?.header.previous_block;
    if block.header.previous_block != expected_prev {
        return Err("Block chain mismatch".to_string());
    }
    
    // 2. Verify block height is sequential
    if block.height != self.height + 1 {
        return Err("Non-sequential height".to_string());
    }
    
    // 3. Apply transactions (update balances, validators)
    for tx in &block.transactions {
        self.apply_transaction(tx)?;
    }
    
    // 4. Update validators from block consensus data
    // (if block contains validator set changes)
    
    // 5. Update height and add to chain
    self.state.blocks.push(block);
    self.height += 1;
    
    Ok(())
}
```

### 3. Sync Timeout & Fallback
If sync takes >30 seconds or fails:
- Continue with local blockchain state
- Enter consensus loop with only local validators
- Single-node will produce blocks fine (1 validator = consensus)
- Multi-node will wait for consensus (won't reach 2/3 if validators differ)

### 4. Validator Set Agreement
After sync completes, all nodes must load the same validators:
```rust
// After sync succeeds in run_node():
consensus = storage.load_consensus(100, 1.5)?;
// consensus.validators now matches the synced blockchain state
```

## Validation Checklist

- [ ] GetBlocks handler retrieves correct blocks from blockchain
- [ ] BlocksResponse handler queues blocks for sync
- [ ] Blockchain validates each synced block (hash chain, height)
- [ ] Node syncs before entering consensus loop
- [ ] After sync, all nodes see same validator set
- [ ] Consensus produces blocks with 2/3+ validator agreement
- [ ] Single node works (no peers to sync from)
- [ ] 3-node cluster produces blocks every 5 seconds
- [ ] Finality confirmed (5-block requirement works)

## Testing on Raspberry Pi

```bash
# Terminal 1: Start Node 1 (bootstrap)
ssh noktirnal@pi3.local 'nohup ~/nomadcoin/target/release/nomadcoin node --port 9333 --data-dir ~/nomadcoin/node1 > ~/node1.log 2>&1 &'

# Terminal 2: Node 2 (syncs from Node 1)
ssh noktirnal@pi3.local 'nohup ~/nomadcoin/target/release/nomadcoin node --port 9334 --data-dir ~/nomadcoin/node2 --peers 127.0.0.1:9333 > ~/node2.log 2>&1 &'

# Terminal 3: Node 3 (syncs from Node 1)
ssh noktirnal@pi3.local 'nohup ~/nomadcoin/target/release/nomadcoin node --port 9335 --data-dir ~/nomadcoin/node3 --peers 127.0.0.1:9333 > ~/node3.log 2>&1 &'

# Monitor logs
ssh noktirnal@pi3.local 'tail -f ~/node1.log'
ssh noktirnal@pi3.local 'tail -f ~/node2.log'
ssh noktirnal@pi3.local 'tail -f ~/node3.log'

# Expected output:
# Node 1: "✅ Block 2 created with 0 transactions"
# Node 2: "✅ Blockchain synced! Height: 1"
# Node 3: "✅ Blockchain synced! Height: 1"
# (All nodes then produce blocks every 5 seconds)
```

## Success Criteria

✅ Phase 3 Complete when:
1. All 3 nodes start consensus loop (no hang on genesis)
2. Blocks produced every 5 seconds on all nodes
3. All nodes reach same height within 1 block
4. Node logs show: "✅ Blockchain synced!" before consensus starts
5. Finality test shows blocks confirmed after 5 blocks (~25 seconds)
6. Validators visible on all 3 nodes: `nomadcoin validators --data-dir ~/nomadcoin/nodeX`
