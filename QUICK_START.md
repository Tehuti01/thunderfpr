# 🎸 LH Thunderforge - QUICK START

## ✅ What's Running Now

Your **LH Thunderforge** guitar amp simulator is **built and installed** on your MacBook!

### Plugin Location:
```
~/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3/
```

### Demo Audio Files:
Listen to these to hear the DSP processing:
```
/Users/tehuti01/Desktop/Tehuti-vst-rust/demo_output/
  ├── 01_clean_guitar.wav       - Clean guitar signal
  ├── 02_with_distortion.wav    - With amp distortion
  ├── 03_with_eq.wav            - With EQ applied
  └── 04_high_gain_metal.wav    - High gain metal tone
```

---

## 🎹 How to Use in FL Studio

### Step 1: Open FL Studio
- Launch FL Studio from your Applications folder

### Step 2: Find the Plugin
1. Go to **Options** → **Manage plugins**
2. Click **"Find installed plugins"** (or "Scan plugins")
3. Wait for scan to complete
4. Look for **"LH Thunderforge"** in the list
5. Right-click → **"Add to favorites"**

### Step 3: Set Up Guitar Input
1. **Connect your guitar** to your audio interface or Mac
2. In FL Studio: **Options** → **Audio Settings**
3. Select your audio interface as **Input Device**
4. Set **Buffer Size** to 128 samples (for low latency)

### Step 4: Add Plugin to Mixer
1. Open the **Mixer** (F9)
2. Select an empty insert (e.g., Insert 1)
3. In the slot area, add **LH Thunderforge**
4. Set the track input to your guitar input channel
5. Click the **"IN"** button (input monitoring)

### Step 5: Play Guitar! 🎸
- Strum your guitar and you should hear the processed sound
- Adjust the knobs to change your tone

---

## 🎛️ Quick Tone Presets

### Clean Tone (Fender-style)
```
INPUT:   0 dB
GAIN:    3.0
BASS:    5
MID:     6
TREBLE:  7
MASTER:  -6 dB
CAB:     2x12 Blue
```

### Classic Rock (Marshall-style)
```
INPUT:   0 dB
GAIN:    6.5
BASS:    6
MID:     7
TREBLE:  6
MASTER:  -6 dB
CAB:     4x12 Greenback
TS:      ON (for extra drive)
```

### Metal Tone (High Gain)
```
INPUT:   -3 dB
GAIN:    8.5
BASS:    4
MID:     6
TREBLE:  7
MASTER:  -3 dB
CAB:     4x12 V30
GATE:    ON
```

---

## 🎵 Also Works In:

- **Logic Pro X** - Plugin auto-detected
- **GarageBand** - Plugin auto-detected  
- **Ableton Live** - Scan VST3 folder in preferences
- **Cubase** - VST3 plugin manager

---

## 🔧 Commands

### Rebuild Plugin:
```bash
cd /Users/tehuti01/Desktop/Tehuti-vst-rust
cargo build --release
```

### Test Plugin:
```bash
python3 scripts/test_plugin.py
```

### Generate Audio Demo:
```bash
python3 scripts/audio_demo.py
```

---

## 📖 Documentation

- `FL_STUDIO_SETUP.md` - Detailed FL Studio setup guide
- `INSTALLATION.md` - Installation and troubleshooting
- `demo_output/` - Audio demo files

---

## ⚠️ Troubleshooting

### No Sound?
- Check guitar cable is plugged in
- Enable "IN" button on mixer track
- Check MASTER volume in plugin
- Verify audio interface is selected in Audio Settings

### High Latency (delay)?
- Lower buffer size (64-128 samples)
- Use audio interface instead of built-in audio
- Close other apps

### Plugin Not Found?
- Restart FL Studio
- Re-scan plugins in Plugin Manager
- Check plugin exists at: `~/Library/Audio/Plug-Ins/VST3/`

---

**Ready to rock! 🎸🤘**

Double-click any file in `demo_output/` to hear the processing, then open FL Studio to use the full plugin with your guitar!
