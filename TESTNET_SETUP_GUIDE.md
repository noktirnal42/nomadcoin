# NomadCoin Testnet Setup Guide

Complete guide for deploying NomadCoin bootstrap nodes and running a public testnet using automated deployment scripts.

## Quick Start

### Option 1: Deploy Single Node

```bash
# Deploy a single bootstrap node with automated setup
./scripts/deploy-testnet-node.sh bootstrap1 9333 "/var/lib/nomadcoin/bootstrap1" 10000 "nomad14ee5cad7faf6089c5b348f22f88a529a1eb53b"
```

### Option 2: Deploy 3-Node Cluster

```bash
# Deploy full 3-node bootstrap cluster (recommended)
./scripts/deploy-testnet-cluster.sh
```

### Option 3: Manual Deployment

See [TESTNET_DEPLOYMENT.md](./TESTNET_DEPLOYMENT.md) for step-by-step manual instructions.

## Deployment Scripts

### `deploy-testnet-node.sh`

Deploys a single bootstrap node with full configuration.

**Usage:**
```bash
./scripts/deploy-testnet-node.sh <node_name> <port> <data_dir> <stake> <validator_address>
```

**Example:**
```bash
./scripts/deploy-testnet-node.sh bootstrap1 9333 "/var/lib/nomadcoin/bootstrap1" 10000 "nomad14ee5cad7faf6089c5b348f22f88a529a1eb53b"
```

**What it does:**
- ✓ Checks prerequisites (Rust, systemd)
- ✓ Builds NomadCoin binary in release mode
- ✓ Creates nomadcoin system user
- ✓ Registers validator with specified stake
- ✓ Creates systemd service file
- ✓ Configures firewall rules
- ✓ Starts and verifies service
- ✓ Shows initial logs

### `deploy-testnet-cluster.sh`

Deploys a complete 3-node bootstrap cluster with preconfigured addresses.

**Usage:**
```bash
./scripts/deploy-testnet-cluster.sh
```

**Configuration:**
Edit the `BOOTSTRAP_NODES` array in the script to customize:
- Node names (bootstrap1, bootstrap2, bootstrap3)
- Ports (9333, 9334, 9335)
- Validator addresses
- Stake amounts

**What it does:**
- ✓ Builds binary once
- ✓ Creates system user
- ✓ Deploys 3 nodes with different ports
- ✓ Registers all validators
- ✓ Creates all systemd services
- ✓ Configures firewall for all ports
- ✓ Starts and verifies all services
- ✓ Displays cluster summary

### `monitor-testnet.sh`

Continuous monitoring script that tracks cluster health every 30 seconds.

**Usage:**
```bash
./scripts/monitor-testnet.sh
```

**Monitors:**
- Service status for each node
- Block height and validator count
- Disk usage per node
- System resources (CPU, memory, disk)
- Logs to `/var/log/nomadcoin-monitor.log`

### `verify-testnet-deployment.sh`

Pre-deployment verification that checks all prerequisites.

**Usage:**
```bash
./scripts/verify-testnet-deployment.sh
```

**Checks:**
- System prerequisites (Rust, systemd, sudo)
- NomadCoin binary availability
- User and directory setup
- Systemd service files
- Port availability
- Firewall rules
- Documentation

## Deployment Workflow

### Step 1: Verify Prerequisites

```bash
./scripts/verify-testnet-deployment.sh
```

All checks must pass before proceeding.

### Step 2: Build Binary

```bash
cargo build --release -j 4
```

Or let the deployment script build automatically.

### Step 3: Deploy Cluster

```bash
./scripts/deploy-testnet-cluster.sh
```

This will:
1. Build the binary (if not already built)
2. Create the nomadcoin user
3. Deploy 3 nodes on ports 9333, 9334, 9335
4. Configure firewall
5. Start all services
6. Verify all services are running

### Step 4: Monitor Cluster

```bash
./scripts/monitor-testnet.sh
```

Leave this running to track health metrics. Press Ctrl+C to stop.

## Service Management

### View Service Status

```bash
# All NomadCoin services
sudo systemctl list-units 'nomadcoin-*'

# Specific service
sudo systemctl status nomadcoin-bootstrap1
```

### View Logs

```bash
# Follow logs in real-time
sudo journalctl -u nomadcoin-bootstrap1 -f

# View last 50 lines
sudo journalctl -u nomadcoin-bootstrap1 -n 50

# Search logs
sudo journalctl -u nomadcoin-bootstrap1 | grep "ERROR"
```

### Start/Stop/Restart

```bash
# Start service
sudo systemctl start nomadcoin-bootstrap1

# Stop service
sudo systemctl stop nomadcoin-bootstrap1

# Restart service
sudo systemctl restart nomadcoin-bootstrap1

# Restart all nodes
for i in 1 2 3; do
  sudo systemctl restart nomadcoin-bootstrap$i
done
```

### Enable/Disable Auto-Start

```bash
# Enable (auto-start on boot)
sudo systemctl enable nomadcoin-bootstrap1

# Disable (don't auto-start)
sudo systemctl disable nomadcoin-bootstrap1
```

## Network Configuration

### Testnet Chain Parameters

```
Network Name:         nomadcoin-testnet
Chain ID:             nomadcoin
Block Time:           5 seconds
Finality:             5 blocks (~25 seconds)
Min Validator Stake:  100 NOMAD
Validator Reward:     1M NOMAD/year
Total Supply:         100M NOMAD
Community Allocation: 10M NOMAD (genesis)
```

### Bootstrap Node Ports

```
bootstrap1: 9333 (P2P and RPC)
bootstrap2: 9334 (P2P and RPC)
bootstrap3: 9335 (P2P and RPC)
```

### Firewall Configuration

Ports are automatically configured by the deployment scripts. Manual setup:

```bash
# Allow P2P ports
sudo ufw allow 9333/tcp
sudo ufw allow 9334/tcp
sudo ufw allow 9335/tcp

# Allow SSH (essential!)
sudo ufw allow 22/tcp

# Enable firewall
sudo ufw enable

# Check rules
sudo ufw status
```

## Troubleshooting

### Service Won't Start

1. Check if port is in use:
```bash
lsof -i :9333
```

2. Verify permissions:
```bash
ls -la /var/lib/nomadcoin/bootstrap1
```

3. Check systemd service:
```bash
sudo systemctl status nomadcoin-bootstrap1
sudo journalctl -u nomadcoin-bootstrap1 -n 20
```

### Peers Not Connecting

1. Verify firewall rules:
```bash
sudo ufw status
```

2. Check peer addresses in service configuration:
```bash
cat /etc/systemd/system/nomadcoin-bootstrap1.service
```

3. Verify network connectivity:
```bash
ping <peer_ip>
nc -zv <peer_ip> <peer_port>
```

### High Memory Usage

1. Check memory with process info:
```bash
ps aux | grep nomadcoin
top -p $(pgrep -f nomadcoin)
```

2. Restart service:
```bash
sudo systemctl restart nomadcoin-bootstrap1
```

3. Check for memory leaks in logs:
```bash
sudo journalctl -u nomadcoin-bootstrap1 | grep -i memory
```

### Disk Full

1. Check disk usage:
```bash
df -h
du -sh /var/lib/nomadcoin/*
```

2. Backup and clean old data:
```bash
# Backup
sudo cp -r /var/lib/nomadcoin /backup/

# Clean (requires downtime)
sudo systemctl stop nomadcoin-bootstrap1
sudo rm -rf /var/lib/nomadcoin/bootstrap1/*
```

3. Expand disk if necessary

## Monitoring Dashboard

Create a simple monitoring script to track all nodes:

```bash
#!/bin/bash
watch -n 5 'echo "=== NomadCoin Cluster Status ===" && \
  for i in 1 2 3; do \
    echo "bootstrap$i: $(sudo systemctl is-active nomadcoin-bootstrap$i)" && \
    du -sh /var/lib/nomadcoin/bootstrap$i; \
  done && \
  echo "=== System Resources ===" && \
  free -h && df -h /'
```

## Health Check Endpoint

Each node provides a health check endpoint:

```bash
curl http://localhost:9333/health
```

Response format:
```json
{
  "status": "up",
  "height": 12345,
  "validators": 3,
  "peers": 2
}
```

## Next Steps

After deployment:

1. **Verify Consensus**: Register validators and confirm consensus works
   ```bash
   ./target/release/nomadcoin register-validator \
     --address nomad1xxx \
     --stake 1000
   ```

2. **Test Transactions**: Send test transactions between wallets
   ```bash
   ./target/release/nomadcoin send --to <address> --amount 100
   ```

3. **Monitor Blocks**: Watch blocks being produced
   ```bash
   sudo journalctl -u nomadcoin-bootstrap1 -f | grep "Block"
   ```

4. **Stress Test**: Run load testing with multiple transactions

5. **Expand Network**: Add more validator nodes beyond initial bootstrap

## Security Notes

- ⚠️ **Never expose port 9333 to untrusted networks**
- ⚠️ **Keep validator private keys secure**
- ⚠️ **Use firewall to restrict SSH access**
- ⚠️ **Enable automatic security updates**
- ⚠️ **Monitor logs for suspicious activity**
- ✅ **Use strong SSH keys, not passwords**
- ✅ **Enable SELinux or AppArmor**
- ✅ **Regular backups of validator keys**

## Support

For issues or questions:
1. Check logs: `sudo journalctl -u nomadcoin-bootstrap1 -f`
2. Review [TESTNET_DEPLOYMENT.md](./TESTNET_DEPLOYMENT.md)
3. Check GitHub issues: https://github.com/noktirnal42/nomadcoin/issues
