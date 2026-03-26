#!/bin/bash
# LH Thunderforge - Standalone Test Runner
# Test the plugin without a DAW!

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🎸 Building LH Thunderforge Standalone..."

# Build
cargo build --package thunderforge-standalone --release

echo ""
echo "✅ Build complete! Starting standalone test app..."
echo ""
echo "🎹 Controls:"
echo "  - Drag knobs to adjust values"
echo "  - Click buttons to toggle effects"
echo "  - Use ◀ ▶ to switch presets"
echo "  - Close window to exit"
echo ""

# Run
./target/release/thunderforge-standalone
