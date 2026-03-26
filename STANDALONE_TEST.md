# 🎸 LH Thunderforge - Standalone Test Guide

**Test the plugin WITHOUT FL Studio or any DAW!**

---

## 🚀 Quick Start

### Run Standalone Test App

```bash
# Method 1: Use the script
./run-standalone.sh

# Method 2: Direct command
cd /Users/tehuti01/Desktop/Tehuti-vst-rust
cargo run --package thunderforge-standalone --release
```

### What You'll See

When you run the standalone app, a window opens with:

1. **Top Panel** - Effect toggles with LED indicators
   - GATE (green LED)
   - TS - Tube Screamer (orange LED)
   - COMP - Compressor (blue LED)
   - AMP & CAB (always on)
   - DLY - Delay (purple LED)
   - REV - Reverb (cyan LED)
   - CHO - Chorus (pink LED)

2. **Main Knobs** (left side)
   - **GAIN** - Amp distortion (orange ring)
   - **BASS** - Low frequencies (blue ring)
   - **MID** - Mid frequencies (blue ring)
   - **TREBLE** - High frequencies (blue ring)
   - **PRESENCE** - Ultra-highs (blue ring)
   - **INPUT** - Input trim (white ring)
   - **MASTER** - Output volume (red ring)

3. **Preset Navigator**
   - **◀ ▶** buttons to switch presets
   - Current preset name display
   - **Play Test Tone** button

4. **Selectors**
   - **AMP MODEL** - Choose from 8 amp types
   - **CABINET** - Choose from 5 cabinet IRs

5. **Side Panel** - VU Meters
   - **IN** - Input level (green/yellow/red)
   - **OUT** - Output level (green/yellow/red)

6. **Bottom Bar** - Status
   - Sample rate & buffer size
   - CPU usage
   - Latency

---

## 🎛️ How to Test

### 1. Adjust Knobs
- **Click and drag** any knob up/down to change value
- Watch the **orange arc** show the current value
- **Hover** over knob to see exact value in tooltip

### 2. Toggle Effects
- **Click** any effect button (GATE, TS, COMP, etc.)
- **LED lights up** when effect is active
- Color-coded LEDs for each effect

### 3. Switch Presets
- Click **◀** or **▶** to navigate presets
- See preset name change
- Settings update automatically

### 4. Change Amp/Cab
- Click **AMP MODEL** dropdown
- Select different amp type
- Click **CABINET** dropdown
- Select different cabinet

### 5. Test Tone Generator
- Click **"▶ Play Test Tone"**
- Watch VU meters move
- See real-time level changes
- Click **"⏹ Stop"** to stop

---

## 🎨 GUI Features to Test

### Knobs
✅ 270° rotation range
✅ Orange value arc shows current position
✅ White indicator line
✅ Smooth dragging
✅ Value tooltip on hover
✅ Colored rings for easy identification

### Buttons
✅ LED indicators (on/off)
✅ Color-coded per effect
✅ Visual feedback on click
✅ Label shows function

### Selectors
✅ Dropdown menus
✅ Current value displayed
✅ Multiple options

### Meters
✅ Real-time level display
✅ Color changes (green → yellow → red)
✅ Smooth animation

### Layout
✅ Dark theme
✅ Professional appearance
✅ Organized sections
✅ Clear labels
✅ Responsive design

---

## 📊 Test Checklist

Run through this to test everything:

### Visual Tests
- [ ] Window opens correctly
- [ ] All knobs visible
- [ ] All buttons visible
- [ ] Labels readable
- [ ] Meters display
- [ ] Colors correct

### Interaction Tests
- [ ] Drag GAIN knob - see orange arc move
- [ ] Drag EQ knobs - see blue arcs move
- [ ] Drag MASTER knob - see red arc move
- [ ] Click GATE button - LED toggles
- [ ] Click TS button - LED toggles
- [ ] Click preset ◀ ▶ - name changes
- [ ] Change AMP MODEL - dropdown works
- [ ] Change CABINET - dropdown works
- [ ] Hover over knob - tooltip appears

### Visual Feedback
- [ ] Knob arcs update in real-time
- [ ] LEDs light up when enabled
- [ ] Meters respond (when playing)
- [ ] Smooth animations
- [ ] No visual glitches

---

## 🎹 Test Presets

Try each preset and hear the difference:

| # | Preset | Character |
|---|--------|-----------|
| 0 | Clean Sparkle | Clean, sparkly |
| 1 | Highway Crunch | Classic rock |
| 2 | British Invasion | British high-gain |
| 3 | Metal Thunder | Modern metal |
| 4 | Ambient Shimmer | Atmospheric |
| 5 | Smooth Lead | Lead tone |
| 6 | Nu Metal Chunk | Heavy chunk |
| 7 | Vox Jangle | Chimey clean |

---

## 🔧 Troubleshooting

### App Won't Start?
```bash
# Rebuild
cargo build --package thunderforge-standalone --release

# Check for errors
cargo check --package thunderforge-standalone
```

### Window Too Small?
- Window is resizable
- Minimum size: 800x500
- Recommended: 1000x600

### Knobs Not Responding?
- Make sure you're dragging up/down
- Try clicking directly on knob
- Check terminal for errors

### Meters Not Moving?
- Click "Play Test Tone" button
- Meters simulate audio when playing
- Real audio input coming soon

---

## 🎯 What This Tests

The standalone app tests:

✅ **GUI Rendering** - All visual elements
✅ **User Interaction** - Knobs, buttons, selectors
✅ **Parameter Changes** - Real-time updates
✅ **Preset System** - Navigation and loading
✅ **Visual Feedback** - LEDs, meters, tooltips
✅ **Layout** - Organization and spacing
✅ **Theme** - Dark mode, colors

---

## 🚀 Next Steps

After testing standalone:

1. **Test in FL Studio**
   - Install plugin: `./scripts/install.sh`
   - Open FL Studio
   - Load plugin

2. **Test with Real Guitar**
   - Connect guitar
   - Enable input monitoring
   - Play through plugin

3. **Record Audio**
   - Arm track in DAW
   - Record guitar
   - Export and share

---

## 📞 Quick Commands

```bash
# Run standalone test
./run-standalone.sh

# Rebuild if needed
cargo build --release

# Install for DAW use
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash

# Update
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

---

**🎸 You can now test the complete GUI without any DAW!**

Run `./run-standalone.sh` to start testing!
