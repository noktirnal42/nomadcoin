#!/bin/bash

# Initialize 3-node cluster on Pi
# Run this once the binary build completes

echo "🚀 Initializing 3-Node Cluster on Pi3"
echo "======================================"
echo ""

ssh noktirnal@pi3.local << 'PISCRIPT'
cd ~/nomadcoin

# Wait for binary if still building
if [ ! -f target/release/nomadcoin ]; then
    echo "⏳ Waiting for binary to compile..."
    while [ ! -f target/release/nomadcoin ]; do
        sleep 5
    done
    echo "✅ Binary compiled!"
fi

# Copy to accessible location
cp target/release/nomadcoin ./nomadcoin

echo "📍 Initializing 3 nodes..."
for i in 1 2 3; do
    rm -rf node$i 2>/dev/null || true
    ./nomadcoin init \
        --chain-id nomad-pi-cluster-1 \
        --allocation 10000000 \
        --address nomad1picluster000000000000000000000 \
        --data-dir ./node$i 2>&1 | tail -3
done

echo ""
echo "✅ 3-node cluster initialized!"
echo ""
echo "Next steps:"
echo "  Terminal 1: ./nomadcoin node --port 9333 --data-dir ./node1"
echo "  Terminal 2: ./nomadcoin node --port 9334 --bootstrap 127.0.0.1:9333 --data-dir ./node2"
echo "  Terminal 3: ./nomadcoin node --port 9335 --bootstrap 127.0.0.1:9333 --data-dir ./node3"

PISCRIPT
