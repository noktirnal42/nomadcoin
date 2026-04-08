# NomadCoin Mainnet Deployment Guide

## Overview

This guide covers deploying NomadCoin to a Raspberry Pi 3 mainnet cluster. The deployment consists of:

1. **Building** the binary (native ARMv7 on Raspberry Pi)
2. **Testing** locally on macOS with a 3-node cluster
3. **Deploying** to Raspberry Pi mainnet
4. **Verifying** cluster consensus and finality

## Prerequisites

### Local (macOS)
- Rust toolchain (already installed)
- Binary from Pi build
- SSH access to Raspberry Pi

### Raspberry Pi 3
- Raspbian OS (latest)
- Rust toolchain installed
- Git repository cloned
- ~115GB disk space available

## Building on Raspberry Pi

The binary is built natively on the Pi 3 using:

```bash
ssh noktirnal@pi3.local
source ~/.cargo/env
cd ~/nomadcoin
cargo build --release -j 2
```

**Estimated time**: 6-8 hours

## Testing Locally

### Retrieve Binary
```bash
scp noktirnal@pi3.local:~/nomadcoin/target/release/nomadcoin ./target/release/nomadcoin
chmod +x target/release/nomadcoin
```

### Start Local 3-Node Cluster
```bash
chmod +x deploy_mainnet_local.sh
./deploy_mainnet_local.sh
```

### Stop Cluster
```bash
# Ctrl+C in terminal or killall nomadcoin
```

## Deploying to Pi Mainnet

### Deploy Binary & Config
```bash
chmod +x deploy_to_pi.sh
./deploy_to_pi.sh pi3.local noktirnal target/release/nomadcoin mainnet/config.mainnet.json
```

### Start Nodes
```bash
ssh noktirnal@pi3.local

# Node 1 (bootstrap)
~/nomadcoin node --port 9333 --data-dir ~/nomadcoin/node1 --config ~/nomadcoin/config/mainnet.json &

# Node 2
~/nomadcoin node --port 9334 --data-dir ~/nomadcoin/node2 --config ~/nomadcoin/config/mainnet.json --peer 127.0.0.1:9333 &

# Node 3
~/nomadcoin node --port 9335 --data-dir ~/nomadcoin/node3 --config ~/nomadcoin/config/mainnet.json --peer 127.0.0.1:9333 &
```

## Mainnet Configuration

File: `mainnet/config.mainnet.json`

- **chain_id**: "nomadcoin-mainnet-1" (prevents cross-chain replays)
- **block_time**: 5 seconds
- **finality**: 5 blocks (~25 seconds)
- **bootstrap_peers**: Localhost nodes for initial cluster

## Security Features

✅ Replay attack protection (nonce/chain_id validation)
✅ TLS certificate handling (custom verifier)
✅ Finality enforcement (5-block requirement)
✅ Configuration externalization (JSON-based)

## Troubleshooting

**Build stalls**: Check `tail -f ~/build.log`, disk space (`df -h`), reduce parallelism
**Nodes don't connect**: Verify ports (netstat), check bootstrap peer reachability
**Finality issues**: Wait 25+ seconds after block inclusion

---

**Status**: Ready for Pi deployment
**Created**: 2026-04-08
