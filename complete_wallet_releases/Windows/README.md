# NomadCoin Windows Wallet v1.0.0

## Installation

### Build from Source
Windows wallet can be built from Flutter source:

```bash
# Clone repository
git clone https://github.com/noktirnal42/nomadcoin.git
cd nomadcoin\nomad_mobile_wallet

# Install dependencies
flutter pub get

# Build for Windows
flutter build windows --release

# Run
.\build\windows\runner\Release\nomad_mobile_wallet.exe
```

### Installer (Coming Soon)
- Windows .msi installer: Coming Q2 2026
- Automatic updates included
- System-wide installation

### Portable Version
- No installation required
- Run from USB drive
- Perfect for nomads!

## Features
- 💻 Native Windows application
- 💰 Complete wallet functionality
- 📤 Send transactions
- 📥 Receive with QR codes
- 🔐 Windows credential storage
- ⚡ Fast performance
- 🔄 Auto-update support

## System Requirements
- Windows 10 or 11
- 64-bit processor
- 100MB free storage
- .NET Framework 4.6+

## Installation Steps

1. **Install Visual Studio Build Tools** (required)
   - Download: https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++"

2. **Install Flutter**
   - Download: https://flutter.dev/docs/get-started/install/windows
   - Add Flutter to PATH

3. **Build Application**
   ```bash
   git clone https://github.com/noktirnal42/nomadcoin.git
   cd nomadcoin\nomad_mobile_wallet
   flutter pub get
   flutter build windows --release
   ```

4. **Run**
   ```bash
   .\build\windows\runner\Release\nomad_mobile_wallet.exe
   ```

## Troubleshooting
- **"Flutter is not recognized"**: Add Flutter to PATH environment variable
- **Build fails**: Ensure Visual C++ build tools are installed
- **Missing libraries**: Install latest Windows updates

## Windows Defender
- Windows Defender may warn on first run (normal for new apps)
- Click "More info" → "Run anyway"
- App is fully open-source and safe

## Security Features
- Private keys never leave device
- AES-256 encryption
- Windows Credential Manager integration
- Ed25519 cryptography

## Support
- GitHub: https://github.com/noktirnal42/nomadcoin
- Issues: https://github.com/noktirnal42/nomadcoin/issues
- Discussions: https://github.com/noktirnal42/nomadcoin/discussions

---
Built with ❤️ for nomads
