#!/bin/bash
# LH Thunderforge - Build Script
# Builds the plugin and creates VST3/CLAP bundles

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

echo "🔨 Building LH Thunderforge..."

# Build the plugin
echo "Building plugin (release)..."
cargo build --release

DYLIB="target/release/libthunderforge_plugin.dylib"
if [ ! -f "$DYLIB" ]; then
    echo "❌ Build failed: $DYLIB not found"
    exit 1
fi

# --- VST3 Bundle ---
VST3_DIR="$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/Contents"
echo ""
echo "Installing VST3 bundle..."
mkdir -p "$VST3_DIR/MacOS"

cp "$DYLIB" "$VST3_DIR/MacOS/LH Thunderforge"
chmod +x "$VST3_DIR/MacOS/LH Thunderforge"

printf "BNDL????" > "$VST3_DIR/PkgInfo"

cat > "$VST3_DIR/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>LH Thunderforge</string>
    <key>CFBundleIdentifier</key>
    <string>capital.stradego.thunderforge</string>
    <key>CFBundleName</key>
    <string>LH Thunderforge</string>
    <key>CFBundleDisplayName</key>
    <string>LH Thunderforge</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CSResourcesFileMapped</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo "✅ VST3 installed to: $HOME/Library/Audio/Plug-Ins/VST3/"

# --- CLAP Bundle ---
CLAP_DIR="$HOME/Library/Audio/Plug-Ins/CLAP/LH Thunderforge.clap/Contents"
echo ""
echo "Installing CLAP bundle..."
mkdir -p "$CLAP_DIR/MacOS"

cp "$DYLIB" "$CLAP_DIR/MacOS/LH Thunderforge"
chmod +x "$CLAP_DIR/MacOS/LH Thunderforge"

printf "BNDL????" > "$CLAP_DIR/PkgInfo"

cat > "$CLAP_DIR/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>LH Thunderforge</string>
    <key>CFBundleIdentifier</key>
    <string>capital.stradego.thunderforge.clap</string>
    <key>CFBundleName</key>
    <string>LH Thunderforge</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
</dict>
</plist>
EOF

echo "✅ CLAP installed to: $HOME/Library/Audio/Plug-Ins/CLAP/"

echo ""
echo "╔══════════════════════════════════════╗"
echo "║  ✅ Build & Install Complete!        ║"
echo "╚══════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "  1. Restart your DAW"
echo "  2. Scan for new plugins"
echo "  3. Load 'LH Thunderforge' on a mixer track"
echo ""
