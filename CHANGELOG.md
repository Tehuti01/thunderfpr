# Changelog

All notable changes to LH Thunderforge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0] - 2026-03-26

### ✨ Added
- Initial public release
- 8 amp models (Plexi, JCM800, Recto, AC30, Twin, 5150, Soldano)
- 5 cabinet IRs (4x12 Greenback, V30, 2x12 Blue, 1x12 Open, 4x12 T75)
- 8 factory presets
- Complete effects chain (Gate, TS, Comp, Delay, Reverb, Chorus)
- 3-band EQ + Presence control
- VST3 and CLAP support for macOS
- Auto-install and auto-update scripts
- Custom IR loading support

### 🔧 Technical
- Built with Rust and nih-plug framework
- FFT-based cabinet convolution
- Real-time parameter smoothing
- Atomic metering for VU displays
- Zero-allocation audio processing

### 📝 Known Issues
- GUI is minimal (basic parameter controls)
- No preset browser UI yet
- Windows and Linux versions not available

### 📋 Requirements
- macOS 11.0 or later
- DAW supporting VST3 or CLAP
- Apple Silicon or Intel processor

---

## [Unreleased]

### 🚧 In Development
- Enhanced GUI with skeuomorphic design
- Preset browser with search
- MIDI CC support
- Additional amp models
- Windows version (planned)
- Linux version (planned)

### 💡 Planned Features
- Neural Amp Modeler (NAM) integration
- Cab IR designer/capture tool
- Preset sharing community
- Mobile companion app (iOS)

---

## Version History

| Version | Release Date | Notes |
|---------|-------------|-------|
| v1.0.0  | 2026-03-26  | Initial Release |

---

## Update Instructions

### Auto-Update (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderforge/main/scripts/update.sh | bash
```

### Manual Update
1. Download latest release from [Releases](https://github.com/Tehuti01/thunderforge/releases)
2. Extract ZIP file
3. Replace existing plugin in `~/Library/Audio/Plug-Ins/VST3/`
4. Restart DAW

---

## Reporting Issues

Found a bug or have a feature request?
- Open an issue: https://github.com/Tehuti01/thunderforge/issues
- Join discussions: https://github.com/Tehuti01/thunderforge/discussions

---

**Thank you for using LH Thunderforge! 🎸**
