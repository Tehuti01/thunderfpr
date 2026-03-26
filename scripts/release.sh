#!/bin/bash
# LH Thunderforge - Release Builder
# Creates release packages for GitHub

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
PLUGIN_NAME="LH Thunderforge"
VERSION="${1:-v1.0.0}"
BUILD_DIR="build"
VST3_BUNDLE="LH Thunderforge.vst3"
CLAP_BUNDLE="LH Thunderforge.clap"
REPO="Tehuti01/thunderfpr"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║${NC}          ${YELLOW}LH Thunderforge Release Builder${NC}                ${BLUE}║${NC}"
echo -e "${BLUE}║${NC}          Building version: ${YELLOW}$VERSION${NC}                     ${BLUE}║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Clean build directory
echo -e "${BLUE}ℹ${NC} Cleaning build directory..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Build plugin
echo ""
echo -e "${BLUE}ℹ${NC} Building plugin (release)..."
cargo build --release

DYLIB="target/release/libthunderforge_plugin.dylib"
if [ ! -f "$DYLIB" ]; then
    echo -e "${RED}❌ Build failed: $DYLIB not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} Build successful"

# ── VST3 Bundle ──────────────────────────────────────────────
echo ""
echo -e "${BLUE}ℹ${NC} Creating VST3 bundle..."
VST3_CONTENTS="$BUILD_DIR/$VST3_BUNDLE/Contents"
mkdir -p "$VST3_CONTENTS/MacOS"
mkdir -p "$VST3_CONTENTS/Resources"

cp "$DYLIB" "$VST3_CONTENTS/MacOS/$PLUGIN_NAME"
chmod +x "$VST3_CONTENTS/MacOS/$PLUGIN_NAME"

# PkgInfo required by some DAWs
printf "BNDL????" > "$VST3_CONTENTS/PkgInfo"

cat > "$VST3_CONTENTS/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>$PLUGIN_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>capital.stradego.thunderforge</string>
    <key>CFBundleName</key>
    <string>$PLUGIN_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$PLUGIN_NAME</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
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

echo -e "${GREEN}✓${NC} VST3 bundle created"

# ── CLAP Bundle ──────────────────────────────────────────────
echo ""
echo -e "${BLUE}ℹ${NC} Creating CLAP bundle..."
CLAP_CONTENTS="$BUILD_DIR/$CLAP_BUNDLE/Contents"
mkdir -p "$CLAP_CONTENTS/MacOS"

cp "$DYLIB" "$CLAP_CONTENTS/MacOS/$PLUGIN_NAME"
chmod +x "$CLAP_CONTENTS/MacOS/$PLUGIN_NAME"

printf "BNDL????" > "$CLAP_CONTENTS/PkgInfo"

cat > "$CLAP_CONTENTS/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$PLUGIN_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>capital.stradego.thunderforge.clap</string>
    <key>CFBundleName</key>
    <string>$PLUGIN_NAME</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
</dict>
</plist>
EOF

echo -e "${GREEN}✓${NC} CLAP bundle created"

# ── Standalone ───────────────────────────────────────────────
echo ""
echo -e "${BLUE}ℹ${NC} Building standalone test app..."
cargo build --package thunderforge-standalone --release
cp target/release/thunderforge-standalone "$BUILD_DIR/LH-Thunderforge-Standalone"
echo -e "${GREEN}✓${NC} Standalone app built"

# ── ZIP packages ─────────────────────────────────────────────
echo ""
echo -e "${BLUE}ℹ${NC} Creating release packages..."

cd "$BUILD_DIR"
zip -r "LH-Thunderforge-$VERSION-macos.zip" "$VST3_BUNDLE" "$CLAP_BUNDLE" "LH-Thunderforge-Standalone"
cp "LH-Thunderforge-$VERSION-macos.zip" "LH-Thunderforge-macos.zip"
cd "$PROJECT_DIR"

echo -e "${GREEN}✓${NC} Packages created:"
ls -lh "$BUILD_DIR"/*.zip
echo ""

# ── SHA256 ───────────────────────────────────────────────────
SHA256=$(shasum -a 256 "$BUILD_DIR/LH-Thunderforge-$VERSION-macos.zip" | awk '{print $1}')
echo -e "${BLUE}ℹ${NC} SHA256: ${YELLOW}$SHA256${NC}"
echo ""

# Update Homebrew formula with real sha256
sed -i '' "s/sha256 .*/sha256 \"$SHA256\"/" homebrew-formula/lh-thunderforge.rb
sed -i '' "s|releases/download/v[^/]*/|releases/download/$VERSION/|" homebrew-formula/lh-thunderforge.rb
echo -e "${GREEN}✓${NC} Homebrew formula updated with SHA256"
echo ""

echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}            ${YELLOW}Release Build Complete!${NC}                    ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Commit the updated Homebrew formula:"
echo "     git add -A && git commit -m 'Release $VERSION'"
echo "     git push"
echo ""
echo "  2. Create GitHub release:"
echo "     gh release create $VERSION \\"
echo "       \"$BUILD_DIR/LH-Thunderforge-$VERSION-macos.zip\" \\"
echo "       --title \"LH Thunderforge $VERSION\" \\"
echo "       --notes \"See CHANGELOG.md\""
echo ""
echo "  3. Homebrew install (after release is published):"
echo "     brew install --HEAD https://raw.githubusercontent.com/$REPO/main/homebrew-formula/lh-thunderforge.rb"
echo ""
