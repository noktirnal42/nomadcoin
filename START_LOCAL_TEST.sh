#!/bin/bash

# NomadCoin Local Testing Quick Start Script
# This script sets up and starts the three-node local test network

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$PROJECT_DIR/target/release/nomadcoin"
NODE1_DIR="$PROJECT_DIR/mainnet/node1"
NODE2_DIR="$PROJECT_DIR/mainnet/node2"
NODE3_DIR="$PROJECT_DIR/mainnet/node3"

echo "🚀 NomadCoin Local Testing Setup"
echo "=================================="
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found. Building..."
    cd "$PROJECT_DIR"
    ~/.cargo/bin/cargo build --release
fi

echo "✅ Binary found: $BINARY"
echo ""

# Clean up old data (optional - comment out to keep state)
# echo "Cleaning up old blockchain data..."
# rm -rf "$NODE1_DIR/chaindata" "$NODE2_DIR/chaindata" "$NODE3_DIR/chaindata"

# Initialize blockchain if needed
if [ ! -d "$NODE1_DIR/chaindata" ]; then
    echo "📦 Initializing blockchain for node1..."
    "$BINARY" init \
        --chain_id nomadcoin-dev-1 \
        --allocation 10000000 \
        --address nomad1community0000000000000000000000000 \
        --data_dir "$NODE1_DIR"
    echo "✅ Blockchain initialized"
fi

echo ""
echo "🎬 Starting three-node network..."
echo ""
echo "You will need to open 3 terminal windows:"
echo ""
echo "┌─ Terminal 1 (Node 1 - Primary) ─────────────────────────"
echo "│ $BINARY node --port 9333 --data_dir $NODE1_DIR"
echo "├─ Terminal 2 (Node 2) ──────────────────────────────────"
echo "│ $BINARY node --port 9334 --bootstrap /ip4/127.0.0.1/tcp/9333 --data_dir $NODE2_DIR"
echo "├─ Terminal 3 (Node 3) ──────────────────────────────────"
echo "│ $BINARY node --port 9335 --bootstrap /ip4/127.0.0.1/tcp/9333 --data_dir $NODE3_DIR"
echo "└─ Terminal 4 (Testing) ─────────────────────────────────"
echo "│ Run wallet tests (see LOCAL_TESTING_GUIDE.md)"
echo "└─────────────────────────────────────────────────────────"
echo ""

echo "Would you like to start the first node now? (y/n)"
read -r response

if [[ "$response" =~ ^[Yy]$ ]]; then
    echo ""
    echo "Starting Node 1..."
    echo "Press Ctrl+C to stop"
    echo ""
    "$BINARY" node --port 9333 --data_dir "$NODE1_DIR"
else
    echo ""
    echo "Ready to start testing! Copy and paste these commands in separate terminals:"
    echo ""
    echo "# Terminal 1"
    echo "$BINARY node --port 9333 --data_dir $NODE1_DIR"
    echo ""
    echo "# Terminal 2"
    echo "$BINARY node --port 9334 --bootstrap /ip4/127.0.0.1/tcp/9333 --data_dir $NODE2_DIR"
    echo ""
    echo "# Terminal 3"
    echo "$BINARY node --port 9335 --bootstrap /ip4/127.0.0.1/tcp/9333 --data_dir $NODE3_DIR"
fi
