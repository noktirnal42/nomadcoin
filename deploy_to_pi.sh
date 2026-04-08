#!/bin/bash
set -e

# NomadCoin Raspberry Pi Mainnet Deployment Script
# Deploys mainnet binary and configuration to Raspberry Pi 3

PI_HOST="${1:-pi3.local}"
PI_USER="${2:-noktirnal}"
SOURCE_BINARY="${3:-./target/release/nomadcoin}"
SOURCE_CONFIG="${4:-./mainnet/config.mainnet.json}"

if [ ! -f "$SOURCE_BINARY" ]; then
    echo "❌ Binary not found: $SOURCE_BINARY"
    exit 1
fi

if [ ! -f "$SOURCE_CONFIG" ]; then
    echo "❌ Config not found: $SOURCE_CONFIG"
    exit 1
fi

echo "🚀 Deploying NomadCoin to $PI_HOST"
echo ""

# Copy binary
echo "📦 Copying binary..."
scp -q "$SOURCE_BINARY" "$PI_USER@$PI_HOST:~/.local/bin/nomadcoin" || \
    scp -q "$SOURCE_BINARY" "$PI_USER@$PI_HOST:~/nomadcoin"
echo "✓ Binary deployed"

# Copy config
echo "📋 Copying configuration..."
ssh "$PI_USER@$PI_HOST" "mkdir -p ~/nomadcoin/config"
scp -q "$SOURCE_CONFIG" "$PI_USER@$PI_HOST:~/nomadcoin/config/mainnet.json"
echo "✓ Configuration deployed"

# Set permissions
echo "🔧 Setting up permissions..."
ssh "$PI_USER@$PI_HOST" "chmod +x ~/nomadcoin 2>/dev/null || chmod +x ~/.local/bin/nomadcoin 2>/dev/null || true"

echo ""
echo "✅ Deployment complete!"
echo ""
echo "To start mainnet node on Pi:"
echo "  ssh $PI_USER@$PI_HOST"
echo "  ~/nomadcoin node --port 9333 --data-dir ~/nomadcoin/node1 --config ~/nomadcoin/config/mainnet.json"
echo ""
echo "For 3-node cluster, use different ports (9334, 9335) and peers:"
echo "  --peer 127.0.0.1:9333"
