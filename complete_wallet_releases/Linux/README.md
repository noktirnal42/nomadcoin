# NomadCoin Linux Wallet v1.0.0

## Installation

### Build from Source
Linux wallet can be built from the Flutter source:

```bash
# Clone repository
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin/nomad_mobile_wallet

# Install dependencies
flutter pub get

# Build for Linux
flutter build linux --release

# Run
./build/linux/x64/release/bundle/nomad_mobile_wallet
```

### AppImage (Recommended)
Coming soon - standalone executable with no dependencies

### Docker
```bash
docker pull nomadcoin/wallet:latest
docker run -it nomadcoin/wallet:latest
```

## Features
- 💰 Multi-platform wallet
- 📤 Transaction support
- 📥 QR code generation
- 🔒 Encrypted storage
- ⚡ Fast performance
- 🐧 Linux native

## Requirements
- Ubuntu 20.04+ / Fedora 33+ / Debian 11+
- 100MB free storage
- GTK 3.0+ development libraries

## System Prerequisites

### Ubuntu/Debian:
```bash
sudo apt-get install libgtk-3-dev
```

### Fedora/CentOS:
```bash
sudo dnf install gtk3-devel
```

### Arch:
```bash
sudo pacman -S gtk3
```

## Building Instructions

1. Install Flutter: https://flutter.dev/docs/get-started/install/linux
2. Clone NomadCoin: `git clone https://github.com/noktirnal42/nomadcoin.git`
3. Navigate to mobile wallet: `cd nomadcoin/nomad_mobile_wallet`
4. Get dependencies: `flutter pub get`
5. Build: `flutter build linux --release`
6. Run: `./build/linux/x64/release/bundle/nomad_mobile_wallet`

## Keyboard Shortcuts
- `Ctrl+Q` - Quit wallet
- `Ctrl+C` - Copy address
- `Ctrl+X` - Cut/Export key

## Support
- Documentation: https://github.com/noktirnal42/nomadcoin
- Issues: https://github.com/noktirnal42/nomadcoin/issues
- Forum: https://github.com/noktirnal42/nomadcoin/discussions

---
Built with ❤️ by NomadCoin Team
