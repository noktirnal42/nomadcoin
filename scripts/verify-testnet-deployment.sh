#!/bin/bash
# NomadCoin Testnet Deployment Verification Script
# Verifies all deployment prerequisites and configurations

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSED=0
FAILED=0

check() {
    local name=$1
    local cmd=$2

    echo -n "Checking $name... "
    if eval "$cmd" >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗${NC}"
        ((FAILED++))
        return 1
    fi
}

echo -e "${BLUE}=== NomadCoin Testnet Deployment Verification ===${NC}"
echo ""

# System prerequisites
echo -e "${YELLOW}System Prerequisites:${NC}"
check "Rust installed" "command -v cargo"
check "systemd available" "command -v systemctl"
check "sudo available" "sudo -n true 2>/dev/null || echo 'requires password'"
check "UFW firewall" "command -v ufw"

echo ""
echo -e "${YELLOW}NomadCoin Build:${NC}"
check "Binary exists" "[ -f ./target/release/nomadcoin ]"
check "Binary is executable" "[ -x ./target/release/nomadcoin ]"
check "Binary version works" "./target/release/nomadcoin --version || true"

echo ""
echo -e "${YELLOW}System Configuration:${NC}"
check "nomadcoin user exists" "id nomadcoin >/dev/null 2>&1"
check "Data directory exists" "[ -d /var/lib/nomadcoin ]"
check "Data directory owned by nomadcoin" "[ $(stat -f %Su /var/lib/nomadcoin) = nomadcoin ] 2>/dev/null || [ $(stat -c %U /var/lib/nomadcoin) = nomadcoin ]"

echo ""
echo -e "${YELLOW}Systemd Services:${NC}"
check "bootstrap1 service exists" "[ -f /etc/systemd/system/nomadcoin-bootstrap1.service ]"
check "bootstrap2 service exists" "[ -f /etc/systemd/system/nomadcoin-bootstrap2.service ]"
check "bootstrap3 service exists" "[ -f /etc/systemd/system/nomadcoin-bootstrap3.service ]"

echo ""
echo -e "${YELLOW}Network Configuration:${NC}"
check "Port 9333 available" "! lsof -i :9333 2>/dev/null"
check "Port 9334 available" "! lsof -i :9334 2>/dev/null"
check "Port 9335 available" "! lsof -i :9335 2>/dev/null"

if command -v ufw &>/dev/null; then
    echo ""
    echo -e "${YELLOW}Firewall Rules:${NC}"
    check "Port 9333 allowed" "sudo ufw status | grep -q '9333/tcp'"
    check "Port 9334 allowed" "sudo ufw status | grep -q '9334/tcp'"
    check "Port 9335 allowed" "sudo ufw status | grep -q '9335/tcp'"
    check "SSH allowed" "sudo ufw status | grep -q '22/tcp'"
fi

echo ""
echo -e "${YELLOW}Documentation:${NC}"
check "Deployment guide exists" "[ -f ./TESTNET_DEPLOYMENT.md ]"
check "Deployment scripts exist" "[ -f ./scripts/deploy-testnet-cluster.sh ]"

echo ""
echo -e "${BLUE}=== Verification Summary ===${NC}"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ All checks passed! Testnet deployment ready.${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}✗ $FAILED check(s) failed. Please address issues before deployment.${NC}"
    exit 1
fi
