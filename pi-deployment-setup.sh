#!/bin/bash

# NomadCoin Raspberry Pi 3 Deployment Setup
# This script handles SSH key setup, compilation, and deployment to Pi

set -e

PI_HOST="${1:-pi3.local}"
PI_USER="${2:-noktirnal}"
PI_ADDR="${PI_USER}@${PI_HOST}"

echo "🚀 NomadCoin Raspberry Pi Deployment"
echo "===================================="
echo "Target: $PI_ADDR"
echo ""

# Step 1: SSH Key Setup
echo "Step 1️⃣: Setting up SSH Key Authentication"
echo "============================================"

if [ ! -f ~/.ssh/id_rsa ]; then
    echo "Generating SSH key..."
    ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa -N "" -C "nomadcoin-pi"
fi

echo "Installing SSH key on Pi..."
ssh-copy-id -i ~/.ssh/id_rsa.pub "$PI_ADDR" 2>/dev/null || {
    echo "⚠️  SSH key installation failed"
    echo "You may need to:"
    echo "  1. Connect to Pi manually: ssh $PI_ADDR"
    echo "  2. Create ~/.ssh/authorized_keys if needed"
    echo "  3. Or try: ssh-copy-id -i ~/.ssh/id_rsa.pub $PI_ADDR"
    exit 1
}

echo "✓ SSH key installed"

# Step 2: Test Connection
echo ""
echo "Step 2️⃣: Testing SSH Connection"
echo "================================"

if ssh -o ConnectTimeout=5 "$PI_ADDR" "uname -m" | grep -q "armv7l\|aarch64"; then
    echo "✓ Connected to Pi ($(ssh $PI_ADDR uname -m))"
else
    echo "✗ Connection test failed"
    exit 1
fi

# Step 3: Prepare Pi
echo ""
echo "Step 3️⃣: Preparing Pi Environment"
echo "==================================="

ssh "$PI_ADDR" << 'PISCRIPT'
echo "Updating system packages..."
sudo apt-get update -qq
sudo apt-get install -y -qq libssl-dev build-essential git

echo "Creating NomadCoin directory..."
mkdir -p ~/nomadcoin
echo "✓ Pi ready"
PISCRIPT

# Step 4: Build for ARM
echo ""
echo "Step 4️⃣: Building for ARM64"
echo "============================="

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet
    source "$HOME/.cargo/env"
fi

# Install ARM target
echo "Adding ARM target..."
rustup target add armv7-unknown-linux-gnueabihf --quiet || echo "ARM target already installed"

echo "Building for ARM..."
cargo build --release --target armv7-unknown-linux-gnueabihf --bin nomadcoin 2>&1 | grep -E "(Compiling|Finished)" || true

# Step 5: Transfer Binary
echo ""
echo "Step 5️⃣: Transferring Binary to Pi"
echo "===================================="

echo "Copying nomadcoin binary..."
scp target/armv7-unknown-linux-gnueabihf/release/nomadcoin "$PI_ADDR:~/nomadcoin/"

ssh "$PI_ADDR" "chmod +x ~/nomadcoin/nomadcoin && echo '✓ Binary ready on Pi'"

# Step 6: Initialize Cluster
echo ""
echo "Step 6️⃣: Initializing 3-Node Cluster on Pi"
echo "=========================================="

ssh "$PI_ADDR" << 'PISCRIPT'
cd ~/nomadcoin

# Initialize 3 nodes
echo "Initializing nodes..."
for i in 1 2 3; do
    rm -rf node$i 2>/dev/null || true
    ./nomadcoin init \
        --chain-id nomad-pi-cluster-1 \
        --allocation 10000000 \
        --address nomad1picluster000000000000000000000 \
        --data-dir ./node$i > /dev/null
    echo "✓ Node $i initialized"
done
PISCRIPT

echo ""
echo "✅ Pi Deployment Complete!"
echo ""
echo "🎯 Next: Start the 3-node cluster"
echo "=================================="
echo "SSH into Pi and run:"
echo "  ssh $PI_ADDR"
echo ""
echo "Then execute:"
echo "  cd ~/nomadcoin"
echo "  # Terminal 1: Start Node 1"
echo "  ./nomadcoin node --port 9333 --data-dir ./node1"
echo ""
echo "  # Terminal 2: Start Node 2 (bootstrap to Node 1)"
echo "  ./nomadcoin node --port 9334 --bootstrap 127.0.0.1:9333 --data-dir ./node2"
echo ""
echo "  # Terminal 3: Start Node 3 (bootstrap to Node 1)"
echo "  ./nomadcoin node --port 9335 --bootstrap 127.0.0.1:9333 --data-dir ./node3"
echo ""
echo "Expected output: 'Connected to peer: 127.0.0.1:...' on nodes 2 and 3"

