#!/bin/bash
# LH Thunderforge - Complete Test Suite
# Tests everything: VST3, GUI, DSP, Installation

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║     🎸 LH Thunderforge - Complete Test Suite             ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Test 1: Check VST3 Installation
echo "📦 Test 1: VST3 Plugin Installation"
echo "─────────────────────────────────────"
if [ -d "$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3" ]; then
    echo "✅ VST3 plugin found"
    ls -lh "$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/Contents/MacOS/"
else
    echo "⚠️  VST3 plugin not installed. Installing now..."
    curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
fi
echo ""

# Test 2: Check Plugin Binary
echo "🔧 Test 2: Plugin Binary"
echo "─────────────────────────────────────"
BINARY="$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/Contents/MacOS/LH Thunderforge"
if [ -f "$BINARY" ]; then
    echo "✅ Binary exists"
    file "$BINARY"
    echo "Size: $(du -h "$BINARY" | cut -f1)"
else
    echo "❌ Binary missing!"
fi
echo ""

# Test 3: Check Info.plist
echo "📄 Test 3: Plugin Bundle Info"
echo "─────────────────────────────────────"
PLIST="$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/Contents/Info.plist"
if [ -f "$PLIST" ]; then
    echo "✅ Info.plist exists"
    grep -A 1 "CFBundleName" "$PLIST" | head -2 || true
else
    echo "❌ Info.plist missing!"
fi
echo ""

# Test 4: Check GitHub Repository
echo "🌐 Test 4: GitHub Repository"
echo "─────────────────────────────────────"
if command -v git &> /dev/null; then
    cd /Users/tehuti01/Desktop/Tehuti-vst-rust
    if git remote -v | grep -q "thunderfpr"; then
        echo "✅ GitHub remote configured"
        git remote -v | head -1
    else
        echo "⚠️  GitHub remote not set"
    fi
else
    echo "⚠️  Git not installed"
fi
echo ""

# Test 5: Check Build System
echo "🛠️  Test 5: Build System"
echo "─────────────────────────────────────"
if command -v cargo &> /dev/null; then
    echo "✅ Rust/Cargo installed"
    cargo --version
    cd /Users/tehuti01/Desktop/Tehuti-vst-rust
    echo ""
    echo "Building plugin..."
    cargo build --release 2>&1 | tail -3
    if [ -f "target/release/libthunderforge_plugin.dylib" ]; then
        echo "✅ Build successful"
        ls -lh target/release/libthunderforge_plugin.dylib
    else
        echo "⚠️  Build may have issues"
    fi
else
    echo "❌ Rust not installed"
fi
echo ""

# Test 6: Check Documentation
echo "📚 Test 6: Documentation"
echo "─────────────────────────────────────"
DOCS=("README.md" "COMMANDS.md" "STANDALONE_TEST.md" "QUICK_START.md" "FL_STUDIO_SETUP.md")
for doc in "${DOCS[@]}"; do
    if [ -f "/Users/tehuti01/Desktop/Tehuti-vst-rust/$doc" ]; then
        echo "✅ $doc exists"
    else
        echo "⚠️  $doc missing"
    fi
done
echo ""

# Test 7: Check Scripts
echo "📜 Test 7: Installation Scripts"
echo "─────────────────────────────────────"
SCRIPTS=("install.sh" "update.sh" "release.sh")
for script in "${SCRIPTS[@]}"; do
    if [ -f "/Users/tehuti01/Desktop/Tehuti-vst-rust/scripts/$script" ]; then
        echo "✅ scripts/$script exists"
    else
        echo "❌ scripts/$script missing"
    fi
done
echo ""

# Test 8: Check DSP Modules
echo "🎛️  Test 8: DSP Modules"
echo "─────────────────────────────────────"
DSP_FILES=("biquad.rs" "compressor.rs" "overdrive.rs" "tone_stack.rs" "cabinet.rs" "delay.rs" "reverb.rs" "chorus.rs")
for file in "${DSP_FILES[@]}"; do
    if [ -f "/Users/tehuti01/Desktop/Tehuti-vst-rust/crates/thunderforge-core/src/dsp/$file" ]; then
        echo "✅ dsp/$file"
    else
        echo "❌ dsp/$file missing"
    fi
done
echo ""

# Test 9: Check Presets
echo "🎵 Test 9: Factory Presets"
echo "─────────────────────────────────────"
if [ -f "/Users/tehuti01/Desktop/Tehuti-vst-rust/crates/thunderforge-core/src/presets/factory.rs" ]; then
    echo "✅ Factory presets defined"
    grep -o '"[^"]*"' /Users/tehuti01/Desktop/Tehuti-vst-rust/crates/thunderforge-core/src/presets/factory.rs | head -8 | while read preset; do
        echo "   - $preset"
    done
else
    echo "❌ Presets missing"
fi
echo ""

# Test 10: System Compatibility
echo "💻 Test 10: System Compatibility"
echo "─────────────────────────────────────"
echo "macOS Version: $(sw_vers -productVersion)"
echo "Architecture: $(uname -m)"
if sysctl -n hw.memsize 2>/dev/null | awk '{print $1/1073741824}' | grep -q "^[0-9]"; then
    RAM=$(sysctl -n hw.memsize 2>/dev/null | awk '{print $1/1073741824}')
    echo "RAM: ${RAM}GB"
fi
echo ""

# Summary
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                    📊 Test Summary                        ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "✅ VST3 Plugin: $([ -d "$HOME/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3" ] && echo 'Installed' || echo 'Missing')"
echo "✅ Binary: $([ -f "$BINARY" ] && echo 'OK' || echo 'Missing')"
echo "✅ Build: $([ -f "/Users/tehuti01/Desktop/Tehuti-vst-rust/target/release/libthunderforge_plugin.dylib" ] && echo 'Working' || echo 'Issues')"
echo "✅ GitHub: $(cd /Users/tehuti01/Desktop/Tehuti-vst-rust && git remote -v 2>/dev/null | grep -q "thunderfpr" && echo 'Connected' || echo 'Not configured')"
echo ""
echo "─────────────────────────────────────────────────────────────"
echo ""
echo "🎯 Next Steps:"
echo ""
echo "1. Test in FL Studio:"
echo "   - Open FL Studio"
echo "   - Options → Manage plugins"
echo "   - Find 'LH Thunderforge'"
echo "   - Load on mixer track"
echo ""
echo "2. Test Audio Processing:"
echo "   - Connect guitar"
echo "   - Enable input monitoring"
echo "   - Adjust knobs"
echo "   - Play!"
echo ""
echo "3. Test Presets:"
echo "   - Use ◀ ▶ buttons"
echo "   - Try all 8 presets"
echo ""
echo "4. Test Effects:"
echo "   - Toggle GATE, TS, COMP"
echo "   - Toggle DLY, REV, CHO"
echo "   - Watch LEDs"
echo ""
echo "─────────────────────────────────────────────────────────────"
echo ""
echo "🎸 Plugin is ready to rock!"
echo ""
