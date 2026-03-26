#!/bin/bash
# LH Thunderforge - Auto-Installer
# Downloads and installs the latest release from GitHub

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="Tehuti01/thunderfpr"
PLUGIN_NAME="LH Thunderforge"
INSTALL_DIR="$HOME/Library/Audio/Plug-Ins/VST3"
CLAP_INSTALL_DIR="$HOME/Library/Audio/Plug-Ins/CLAP"
TEMP_DIR=$(mktemp -d)

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║${NC}          ${YELLOW}LH Thunderforge Installer${NC}                      ${BLUE}║${NC}"
echo -e "${BLUE}║${NC}          Professional Guitar Amp Simulator         ${BLUE}║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check for curl
if ! command -v curl &> /dev/null; then
    echo -e "${RED}❌ curl is required but not installed.${NC}"
    echo "Please install curl: brew install curl"
    exit 1
fi

# Get latest release info
echo -e "${BLUE}ℹ${NC} Checking for latest release..."
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")

if [ -z "$LATEST" ]; then
    echo -e "${RED}❌ Failed to fetch latest release${NC}"
    exit 1
fi

# Extract version
VERSION=$(echo "$LATEST" | grep -o '"tag_name": "v[^"]*"' | cut -d'"' -f4)
if [ -z "$VERSION" ]; then
    VERSION="v1.0.0"
fi

echo -e "${GREEN}✓${NC} Latest version: ${YELLOW}$VERSION${NC}"
echo ""

# Download VST3
echo -e "${BLUE}ℹ${NC} Downloading VST3 plugin..."
DOWNLOAD_URL=$(echo "$LATEST" | grep -o '"browser_download_url": "[^"]*VST3[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    # Fallback to constructing URL
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/LH-Thunderforge-macos.zip"
fi

curl -L -o "$TEMP_DIR/plugin.zip" "$DOWNLOAD_URL" 2>/dev/null

if [ ! -f "$TEMP_DIR/plugin.zip" ]; then
    echo -e "${RED}❌ Download failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Downloaded successfully"
echo ""

# Extract
echo -e "${BLUE}ℹ${NC} Extracting..."
unzip -q -o "$TEMP_DIR/plugin.zip" -d "$TEMP_DIR"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Install VST3
echo -e "${BLUE}ℹ${NC} Installing VST3 plugin..."
if [ -d "$TEMP_DIR/LH Thunderforge.vst3" ]; then
    rm -rf "$INSTALL_DIR/LH Thunderforge.vst3"
    cp -R "$TEMP_DIR/LH Thunderforge.vst3" "$INSTALL_DIR/"
    echo -e "${GREEN}✓${NC} VST3 installed to: $INSTALL_DIR"
else
    echo -e "${YELLOW}⚠${NC} VST3 bundle not found in download"
fi

# Install CLAP if available
if [ -d "$TEMP_DIR/LH Thunderforge.clap" ]; then
    mkdir -p "$CLAP_INSTALL_DIR"
    rm -rf "$CLAP_INSTALL_DIR/LH Thunderforge.clap"
    cp -R "$TEMP_DIR/LH Thunderforge.clap" "$CLAP_INSTALL_DIR/"
    echo -e "${GREEN}✓${NC} CLAP installed to: $CLAP_INSTALL_DIR"
fi

echo ""

# Cleanup
rm -rf "$TEMP_DIR"

# Success message
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}            ${YELLOW}Installation Complete!${NC}                       ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}ℹ${NC} Plugin installed successfully!"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Restart your DAW (FL Studio, Logic Pro, etc.)"
echo "  2. Scan for new plugins if needed"
echo "  3. Load 'LH Thunderforge' and start playing!"
echo ""
echo -e "${BLUE}ℹ${NC} To update later, run:"
echo "   curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/update.sh | bash"
echo ""
echo -e "${BLUE}📖${NC} Documentation: https://github.com/$REPO#readme"
echo ""
