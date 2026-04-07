# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Continuous mining mode (`--continuous` flag)
- Auto-detection of device type (macOS, Linux, Windows, iOS, Android)
- Desktop GUI application with wallet, miner, send, and community tabs
- QR code display for wallet addresses
- macOS .app bundle build script
- GitHub Actions CI/CD workflows
- Comprehensive README with branding
- CONTRIBUTING.md guide
- BRANDING.md style guide

### Changed
- Fixed all compiler warnings
- Improved miner output formatting
- Updated Flutter dependencies

## [0.1.0] - 2026-04-07

### Added
- Rust core blockchain implementation
- NomadPOS consensus mechanism with mobile validator boost (1.5x)
- Ed25519 cryptography + BLAKE3 hashing
- RocksDB persistent storage
- QUIC P2P networking
- Mesh networking for offline transactions
- Mobile miner service with device-specific boosts
- Flutter mobile app structure (iOS + Android)
- CLI with 7 commands: wallet, send, init, mine, node, register-validator, status
- 44 comprehensive unit tests
- 8 documentation files

[Unreleased]: https://github.com/noktirnal42/nomadcoin/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/noktirnal42/nomadcoin/releases/tag/v0.1.0
