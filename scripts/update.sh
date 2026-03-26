#!/bin/bash
# LH Thunderforge - Auto-Updater
# Checks for updates and installs them automatically

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
REPO="Tehuti01/thunderfpr"
PLUGIN_NAME="LH Thunderforge"
VST3_DIR="$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3"
CLAP_DIR="$HOME/Library/Audio/Plug-Ins/CLAP/LH Thunderforge.clap"
BACKUP_DIR="$HOME/Library/Audio/Plug-Ins/Backups"
TEMP_DIR=$(mktemp -d)

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║${NC}          ${YELLOW}LH Thunderforge Updater${NC}                        ${BLUE}║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if plugin is installed
if [ ! -d "$VST3_DIR" ] && [ ! -d "$CLAP_DIR" ]; then
    echo -e "${RED}❌ Plugin not found!${NC}"
    echo "Install first with:"
    echo "  curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/install.sh | bash"
    exit 1
fi

# Get current version (from file or assume v1.0.0)
CURRENT_VERSION="v1.0.0"
if [ -f "$VST3_DIR/Contents/Info.plist" ]; then
    # Could parse version from plist, but for now assume latest
    CURRENT_VERSION="v1.0.0"
fi

echo -e "${BLUE}ℹ${NC} Current version: ${YELLOW}$CURRENT_VERSION${NC}"

# Get latest release
echo -e "${BLUE}ℹ${NC} Checking for updates..."
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")

if [ -z "$LATEST" ]; then
    echo -e "${RED}❌ Failed to check for updates${NC}"
    exit 1
fi

# Extract latest version
LATEST_VERSION=$(echo "$LATEST" | grep -o '"tag_name": "v[^"]*"' | cut -d'"' -f4)
if [ -z "$LATEST_VERSION" ]; then
    LATEST_VERSION="v1.0.0"
fi

echo -e "${BLUE}ℹ${NC} Latest version: ${YELLOW}$LATEST_VERSION${NC}"

# Compare versions
if [ "$CURRENT_VERSION" = "$LATEST_VERSION" ]; then
    echo ""
    echo -e "${GREEN}✓${NC} You're already on the latest version!"
    echo ""
    exit 0
fi

echo ""
echo -e "${GREEN}🎉${NC} New version available: ${YELLOW}$LATEST_VERSION${NC}"
echo -e "${BLUE}ℹ${NC} Updating from $CURRENT_VERSION to $LATEST_VERSION..."
echo ""

# Create backup
if [ -d "$VST3_DIR" ]; then
    mkdir -p "$BACKUP_DIR"
    BACKUP_NAME="LH-Thunderforge-${CURRENT_VERSION}-$(date +%Y%m%d-%H%M%S)"
    echo -e "${BLUE}ℹ${NC} Creating backup: $BACKUP_NAME"
    cp -R "$VST3_DIR" "$BACKUP_DIR/$BACKUP_NAME.vst3"
    echo -e "${GREEN}✓${NC} Backup created"
    echo ""
fi

# Download update
echo -e "${BLUE}ℹ${NC} Downloading update..."
DOWNLOAD_URL=$(echo "$LATEST" | grep -o '"browser_download_url": "[^"]*VST3[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION/LH-Thunderforge-macos.zip"
fi

curl -L -o "$TEMP_DIR/update.zip" "$DOWNLOAD_URL" 2>/dev/null

if [ ! -f "$TEMP_DIR/update.zip" ]; then
    echo -e "${RED}❌ Download failed${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Downloaded"
echo ""

# Extract and install
echo -e "${BLUE}ℹ${NC} Installing update..."
unzip -q -o "$TEMP_DIR/update.zip" -d "$TEMP_DIR"

# Install VST3
if [ -d "$TEMP_DIR/LH Thunderforge.vst3" ]; then
    rm -rf "$VST3_DIR"
    cp -R "$TEMP_DIR/LH Thunderforge.vst3" "$VST3_DIR"
    echo -e "${GREEN}✓${NC} VST3 updated"
fi

# Install CLAP if available
if [ -d "$TEMP_DIR/LH Thunderforge.clap" ] && [ -d "$CLAP_DIR" ]; then
    rm -rf "$CLAP_DIR"
    cp -R "$TEMP_DIR/LH Thunderforge.clap" "$CLAP_DIR"
    echo -e "${GREEN}✓${NC} CLAP updated"
fi

# Cleanup
rm -rf "$TEMP_DIR"

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}            ${YELLOW}Update Complete!${NC}                           ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✓${NC} Successfully updated to ${YELLOW}$LATEST_VERSION${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Restart your DAW"
echo "  2. Load the updated plugin"
echo "  3. Enjoy the new features!"
echo ""
echo -e "${BLUE}ℹ${NC} Backup saved to: $BACKUP_DIR"
echo -e "${BLUE}ℹ${NC} View changelog: https://github.com/$REPO/releases/latest"
echo ""
