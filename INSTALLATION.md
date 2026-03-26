# LH Thunderforge - Installation Complete! 🎸

## Plugin Installed Successfully

Your **LH Thunderforge** guitar amp simulator plugin has been built and installed on your macOS system.

## Installation Locations

### VST3 Format
```
~/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/
```

### CLAP Format  
```
~/Library/Audio/Plug-Ins/CLAP/LH Thunderforge.clap/
```

## Using the Plugin

### In Your DAW
1. **Restart your DAW** (Logic Pro, Ableton Live, GarageBand, etc.)
2. **Scan for new plugins** if required
3. Look for **"LH Thunderforge"** in your plugin manager
4. Insert it on a guitar track

### Plugin Controls
The plugin currently operates **headless** (no GUI), but all parameters are automatable via your DAW:

- **Input Gain**: -12 to +12 dB trim
- **Amp Gain**: 0-10 (drive amount)
- **EQ**: Bass, Mid, Treble, Presence (0-10 each)
- **Noise Gate**: Threshold, Attack, Hold, Release
- **Overdrive**: Drive, Tone, Level
- **Compressor**: Threshold, Ratio, Attack, Release, Makeup
- **Cabinet**: Model selection, Mix
- **Delay**: Time, Feedback, Mix
- **Reverb**: Size, Decay, Damping, Pre-Delay, Mix
- **Chorus**: Rate, Depth, Mix
- **Master Volume**: -60 to +12 dB

## Signal Chain
```
Guitar → Noise Gate → Overdrive → Compressor → Amp → EQ → Power Amp → Cabinet → Chorus → Delay → Reverb → Master → Output
```

## Factory Presets Included
1. Clean Sparkle
2. Highway Crunch
3. British Invasion
4. Metal Thunder
5. Ambient Shimmer
6. Smooth Lead
7. Nu Metal Chunk
8. Vox Jangle

## Rebuilding the Plugin

To make changes and rebuild:

```bash
cd /Users/tehuti01/Desktop/Tehuti-vst-rust
cargo build --release
```

Then reinstall:
```bash
# The install script copies the new binary automatically
./scripts/install.sh  # (you can create this)
```

## Next Steps

### Add a GUI (Optional)
The plugin currently has no visual interface. To add the skeuomorphic UI from the spec:
- Implement WebView UI with HTML/CSS/JS
- Or use egui for a simpler Rust-native GUI

### Load Real Cabinet IRs
Add `.wav` impulse responses to the `cabinets/` folder and implement loading in `cabinet.rs`

### Neural Amp Models
Integrate NeuralAmpModelerCore for AI-powered amp modeling

## Troubleshooting

### Plugin Not Showing in DAW?
1. Make sure DAW has scanned the correct plugin folder
2. Try rescan plugins in DAW preferences
3. Check Console.app for any plugin loading errors

### Audio Issues?
- Adjust buffer size in DAW (128-512 samples recommended)
- Check input/output levels
- Ensure sample rate matches (44.1kHz, 48kHz, etc.)

## Credits
**LH Thunderforge v1.0.0**
- Developer: Lukas Hansen / Stradego Capital
- Framework: nih-plug (Rust)
- License: GPL-3.0-or-later

---
*"Minimalist Code. Maximalist Sound."*
