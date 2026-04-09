#!/bin/bash
# NomadCoin Testnet Monitoring Script
# Monitors cluster health and logs key metrics

# Configuration
BOOTSTRAP_NODES=("bootstrap1:9333" "bootstrap2:9334" "bootstrap3:9335")
MONITOR_INTERVAL=30  # seconds
LOG_FILE="/var/log/nomadcoin-monitor.log"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Ensure log directory exists
sudo mkdir -p /var/log
sudo touch "$LOG_FILE"
sudo chmod 666 "$LOG_FILE"

log_status() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] $1" >> "$LOG_FILE"
    echo -e "${BLUE}[$timestamp]${NC} $1"
}

check_service() {
    local service_name=$1
    if sudo systemctl is-active --quiet "$service_name"; then
        echo -e "${GREEN}✓${NC} $service_name"
        return 0
    else
        echo -e "${RED}✗${NC} $service_name"
        return 1
    fi
}

check_node_health() {
    local node_name=$1
    local port=$2

    local response=$(curl -s "http://localhost:$port/health" 2>/dev/null || echo '{"status":"down"}')

    if echo "$response" | grep -q '"status":"up"'; then
        local height=$(echo "$response" | grep -o '"height":[0-9]*' | cut -d':' -f2)
        local validators=$(echo "$response" | grep -o '"validators":[0-9]*' | cut -d':' -f2)
        echo "  Height: $height | Validators: $validators"
        return 0
    else
        echo "  Status: DOWN"
        return 1
    fi
}

check_disk_usage() {
    local data_dir=$1
    if [ -d "$data_dir" ]; then
        local usage=$(du -sh "$data_dir" 2>/dev/null | cut -f1)
        echo "  Disk: $usage"
    fi
}

monitor_loop() {
    log_status "Starting NomadCoin testnet monitor"

    while true; do
        echo ""
        log_status "=== Cluster Health Check ==="

        local all_healthy=true

        # Check each bootstrap node
        for node_config in "${BOOTSTRAP_NODES[@]}"; do
            IFS=':' read -r NODE_NAME PORT <<< "$node_config"

            echo "Checking $NODE_NAME..."
            if ! check_service "nomadcoin-$NODE_NAME"; then
                all_healthy=false
            fi

            check_node_health "$NODE_NAME" "$PORT"
            check_disk_usage "/var/lib/nomadcoin/$NODE_NAME"
        done

        # Overall status
        if [ "$all_healthy" = true ]; then
            log_status "All services healthy ✓"
        else
            log_status "Warning: Some services down ⚠"
        fi

        # Show system resource usage
        log_status "System Resources:"
        echo "  CPU Load: $(uptime | sed 's/.*load average: //')"
        echo "  Memory: $(free -h | grep Mem | awk '{print $3 "/" $2}')"
        echo "  Disk: $(df -h / | tail -1 | awk '{print $3 "/" $2}')"

        sleep "$MONITOR_INTERVAL"
    done
}

# Run monitor
log_status "NomadCoin Testnet Monitor Started"
monitor_loop
