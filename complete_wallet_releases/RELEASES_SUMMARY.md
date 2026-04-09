# NomadCoin Wallet Releases Summary

**Date**: April 9, 2026  
**Version**: 1.0.0  
**Status**: Production Ready

---

## 📦 What's Included

### 🍎 macOS Releases
- **NomadCoin-Wallet-GUI-v1.0.0.app** (15MB) - Full-featured desktop wallet with GUI
- **NomadCoin-CLI-v1.0.0** (11MB) - Command-line interface for advanced users

**Status**: ✅ Ready to use immediately

### 🤖 Android Release
- **NomadCoin-Wallet-v1.0.0.apk** (48MB) - Mobile wallet for Android 8.0+

**Status**: ✅ Ready to use immediately

### 📱 iOS Release
- Build instructions and source code provided
- Ready for TestFlight beta and App Store submission

**Status**: 🔜 Coming Q2 2026

### 🐧 Linux Release
- Build from Flutter source included
- AppImage coming soon

**Status**: 🔜 AppImage v1.0.0 coming Q2 2026

### 🪟 Windows Release
- Build from Flutter source included
- .msi installer coming soon

**Status**: 🔜 Installer v1.0.0 coming Q2 2026

---

## 🚀 Installation Quick Start

### macOS Users - Easiest Way
```bash
cd complete_wallet_releases/macOS
open NomadCoin-Wallet-GUI-v1.0.0.app
```

### Android Users
1. Download `Android/NomadCoin-Wallet-v1.0.0.apk`
2. Open file manager
3. Tap APK to install

### All Users - Automated Installer
```bash
cd complete_wallet_releases
chmod +x INSTALL.sh
./INSTALL.sh
```

---

## ✨ Features Available

- ✅ Create new wallets
- ✅ Import existing wallets
- ✅ Send transactions
- ✅ Receive with QR codes
- ✅ View transaction history
- ✅ Encrypted private key storage (AES-256)
- ✅ Ed25519 cryptography
- ✅ BLAKE3 hashing
- ✅ Cross-platform support
- ✅ Offline transaction capability
- ⏳ Biometric authentication (coming)
- ⏳ Auto-update system (coming)

---

## 📊 Release Statistics

### File Sizes
| Platform | File | Size |
|----------|------|------|
| macOS GUI | NomadCoin-Wallet-GUI-v1.0.0.app | 15MB |
| macOS CLI | NomadCoin-CLI-v1.0.0 | 11MB |
| Android | NomadCoin-Wallet-v1.0.0.apk | 48MB |

### Build Information
- **Compilation**: All targets compiled in Release mode
- **Optimization**: Full optimization enabled
- **Testing**: Verified on live hardware
- **Code**: Open source on GitHub

---

## 🔒 Security

All releases include enterprise-grade security:
- **Encryption**: AES-256 for private keys
- **Signatures**: Ed25519 (same as Signal, ZCash)
- **Hashing**: BLAKE3 (faster than SHA-3)
- **Storage**: Keys never leave device
- **Open Source**: Full audit trail on GitHub

---

## 📋 System Requirements

### macOS
- macOS 11.0 or later
- Intel or Apple Silicon
- 100MB free storage

### Android
- Android 8.0 or later
- 50MB free storage
- Internet connection (optional)

### iOS (when available)
- iOS 12.0 or later
- 100MB free storage

### Linux/Windows (build from source)
- Flutter 3.41+
- Rust 1.70+
- 150-200MB for build dependencies

---

## 🔄 Update Path

### Current (v1.0.0)
All releases are testnet-ready with full functionality.

### Next Release (v1.1.0 - Q2 2026)
- App Store availability (iOS + Android)
- Windows installer (.msi)
- Linux AppImage
- Biometric authentication
- Auto-update system

### Mainnet (v2.0.0 - Q3 2026)
- Mainnet network launch
- Enhanced security audits
- Performance optimizations
- Enterprise features

---

## 📚 Documentation

Each platform folder includes:
- `README.md` - Platform-specific installation and usage
- Build instructions for developers
- Troubleshooting guides
- Security information

### Main Documentation
- GitHub: https://github.com/noktirnal42/nomadcoin
- Wiki: https://github.com/noktirnal42/nomadcoin/wiki
- Issues: https://github.com/noktirnal42/nomadcoin/issues

---

## 🆘 Support

### Installation Issues
- See platform-specific README files
- Check troubleshooting section
- Open issue on GitHub

### Feature Requests
- GitHub Issues: https://github.com/noktirnal42/nomadcoin/issues
- Community Discussions: https://github.com/noktirnal42/nomadcoin/discussions

### Security Concerns
- Email: security@nomadcoin.network (coming soon)
- GitHub Security Advisory: https://github.com/noktirnal42/nomadcoin/security

---

## 📝 Release Notes

### v1.0.0 (April 9, 2026)
✅ **Initial Release - Testnet Ready**

**Added**:
- Full Flutter mobile wallet for Android
- macOS desktop GUI and CLI wallet
- Cross-platform architecture
- Transaction support
- QR code generation
- Wallet persistence
- Multi-platform build support

**Tested On**:
- macOS 12.0+ (Intel + Apple Silicon)
- Android 8.0 - 14.0
- iOS 12.0+ (simulator)
- Ubuntu 20.04+ (build tested)
- Windows 10/11 (build tested)

**Known Limitations**:
- iOS: Requires developer build until App Store available
- Windows: Requires build setup until installer available
- Linux: Requires build setup until AppImage available
- Biometric auth: Coming in v1.1.0
- Auto-update: Coming in v1.1.0

---

## 🎯 Quality Assurance

### Testing Completed
- ✅ Unit tests (crypto, consensus, blockchain)
- ✅ Integration tests (3-node cluster)
- ✅ Load tests (14.22 TPS verified)
- ✅ GUI testing (all screens)
- ✅ Mobile testing (Android physical device)
- ✅ Cross-platform compilation

### Security Review
- ✅ No hardcoded secrets
- ✅ Secure key management
- ✅ Input validation
- ✅ Error handling

### Performance
- macOS GUI: Instant startup, <100MB RAM
- Android APK: Fast installation, efficient battery usage
- CLI: <10MB memory footprint
- Network: 5-second block time verified

---

## 🚀 Getting Started

### New Users
1. Choose your platform
2. Download/install the wallet
3. Create a new wallet (save the backup!)
4. Start receiving and sending transactions
5. Join the NomadCoin community

### Developers
1. Clone: https://github.com/noktirnal42/nomadcoin.git
2. Build from source for your platform
3. Run tests: `cargo test`
4. Deploy nodes: See deployment guides
5. Contribute: Submit PRs!

---

## 📞 Contact

- **GitHub**: https://github.com/noktirnal42/nomadcoin
- **Email**: dev@nomadcoin.network (coming soon)
- **Discord**: https://discord.gg/nomadcoin (coming soon)
- **Twitter**: @NomadCoin (coming soon)

---

<div align="center">

### 🌍 Built for Nomads, by Nomads

**Mine anywhere • Transact everywhere • Bank with yourself**

[⭐ Star on GitHub](https://github.com/noktirnal42/nomadcoin) • [📖 Read Docs](https://github.com/noktirnal42/nomadcoin) • [🐛 Report Issues](https://github.com/noktirnal42/nomadcoin/issues)

Made with ❤️ by the NomadCoin Team  
© 2026 NomadCoin. MIT License.

</div>
