#!/bin/bash
# LH Thunderforge - Build Script
# Builds the plugin and bundles the UI

set -e

echo "🔨 Building LH Thunderforge..."

# Build the plugin
echo "Building plugin..."
cargo build --release

# Create UI dist directory
echo "Preparing UI..."
mkdir -p ui/dist
cp -r ui/src/* ui/dist/

# Copy plugin binary to VST3 bundle
echo "Installing plugin..."
PLUGIN_DIR="$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3"
mkdir -p "$PLUGIN_DIR/Contents/MacOS"

cp target/release/libthunderforge_plugin.dylib "$PLUGIN_DIR/Contents/MacOS/LH Thunderforge"

# Copy UI files to plugin bundle
mkdir -p "$PLUGIN_DIR/Contents/Resources/ui"
cp -r ui/dist/* "$PLUGIN_DIR/Contents/Resources/ui/"

echo ""
echo "✅ Build complete!"
echo ""
echo "Plugin installed to: $PLUGIN_DIR"
echo ""
echo "To test:"
echo "1. Restart your DAW"
echo "2. Look for 'LH Thunderforge' in the plugin list"
echo "3. Load the plugin and enjoy!"
