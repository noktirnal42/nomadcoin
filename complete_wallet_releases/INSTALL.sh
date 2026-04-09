#!/bin/bash

# NomadCoin Wallet Installation Script
# Detects OS and runs appropriate installation

OS_TYPE=$(uname -s)

echo "🌍 NomadCoin Wallet Installer v1.0.0"
echo "=================================="
echo ""

case "$OS_TYPE" in
    Darwin)
        echo "🍎 macOS detected"
        echo ""
        echo "Installation options:"
        echo "1) GUI Wallet (recommended)"
        echo "2) CLI Wallet (advanced)"
        echo ""
        read -p "Select option (1-2): " choice
        
        case $choice in
            1)
                echo "📦 Installing GUI Wallet..."
                if [ -d "macOS/NomadCoin-Wallet-GUI-v1.0.0.app" ]; then
                    cp -r macOS/NomadCoin-Wallet-GUI-v1.0.0.app ~/Applications/ 2>/dev/null || sudo cp -r macOS/NomadCoin-Wallet-GUI-v1.0.0.app /Applications/
                    echo "✅ GUI Wallet installed!"
                    echo "📍 Location: /Applications/NomadCoin-Wallet-GUI-v1.0.0.app"
                    echo "🚀 Launch it from Applications folder or Spotlight"
                else
                    echo "❌ GUI Wallet not found!"
                fi
                ;;
            2)
                echo "📦 Installing CLI Wallet..."
                if [ -f "macOS/NomadCoin-CLI-v1.0.0" ]; then
                    cp macOS/NomadCoin-CLI-v1.0.0 ~/bin/nomadcoin 2>/dev/null || sudo cp macOS/NomadCoin-CLI-v1.0.0 /usr/local/bin/nomadcoin
                    chmod +x ~/bin/nomadcoin 2>/dev/null || sudo chmod +x /usr/local/bin/nomadcoin
                    echo "✅ CLI Wallet installed!"
                    echo "🚀 Run: nomadcoin --help"
                else
                    echo "❌ CLI Wallet not found!"
                fi
                ;;
            *)
                echo "❌ Invalid option"
                exit 1
                ;;
        esac
        ;;
    Linux)
        echo "🐧 Linux detected"
        echo ""
        echo "⏳ AppImage coming soon!"
        echo "📖 For now, build from source:"
        echo ""
        echo "git clone https://github.com/noktirnal42/nomadcoin.git"
        echo "cd nomadcoin/nomad_mobile_wallet"
        echo "flutter build linux --release"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "🪟 Windows detected"
        echo ""
        echo "⏳ Installer coming soon!"
        echo "📖 For now, build from source:"
        echo ""
        echo "git clone https://github.com/noktirnal42/nomadcoin.git"
        echo "cd nomadcoin\nomad_mobile_wallet"
        echo "flutter build windows --release"
        ;;
    *)
        echo "❌ Unsupported OS: $OS_TYPE"
        exit 1
        ;;
esac

echo ""
echo "📚 Documentation: https://github.com/noktirnal42/nomadcoin"
echo "🐛 Issues: https://github.com/noktirnal42/nomadcoin/issues"
echo ""
