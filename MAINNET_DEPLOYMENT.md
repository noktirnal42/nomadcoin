# NomadCoin Mainnet Deployment Guide

## Pre-Deployment Checklist

### Security (CRITICAL - Must Complete)
- [x] TLS certificate verification implemented (proper validation, not bypassed)
- [x] Replay attack protection (nonce, chain_id, sequence_number)
- [x] Finality window implemented (5 blocks = ~25 seconds)
- [x] Bootstrap peer configuration externalized (not hardcoded)
- [x] Private keys not logged or exposed in output
- [x] Wallet files encrypted with password protection
- [x] Configuration validation enforced at startup

### Features (REQUIRED for MVP)
- [x] Wallet import via CLI and GUI
- [x] Address generation and key derivation
- [x] Transaction signing and verification
- [x] Block creation and consensus
- [x] Balance tracking per address
- [x] QR code display for receiving addresses
- [x] Mobile mining support with boost factor
- [x] Mesh network framework (offline support ready)

### Testing (RECOMMENDED)
- [ ] Unit tests all pass: `cargo test`
- [ ] Integration tests verify full workflows
- [ ] No panic!() calls on malformed input
- [ ] Release binary builds without warnings: `cargo build --release`

### Documentation
- [x] Config schema documented (config.rs)
- [x] Wallet persistence documented (wallet_persistence.rs)
- [x] Deployment instructions provided (this file)

---

## Phase 1: Local Development (Single Machine)

### Setup
```bash
# Initialize local node
./target/release/nomadcoin init \
  --chain_id nomadcoin-dev-1 \
  --allocation 10000000 \
  --data_dir ./mainnet/node1
```

### Start First Node
```bash
./target/release/nomadcoin node \
  --port 9333 \
  --data_dir ./mainnet/node1
```

### Test Wallet Operations
```bash
# Create wallet
./target/release/nomadcoin wallet --count 1

# Import address
./target/release/nomadcoin import --key <64_hex_key>

# Check balance
./target/release/nomadcoin balance --address nomad1xxx...
```

---

## Phase 2: Multi-Node Local Network

### Start Three Local Nodes (3-terminal setup)

**Terminal 1: Node 1 (Primary)**
```bash
./target/release/nomadcoin node \
  --port 9333 \
  --data_dir ./mainnet/node1
```

**Terminal 2: Node 2 (Bootstrap from Node 1)**
```bash
./target/release/nomadcoin node \
  --port 9334 \
  --bootstrap /ip4/127.0.0.1/tcp/9333/p2p/NODE1_PEER_ID \
  --data_dir ./mainnet/node2
```

**Terminal 3: Node 3 (Bootstrap from Node 1)**
```bash
./target/release/nomadcoin node \
  --port 9335 \
  --bootstrap /ip4/127.0.0.1/tcp/9333/p2p/NODE1_PEER_ID \
  --data_dir ./mainnet/node3
```

### Register Validators
```bash
# Register validator on node 1 (admin only)
./target/release/nomadcoin register-validator \
  --address nomad1xxx... \
  --stake 1000 \
  --mobile
```

### Verify Consensus
- Check that blocks are created every 5 seconds
- Verify that all 3 nodes reach consensus
- Confirm transactions finalize after 5 blocks (~25 seconds)

---

## Phase 3: Multi-Machine Local Network (Mac + Raspberry Pi)

### Network Setup
1. **Determine Local IP Addresses**
   ```bash
   # On Mac
   ifconfig | grep "inet "
   
   # On Raspberry Pi
   hostname -I
   ```

2. **Update Bootstrap Peers in config.mainnet.json**
   ```json
   {
     "bootstrap_peers": ["192.168.1.100:9333"]  // Mac's local IP
   }
   ```

3. **Copy Files to Raspberry Pi**
   ```bash
   scp -r ./target/release/nomadcoin pi@raspberrypi.local:/home/pi/nomadcoin/
   scp config.mainnet.json pi@raspberrypi.local:/home/pi/nomadcoin/
   ```

4. **Start Nodes**
   - Mac: `./nomadcoin node --port 9333`
   - RPi: `./nomadcoin node --port 9333 --bootstrap /ip4/192.168.1.100/tcp/9333`

---

## Phase 4: Public Testnet Deployment

### Prerequisites
- [ ] Domain name for bootstrap peer (e.g., `bootstrap.nomadcoin.testnet`)
- [ ] Static IP address or DynDNS setup
- [ ] Firewall rules (allow TCP 9333)
- [ ] TLS certificates for P2P (use `config.mainnet.json`)

### Deployment
1. Update `config.mainnet.json` with testnet bootstrap peers (DNS names, not IPs)
2. Deploy bootstrap node to public infrastructure
3. Provide bootstrap peer address to community
4. Monitor node health and consensus progress

### Monitoring
```bash
# Check node status
curl http://localhost:9334/status  # Requires RPC endpoint implementation

# Monitor blocks
./target/release/nomadcoin blockchain --stats

# Track consensus rounds
./target/release/nomadcoin consensus --monitor
```

---

## Phase 5: Public Mainnet Deployment

### CRITICAL REQUIREMENTS
1. **Bootstrap Peers**: Configure 3+ public bootstrap nodes with DNS
2. **TLS Certificates**: Use proper PKI (not self-signed)
3. **Key Management**: Use HSM or KMS for validator keys
4. **Monitoring**: Set up Prometheus/Grafana dashboards
5. **Alerting**: Configure PagerDuty/Slack notifications

### Mainnet Configuration
```json
{
  "chain_id": "nomadcoin-mainnet-1",
  "bootstrap_peers": [
    "bootstrap1.nomadcoin.io:9333",
    "bootstrap2.nomadcoin.io:9333",
    "bootstrap3.nomadcoin.io:9333"
  ]
}
```

### Launch Steps
1. Deploy bootstrap infrastructure
2. Announce genesis timestamp
3. Start validators
4. Monitor block production
5. Enable public RPC endpoints (once stable)

---

## Security Considerations

### For Testnet
- Self-signed TLS certificates acceptable
- Single-machine testing sufficient
- Shared validator keys for testing

### For Mainnet
- **TLS**: Proper CA-signed certificates required
- **Keys**: Each validator must have unique, securely stored keys
- **Consensus**: Monitor 2/3+ validator participation
- **Finality**: Require 5+ block confirmations before considering transactions final
- **Rate Limiting**: Implement DOS protection on P2P layer
- **Monitoring**: Real-time alerting on consensus failures

---

## Troubleshooting

### Nodes Not Connecting
- Check firewall rules: `sudo ufw allow 9333/tcp`
- Verify IP addresses: `netstat -an | grep 9333`
- Check logs for "Connection refused" errors

### Consensus Not Reached
- Verify validator count >= 3
- Check that all validators are staking >= 100 NOMAD
- Monitor network latency between nodes

### Blocks Not Finalizing
- Confirm finality_blocks = 5 in config
- Check that all blocks are properly signed
- Verify validator uptime

---

## Next Steps After Launch

1. **Block Explorer**: Deploy public block explorer UI
2. **RPC Endpoints**: Set up JSON-RPC API for wallets
3. **Mobile Apps**: Update wallet apps with mainnet chain_id
4. **Governance**: Implement on-chain governance for parameter changes
5. **Upgrades**: Plan upgrade procedures for protocol changes

---

## Support & Contact

- GitHub Issues: github.com/noktirnal42/nomadcoin/issues
- Community Discord: [TBD]
- Security Reports: security@nomadcoin.io
