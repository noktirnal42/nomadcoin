# 📚 NomadCoin Documentation

> Complete documentation for the NomadCoin ecosystem

![NomadCoin](https://img.shields.io/badge/NOMAD-Cryptocurrency-00BCD4)
![License](https://img.shields.io/badge/License-MIT-7C4DFF)
![Status](https://img.shields.io/badge/Status-v0.1.0%20Release-4CAF50)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
![Tests](https://img.shields.io/badge/Tests-44%20Passing-brightgreen)

---

## Vision

NomadCoin enables seamless peer-to-peer transactions for nomads worldwide, regardless of internet connectivity, through innovative mesh networking and lightweight blockchain technology.

## Core Features

- **Mobile-First Mining**: Nomad miners that run on any desktop or mobile platform
- **Offline Transactions**: Transact without internet via mesh networking
- **Privacy by Default**: Enhanced privacy features for financial autonomy
- **Borderless Design**: No geographic restrictions or banking dependencies
- **Energy Efficient**: No energy-intensive proof-of-work mining

## Quick Stats

| Parameter | Value |
|-----------|-------|
| Token Symbol | NOMAD |
| Total Supply | 100,000,000 NOMAD |
| Block Time | 5 seconds |
| Consensus | NomadPOS (Proof-of-Stake) |
| Max TPS | 1,000+ |
| Tests | 26 passing ✅ |

## Documentation

All project documentation is located in the `docs/` directory:

| Document | Description |
|----------|-------------|
| [01_executive_summary.md](docs/01_executive_summary.md) | High-level project overview |
| [02_technical_specification.md](docs/02_technical_specification.md) | Detailed technical specifications |
| [03_development_guide.md](docs/03_development_guide.md) | Step-by-step development instructions |
| [04_development_plan.md](docs/04_development_plan.md) | Timeline and milestone planning |
| [05_ai_development_strategy.md](docs/05_ai_development_strategy.md) | AI agent integration strategy |
| [06_cost_estimation.md](docs/06_cost_estimation.md) | Budget and cost analysis |
| [07_whitepaper.md](docs/07_whitepaper.md) | Project whitepaper |
| [08_build_and_run.md](docs/08_build_and_run.md) | Build, run, and deploy guide |

## Technology Stack

### Core Blockchain (Rust)
- **Language**: Rust 2021
- **Hashing**: BLAKE3
- **Signatures**: Ed25519 (dalek)
- **Serialization**: serde + serde_json
- **Async**: Tokio
- **CLI**: clap
- **Database**: RocksDB
- **Networking**: QUIC

### Mobile Applications (Flutter)
- **Framework**: Flutter 3.x
- **Language**: Dart
- **State Management**: Provider
- **HTTP**: Dio
- **Storage**: shared_preferences, flutter_secure_storage

### Mesh Networking
- **Protocol**: Reticulum-based custom mesh
- **Communication**: Bluetooth, WiFi Direct, LoRa
- **Offline Storage**: Local transaction queue

## Getting Started

### Quick Start - Rust Core

```bash
# Clone and build
cd ~/dev/nomad_coin
cargo build

# Run tests
cargo test

# Generate wallet
cargo run -- wallet --count 3

# Initialize blockchain
cargo run -- init --chain-id nomadcoin --allocation 10000000

# Start mining
cargo run -- mine --address nomad1test --device android

# Run node
cargo run -- node --port 9333
```

### Quick Start - Flutter Mobile

```bash
cd mobile/nomadcoin_miner
flutter pub get
flutter run
```

See [08_build_and_run.md](docs/08_build_and_run.md) for complete build instructions.

## Project Structure

```
nomad_coin/
├── Cargo.toml                          # Rust project manifest
├── src/
│   ├── main.rs                         # CLI entry point
│   ├── types.rs                        # Core data structures
│   ├── crypto.rs                       # Cryptographic operations (Ed25519, BLAKE3)
│   ├── blockchain.rs                   # Blockchain implementation
│   ├── wallet.rs                       # Wallet management
│   ├── mesh.rs                         # Mesh networking
│   └── miner.rs                        # Mobile miner service
├── mobile/
│   └── nomadcoin_miner/                # Flutter mobile app
│       ├── pubspec.yaml
│       └── lib/
│           ├── main.dart               # App entry point
│           ├── screens/
│           │   ├── miner_screen.dart   # Mining UI
│           │   ├── wallet_screen.dart  # Wallet UI
│           │   └── settings_screen.dart # Settings UI
│           └── services/
│               ├── miner_service.dart  # Mining logic
│               └── wallet_service.dart # Wallet logic
├── docs/                               # Documentation
│   ├── 01_executive_summary.md
│   ├── 02_technical_specification.md
│   ├── 03_development_guide.md
│   ├── 04_development_plan.md
│   ├── 05_ai_development_strategy.md
│   ├── 06_cost_estimation.md
│   ├── 07_whitepaper.md
│   └── 08_build_and_run.md
└── target/                             # Build artifacts
```

## Development Roadmap

| Phase | Timeline | Status | Description |
|-------|----------|--------|-------------|
| Phase 1 | Weeks 1-4 | ✅ Done | Foundation & Specification |
| Phase 2 | Weeks 5-14 | 🔄 In Progress | Core Protocol Development |
| Phase 3 | Weeks 15-22 | ⏳ Pending | Mesh Networking |
| Phase 4 | Weeks 23-34 | ⏳ Pending | Mobile Wallet & Miner |
| Phase 5 | Weeks 35-42 | ⏳ Pending | Testing & Security Audit |
| Phase 6 | Weeks 43-48 | ⏳ Pending | Mainnet Launch |

**Estimated Timeline**: 48 weeks (approximately 12 months)

## Budget

**Estimated Development Cost**: $330,000 - $450,000

See [06_cost_estimation.md](docs/06_cost_estimation.md) for detailed breakdown.

## Contributing

This project is in active development. Contributions welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test && flutter test`
5. Submit a pull request

## License

MIT License - See LICENSE file for details.

## Contact

- **Website**: https://nomadcoin.network (placeholder)
- **Email**: team@nomadcoin.network (placeholder)
- **Telegram**: https://t.me/nomadcoin (placeholder)
- **Discord**: https://discord.gg/nomadcoin (placeholder)

---

*NomadCoin - Finance for the Free-Spirited*