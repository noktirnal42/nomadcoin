#!/bin/bash
set -e

# NomadCoin Mainnet Local Deployment Script
# Initializes and starts a 3-node mainnet cluster on localhost

BINARY="${1:-./target/release/nomadcoin}"
CONFIG_DIR="./mainnet"
MAINNET_CONFIG="$CONFIG_DIR/config.mainnet.json"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found: $BINARY"
    echo "Build with: cargo build --release"
    exit 1
fi

if [ ! -f "$MAINNET_CONFIG" ]; then
    echo "❌ Config not found: $MAINNET_CONFIG"
    exit 1
fi

echo "🚀 Starting NomadCoin Mainnet (3-node cluster)"
echo "📁 Using config: $MAINNET_CONFIG"
echo ""

# Clean previous chaindata
for i in 1 2 3; do
    NODE_DIR="$CONFIG_DIR/node$i"
    rm -rf "$NODE_DIR/chaindata"
    mkdir -p "$NODE_DIR/chaindata"
done

# Start nodes with staggered timing
echo "🟢 Node 1 (port 9333)..."
"$BINARY" node --port 9333 --data-dir "$CONFIG_DIR/node1" --config "$MAINNET_CONFIG" &
NODE1_PID=$!
sleep 2

echo "🟡 Node 2 (port 9334)..."
"$BINARY" node --port 9334 --data-dir "$CONFIG_DIR/node2" --config "$MAINNET_CONFIG" --peer "127.0.0.1:9333" &
NODE2_PID=$!
sleep 2

echo "🔵 Node 3 (port 9335)..."
"$BINARY" node --port 9335 --data-dir "$CONFIG_DIR/node3" --config "$MAINNET_CONFIG" --peer "127.0.0.1:9333" &
NODE3_PID=$!

echo ""
echo "✅ All nodes started!"
echo "Node 1 PID: $NODE1_PID (port 9333)"
echo "Node 2 PID: $NODE2_PID (port 9334)"
echo "Node 3 PID: $NODE3_PID (port 9335)"
echo ""
echo "Monitoring nodes (Ctrl+C to stop)..."
echo ""

# Monitor nodes
cleanup() {
    echo ""
    echo "🛑 Stopping nodes..."
    kill $NODE1_PID $NODE2_PID $NODE3_PID 2>/dev/null || true
    wait 2>/dev/null || true
    echo "✓ Nodes stopped"
}

trap cleanup EXIT

# Keep script running
wait $NODE1_PID 2>/dev/null || true
