#!/bin/bash
# NomadCoin Testnet Cluster Deployment Script
# Deploys a 3-node bootstrap cluster for public testnet

set -e

# Configuration
BOOTSTRAP_NODES=(
    "bootstrap1:9333:nomad14ee5cad7faf6089c5b348f22f88a529a1eb53b:10000"
    "bootstrap2:9334:nomad12345678901234567890123456789012345678901:10000"
    "bootstrap3:9335:nomad19876543210987654321098765432109876543210:10000"
)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_section() {
    echo ""
    echo -e "${BLUE}=== $1 ===${NC}"
    echo ""
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Build once
log_section "Building NomadCoin Binary"
log_info "Building release binary..."
cargo build --release -j 4
log_info "Build complete"

# Create system user once
log_section "Setting up System User"
if id "nomadcoin" &>/dev/null; then
    log_info "User nomadcoin already exists"
else
    sudo useradd -m -s /bin/bash nomadcoin
    log_info "Created user nomadcoin"
fi

# Deploy each node
log_section "Deploying Bootstrap Nodes"

for node_config in "${BOOTSTRAP_NODES[@]}"; do
    IFS=':' read -r NODE_NAME PORT ADDRESS STAKE <<< "$node_config"

    log_info "Deploying $NODE_NAME on port $PORT..."

    DATA_DIR="/var/lib/nomadcoin/$NODE_NAME"

    # Create data directory
    sudo mkdir -p "$DATA_DIR"
    sudo chown nomadcoin:nomadcoin "$DATA_DIR"
    sudo chmod 700 "$DATA_DIR"

    # Register validator
    log_info "Registering validator: $ADDRESS"
    ./target/release/nomadcoin register-validator \
        --address "$ADDRESS" \
        --stake "$STAKE" \
        --data-dir "$DATA_DIR"

    # Create systemd service
    SERVICE_FILE="/etc/systemd/system/nomadcoin-$NODE_NAME.service"
    log_info "Creating systemd service..."

    sudo tee "$SERVICE_FILE" > /dev/null <<EOF
[Unit]
Description=NomadCoin Bootstrap Node ($NODE_NAME)
After=network.target

[Service]
Type=simple
User=nomadcoin
WorkingDirectory=$DATA_DIR
ExecStart=/root/nomadcoin/target/release/nomadcoin node \\
  --port $PORT \\
  --data-dir $DATA_DIR

Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal

LimitNOFILE=65535
LimitNPROC=32768

[Install]
WantedBy=multi-user.target
EOF

    # Configure firewall
    if command -v ufw &> /dev/null; then
        sudo ufw allow "$PORT/tcp" 2>/dev/null || true
    fi
done

# Start all services
log_section "Starting Services"

for node_config in "${BOOTSTRAP_NODES[@]}"; do
    IFS=':' read -r NODE_NAME PORT ADDRESS STAKE <<< "$node_config"
    log_info "Starting nomadcoin-$NODE_NAME..."
    sudo systemctl daemon-reload
    sudo systemctl enable "nomadcoin-$NODE_NAME"
    sudo systemctl start "nomadcoin-$NODE_NAME"
done

sleep 3

# Verify all services
log_section "Verifying Services"

for node_config in "${BOOTSTRAP_NODES[@]}"; do
    IFS=':' read -r NODE_NAME PORT ADDRESS STAKE <<< "$node_config"

    if sudo systemctl is-active --quiet "nomadcoin-$NODE_NAME"; then
        log_info "✓ nomadcoin-$NODE_NAME is running on port $PORT"
    else
        log_error "✗ nomadcoin-$NODE_NAME failed to start"
        sudo systemctl status "nomadcoin-$NODE_NAME" || true
    fi
done

log_section "Cluster Deployment Complete"
log_info "3 bootstrap nodes deployed successfully"
echo ""
log_info "Service Management Commands:"
echo "  View all services:    sudo systemctl list-units 'nomadcoin-*'"
echo "  View logs:            sudo journalctl -u nomadcoin-bootstrap1 -f"
echo "  Stop service:         sudo systemctl stop nomadcoin-bootstrap1"
echo "  Restart service:      sudo systemctl restart nomadcoin-bootstrap1"
echo ""
log_info "Testnet Chain Parameters:"
echo "  Network:              nomadcoin-testnet"
echo "  Block Time:           5 seconds"
echo "  Finality:             5 blocks (~25 seconds)"
echo "  Bootstrap Nodes:      3"
echo "  Bootstrap Ports:      9333, 9334, 9335"
echo ""
