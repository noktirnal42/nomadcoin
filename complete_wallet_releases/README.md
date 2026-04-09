# 🌍 NomadCoin Complete Wallet Releases v1.0.0

Complete, ready-to-use wallet applications for all platforms. Choose your platform below to get started.

---

## 📱 Platform Releases

### 🍎 [macOS](macOS/README.md)
**Desktop GUI Wallet & CLI**
- `NomadCoin-Wallet-GUI-v1.0.0.app` - Graphical interface (recommended)
- `NomadCoin-CLI-v1.0.0` - Command-line interface

**Requirements**: macOS 11.0+  
**Download Size**: ~15MB GUI + 11MB CLI  
**Status**: ✅ Ready for use

---

### 🤖 [Android](Android/README.md)
**Mobile Wallet**
- `NomadCoin-Wallet-v1.0.0.apk` - Universal APK for all devices

**Requirements**: Android 8.0+  
**Download Size**: ~48MB  
**Status**: ✅ Ready for use

**Installation Options:**
1. Direct APK installation
2. Google Play Store (coming soon)
3. Samsung Galaxy Store (coming soon)

---

### 🍊 [iOS](iOS/README.md)
**Mobile Wallet**

**Requirements**: iOS 12.0+  
**Status**: 🔜 Coming Q2 2026

**Installation Options:**
1. Apple App Store (Q2 2026)
2. TestFlight Beta (Q2 2026)
3. Developer build from source

---

### 🐧 [Linux](Linux/README.md)
**Desktop Wallet**

**Requirements**: Ubuntu 20.04+, Fedora 33+, or Debian 11+  
**Status**: 🔜 AppImage coming Q2 2026

**Current Method:**
- Build from Flutter source

---

### 🪟 [Windows](Windows/README.md)
**Desktop Wallet**

**Requirements**: Windows 10 or 11  
**Status**: 🔜 Installer (.msi) coming Q2 2026

**Current Method:**
- Build from Flutter source

---

## 🚀 Quick Start by Platform

### macOS Users
```bash
# GUI Wallet (easiest)
cd macOS
open NomadCoin-Wallet-GUI-v1.0.0.app

# OR CLI (advanced)
./macOS/NomadCoin-CLI-v1.0.0 wallet --count 1
```

### Android Users
1. Download `Android/NomadCoin-Wallet-v1.0.0.apk`
2. Transfer to device
3. Open file manager → Downloads
4. Tap APK to install

### Linux/Windows/iOS Developers
See platform-specific README files for build instructions

---

## 💻 Feature Matrix

| Feature | macOS | Android | iOS | Linux | Windows |
|---------|-------|---------|-----|-------|---------|
| Create Wallet | ✅ | ✅ | ✅ | ✅ | ✅ |
| Import Wallet | ✅ | ✅ | ✅ | ✅ | ✅ |
| Send Transactions | ✅ | ✅ | ✅ | ✅ | ✅ |
| Receive QR Codes | ✅ | ✅ | ✅ | ✅ | ✅ |
| Transaction History | ✅ | ✅ | ✅ | ✅ | ✅ |
| Encrypted Storage | ✅ | ✅ | ✅ | ✅ | ✅ |
| Biometric Auth | ⏳ | ✅ | ✅ | ⏳ | ⏳ |
| Offline Mode | ✅ | ✅ | ✅ | ✅ | ✅ |
| Auto-Update | ⏳ | ✅ | ✅ | ⏳ | ✅ |

✅ = Available  
⏳ = Coming Soon

---

## 🔒 Security Features

All wallets include:
- **AES-256 Encryption** - Private keys encrypted locally
- **Ed25519 Signatures** - Industry-standard cryptography
- **BLAKE3 Hashing** - Secure transaction verification
- **Zero Cloud Storage** - Keys never leave your device
- **Open Source** - Full audit trail on GitHub

---

## 📋 System Requirements Summary

| Platform | OS | Version | Storage |
|----------|----|---------| --------|
| **macOS** | macOS | 11.0+ | 100MB |
| **Android** | Android | 8.0+ | 50MB |
| **iOS** | iOS | 12.0+ | 100MB |
| **Linux** | Ubuntu/Fedora | 20.04+/33+ | 150MB |
| **Windows** | Windows | 10/11 | 100MB |

---

## 📝 Installation by Platform

### macOS - GUI (Recommended)
```bash
1. Open Finder
2. Navigate to macOS/ folder
3. Drag NomadCoin-Wallet-GUI-v1.0.0.app to Applications
4. Double-click to launch
5. Grant accessibility permissions if prompted
```

### Android
```bash
1. Download NomadCoin-Wallet-v1.0.0.apk to device
2. Open file manager
3. Navigate to Downloads
4. Tap APK file
5. Grant permissions
6. Tap Install
```

### Linux (Build from Source)
```bash
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin/nomad_mobile_wallet
flutter pub get
flutter build linux --release
./build/linux/x64/release/bundle/nomad_mobile_wallet
```

### Windows (Build from Source)
```bash
# Requires Visual Studio Build Tools
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin\nomad_mobile_wallet
flutter pub get
flutter build windows --release
.\build\windows\runner\Release\nomad_mobile_wallet.exe
```

---

## 🔄 Updating Wallets

### macOS
- Check for updates in application menu
- Auto-download available in future releases

### Android
- Check Google Play Store (when available)
- Manual APK updates until store release

### Others
- Source code updates available on GitHub
- Follow rebuild instructions for your platform

---

## 🆘 Support & Troubleshooting

### Common Issues

**macOS: "App is damaged" error**
```bash
sudo xattr -rd com.apple.quarantine /Applications/NomadCoin-Wallet-GUI-v1.0.0.app
```

**Android: Installation blocked**
- Enable "Unknown Sources" in Security settings
- Check available storage space

**Linux: Missing libraries**
```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev

# Fedora
sudo dnf install gtk3-devel
```

**Windows: Visual C++ errors**
- Download Visual Studio Build Tools
- Install "Desktop development with C++"

---

## 📚 Resources

- **GitHub**: https://github.com/noktirnal42/nomadcoin
- **Documentation**: https://github.com/noktirnal42/nomadcoin/wiki
- **Issues**: https://github.com/noktirnal42/nomadcoin/issues
- **Discussions**: https://github.com/noktirnal42/nomadcoin/discussions
- **Community**: https://discord.gg/nomadcoin (coming soon)

---

## 📊 Release Information

**Version**: 1.0.0  
**Release Date**: April 9, 2026  
**Status**: Testnet Ready  
**License**: MIT

### Build Details
- **Rust Core**: 1.70+
- **Flutter**: 3.41+
- **Target**: Cross-platform mobile-first cryptocurrency

---

## 🙏 Contributing

Help improve NomadCoin:
1. Fork on GitHub
2. Report bugs via Issues
3. Submit code via Pull Requests
4. Join community discussions

---

<div align="center">

### 🌍 NomadCoin - Built for Nomads, by Nomads

**Mine anywhere • Transact everywhere • Bank with yourself**

[⭐ Star on GitHub](https://github.com/noktirnal42/nomadcoin) • [📖 Read Docs](https://github.com/noktirnal42/nomadcoin/wiki) • [🐛 Report Issues](https://github.com/noktirnal42/nomadcoin/issues)

Made with ❤️ by the NomadCoin Team

</div>
