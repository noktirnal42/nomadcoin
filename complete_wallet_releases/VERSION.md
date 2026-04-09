# NomadCoin Wallet Releases v1.0.1

**Release Date**: April 9, 2026 (Updated)  
**Status**: Testnet & Mainnet Ready

## What's New in v1.0.1

### ✨ Features Added
- **Network Selection**: Toggle between TESTNET and MAINNET in GUI
- **Real-Time Balance Updates**: Balance refreshes every 2 seconds
- **Image-Based QR Codes**: Proper QR code images instead of ASCII
- **Active Mining Display**: See balance increase as you mine

### 🔧 Fixes
- Fixed GUI QR code rendering (was ASCII, now images)
- Fixed wallet balance not updating from mining rewards
- Added network mode selector to header
- Improved blockchain state synchronization

### 📱 Platform Releases

#### macOS
- **NomadCoin-Wallet-GUI-v1.0.1.app** (15MB)
  - Desktop graphical wallet
  - Network selector (TESTNET/MAINNET)
  - Real-time balance updates
  - Image-based QR codes

- **NomadCoin-CLI-v1.0.1** (11MB)
  - Command-line wallet
  - Full transaction support
  - Advanced user tool

#### Android
- **NomadCoin-Wallet-v1.0.1.apk** (48MB)
  - Mobile wallet
  - Optimized for on-the-go use
  - 1.5x mining boost on mobile

#### iOS (Coming Soon)
- Build from source or wait for App Store

#### Linux (Coming Soon)
- AppImage available soon

#### Windows (Coming Soon)
- .msi installer available soon

---

## Key Improvements

| Component | v1.0.0 | v1.0.1 |
|-----------|--------|--------|
| QR Codes | ASCII | ✅ Image |
| Network Mode | Hardcoded | ✅ Selectable |
| Balance Updates | Static | ✅ Real-time |
| Mining Rewards | Not visible | ✅ Instant |
| Testnet Support | ✅ | ✅ Enhanced |

---

## Installation

### Quick Start
```bash
# macOS GUI (easiest)
open complete_wallet_releases/macOS/NomadCoin-Wallet-GUI-v1.0.1.app

# Android
# Transfer APK and tap to install

# Automated installer
./complete_wallet_releases/INSTALL.sh
```

---

## Known Issues Fixed
- ✅ GUI QR codes showing as ASCII
- ✅ Wallet balance not reflecting mining rewards
- ✅ No way to switch between testnet/mainnet
- ✅ Balance updates only on app restart

---

## Testing Checklist
- ✅ macOS GUI builds and runs
- ✅ Network selector works
- ✅ QR codes display as images
- ✅ Balance updates in real-time
- ✅ Mining rewards visible
- ✅ Android APK builds
- ✅ Cross-platform functionality

---

Built with ❤️ for nomads  
© 2026 NomadCoin. MIT License.
