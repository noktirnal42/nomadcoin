#!/bin/bash
# NomadCoin Testnet Bootstrap Node Deployment Script
# Automates deployment of validators and bootstrap nodes for public testnet

set -e

# Configuration
NODE_NAME=${1:-"bootstrap1"}
PORT=${2:-"9333"}
DATA_DIR=${3:-"/var/lib/nomadcoin/$NODE_NAME"}
STAKE=${4:-"10000"}
VALIDATOR_ADDRESS=${5:-""}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo not installed"
        exit 1
    fi

    if ! command -v systemctl &> /dev/null; then
        log_error "systemd not available"
        exit 1
    fi

    log_info "Prerequisites OK"
}

# Build binary
build_binary() {
    log_info "Building NomadCoin binary..."
    cargo build --release -j 4
    log_info "Build complete"
}

# Create system user
setup_user() {
    log_info "Setting up nomadcoin system user..."

    if id "nomadcoin" &>/dev/null; then
        log_warn "User nomadcoin already exists"
    else
        sudo useradd -m -s /bin/bash nomadcoin
        log_info "User nomadcoin created"
    fi

    # Create data directory
    sudo mkdir -p "$DATA_DIR"
    sudo chown nomadcoin:nomadcoin "$DATA_DIR"
    sudo chmod 700 "$DATA_DIR"
    log_info "Data directory: $DATA_DIR"
}

# Register validator
register_validator() {
    log_info "Registering validator..."

    if [ -z "$VALIDATOR_ADDRESS" ]; then
        log_error "Validator address required for registration"
        exit 1
    fi

    ./target/release/nomadcoin register-validator \
        --address "$VALIDATOR_ADDRESS" \
        --stake "$STAKE" \
        --data-dir "$DATA_DIR"

    log_info "Validator registered: $VALIDATOR_ADDRESS"
}

# Create systemd service
create_systemd_service() {
    log_info "Creating systemd service: nomadcoin-$NODE_NAME"

    SERVICE_FILE="/etc/systemd/system/nomadcoin-$NODE_NAME.service"

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

# Resource limits
LimitNOFILE=65535
LimitNPROC=32768

[Install]
WantedBy=multi-user.target
EOF

    log_info "Systemd service created"
}

# Configure firewall
configure_firewall() {
    log_info "Configuring firewall..."

    if command -v ufw &> /dev/null; then
        sudo ufw allow "$PORT/tcp" || log_warn "Failed to add firewall rule"
        sudo ufw allow 22/tcp || log_warn "SSH already allowed"
        log_info "Firewall rules added"
    else
        log_warn "ufw not found, skipping firewall configuration"
    fi
}

# Start service
start_service() {
    log_info "Starting nomadcoin-$NODE_NAME service..."

    sudo systemctl daemon-reload
    sudo systemctl enable "nomadcoin-$NODE_NAME"
    sudo systemctl start "nomadcoin-$NODE_NAME"

    sleep 2
    if sudo systemctl is-active --quiet "nomadcoin-$NODE_NAME"; then
        log_info "Service started successfully"
    else
        log_error "Service failed to start"
        sudo systemctl status "nomadcoin-$NODE_NAME"
        exit 1
    fi
}

# Verify service
verify_service() {
    log_info "Verifying service health..."

    sleep 3

    # Check if process is running
    if sudo systemctl is-active --quiet "nomadcoin-$NODE_NAME"; then
        log_info "✓ Service is running"

        # Show recent logs
        echo ""
        log_info "Recent service logs:"
        sudo journalctl -u "nomadcoin-$NODE_NAME" -n 10 --no-pager
    else
        log_error "Service is not running"
        exit 1
    fi
}

# Main deployment flow
main() {
    log_info "Starting NomadCoin testnet node deployment: $NODE_NAME"
    echo ""

    check_prerequisites
    build_binary
    setup_user

    if [ -n "$VALIDATOR_ADDRESS" ]; then
        register_validator
    else
        log_warn "No validator address provided, skipping validator registration"
    fi

    create_systemd_service
    configure_firewall
    start_service
    verify_service

    echo ""
    log_info "Deployment complete!"
    log_info "Node name: $NODE_NAME"
    log_info "Data directory: $DATA_DIR"
    log_info "Port: $PORT"
    echo ""
    log_info "View logs: sudo journalctl -u nomadcoin-$NODE_NAME -f"
}

main
