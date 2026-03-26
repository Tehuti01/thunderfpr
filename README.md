# 🎸 LH Thunderforge

**Professional Guitar Amp Simulator & Neural Modeler**  
*Minimalist Code. Maximalist Sound.*

[![Release](https://img.shields.io/github/v/release/Tehuti01/thunderfpr?label=latest%20release&color=orange)](https://github.com/Tehuti01/thunderfpr/releases)
[![License](https://img.shields.io/github/license/Tehuti01/thunderfpr?color=blue)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%2011%2B-lightgrey)](https://github.com/Tehuti01/thunderfpr/releases)
[![Plugin Format](https://img.shields.io/badge/format-VST3-purple)](https://github.com/Tehuti01/thunderfpr/releases)

---

## 🚀 Quick Install

### Method 1: Homebrew (Recommended)

```bash
# Install
brew install Tehuti01/threwfpr/lh-thunderforge

# Update
brew update && brew upgrade lh-thunderforge

# Uninstall
brew uninstall lh-thunderforge
```

### Method 2: Auto-Install Script

```bash
# Install latest release
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash

# Update later
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

### Method 3: Manual Download

1. Download from [Releases](https://github.com/Tehuti01/thunderfpr/releases)
2. Extract ZIP file
3. Move `LH Thunderforge.vst3` to:
   ```
   ~/Library/Audio/Plug-Ins/VST3/
   ```
4. Restart your DAW

---

## 🎹 Usage in FL Studio

### Setup (First Time)

1. **Open FL Studio**
2. Go to **Options** → **Manage plugins**
3. Click **"Find installed plugins"** or add folder:
   - Homebrew: `/opt/homebrew/lib/VST3/` (Apple Silicon)
   - Manual: `~/Library/Audio/Plug-Ins/VST3/`
4. Look for **"LH Thunderforge"** in the list
5. Drag to Favorites (star icon)

### Using with Guitar

1. **Connect Guitar:**
   - Audio interface → USB → Mac
   - Or guitar → 1/4" adapter → Mac

2. **Create Track:**
   - Open Mixer (F9)
   - Select empty insert
   - Add **LH Thunderforge** in effects slot
   - Set input to your guitar
   - Enable **IN** (input monitoring)

3. **Play!** 🎸

### GUI Controls

#### Top Panel - Effects Chain
- **GATE** - Noise gate on/off (green LED)
- **TS** - Tube Screamer overdrive (orange LED)
- **COMP** - Compressor (blue LED)
- **AMP** - Amp section (always on)
- **CAB** - Cabinet sim (always on)
- **DLY** - Delay (purple LED)
- **REV** - Reverb (cyan LED)
- **CHO** - Chorus (pink LED)

#### Main Knobs
- **GAIN** - Amp drive/distortion (orange ring)
- **BASS** - Low frequencies (blue ring)
- **MID** - Mid frequencies (blue ring)
- **TREBLE** - High frequencies (blue ring)
- **PRESENCE** - Ultra-high sparkle (blue ring)
- **INPUT** - Input trim/gain (white ring)
- **MASTER** - Output volume (red ring)

#### Selectors
- **AMP MODEL** - Choose amp type (8 models)
- **CABINET** - Choose cabinet IR (5 + custom)

#### Preset Navigator
- **◀ ▶** - Arrow buttons to switch presets
- **Preset Name** - Current preset display

#### Right Panel
- **VU Meter** - Real-time output level
  - Green: Normal level
  - Yellow: Hot signal
  - Red: Clipping (reduce gain!)

#### Bottom Bar
- Sample rate & buffer size
- CPU usage
- Latency
- Plugin version

---

## 🎛️ Quick Tone Presets

### 🎵 Clean Sparkle (Preset 0)
Perfect for jazz, country, clean rhythm
```
GAIN: 3.0 | BASS: 5 | MID: 6 | TREBLE: 7 | PRESENCE: 6
AMP: Twin Clean | CAB: 2x12 Blue
FX: Chorus ON, Reverb ON
```

### 🎸 Highway Crunch (Preset 1)
Classic rock rhythm, AC/DC style
```
GAIN: 6.5 | BASS: 6 | MID: 7 | TREBLE: 6 | PRESENCE: 5
AMP: Plexi Bright | CAB: 4x12 Greenback
FX: TS ON for solo boost
```

### 🇬🇧 British Invasion (Preset 2)
British high-gain, hard rock
```
GAIN: 7.5 | BASS: 5 | MID: 8 | TREBLE: 5.5 | PRESENCE: 6
AMP: JCM800 | CAB: 4x12 T75
FX: All OFF (raw amp tone)
```

### 🤘 Metal Thunder (Preset 3)
Modern metal, high gain
```
GAIN: 8.5 | BASS: 4 | MID: 6 | TREBLE: 7 | PRESENCE: 7
AMP: 5150 Red | CAB: 4x12 V30
FX: GATE ON (essential!)
```

### 🌊 Ambient Shimmer (Preset 4)
Atmospheric, post-rock, shoegaze
```
GAIN: 4.0 | BASS: 4 | MID: 5 | TREBLE: 7 | PRESENCE: 8
AMP: AC30 Top | CAB: 1x12 Open
FX: Chorus ON, Delay ON, Reverb ON
```

### 🎶 Smooth Lead (Preset 5)
Creamy sustain for solos
```
GAIN: 7.0 | BASS: 5 | MID: 7 | TREBLE: 5 | PRESENCE: 4
AMP: Soldano | CAB: 4x12 Greenback
FX: Delay ON (375ms), Reverb ON
```

### 🎚️ Nu Metal Chunk (Preset 6)
Drop-tuned aggression
```
GAIN: 8.0 | BASS: 5 | MID: 5 | TREBLE: 7 | PRESENCE: 6
AMP: Recto Modern | CAB: 4x12 V30
FX: GATE ON, COMP ON
```

### ✨ Vox Jangle (Preset 7)
Chimey British cleans
```
GAIN: 5.0 | BASS: 4 | MID: 6 | TREBLE: 8 | PRESENCE: 7
AMP: AC30 Top | CAB: 2x12 Blue
FX: Chorus ON (subtle), Spring Reverb
```

---

## 🔧 Commands Reference

### Installation
```bash
# Homebrew install
brew install Tehuti01/threwfpr/lh-thunderforge

# Auto-install script
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

### Updates
```bash
# Homebrew update
brew update && brew upgrade lh-thunderforge

# Auto-update script
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

### Build from Source
```bash
# Clone
git clone https://github.com/Tehuti01/thunderfpr.git
cd thunderfpr

# Build release
cargo build --release

# Bundle
./scripts/release.sh v1.0.0
```

### Uninstall
```bash
# Homebrew
brew uninstall lh-thunderforge

# Manual
rm -rf ~/Library/Audio/Plug-Ins/VST3/"LH Thunderforge.vst3"
```

---

## 📦 Features

### 8 Amp Models
- **Plexi '68** - British clean (JTM45 style)
- **Plexi Bright** - British bright channel
- **JCM800** - British high-gain legend
- **Recto Modern** - American high-gain beast
- **AC30 Top Boost** - British chime machine
- **Twin Clean** - American pristine clean
- **5150 Red** - Modern metal monster
- **Soldano** - Smooth high-gain lead

### 5 Cabinet IRs
- **4x12 Greenback** - Marshall 1960A vintage
- **4x12 V30** - Modern rock standard
- **2x12 Blue** - Fender-style sparkle
- **1x12 Open** - Vintage combo warmth
- **4x12 T75** - High-gain punch

### Effects Chain
- Noise Gate (threshold, attack, hold, release)
- Tube Screamer (drive, tone, level)
- Compressor (threshold, ratio, attack, release, makeup)
- Delay (time, feedback, mix, sync)
- Reverb (size, decay, damping, predelay, mix)
- Chorus (rate, depth, mix)

### Technical Specs
- **Formats:** VST3
- **Sample Rates:** 44.1 - 192 kHz
- **Buffer Sizes:** 32 - 4096 samples
- **Latency:** < 5ms at 128 samples
- **CPU:** < 5% (clean), < 25% (full chain)
- **Platform:** macOS 11+ (Apple Silicon & Intel)

---

## 🎨 GUI Features

- ✨ **Beautiful Skeuomorphic Design** - Realistic knobs with metallic finish
- 🎛️ **270° Knob Rotation** - Precise control with visual feedback
- 💡 **LED Indicators** - Color-coded effect status
- 📊 **Real-time VU Meter** - Green/Yellow/Red level monitoring
- 🎚️ **Preset Browser** - Arrow navigation between 8 presets
- 🔄 **Live Updates** - Change settings while playing, no audio interruption
- 🌈 **Color-Coded Rings** - Easy identification of knob functions
- 💻 **Retina Display** - Crisp rendering on all Mac displays

---

## 🐛 Troubleshooting

### Plugin Not Showing in FL Studio?
```bash
# Rescan plugins
FL Studio → Options → Manage plugins → Scan

# Check installation location
ls ~/Library/Audio/Plug-Ins/VST3/
ls /opt/homebrew/lib/VST3/  # Homebrew location
```

### High Latency?
- Reduce buffer size: **Options** → **Audio Settings** → Buffer Size: **128** or **256**
- Close other applications
- Use audio interface instead of built-in audio

### No Sound?
- Check guitar is plugged in
- Enable **IN** button on mixer track
- Check **MASTER** volume in plugin
- Verify audio interface selected in FL Studio

### Crashing?
- Update to latest version
- Check DAW is 64-bit
- Try larger buffer size (256 or 512)
- Report issue on GitHub

---

## 📖 Documentation

- `QUICK_START.md` - Quick start guide
- `FL_STUDIO_SETUP.md` - Detailed FL Studio setup
- `INSTALLATION.md` - Installation troubleshooting
- `CHANGELOG.md` - Version history

---

## 🤝 Contributing

Found a bug or want a feature?

1. **Issues:** [GitHub Issues](https://github.com/Tehuti01/thunderfpr/issues)
2. **Discussions:** [GitHub Discussions](https://github.com/Tehuti01/thunderfpr/discussions)
3. **Email:** tehuti01@example.com

---

## 📄 License

GPL-3.0-or-later - See [LICENSE](LICENSE) for details

---

## 🙏 Credits

**Developer:** Lukas Hansen / Stradego Capital  
**Framework:** [nih-plug](https://github.com/robbert-vdh/nih-plug) + [nih-plug-egui](https://github.com/robbert-vdh/nih-plug)  
**DSP:** Custom implementation  
**GUI:** Egui immediate mode

---

<div align="center">

**"Minimalist Code. Maximalist Sound."**

Made with ❤️ for guitar players

[⬇️ Download](https://github.com/Tehuti01/thunderfpr/releases) • [📖 Docs](#-usage-in-fl-studio) • [💬 Discussions](https://github.com/Tehuti01/thunderfpr/discussions)

### Quick Commands
```bash
# Install
brew install Tehuti01/threwfpr/lh-thunderforge

# Update
brew update && brew upgrade lh-thunderforge

# Or use auto-installer
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

</div>
