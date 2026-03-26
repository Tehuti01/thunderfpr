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
VERSION="v1.0.0"
BUILD_DIR="build/release"
VST3_BUNDLE="LH Thunderforge.vst3"
CLAP_BUNDLE="LH Thunderforge.clap"

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║${NC}          ${YELLOW}LH Thunderforge Release Builder${NC}                ${BLUE}║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check version argument
if [ -n "$1" ]; then
    VERSION="$1"
    echo -e "${BLUE}ℹ${NC} Building version: ${YELLOW}$VERSION${NC}"
else
    echo -e "${BLUE}ℹ${NC} Building version: ${YELLOW}$VERSION${NC}"
fi

# Clean build directory
echo ""
echo -e "${BLUE}ℹ${NC} Cleaning build directory..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Build plugin
echo ""
echo -e "${BLUE}ℹ${NC} Building plugin..."
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Build successful"

# Create VST3 bundle
echo ""
echo -e "${BLUE}ℹ${NC} Creating VST3 bundle..."
mkdir -p "$BUILD_DIR/$VST3_BUNDLE/Contents/MacOS"
mkdir -p "$BUILD_DIR/$VST3_BUNDLE/Contents/Resources"

# Copy binary
cp target/release/libthunderforge_plugin.dylib "$BUILD_DIR/$VST3_BUNDLE/Contents/MacOS/LH Thunderforge"

# Create Info.plist
cat > "$BUILD_DIR/$VST3_BUNDLE/Contents/Info.plist" << EOF
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
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CSResourcesFileMapped</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>LSRequiresCarbon</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo -e "${GREEN}✓${NC} VST3 bundle created"

# Create CLAP bundle (optional)
echo ""
echo -e "${BLUE}ℹ${NC} Creating CLAP bundle..."
mkdir -p "$BUILD_DIR/$CLAP_BUNDLE/Contents/MacOS"
mkdir -p "$BUILD_DIR/$CLAP_BUNDLE/Contents/Resources"

cp target/release/libthunderforge_plugin.dylib "$BUILD_DIR/$CLAP_BUNDLE/Contents/MacOS/LH Thunderforge.clap"

cat > "$BUILD_DIR/$CLAP_BUNDLE/Contents/Info.plist" << EOF
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
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
</dict>
</plist>
EOF

echo -e "${GREEN}✓${NC} CLAP bundle created"

# Create ZIP packages
echo ""
echo -e "${BLUE}ℹ${NC} Creating release packages..."

# VST3 ZIP
cd "$BUILD_DIR"
zip -r "../LH-Thunderforge-macos.zip" "$VST3_BUNDLE"
echo -e "${GREEN}✓${NC} Created: LH-Thunderforge-macos.zip"

# CLAP ZIP
zip -r "../LH-Thunderforge-CLAP-macos.zip" "$CLAP_BUNDLE"
echo -e "${GREEN}✓${NC} Created: LH-Thunderforge-CLAP-macos.zip"

# Combined ZIP
zip -r "../LH-Thunderforge-$VERSION-macos.zip" "$VST3_BUNDLE" "$CLAP_BUNDLE"
echo -e "${GREEN}✓${NC} Created: LH-Thunderforge-$VERSION-macos.zip"

cd ../..

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}            ${YELLOW}Release Build Complete!${NC}                    ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}ℹ${NC} Release packages created in: $BUILD_DIR/"
echo ""
echo -e "${YELLOW}Files:${NC}"
ls -lh "$BUILD_DIR"/*.zip
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Test the plugins from $BUILD_DIR/"
echo "  2. Create GitHub release:"
echo "     gh release create $VERSION $BUILD_DIR/*.zip --title \"LH Thunderforge $VERSION\" --notes \"See CHANGELOG.md for details\""
echo "  3. Or upload manually at: https://github.com/$REPO/releases/new"
echo ""
