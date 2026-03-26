# LH Thunderforge - FL Studio Setup Guide

## 🎸 Using with Electric Guitar in FL Studio

### Prerequisites
- MacBook with macOS 11 or later
- FL Studio 20 or later
- Audio interface (recommended for low latency)
- Electric guitar

## Installation

The plugin is already installed at:
```
~/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/
```

## Setup in FL Studio

### 1. Add Plugin to FL Studio
1. Open FL Studio
2. Go to **Options** → **Manage plugins**
3. Click **Find installed plugins** (or scan for new plugins)
4. Look for **"LH Thunderforge"** in the plugin list
5. Drag it to your Favorites for easy access

### 2. Connect Your Guitar

#### Option A: Using Audio Interface (Recommended)
1. Connect guitar to audio interface
2. Connect interface to MacBook via USB
3. In FL Studio: **Options** → **Audio Settings**
4. Select your audio interface as **Input/Output Device**
5. Set **Buffer Size** to 128 or 256 samples for low latency

#### Option B: Using Built-in Audio Input
1. Connect guitar to MacBook (may need adapter)
2. In FL Studio: **Options** → **Audio Settings**
3. Select **Built-in Input** as input device
4. Enable **Input monitoring**

### 3. Create Guitar Track
1. In Mixer, select an empty insert
2. In the slot area, add **LH Thunderforge**
3. Set the mixer track input to your guitar input
4. Enable **IN** (input monitoring) on the mixer track

### 4. Adjust Settings

#### For Clean Tone:
- **INPUT**: 0 dB
- **GAIN**: 3-4
- **BASS**: 5
- **MID**: 6
- **TREBLE**: 7
- **PRESENCE**: 6
- **MASTER**: -6 dB
- **CABINET**: 2x12 Blue

#### For Crunch/Rock:
- **INPUT**: 0 dB
- **GAIN**: 6-7
- **BASS**: 6
- **MID**: 7
- **TREBLE**: 6
- **PRESENCE**: 5
- **MASTER**: -6 dB
- **CABINET**: 4x12 Greenback
- **TS (Tube Screamer)**: Enable for extra drive

#### For High Gain/Metal:
- **INPUT**: -3 dB
- **GAIN**: 8-9
- **BASS**: 4
- **MID**: 5-6
- **TREBLE**: 7
- **PRESENCE**: 7
- **MASTER**: -3 dB
- **CABINET**: 4x12 V30
- **GATE**: Enable (to reduce noise)

### 5. Add Effects (Optional)
In the signal chain, you can enable:
- **DLY (Delay)**: For echo effects
- **REV (Reverb)**: For space/ambience
- **CHO (Chorus)**: For thickening effect

## Troubleshooting

### High Latency?
- Reduce buffer size in Audio Settings (try 64 or 128 samples)
- Close other applications
- Use an audio interface instead of built-in audio

### No Sound?
- Check guitar cable and connections
- Ensure mixer track input is set correctly
- Enable input monitoring (IN button) on mixer track
- Check MASTER volume in the plugin

### Plugin Not Showing?
- Restart FL Studio
- Re-scan plugins in Plugin Manager
- Check that plugin is in correct folder

### Too Much Noise?
- Enable the **GATE** (noise gate)
- Reduce **INPUT** level
- Use a noise gate plugin before Thunderforge

## Tips for Best Sound

1. **Gain Staging**: Keep INPUT around 0dB, adjust GAIN for distortion
2. **EQ**: Cut bass if sound is muddy, boost mids for cut
3. **Cabinet**: Different cabs dramatically change tone
4. **Effects Chain**: Order matters! TS → COMP → AMP → CAB → DLY → REV

## Keyboard Shortcuts in Plugin

- **Double-click knobs**: Reset to default
- **Drag up/down on knobs**: Adjust value
- **Click switches**: Toggle on/off

## Presets

The plugin includes 8 factory presets accessible via the preset menu:
1. Clean Sparkle
2. Highway Crunch
3. British Invasion
4. Metal Thunder
5. Ambient Shimmer
6. Smooth Lead
7. Nu Metal Chunk
8. Vox Jangle

## Recording

1. Arm the track for recording (click record button on track)
2. Press Record in FL Studio transport
3. Play your guitar!
4. The plugin settings are automated if you adjust them during recording

---

**Enjoy playing through LH Thunderforge! 🎸**

For questions or issues, check the INSTALLATION.md file.
