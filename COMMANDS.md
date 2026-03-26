# 🎸 LH Thunderforge - Complete Command Reference

**Save this file for easy reference!**

---

## 📥 INSTALLATION COMMANDS

### Method 1: Auto-Install (Easiest)
```bash
# One-line install
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

### Method 2: Homebrew (When tap is set up)
```bash
# Add tap
brew tap Tehuti01/threwfpr

# Install
brew install lh-thunderforge
```

### Method 3: Manual
```bash
# Download from releases
cd ~/Downloads
curl -LO https://github.com/Tehuti01/thunderfpr/releases/download/v1.0.0/LH-Thunderforge-macos.zip

# Extract and install
unzip LH-Thunderforge-macos.zip
mkdir -p ~/Library/Audio/Plug-Ins/VST3/
mv "LH Thunderforge.vst3" ~/Library/Audio/Plug-Ins/VST3/
```

---

## 🔄 UPDATE COMMANDS

### Auto-Update
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

### Homebrew Update
```bash
brew update && brew upgrade lh-thunderforge
```

---

## 🛠️ BUILD COMMANDS (For Development)

```bash
# Navigate to project
cd /Users/tehuti01/Desktop/Tehuti-vst-rust

# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Clean build
cargo clean && cargo build --release

# Build release package
./scripts/release.sh v1.0.0

# Run tests
cargo test

# Check for errors
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

## 📦 GIT COMMANDS

```bash
# Check status
git status

# Add all changes
git add -A

# Commit
git commit -m "Your message here"

# Push to GitHub
git push

# Pull latest changes
git pull

# View log
git log --oneline -5

# Create new branch
git checkout -b feature-name

# Switch branches
git checkout main
```

---

## 🎹 FL STUDIO COMMANDS

### First-Time Setup
1. Open FL Studio
2. Press `F10` (Options → Manage plugins)
3. Click **"Find installed plugins"**
4. OR add folder: `~/Library/Audio/Plug-Ins/VST3/`
5. Find "LH Thunderforge" in list
6. Click star ⭐ to add to favorites

### Quick Usage
1. Press `F9` to open Mixer
2. Select empty insert
3. Add LH Thunderforge in slot
4. Set input to guitar
5. Click **IN** button
6. Play guitar! 🎸

---

## 🔧 PRESET QUICK REFERENCE

| Preset | Gain | Bass | Mid | Treble | Presence | Amp | Cab |
|--------|------|------|-----|--------|----------|-----|-----|
| Clean | 3 | 5 | 6 | 7 | 6 | Twin | 2x12 |
| Rock | 7 | 6 | 7 | 6 | 5 | Plexi | 4x12 GB |
| Metal | 9 | 4 | 6 | 7 | 7 | 5150 | 4x12 V30 |
| Ambient | 4 | 4 | 5 | 7 | 8 | AC30 | 1x12 |

---

## 🎛️ KNOB VALUES

### Gain Stages
- **Clean**: 0-3
- **Crunch**: 4-6
- **Lead**: 7-8
- **High Gain**: 9-10

### EQ Guidelines
- **Bass**: 0-3 (cut), 4-6 (neutral), 7-10 (boost)
- **Mid**: 0-3 (scooped), 4-6 (neutral), 7-10 (present)
- **Treble**: 0-3 (dark), 4-6 (neutral), 7-10 (bright)
- **Presence**: 0-3 (smooth), 4-6 (neutral), 7-10 (sparkle)

---

## 📊 TROUBLESHOOTING COMMANDS

### Check Installation
```bash
# Verify plugin exists
ls -la ~/Library/Audio/Plug-Ins/VST3/"LH Thunderforge.vst3"/Contents/MacOS/

# Check file size (should be ~1.6MB)
du -h ~/Library/Audio/Plug-Ins/VST3/"LH Thunderforge.vst3"
```

### Clean Reinstall
```bash
# Remove old version
rm -rf ~/Library/Audio/Plug-Ins/VST3/"LH Thunderforge.vst3"

# Install fresh
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

### Check Logs
```bash
# macOS Console
open /Applications/Console.app

# Filter for "thunderforge" or "LH Thunderforge"
```

---

## 🚀 RELEASE WORKFLOW

### To Create New Release:
```bash
# 1. Update version in files
# 2. Update CHANGELOG.md
# 3. Build release
./scripts/release.sh v1.1.0

# 4. Test plugin
# 5. Commit changes
git add -A
git commit -m "Release v1.1.0"
git push

# 6. Create GitHub release
gh release create v1.1.0 build/LH-Thunderforge-macos.zip \
  --title "LH Thunderforge v1.1.0" \
  --notes "See CHANGELOG.md"
```

---

## 📱 QUICK HELP

### Install
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

### Update
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

### Build
```bash
cd /Users/tehuti01/Desktop/Tehuti-vst-rust && cargo build --release
```

### Push to GitHub
```bash
git add -A && git commit -m "message" && git push
```

---

## 🎯 FL STUDIO SHORTCUTS

| Key | Action |
|-----|--------|
| F9 | Open Mixer |
| F10 | Plugin Manager |
| F11 | Playlist |
| F12 | Browser |
| Cmd+R | Last rendered track |
| Space | Play/Stop |
| Record | Arm recording |

---

## 📞 SUPPORT

- **Issues**: https://github.com/Tehuti01/thunderfpr/issues
- **Discussions**: https://github.com/Tehuti01/thunderfpr/discussions
- **Releases**: https://github.com/Tehuti01/thunderfpr/releases

---

**Remember these key commands:**

```bash
# Install: curl .../install.sh | bash
# Update: curl .../update.sh | bash  
# Build: cargo build --release
# Push: git push
```

🎸 **Rock on!**
