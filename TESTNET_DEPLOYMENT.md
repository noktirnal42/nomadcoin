# NomadCoin Public Testnet Deployment Guide

## Overview
This guide covers deploying NomadCoin validators and bootstrap nodes for public testnet operations.

## Bootstrap Node Requirements

### Hardware Specs (Minimum)
- **CPU**: 2 cores
- **RAM**: 2GB
- **Storage**: 10GB SSD
- **Network**: 10 Mbps stable connection
- **Uptime**: 99%+ availability

### Recommended Hardware
- **CPU**: 4+ cores
- **RAM**: 8GB+
- **Storage**: 100GB SSD
- **Network**: 100+ Mbps
- **Dedicated Server**: AWS, Hetzner, or similar

## Setup Instructions

### 1. Clone and Build

```bash
# Clone repository
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin

# Build release binary
cargo build --release -j 4

# Binary location
./target/release/nomadcoin
```

### 2. Create System User

```bash
sudo useradd -m -s /bin/bash nomadcoin
sudo mkdir -p /var/lib/nomadcoin
sudo chown nomadcoin:nomadcoin /var/lib/nomadcoin
```

### 3. Initialize Genesis

```bash
# Register bootstrap validators (run 3 validators for 2/3+ consensus)
~/nomadcoin/target/release/nomadcoin register-validator \
  --address nomad14ee5cad7faf6089c5b348f22f88a529a1eb53b \
  --stake 10000 \
  --data-dir /var/lib/nomadcoin/bootstrap1

# Repeat for bootstrap2, bootstrap3 with different addresses
```

### 4. Create Systemd Service

**File**: `/etc/systemd/system/nomadcoin-bootstrap.service`

```ini
[Unit]
Description=NomadCoin Bootstrap Node
After=network.target

[Service]
Type=simple
User=nomadcoin
WorkingDirectory=/var/lib/nomadcoin
ExecStart=/home/nomadcoin/nomadcoin/target/release/nomadcoin node \
  --port 9333 \
  --data-dir /var/lib/nomadcoin/bootstrap1

Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal

# Resource limits
LimitNOFILE=65535
LimitNPROC=32768

[Install]
WantedBy=multi-user.target
```

### 5. Start Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable nomadcoin-bootstrap
sudo systemctl start nomadcoin-bootstrap
sudo systemctl status nomadcoin-bootstrap
```

### 6. Log Monitoring

```bash
# Follow logs
sudo journalctl -u nomadcoin-bootstrap -f

# View recent logs
sudo journalctl -u nomadcoin-bootstrap -n 100
```

## Network Configuration

### Port Forwarding
- **P2P Port**: 9333 (default)
- **RPC Port**: 9334 (optional, for wallets)

### Firewall Rules
```bash
sudo ufw allow 9333/tcp  # P2P network
sudo ufw allow 22/tcp    # SSH (essential!)
sudo ufw enable
```

## Bootstrap Node Peering

Each bootstrap node should connect to at least 2 other bootstrap nodes:

```bash
# Node 2 startup (connects to Node 1)
--peers 201.162.45.67:9333

# Node 3 startup (connects to Nodes 1 and 2)
--peers 201.162.45.67:9333 201.162.45.68:9333
```

## Monitoring

### Health Check Script

```bash
#!/bin/bash
# Health check every 5 minutes

RESULT=$(curl -s http://localhost:9333/health || echo '{"status":"down"}')
HEIGHT=$(echo $RESULT | jq '.height // "unknown"')
VALIDATORS=$(echo $RESULT | jq '.validators // 0')

echo "[$(date)] Height: $HEIGHT | Validators: $VALIDATORS"

if [ "$HEIGHT" = "unknown" ]; then
  systemctl restart nomadcoin-bootstrap
fi
```

## Validator Registration

Before mainnet, all validators must register:

```bash
~/nomadcoin/target/release/nomadcoin register-validator \
  --address <YOUR_ADDRESS> \
  --stake <AMOUNT> \
  --mobile [--mobile for 1.5x boost]
```

## Metrics to Monitor

- **Block Height**: Should increase every ~5 seconds
- **Peer Count**: Should be > 0
- **Validator Count**: Should be >= 3
- **Disk Usage**: Should grow ~1MB per day
- **Memory**: Typically 100-200MB
- **CPU**: Idle otherwise

## Public Testnet Chain Parameters

```
Network Name: nomadcoin-testnet
Chain ID: nomadcoin
Block Time: 5 seconds
Finality: 5 blocks (~25 seconds)
Min Validator Stake: 100 NOMAD
Validator Reward: 1M NOMAD/year
Total Supply: 100M NOMAD
Community Allocation: 10M NOMAD (genesis)
```

## Deployment Checklist

- [ ] Hardware provisioned and prepared
- [ ] NomadCoin binary compiled and tested
- [ ] System user created
- [ ] Data directories prepared
- [ ] Validators registered
- [ ] Systemd service configured
- [ ] Firewall rules set
- [ ] Service started and verified
- [ ] Logs monitored for 1 hour
- [ ] Peer connections confirmed
- [ ] Health check script running
- [ ] Bootstrap announced to network

## Troubleshooting

### Node won't start
- Check port 9333 isn't already in use: `lsof -i :9333`
- Verify permissions: `ls -la /var/lib/nomadcoin`
- Check logs: `systemctl status nomadcoin-bootstrap`

### Peers not connecting
- Verify firewall: `sudo ufw status`
- Check peer address format
- Ensure both nodes have correct ports

### High memory usage
- Check for memory leaks: `ps aux | grep nomadcoin`
- Restart service
- Increase swap if needed

### Disk full
- Check storage: `df -h`
- Archive old blocks (not yet implemented)
- Expand disk

## Security Notes

- ⚠️ **Never expose port 9333 to untrusted networks**
- ⚠️ **Keep validator private keys secure**
- ⚠️ **Use firewall to restrict SSH access**
- ⚠️ **Enable automatic security updates**
- ⚠️ **Monitor logs for suspicious activity**
- ✅ **Use strong SSH keys, not passwords**
- ✅ **Enable SELinux or AppArmor**
- ✅ **Regular backups of validator keys**
