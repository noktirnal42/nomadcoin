# Raspberry Pi 3 Deployment - Phase 2

## Current Status
- ✅ SSH key generated: `~/.ssh/id_rsa`
- ⚠️ Pi3 not reachable at `pi3.local` (connection timeout)
- 📦 Deployment script ready: `pi-deployment-setup.sh`

---

## Troubleshooting Pi Connectivity

### Option 1: Check if Pi is powered on and on network
```bash
# From your Mac:
ping pi3.local
# OR
arp -a | grep -i raspberry
```

### Option 2: Find Pi IP address manually
If `pi3.local` doesn't resolve:
1. Access your router's admin panel (usually 192.168.1.1 or 192.168.0.1)
2. Look for connected devices - find "pi3" or "raspberrypi"
3. Note its IP address (e.g., 192.168.1.100)
4. Then use: `ssh noktirnal@192.168.1.100`

### Option 3: Enable mDNS on Pi (if already connected)
SSH into Pi with password and run:
```bash
sudo apt-get install avahi-daemon
sudo systemctl restart avahi-daemon
```

---

## Deployment Steps (Once Pi is Reachable)

### Step 1: Test Basic SSH Connection
```bash
# If using hostname:
ssh noktirnal@pi3.local

# If using IP address:
ssh noktirnal@192.168.1.x
```

### Step 2: Run Automated Deployment Script
Once you can SSH to the Pi, run the automated deployment:
```bash
./pi-deployment-setup.sh pi3.local noktirnal
# OR with IP:
./pi-deployment-setup.sh 192.168.1.x noktirnal
```

This script will:
1. ✅ Install SSH key authentication
2. ✅ Test connection
3. ✅ Prepare Pi environment (install dependencies)
4. ✅ Build NomadCoin for ARM (armv7)
5. ✅ Transfer binary to Pi
6. ✅ Initialize 3-node cluster

### Step 3: Manual Deployment (if script fails)
```bash
# SSH to Pi
ssh noktirnal@pi3.local

# Create nomadcoin directory
mkdir -p ~/nomadcoin
cd ~/nomadcoin

# Download pre-built binary (if available)
# OR compile on Pi:
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin
cargo build --release --bin nomadcoin

# Initialize 3 nodes
./target/release/nomadcoin init --chain-id nomad-pi-cluster-1 \
  --allocation 10000000 \
  --address nomad1picluster000000000000000000000 \
  --data-dir ~/nomadcoin/node1
```

---

## Running 3-Node Cluster on Pi

Once binary is on Pi, open 3 SSH terminals:

**Terminal 1 - Node 1 (Bootstrap)**
```bash
cd ~/nomadcoin
./nomadcoin node --port 9333 --data-dir ./node1
# Expected: "P2P server listening on port 9333"
```

**Terminal 2 - Node 2 (Connect to Node 1)**
```bash
cd ~/nomadcoin
./nomadcoin node --port 9334 --bootstrap 127.0.0.1:9333 --data-dir ./node2
# Expected: "Connected to peer: 127.0.0.1:..."
```

**Terminal 3 - Node 3 (Connect to Node 1)**
```bash
cd ~/nomadcoin
./nomadcoin node --port 9335 --bootstrap 127.0.0.1:9333 --data-dir ./node3
# Expected: "Connected to peer: 127.0.0.1:..."
```

---

## Verification

### Check P2P Connectivity
When all 3 nodes are running, verify in logs:
- Node 1: Should show "New peer connected" messages
- Nodes 2 & 3: Should show "Connected to peer" messages

### Monitor Consensus
- All nodes should have Height: 1 (genesis)
- Peers: Node 1 should show "2 peers", Nodes 2 & 3 should show "1 peer"

### Next Phase: Test Transactions
Once 3-node cluster is stable:
1. Send transaction from Node 1
2. Verify propagation to Node 2 and 3
3. Check finality (5-block confirmation)

---

## Quick Start Command

Once you know Pi's IP or have mDNS working:
```bash
./pi-deployment-setup.sh pi3.local noktirnal
```

This is all-in-one: SSH key setup → build → transfer → initialize → ready to run!

---

## Troubleshooting Deployment Script

| Issue | Solution |
|-------|----------|
| `Connection refused` | Pi not accessible, verify network/IP |
| `Permission denied (publickey)` | SSH key not installed, run ssh-copy-id first |
| `ARM target not found` | Run: `rustup target add armv7-unknown-linux-gnueabihf` |
| `Build timeout` | Pi is slow - precompile on Mac and transfer binary |
| `Cannot execute binary` | Wrong architecture - verify Pi is ARMv7 or ARM64 |

---

## Next Steps

1. **Get Pi online** - Check router, verify connectivity
2. **Test SSH** - Connect to Pi manually first
3. **Run deployment script** - Automates entire setup
4. **Verify 3-node cluster** - Check logs for connectivity
5. **Run Phase 3 tests** - Transaction propagation, consensus voting

Status: 🟡 **AWAITING PI CONNECTIVITY** - Once reachable, deployment takes ~5-10 minutes

