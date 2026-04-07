# NomadCoin Local Testing Guide

This guide walks you through testing the three-node network locally to verify consensus, finality, and wallet functionality before mainnet deployment.

---

## Step 1: Build Release Binary

```bash
cd /Users/jeremymcvay/dev/nomad_coin

# Build optimized release binary
cargo build --release

# Verify build succeeded
ls -lh target/release/nomadcoin
```

**Expected Output:**
- Binary created at `target/release/nomadcoin`
- Size: ~15-20 MB

---

## Step 2: Initialize First Node

```bash
# Initialize mainnet node1
./target/release/nomadcoin init \
  --chain_id nomadcoin-dev-1 \
  --allocation 10000000 \
  --data_dir ./mainnet/node1

# Verify initialization
ls -la ./mainnet/node1/chaindata/
```

**Expected Output:**
```
✓ Blockchain initialized
✓ Genesis block created: 10,000,000 NOMAD
✓ RocksDB database created
```

---

## Step 3: Start Three-Node Local Network

### Terminal 1: Start Node 1 (Primary Bootstrap)

```bash
cd /Users/jeremymcvay/dev/nomad_coin

# Start node1 on port 9333
./target/release/nomadcoin node \
  --port 9333 \
  --data_dir ./mainnet/node1

# Expected log output:
# P2P server listening on port 9333
# Genesis block created with 10000000 NOMAD
# Waiting for peer connections...
```

**Keep this terminal running**

---

### Terminal 2: Start Node 2 (Bootstrap from Node 1)

```bash
cd /Users/jeremymcvay/dev/nomad_coin

# Get peer ID from node1 logs (look for "p2p/xxx" in output)
# For now, use localhost bootstrap
./target/release/nomadcoin node \
  --port 9334 \
  --bootstrap /ip4/127.0.0.1/tcp/9333 \
  --data_dir ./mainnet/node2

# Expected log output:
# Connecting to bootstrap peer: 127.0.0.1:9333
# Connected to peer: 127.0.0.1:9333
# P2P server listening on port 9334
```

**Keep this terminal running**

---

### Terminal 3: Start Node 3 (Bootstrap from Node 1)

```bash
cd /Users/jeremymcvay/dev/nomad_coin

./target/release/nomadcoin node \
  --port 9335 \
  --bootstrap /ip4/127.0.0.1/tcp/9333 \
  --data_dir ./mainnet/node3

# Expected log output:
# Connecting to bootstrap peer: 127.0.0.1:9333
# Connected to peer: 127.0.0.1:9333
# P2P server listening on port 9335
```

**Keep this terminal running**

---

## Step 4: Test Wallet Operations (New Terminal)

### Create a Wallet

```bash
cd /Users/jeremymcvay/dev/nomad_coin

# Create new wallet with 1 address
./target/release/nomadcoin wallet --count 1

# Expected output:
# Address 1: nomad1xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
# Public Key: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
# Private Key: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**IMPORTANT:** Save these values for testing!

---

### Import Address

```bash
# Use the private key from above (64 hex characters)
./target/release/nomadcoin import --key <PASTE_PRIVATE_KEY_HERE>

# Expected output:
# ✓ Address imported successfully
# nomad1xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

---

### Check Balance

```bash
# Check balance for your address
./target/release/nomadcoin balance --address nomad1xxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# Expected output:
# Balance for nomad1xxx...: 10000000 NOMAD
```

---

## Step 5: Test Transactions

### Create Second Wallet (for sending)

```bash
# Create another wallet to send to
./target/release/nomadcoin wallet --count 1

# Save this address as RECIPIENT_ADDRESS
```

### Send Transaction

```bash
# Send 100 NOMAD from first address to second
./target/release/nomadcoin send \
  --from nomad1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx \
  --to nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy \
  --amount 100 \
  --fee 0.001

# Expected output:
# Transaction created and signed
# TXID: xxxxxxxxxxxxxxxxxxxxx
# Status: Added to mempool
```

---

## Step 6: Verify Consensus & Finality

### Monitor Node Logs

Watch the three terminal windows for:

1. **Block Creation** (should happen every ~5 seconds):
   ```
   Block 2 created with 1 transactions
   Block 3 created with 0 transactions
   Block 4 created with 1 transactions
   ```

2. **Peer Connectivity**:
   ```
   Connected to peer: 127.0.0.1:9334
   Connected to peer: 127.0.0.1:9335
   ```

3. **Consensus Rounds**:
   ```
   Consensus round started for height 5
   Prevotes received: 2/3
   Precommits received: 3/3
   Block finalized with consensus
   ```

### Verify Transaction Finality

```bash
# Check transaction status
./target/release/nomadcoin transaction --txid <YOUR_TXID>

# After 5+ blocks, should show:
# Status: FINALIZED
# Confirmations: 5
# Block Height: 5
```

---

## Step 7: Register Validators

### Create Validator Wallet

```bash
# Create validator wallet
./target/release/nomadcoin wallet --count 1

# Save the address and private key
# We need 100 NOMAD minimum stake
```

### Register as Validator

```bash
# Register on node1
./target/release/nomadcoin register-validator \
  --address nomad1validator1xxxxxxxxxxxxx \
  --stake 1000 \
  --mobile

# Expected output:
# ✓ Validator registered: nomad1validator1xxx
# Stake: 1000 NOMAD
# Mobile boost: 1.5x active
# Effective stake: 1500
```

### Verify Validator Registration

```bash
# List active validators
./target/release/nomadcoin validators --active

# Expected output:
# 1. nomad1validator1xxx - Stake: 1500 (mobile boosted)
# 2. nomad1validator2xxx - Stake: 1000
# 3. nomad1validator3xxx - Stake: 1000
```

---

## Step 8: Test Network Resilience

### Stop a Node

In Terminal 2 or 3, press Ctrl+C to stop one node:

```
^C
Stopping node 2...
```

### Verify Network Continues

Watch Terminal 1 logs:

```
Peer disconnected: 127.0.0.1:9334
Peer count: 1 (was 2)

# Still creating blocks?
Block 10 created with 0 transactions  ✓
Block 11 created with 0 transactions  ✓
```

### Restart the Stopped Node

```bash
cd /Users/jeremymcvay/dev/nomad_coin

./target/release/nomadcoin node \
  --port 9334 \
  --bootstrap /ip4/127.0.0.1/tcp/9333 \
  --data_dir ./mainnet/node2

# Should sync with network and catch up
```

---

## Step 9: Stress Testing (Optional)

### Send Multiple Transactions

```bash
# Send 10 rapid transactions
for i in {1..10}; do
  ./target/release/nomadcoin send \
    --from nomad1address1xxx \
    --to nomad1address2xxx \
    --amount 1 \
    --fee 0.001
  sleep 1
done

# Expected: All transactions in mempool
# Monitor blocks for transaction inclusion
```

### Monitor Mempool

```bash
# Check pending transactions
./target/release/nomadcoin mempool --stats

# Expected output:
# Mempool size: 5 pending transactions
# Total fees: 0.005 NOMAD
```

---

## Verification Checklist

### ✓ Consensus Working
- [ ] Blocks created every ~5 seconds on all nodes
- [ ] All nodes have same block height
- [ ] Block hashes match across nodes

### ✓ Transactions Finalized
- [ ] Transactions confirmed after 5 blocks
- [ ] Balances update immediately
- [ ] Finality window works correctly

### ✓ Wallet Operations
- [ ] Address import successful
- [ ] Balance queries accurate
- [ ] Transaction signing works
- [ ] QR code displays in GUI

### ✓ Network Resilience
- [ ] Nodes reconnect after disconnect
- [ ] Consensus continues with 2 of 3 nodes
- [ ] State syncs when node restarts
- [ ] No data loss on restart

### ✓ Validator Operations
- [ ] Validators register successfully
- [ ] Mobile boost (1.5x) applied correctly
- [ ] Validator selection weighted by stake
- [ ] Rewards calculated per block

---

## Troubleshooting

### Issue: Nodes won't connect

**Solution:**
```bash
# Check port availability
lsof -i :9333
lsof -i :9334
lsof -i :9335

# Kill any conflicting processes
kill -9 <PID>

# Restart nodes
```

### Issue: Consensus not reached

**Solution:**
- Ensure all 3 nodes are running
- Check network connectivity: `ping 127.0.0.1`
- Verify logs for "Consensus round started"
- Make sure validators are registered

### Issue: Blocks not finalizing

**Solution:**
```bash
# Check finality configuration
./target/release/nomadcoin config --show | grep finality

# Should show: confirmations_for_finality: 5

# If not, update config.mainnet.json and restart
```

### Issue: Transaction not in block

**Solution:**
- Check mempool: `nomadcoin mempool --stats`
- Verify sufficient fee: minimum 0.001 NOMAD
- Check transaction format: all fields required
- Monitor logs for validation errors

### Issue: Out of memory / Slow performance

**Solution:**
```bash
# RocksDB grows with block data
# Trim old blocks (if implemented)
./target/release/nomadcoin blockchain --prune --blocks-keep 1000

# Or increase system limits:
ulimit -n 4096  # File descriptors
ulimit -m 4000000  # Memory in KB
```

---

## Expected Test Duration

- Setup & Build: 5-10 minutes
- Node startup: 2-3 minutes
- Basic transactions: 5 minutes
- Finality verification: 2-3 minutes
- Validator registration: 5 minutes
- Resilience testing: 10-15 minutes

**Total: ~30-40 minutes for complete test**

---

## Next Steps After Local Testing

If all tests pass:

1. **Document Results**: Note any issues and fixes
2. **Update Code**: Fix any bugs found
3. **Create Test Report**: Record performance metrics
4. **Push Updates**: Commit and push to GitHub
5. **Prepare for Testnet**: Deploy to Mac + Raspberry Pi network

If tests fail:

1. **Identify Issue**: Check troubleshooting guide
2. **Debug**: Add logging to understand problem
3. **Fix Code**: Resolve the underlying issue
4. **Re-test**: Run tests again
5. **Document**: Record what was wrong and how fixed

---

## Performance Baseline

Expected metrics for local 3-node network:

```
Block Creation: ~5 seconds (5000ms)
Transaction Confirmation: ~25 seconds (5 blocks)
Mempool Processing: <100ms per transaction
Consensus Time: ~500ms for 3 votes
Node Memory: ~100-200 MB per node
CPU Usage: <10% per node (idle)
Network Latency: <1ms (localhost)
```

If your metrics differ significantly, investigate:
- System resource constraints
- Network connectivity issues
- Configuration mismatches
- Database performance

---

## Success Criteria

Your local test is successful when:

1. ✅ All 3 nodes running without errors
2. ✅ Consensus reaches 2/3+ validator agreement
3. ✅ Blocks finalize after exactly 5 confirmations
4. ✅ Transactions included in blocks within 1-2 blocks
5. ✅ Wallet import works in CLI and GUI
6. ✅ Nonce validation prevents replay attacks
7. ✅ Network tolerates 1 node failure (continues with 2)
8. ✅ Restart + sync works correctly
9. ✅ Mobile validator boost (1.5x) applied
10. ✅ QR codes display for addresses

**Once all criteria met, you're ready for testnet deployment! 🚀**
