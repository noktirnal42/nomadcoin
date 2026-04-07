# 🌍 NomadCoin

<div align="center">
  <img src="assets/logo.svg" alt="NomadCoin Logo" width="200" height="200"/>
  
  ### 💰 A Mobile-First Cryptocurrency for the Nomadic Community
  
  [![Build Status](https://github.com/noktirnal42/nomadcoin/actions/workflows/rust.yml/badge.svg)](https://github.com/noktirnal42/nomadcoin/actions)
  [![License: MIT](https://img.shields.io/badge/License-MIT-00BCD4.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/Rust-1.70+-000000?logo=rust)](https://www.rust-lang.org/)
  [![Flutter](https://img.shields.io/badge/Flutter-3.0+-02569B?logo=flutter)](https://flutter.dev/)
  [![Platform](https://img.shields.io/badge/Platform-iOS%20%7C%20Android%20%7C%20macOS%20%7C%20Linux%20%7C%20Windows-7C4DFF)]()
  [![Stars](https://img.shields.io/github/stars/noktirnal42/nomadcoin?style=social)](https://github.com/noktirnal42/nomadcoin)

  **⛓️ Mine on the go • 📱 Offline transactions • 🏕️ Built for nomads**
</div>

---

## 🎯 What is NomadCoin?

NomadCoin (**NOMAD**) is a cryptocurrency designed specifically for the global nomadic community - digital nomads, remote workers, travelers, and anyone who lives life on the move.

### ✨ Key Features

| Feature | Description |
|---------|-------------|
| 📱 **Mobile Mining** | Mine NOMAD directly from your phone with optimized power usage |
| 🏃 **Mobile Boost** | Get **1.5x rewards** when mining from mobile devices |
| 📴 **Offline Mode** | Send transactions without internet via mesh networking |
| 🔗 **Mesh Network** | Peer-to-peer transactions between nearby devices |
| ⚡ **Fast Consensus** | NomadPOS - lightweight Proof-of-Stake for mobile |
| 🔒 **Secure** | Ed25519 signatures + BLAKE3 hashing |
| 🌐 **Cross-Platform** | iOS, Android, macOS, Linux, Windows |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     NomadCoin Ecosystem                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  iOS App     │  │ Android App  │  │ Desktop GUI  │      │
│  │  (Swift)     │  │  (Kotlin)    │  │  (egui/Rust) │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                 │               │
│  ┌──────┴─────────────────┴─────────────────┴───────┐      │
│  │              NomadCoin Core (Rust)                │      │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌────────┐  │      │
│  │  │Blockchain│ │Consensus│ │  Wallet  │ │Miner   │  │      │
│  │  └─────────┘ └─────────┘ └─────────┘ └────────┘  │      │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌────────┐  │      │
│  │  │ Storage │ │ Network │ │  Mesh    │ │Crypto  │  │      │
│  │  └─────────┘ └─────────┘ └─────────┘ └────────┘  │      │
│  └───────────────────────────────────────────────────┘      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70+ ([install](https://rustup.rs/))
- **Flutter** 3.0+ (for mobile apps)
- **Xcode** (for iOS/macOS builds)
- **Android SDK** (for Android builds)

### Install Rust Core

```bash
# Clone the repository
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin

# Build
cargo build --release

# Run CLI
cargo run --bin nomadcoin -- --help
```

### CLI Commands

```bash
# Create a wallet
cargo run --bin nomadcoin -- wallet --count 3

# Initialize blockchain
cargo run --bin nomadcoin -- init --allocation 10000000 --data-dir ./testnet

# Start mining (continuous)
cargo run --bin nomadcoin -- mine --address nomad1youraddress --continuous

# Run a full node
cargo run --bin nomadcoin -- node --port 9333 --data-dir ./testnet

# Register as validator
cargo run --bin nomadcoin -- register-validator --address nomad1youraddress --stake 1000 --mobile

# Check status
cargo run --bin nomadcoin -- status --data-dir ./testnet
```

### Build Mobile Apps

```bash
cd mobile/nomadcoin

# Get dependencies
flutter pub get

# Build for iOS
flutter build ios

# Build for Android
flutter build apk

# Build for web
flutter build web
```

### Build Desktop GUI

```bash
# Build with GUI
cargo build --release --features gui --bin nomadcoin-gui

# Run GUI
cargo run --release --features gui --bin nomadcoin-gui

# Build macOS .app bundle
./scripts/build-app.sh
```

---

## 📱 Mobile Mining

NomadCoin rewards mobile mining with a **1.5x boost** to encourage participation from the nomadic community:

| Device Type | Boost Multiplier | Rewards per Validation |
|-------------|-----------------|----------------------|
| 📱 iOS | **1.5x** | 0.015 NOMAD |
| 📱 Android | **1.5x** | 0.015 NOMAD |
| 💻 macOS | 1.0x | 0.010 NOMAD |
| 🐧 Linux | 1.0x | 0.010 NOMAD |
| 🪟 Windows | 1.0x | 0.010 NOMAD |

---

## 🏗️ Tech Stack

| Component | Technology |
|-----------|-----------|
| **Core Blockchain** | Rust 🦀 |
| **Consensus** | NomadPOS (Proof-of-Stake) |
| **Cryptography** | Ed25519 + BLAKE3 |
| **Storage** | RocksDB |
| **Networking** | QUIC (quinn) |
| **Desktop GUI** | egui/eframe |
| **Mobile Apps** | Flutter |
| **CI/CD** | GitHub Actions |

---

## 📊 Tokenomics

- **Total Supply**: 10,000,000 NOMAD
- **Genesis Allocation**: 100% to community
- **Mining Reward**: 0.01 NOMAD per validation
- **Mobile Boost**: 1.5x for mobile devices
- **Block Time**: ~5 seconds
- **Consensus**: NomadPOS with mobile validator boost

---

## 🤝 Contributing

We welcome contributions from the community! Here's how to get started:

1. **Fork** the repository
2. **Create** your feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust clippy guidelines (`cargo clippy`)
- Write tests for new features
- Update documentation
- Use conventional commits

```bash
# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt
```

---

## 📚 Documentation

- [📖 Whitepaper](docs/05_whitepaper.md) - Technical deep dive
- [🗺️ Roadmap](docs/02_development_roadmap.md) - Development plans
- [💰 Tokenomics](docs/04_tokenomics.md) - Economic model
- [🤖 AI Strategy](docs/05_ai_strategy.md) - AI integration plans
- [💸 Cost Analysis](docs/06_cost_analysis.md) - Infrastructure costs
- [🏗️ Build Guide](docs/08_build_and_run.md) - Detailed build instructions

---

## 🌐 Community

- **Website**: [nomadcoin.network](https://nomadcoin.network) (coming soon)
- **Discord**: [Join our server](https://discord.gg/nomadcoin) (coming soon)
- **Twitter**: [@NomadCoin](https://twitter.com/nomadcoin) (coming soon)
- **Reddit**: [r/NomadCoin](https://reddit.com/r/nomadcoin) (coming soon)

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- The Rust community for amazing crates
- Flutter team for cross-platform mobile development
- The global nomadic community for inspiration

---

<div align="center">
  <sub>Built with ❤️ by the NomadCoin Team</sub>
  <br>
  <sub>🌍 For nomads, by nomads</sub>
</div>
