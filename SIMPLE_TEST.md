# Simple Single-Node Wallet Test

Quick test of the CLI wallet and single node (no P2P networking).

---

## Step 1: Initialize Blockchain

```bash
cd /Users/jeremymcvay/dev/nomad_coin

./target/release/nomadcoin init \
  --chain_id nomadcoin-dev-1 \
  --allocation 10000000 \
  --address nomad1community0000000000000000000000000 \
  --data_dir ./mainnet/node1
```

**Expected Output:**
```
✅ Blockchain Initialized!
  Chain ID:              nomadcoin-dev-1
  Community Allocation:  10000000 NOMAD
  Community Address:     nomad1community0000000000000000000000000
  Genesis Block Height:  1
  Genesis TX Count:      1
  Data Directory:        ./mainnet/node1
```

---

## Step 2: Test Wallet Operations

### Create Wallet

```bash
./target/release/nomadcoin wallet --count 1
```

**Expected Output:**
```
🔐 NomadCoin Wallet Generator
=============================

Wallet #1:
  Address: nomad1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx (43 chars)
  Public Key: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx (64 hex chars)
  Private Key: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx (64 hex chars)
```

**ACTION:** Copy and save the private key (64 hex characters)

---

### Import Address

```bash
./target/release/nomadcoin import --key <PASTE_YOUR_64_HEX_KEY>
```

Replace `<PASTE_YOUR_64_HEX_KEY>` with your private key from above.

**Expected Output:**
```
🔑 Wallet Import
================

✓ Address imported successfully!
  Address: nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
  Public Key: yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
```

---

## Step 3: Check Balance

```bash
./target/release/nomadcoin balance --address nomad1community0000000000000000000000000
```

**Expected Output:**
```
💰 NomadCoin Balance Checker
=========================

Balance for nomad1community0000000000000000000000000:
  10000000.0000 NOMAD
```

Check balance for your imported address (should be 0 initially):

```bash
./target/release/nomadcoin balance --address nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
```

**Expected Output:**
```
Balance for nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy:
  0.0000 NOMAD
```

---

## Step 4: Create Second Address

```bash
./target/release/nomadcoin wallet --count 1
```

Save this address as your recipient address.

---

## Step 5: Send Transaction

```bash
./target/release/nomadcoin send \
  --from nomad1community0000000000000000000000000 \
  --to nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy \
  --amount 100 \
  --fee 0.001
```

Replace:
- `nomad1community...` with community address (genesis)
- `nomad1yyyyy...` with your recipient address

**Expected Output:**
```
💸 Transaction Creator
====================

✓ Transaction created and signed!
  TXID: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
  From: nomad1community0000000000000000000000000
  To: nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
  Amount: 100.0000 NOMAD
  Fee: 0.0010 NOMAD
  Status: Added to mempool
```

---

## Step 6: Verify Balances After Send

```bash
# Community address should be reduced
./target/release/nomadcoin balance --address nomad1community0000000000000000000000000

# Recipient should have increased
./target/release/nomadcoin balance --address nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
```

**Expected Output:**
```
Balance for nomad1community0000000000000000000000000:
  9999899.9990 NOMAD    (10000000 - 100 - 0.001 fee)

Balance for nomad1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy:
  100.0000 NOMAD
```

---

## Success Criteria ✅

- [x] Blockchain initialized with genesis block
- [x] Wallet creation generates valid addresses (43 chars, start with nomad1)
- [x] Wallet import accepts 64-character hex keys
- [x] Balance queries work correctly
- [x] Transactions create and sign successfully
- [x] Balance updates after transaction
- [x] Nonce tracking prevents duplicate sends

---

## Troubleshooting

### "Address not found"
Make sure you're using the correct address format (starts with `nomad1`, 43 characters total).

### "Insufficient funds"
Transaction amount + fee exceeds balance. Try lower amount or higher fee.

### "Invalid signature"
Private key format incorrect. Should be 64 hexadecimal characters (0-9, a-f).

### "Chain ID mismatch"
Make sure all commands use the same `--data_dir` (./mainnet/node1).

---

## Next: Single Node with Mining

After wallet tests pass, start a single node:

```bash
./target/release/nomadcoin node --port 9333 --data_dir ./mainnet/node1
```

You'll see:
```
🔗 NomadCoin Node
=================

P2P server listening on port 9333
Genesis block created with 10000000 NOMAD
Waiting for peer connections...
```

The node will create blocks even without peers (solitary validator).

---

## Testing Roadmap

1. ✅ **Single Node + Wallet** (this test)
2. → **Single Node Mining** (add --mine flag)
3. → **Three-Node Network** (follow LOCAL_TESTING_GUIDE.md)
4. → **Raspberry Pi + Mac** (local network)
5. → **Public Testnet**
6. → **Mainnet Launch**
