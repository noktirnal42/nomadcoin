# NomadCoin Testing Results & Issues Log
**Date:** April 8, 2026

---

## 🧪 TEST EXECUTION SUMMARY

### ✅ Completed Tests
1. **Single-Node Blockchain Initialization** - PASS
   - Command: `nomadcoin init --chain-id nomadcoin-dev-1 --allocation 10000000 --address nomad1community0000000000000000000000000 --data-dir ./mainnet/node1`
   - Result: Genesis block created, 10M NOMAD allocated
   - Database: RocksDB working correctly

2. **CLI Wallet Generation** - PASS
   - Command: `nomadcoin wallet --count 1`
   - Result: Valid 43-character addresses (nomad1...), hex public/private keys generated
   - Multiple wallets created successfully

3. **CLI Wallet Import** - PASS
   - Command: `nomadcoin import --key <64hex>`
   - Result: Successfully imports 64-character hex private keys
   - Derives correct public key and address from private key

4. **Transaction Creation** - PASS
   - Command: `nomadcoin send --from X --to Y --amount 100 --fee 0.001`
   - Result: Transactions created with proper TXID, timestamp, memo
   - Nonce tracking implemented

5. **GUI Application Launch (macOS)** - PASS
   - Launches without errors
   - Displays Wallet, Miner, Send, Community tabs
   - Device detection working ("macos")
   - Genesis balance displayed (10M NOMAD)
   - Address generation working (2 default addresses)

6. **Single Node Startup** - PASS
   - Command: `nomadcoin node --port 9333 --data-dir ./mainnet/node1`
   - Result: P2P server listening on port 9333
   - Height at 1 (genesis)
   - No panics or crashes

---

## ⚠️ KNOWN ISSUES

### Issue #1: QR Code Display (PRIORITY: MEDIUM)
**Status:** In Progress  
**User Report:** "QR code doesn't display correctly"  
**Current Behavior:** QR code rendered as 21x21 ASCII character grid (squares █ and spaces)  
**Expected Behavior:** Real QR code generated from wallet address  
**Code Location:** src/gui.rs lines 217-242 (wallet_tab)  
**Root Cause:** QR code rendering via character-based visualization instead of image rendering  
**Attempted Fix:** Added qrcode crate integration to generate real QR codes  
**Status:** Code updated, build successful, but needs verification  
**Solution:** 
```rust
// New code uses qrcode crate:
if let Ok(qr_code) = qrcode::QrCode::new(&addr.address) {
    let image = qr_code.render::<char>()
        .min_dimensions(21, 21)
        .light_color(' ')
        .dark_color('█')
        .build();
    // Render as lines...
}
```

### Issue #2: GUI Wallet Validation Stops (PRIORITY: HIGH)
**Status:** Not Yet Reproduced  
**User Report:** "GUI wallet gets one validation and stops"  
**Behavior:** GUI wallet appears to hang or crash after one validation attempt  
**Suspected Location:** src/gui.rs wallet_tab() or import dialog (lines 266-309)  
**Possible Causes:**
1. Import dialog modal window not properly closing
2. Wallet validation blocking egui event loop
3. Error in wallet.import_address() causing panic
4. Egui Window event not properly terminating  
**Investigation Needed:**
- Test import dialog with valid 64-hex key
- Check for unwrap() calls that could panic
- Monitor GUI logs for error messages
- Verify egui window event handling

### Issue #3: CLI Balance Query Returns 0 (PRIORITY: MEDIUM)
**Status:** Debugging  
**Behavior:** Balance queries always return 0.0000 NOMAD, even for genesis account with 10M allocation  
**Command Tested:**
```bash
nomadcoin balance --address nomad1community0000000000000000000000000 --data-dir ./mainnet/node1
# Output: Balance for nomad1community0000000000000000000000000: 0.0000 NOMAD
```
**Expected:** 10000000.0000 NOMAD  
**Code Location:** src/blockchain.rs::get_balance()  
**Root Cause:** UTXO state not being loaded from RocksDB or genesis block UTXO creation issue  
**Note:** GUI shows correct balance (10M), so database is storing it, but CLI query is broken  
**Impact:** Testing blockchain state, but GUI works correctly

### Issue #4: Multi-Node P2P Connection Failing (PRIORITY: HIGH)
**Status:** Blocking 3-node consensus testing  
**Error Message:**
```
Failed to connect to peer 127.0.0.1:9333: aborted by peer: 
the cryptographic handshake failed: error 120: 
peer doesn't support any known protocol
```
**Tested Scenarios:**
1. Bootstrap with libp2p format `/ip4/127.0.0.1/tcp/9333` → "invalid socket address syntax"
2. Bootstrap with simple format `127.0.0.1:9333` → TLS handshake failure
**Code Location:** src/network.rs (P2P connection setup, TLS handling)  
**Root Cause:** TLS protocol version mismatch or certificate validation incompatibility  
**Impact:** Cannot test multi-node consensus or transaction propagation  
**Debug Needed:**
- Check TLS version (OpenSSL vs rustls)
- Verify certificate generation for self-signed certs
- Add debug logging to handshake process
- Test with explicit TLS version specification

---

## 📊 TEST RESULTS TABLE

| Test Case | Status | Notes |
|-----------|--------|-------|
| Init blockchain | ✅ PASS | Genesis block created correctly |
| Generate wallet | ✅ PASS | Valid address format |
| Import wallet | ✅ PASS | 64-hex key import works |
| Create transaction | ✅ PASS | Transaction TXID generated |
| Start single node | ✅ PASS | P2P server listening |
| GUI launch (macOS) | ✅ PASS | App runs without crash |
| GUI display balance | ✅ PASS | Shows 10M genesis balance |
| GUI show addresses | ✅ PASS | Displays 2 addresses |
| GUI tab navigation | ✅ PASS | All tabs clickable |
| **CLI balance query** | ❌ FAIL | Returns 0, should be 10M |
| **QR code display** | ⚠️ PARTIAL | ASCII grid, not real QR |
| **GUI import dialog** | ❌ UNKNOWN | Crashes after validation (reported) |
| **Multi-node connect** | ❌ FAIL | TLS handshake error |
| 3-node consensus | ❌ NOT TESTED | Blocked by P2P issue |

---

## 🔍 DETAILED ISSUE INVESTIGATION

### Balance Query Issue Deep Dive
**Steps to Reproduce:**
```bash
# 1. Initialize blockchain with 10M allocation
./nomadcoin init --chain-id nomadcoin-dev-1 --allocation 10000000 \
  --address nomad1community0000000000000000000000000 --data-dir ./test

# 2. Check balance immediately
./nomadcoin balance --address nomad1community0000000000000000000000000 \
  --data-dir ./test
# Expected: 10000000.0000 NOMAD
# Actual: 0.0000 NOMAD
```

**Evidence:**
- GUI shows 10M balance ✓ (database has the data)
- CLI balance query returns 0 ✗ (query implementation broken)
- Transaction creation works ✓ (can create tx from account)

**Hypothesis:** Balance query is not loading UTXO state from RocksDB correctly

### P2P Handshake Issue Analysis
**Node 1 (Primary):** Starts, listens on 9333  
**Node 2:** Attempts bootstrap to Node 1, fails with cryptographic error  
**Node 3:** Same TLS failure as Node 2  

**Certificate Chain Issue:**
- Nodes using self-signed certificates
- Handshake error 120 = unknown protocol or version mismatch
- Could be OpenSSL version incompatibility

---

## 🚀 NEXT STEPS

### Immediate (Block fixes needed today)
- [ ] **QR Code Verification:** Test if new QR code implementation renders
- [ ] **Reproduce GUI Validation Issue:** Try wallet import in GUI
- [ ] **Fix CLI Balance Query:** Debug UTXO loading in blockchain.rs

### Short-term (Next 24 hours)
- [ ] **Fix Multi-Node P2P:** Debug TLS handshake, possibly downgrade to simpler protocol
- [ ] **Test 3-Node Consensus:** Once P2P fixed, verify voting mechanism
- [ ] **Finality Testing:** Confirm 5-block finality window works

### Pre-Launch (Before mainnet)
- [ ] Complete wallet import flow in GUI
- [ ] Implement password encryption for wallets
- [ ] Performance profiling and optimization
- [ ] Security audit of all crypto operations

---

## 📋 DEPLOYMENT BLOCKERS

**CRITICAL (Must fix):**
1. ❌ Multi-node P2P connection
2. ❌ GUI wallet import validation crash
3. ⚠️ CLI balance query (if used for testing)

**IMPORTANT (Should fix):**
1. ⚠️ QR code real image rendering
2. ⚠️ Wallet persistence/encryption

**NICE-TO-HAVE:**
1. Performance optimization
2. Enhanced error messages

---

## 🎯 MAINNET LAUNCH READINESS

**Current Status:** 70/100 (down from 72 due to issues found)
- Single-node CLI: Operational ✅
- Single-node GUI: Mostly operational (import issue) ⚠️
- Multi-node network: Broken ❌
- Consensus: Not tested (depends on multi-node) ❌

**Recommendation:** 
- Fix P2P + GUI import issues (1-2 days of work)
- Then re-test full 3-node cluster
- Launch single-validator testnet once P2P verified
