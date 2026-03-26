# 🎸 LH Thunderforge

**Professional Guitar Amp Simulator & Neural Modeler**  
*Minimalist Code. Maximalist Sound.*

[![Release](https://img.shields.io/github/v/release/Tehuti01/thunderforge?label=latest%20release&color=orange)](https://github.com/Tehuti01/thunderforge/releases)
[![License](https://img.shields.io/github/license/Tehuti01/thunderforge?color=blue)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%2011%2B-lightgrey)](https://github.com/Tehuti01/thunderforge/releases)
[![Plugin Format](https://img.shields.io/badge/format-VST3%20%7C%20CLAP-purple)](https://github.com/Tehuti01/thunderforge/releases)

---

## 🚀 Quick Download

### Latest Version: v1.0.0

**Download for macOS:**
- [📦 LH Thunderforge v1.0.0 (VST3)](https://github.com/Tehuti01/thunderforge/releases/latest/download/LH-Thunderforge-macos.zip)
- [🔌 LH Thunderforge v1.0.0 (CLAP)](https://github.com/Tehuti01/thunderforge/releases/latest/download/LH-Thunderforge-CLAP-macos.zip)

**Auto-Install (Recommended):**
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/install.sh | bash
```

---

## ✨ Features

### 🎛️ Complete Signal Chain
- **Noise Gate** - Eliminate unwanted noise
- **Tube Screamer** - Classic overdrive pedal emulation
- **Compressor** - Smooth dynamics and sustain
- **8 Amp Models** - From clean to high gain:
  - Plexi '68 (British clean)
  - Plexi Bright (British bright)
  - JCM800 (British high-gain)
  - Recto Modern (American high-gain)
  - AC30 Top Boost (British chime)
  - Twin Clean (American clean)
  - 5150 Red (Modern metal)
  - Soldano (Smooth lead)
- **3-Band EQ + Presence** - Sculpt your tone
- **Cabinet Simulation** - 5 built-in IRs + custom IR support
- **Effects** - Delay, Reverb, Chorus

### 🎯 8 Factory Presets
1. Clean Sparkle
2. Highway Crunch
3. British Invasion
4. Metal Thunder
5. Ambient Shimmer
6. Smooth Lead
7. Nu Metal Chunk
8. Vox Jangle

### 🔧 Technical Specs
- **Plugin Formats:** VST3, CLAP
- **Sample Rates:** 44.1kHz - 192kHz
- **Buffer Sizes:** 32 - 4096 samples
- **Latency:** < 5ms at 128 samples
- **CPU Usage:** < 5% (simple presets), < 25% (full chain)
- **Platform:** macOS 11+ (Apple Silicon & Intel)

---

## 📥 Installation

### Option 1: Auto-Install (Recommended)

Open Terminal and run:
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/install.sh | bash
```

This will:
- Download the latest release
- Install to correct plugin folder
- Set up auto-update script

### Option 2: Manual Install

1. **Download** the latest release from [Releases](https://github.com/Tehuti01/thunderforge/releases)
2. **Extract** the ZIP file
3. **Move** `LH Thunderforge.vst3` to:
   ```
   ~/Library/Audio/Plug-Ins/VST3/
   ```
4. **Restart** your DAW

### Option 3: Homebrew (Coming Soon)

```bash
brew install --cask lh-thunderforge
```

---

## 🎹 Usage

### In FL Studio
1. Open FL Studio
2. **Options** → **Manage plugins**
3. Click **"Find installed plugins"**
4. Look for **"LH Thunderforge"**
5. Drag to mixer track with guitar input
6. Enable **"IN"** (input monitoring)
7. Play guitar! 🎸

### In Logic Pro / GarageBand
1. Create a Software Instrument track
2. Open Smart Controls
3. In the plugin slots, select **LH Thunderforge**
4. Enable input monitoring
5. Play!

### In Ableton Live
1. Open Live's Browser
2. Go to **Audio Effects** → **VST3**
3. Drag **LH Thunderforge** to a track
4. Enable monitoring
5. Play!

---

## 🔄 Auto-Update

The plugin includes an auto-update script. To check for updates:

```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/update.sh | bash
```

This will:
- Check for new releases on GitHub
- Download if update available
- Install automatically
- Backup old version

### Enable Auto-Update Notifications

Add to your crontab (check weekly):
```bash
crontab -e
# Add this line:
0 12 * * 0 curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/update.sh | bash
```

---

## 🎛️ Quick Tone Guide

### Clean Tone
```
GAIN: 3 | BASS: 5 | MID: 6 | TREBLE: 7 | CAB: 2x12 Blue
```

### Classic Rock
```
GAIN: 7 | BASS: 6 | MID: 7 | TREBLE: 6 | CAB: 4x12 Greenback
```

### High Gain Metal
```
GAIN: 9 | BASS: 4 | MID: 6 | TREBLE: 7 | CAB: 4x12 V30 | GATE: ON
```

---

## 🛠️ Building from Source

### Requirements
- Rust 1.70+ (`rustup install stable`)
- macOS 11+

### Build Commands

```bash
# Clone the repository
git clone https://github.com/Tehuti01/thunderforge.git
cd thunderforge

# Build release
cargo build --release

# Bundle VST3
./scripts/build.sh

# Run tests
cargo test
```

---

## 📝 Changelog

### v1.0.0 (March 2026)
- ✨ Initial release
- 🎸 8 amp models
- 📦 5 cabinet IRs
- 🎛️ 8 factory presets
- 🔌 VST3 and CLAP support
- 🔄 Auto-update system

### Coming Soon
- [ ] Windows version
- [ ] Linux version
- [ ] MIDI CC support
- [ ] Preset browser UI
- [ ] More cabinet IRs
- [ ] Neural amp model support (NAM)

---

## 🤝 Testing & Feedback

This plugin is in **beta testing**. If you find bugs or have suggestions:

1. **Report Issues:** [GitHub Issues](https://github.com/Tehuti01/thunderforge/issues)
2. **Discussions:** [GitHub Discussions](https://github.com/Tehuti01/thunderforge/discussions)
3. **Email:** tehuti01@example.com

### Known Beta Testers
- Add your friends here!
- @friend1
- @friend2

---

## 📄 License

GPL-3.0-or-later - See [LICENSE](LICENSE) for details

---

## 🙏 Credits

**Developer:** Lukas Hansen / Stradego Capital  
**Framework:** [nih-plug](https://github.com/robbert-vdh/nih-plug)  
**DSP:** Custom implementation  
**Design:** THUNDERFORGE specification v3.0

---

## 📞 Support

- **Documentation:** See `QUICK_START.md` and `FL_STUDIO_SETUP.md`
- **Issues:** [GitHub Issues](https://github.com/Tehuti01/thunderforge/issues)
- **Discussions:** [GitHub Discussions](https://github.com/Tehuti01/thunderforge/discussions)

---

<div align="center">

**"Minimalist Code. Maximalist Sound."**

Made with ❤️ for guitar players

[⬇️ Download](https://github.com/Tehuti01/thunderforge/releases) • [📖 Docs](./QUICK_START.md) • [💬 Discussions](https://github.com/Tehuti01/thunderforge/discussions)

</div>
