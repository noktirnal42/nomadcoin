#!/bin/bash
# Build NomadCoin macOS .app bundle
# Run from nomad_coin directory

set -e

echo "🏗️  Building NomadCoin macOS App..."

# Build in release mode for smaller binary
echo "📦 Building Release..."
cargo build --release --features gui --bin nomadcoin-gui

# Create .app structure
APPNAME="NomadCoin.app"
CONTENTS="$APPNAME/Contents"
MACOS="$CONTENTS/MacOS"
RESOURCES="$CONTENTS/Resources"

echo "📁 Creating App Bundle..."
rm -rf "$APPNAME"
mkdir -p "$MACOS" "$RESOURCES"

# Copy binary
cp target/release/nomadcoin-gui "$MACOS/NomadCoin"

# Create Info.plist
cat > "$CONTENTS/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>NomadCoin</string>
    <key>CFBundleIdentifier</key>
    <string>network.nomadcoin.app</string>
    <key>CFBundleName</key>
    <string>NomadCoin</string>
    <key>CFBundleDisplayName</key>
    <string>NomadCoin</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string></string>
    <key>CFBundleIconName</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.finance</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2026 NomadCoin. All rights reserved.</string>
    <key>CFBundleDocumentTypes</key>
    <array/>
</dict>
</plist>
EOF

# Create PkgInfo
echo -n "APPL????" > "$CONTENTS/PkgInfo"

# Set executable
chmod +x "$MACOS/NomadCoin"

echo "✅ App bundle created: $APPNAME"
echo ""
echo "To run:"
echo "  open $APPNAME"
echo ""
echo "Or drag to Applications:"
echo "  cp -r $APPNAME /Applications/"