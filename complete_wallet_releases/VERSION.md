# NomadCoin Wallet Releases v1.0.3

**Release Date**: April 8, 2026  
**Status**: Testnet Ready with Mining Balance & Multi-Address Support

## What's New in v1.0.3

### ✨ New Features
- **Mining Rewards Persist**: Mining rewards now properly update wallet balance
- **Multiple Addresses**: Generate new addresses within the same wallet
- **Address Management**: Switch between addresses with dropdown selector
- **Add_Balance Method**: Blockchain now has persistent reward mechanism

### 🔧 Critical Fixes
- ✅ **MAJOR**: Mining earnings now persist to blockchain storage (wallet balance reflects actual mining)
- ✅ Generated addresses stored and selectable across app restarts
- ✅ Mining loop now writes rewards every 500ms validation cycle
- ✅ Flutter wallet tracks multiple addresses in SharedPreferences

### 📱 Platform Releases

#### macOS
- **NomadCoin-Wallet-GUI-v1.0.2** (18MB)
  - Desktop graphical wallet with mining persistence
  - Mining rewards now update wallet balance in real-time
  - Network selector (TESTNET/MAINNET)
  - Image-based QR codes

- **NomadCoin-CLI-v1.0.2** (11MB)
  - Command-line wallet with all features
  - Full transaction support
  - Advanced user tool

#### Android
- **NomadCoin-Wallet-v1.0.3.apk** (50MB)
  - Mobile wallet with address management
  - Generate and select multiple addresses
  - Real-time mining balance updates
  - 1.5x mining boost on mobile

#### iOS
- Full source code with all v1.0.3 features

---

## Key Improvements

| Component | v1.0.0 | v1.0.1 | v1.0.2 | v1.0.3 |
|-----------|--------|--------|--------|--------|
| QR Codes | ASCII | ✅ Image | ✅ Image | ✅ Image |
| Network Mode | Hardcoded | ✅ Selectable | ✅ Works | ✅ Works |
| Balance Updates | Static | ✅ Real-time | ✅ Real-time | ✅ Real-time |
| Mining Rewards | Not visible | ✅ Display | ❌ In-memory only | ✅ Persist to blockchain |
| Multiple Addresses | ❌ | ❌ | ❌ | ✅ Generate & Select |
| Blockchain Persistence | ❌ | ❌ | ❌ | ✅ Mining rewards saved |

---

## Installation

### Quick Start
```bash
# macOS GUI (easiest)
open complete_wallet_releases/macOS/NomadCoin-Wallet-GUI-v1.0.2

# Android
# Transfer APK and tap to install

# Automated installer
./complete_wallet_releases/INSTALL.sh
```

---

## Known Issues Fixed
- ✅ Mining balance not persisting to blockchain
- ✅ No way to generate multiple addresses
- ✅ Address selection limited to single address
- ✅ Mining rewards only showing in-memory

---

## Technical Details

### src/blockchain.rs Changes
```rust
pub fn add_balance(&mut self, address: &str, amount: f64) {
    let current = self.get_balance(address);
    self.state.balances.insert(address.to_string(), current + amount);
}
```

### src/gui.rs Mining Logic
- Previous earnings tracked separately
- Every 500ms validation: calculates reward_earned = current - previous
- Writes reward to blockchain storage via add_balance()
- Updates previous_earnings to prevent double-counting
- Balance immediately updates from blockchain sync

### Flutter Provider Changes
- addresses: Vec<String> for multiple addresses
- selectedAddressIndex for current selection
- generateNewAddress() creates and stores new address
- selectAddress(index) switches between addresses
- All addresses persisted to SharedPreferences

---

## Testing Checklist
- ✅ macOS GUI builds with mining persistence
- ✅ Mining rewards update wallet balance in real-time
- ✅ Network selector works
- ✅ QR codes display as images
- ✅ Android v1.0.3 APK builds
- ✅ Address generation creates unique addresses
- ✅ Address selection switches properly
- ✅ Cross-platform functionality consistent

---

Built with ❤️ for nomads  
© 2026 NomadCoin. MIT License.
