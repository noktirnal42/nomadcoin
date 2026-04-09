# NomadCoin macOS Wallet v1.0.0

## Contents
- `NomadCoin-Wallet-GUI-v1.0.0.app` - Desktop GUI wallet (graphical interface)
- `NomadCoin-CLI-v1.0.0` - Command-line interface (CLI)

## Installation

### GUI Wallet (Recommended for Users)
1. Extract `NomadCoin-Wallet-GUI-v1.0.0.app`
2. Move to Applications folder
3. Double-click to launch
4. First run: Grant accessibility permissions if prompted
5. Create or import wallet

### CLI Wallet (Advanced Users)
1. Extract `NomadCoin-CLI-v1.0.0`
2. Make executable: `chmod +x NomadCoin-CLI-v1.0.0`
3. Run: `./NomadCoin-CLI-v1.0.0 --help`

## Security
- Private keys encrypted with AES-256
- No cloud storage or telemetry
- Ed25519 cryptographic signatures
- BLAKE3 hashing

## Features
- 💰 Full wallet management
- 📤 Send transactions
- 📥 Receive with QR codes
- 📊 Real-time balance
- 🔐 Encrypted local storage
- ⚡ Fast consensus (5-second blocks)

## System Requirements
- macOS 11.0 or higher
- Intel or Apple Silicon support
- 100MB free storage
- Internet connection

## Getting Started

### GUI
1. Launch app from Applications
2. Click "Create New Wallet" or "Import Wallet"
3. Set secure password
4. Save your backup phrase safely
5. Start mining or trading!

### CLI
```bash
# Create wallet
./NomadCoin-CLI-v1.0.0 wallet --count 1

# Check balance
./NomadCoin-CLI-v1.0.0 balance --address nomad1xxx

# Send transaction
./NomadCoin-CLI-v1.0.0 send --to nomad1xxx --amount 100
```

## Troubleshooting
- **App won't open**: Check System Preferences > Security & Privacy
- **Permission denied**: Run `chmod +x NomadCoin-CLI-v1.0.0`
- **Network issues**: Ensure firewall allows NomadCoin

## Support
Documentation: https://github.com/noktirnal42/nomadcoin
Issues: https://github.com/noktirnal42/nomadcoin/issues

---
Built with ❤️ by NomadCoin Team
