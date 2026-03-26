<div align="center">

# LH Thunderforge

**Professional Guitar Amp Simulator**
*8 Legendary Amp Models · Full Effects Chain · VST3 & CLAP · macOS*

[![Release](https://img.shields.io/github/v/release/Tehuti01/thunderfpr?label=Latest%20Release&color=orange&style=for-the-badge)](https://github.com/Tehuti01/thunderfpr/releases)
[![Platform](https://img.shields.io/badge/Platform-macOS%2011%2B-lightgrey?style=for-the-badge)](https://github.com/Tehuti01/thunderfpr/releases)
[![Format](https://img.shields.io/badge/Format-VST3%20%7C%20CLAP-purple?style=for-the-badge)](https://github.com/Tehuti01/thunderfpr/releases)
[![License](https://img.shields.io/github/license/Tehuti01/thunderfpr?color=blue&style=for-the-badge)](LICENSE)

*"Minimalist Code. Maximalist Sound."*

[⬇ Download Now](https://github.com/Tehuti01/thunderfpr/releases) · [Quick Start](#-5-minute-setup) · [Presets](#-8-built-in-presets) · [Troubleshooting](#-troubleshooting)

</div>

---

## What Is This?

LH Thunderforge is a **free, open-source guitar amp simulator** that runs inside your DAW (music production software). It lets you plug in your electric guitar and get the sound of real, expensive guitar amplifiers — without owning any of them.

**Think of it like this:**
> You plug in your guitar → Thunderforge makes it sound like a Marshall, a Mesa, a Fender, or a Vox — all from your computer.

**Who is it for?**
- Guitarists who want great amp tones without buying physical amps
- Home studio musicians recording on a Mac
- Producers adding guitar to tracks
- Anyone curious about amp simulation technology

**What does it need?**
- A Mac running macOS 11 Big Sur or newer
- A DAW (FL Studio, Logic Pro, Ableton, GarageBand, etc.)
- An electric guitar (acoustic works too, but amp sims are designed for electric)
- Optionally: an audio interface for best quality (but your Mac's headphone jack works)

---

## Plugin Stats

| Spec | Detail |
|------|--------|
| **Plugin Formats** | VST3, CLAP |
| **Platform** | macOS 11+ (Apple Silicon M1/M2/M3 & Intel) |
| **Sample Rates** | 44.1 kHz, 48 kHz, 88.2 kHz, 96 kHz, 192 kHz |
| **Buffer Sizes** | 32 – 4096 samples |
| **Latency** | < 5 ms at 128 samples |
| **CPU Usage** | ~3% (clean tone) · ~15% (full effects chain) |
| **Amp Models** | 8 |
| **Cabinet IRs** | 5 + Custom |
| **Built-in Presets** | 8 |
| **Effects** | 6 (Gate, Overdrive, Compressor, Delay, Reverb, Chorus) |
| **GUI Size** | 1000 × 600 px |
| **Language** | Rust (nih-plug framework) |
| **License** | GPL-3.0 — Free forever |

---

## Signal Chain

Your guitar signal travels through these stages in order:

```
Guitar Input
    │
    ▼
[ Input Trim ]      ← Adjusts incoming signal level
    │
    ▼
[ Noise Gate ]      ← Silences hum/hiss when you're not playing
    │
    ▼
[ Tube Screamer ]   ← Overdrive pedal (adds grit before the amp)
    │
    ▼
[ Compressor ]      ← Evens out your picking dynamics
    │
    ▼
[ Amp Model ]       ← The heart — simulates 8 real amp heads
    │
    ▼
[ Tone Stack ]      ← Bass / Mid / Treble / Presence EQ
    │
    ▼
[ Cabinet IR ]      ← Simulates a real speaker cabinet
    │
    ▼
[ Chorus ]          ← Shimmering, widening modulation
    │
    ▼
[ Delay ]           ← Echo effect
    │
    ▼
[ Reverb ]          ── Room/hall ambience
    │
    ▼
[ Master Volume ]   ← Final output level
```

Each stage can be bypassed individually with its LED toggle button.

---

## 5-Minute Setup

### Step 1 — Install the Plugin

Choose **one** of the three methods below. Start with Method 1 if you have Homebrew, or Method 2 if you just want a simple download.

---

#### Method 1 — Homebrew `RECOMMENDED`

> Best for: developers, people who already use Homebrew, anyone who wants easy updates

Homebrew is a free package manager for Mac that makes installing and updating software a single command.

**First time only** — install Homebrew if you don't have it:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Then install Thunderforge:
```bash
brew install Tehuti01/threwfpr/lh-thunderforge
```

Done. The plugin is now installed.

**To update later:**
```bash
brew update && brew upgrade lh-thunderforge
```

**To uninstall:**
```bash
brew uninstall lh-thunderforge
```

---

#### Method 2 — Auto-Install Script `EASIEST`

> Best for: beginners who just want to paste one command and go

Open **Terminal** (press `Command + Space`, type "Terminal", press Enter) and paste:

```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

This automatically downloads the latest release and places the plugin in the right folder.

**To update later:**
```bash
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

---

#### Method 3 — Manual Download `OPTIONAL`

> Best for: people who prefer doing things by hand, or when the above methods don't work

1. Go to the [Releases page](https://github.com/Tehuti01/thunderfpr/releases)
2. Download the `.zip` file from the latest release
3. Extract the ZIP (double-click it)
4. Move `LH Thunderforge.vst3` to:
   ```
   ~/Library/Audio/Plug-Ins/VST3/
   ```
   *(You can open this folder in Finder: press `Command + Shift + G` and paste the path above)*
5. Restart your DAW

---

### Step 2 — Add It to Your DAW

#### FL Studio

1. Open FL Studio
2. Go to **Options** → **Manage plugins**
3. Click **"Start scan"** or add the VST3 folder manually:
   - If you used Homebrew: `/opt/homebrew/lib/VST3/`
   - If you used the script or manual install: `~/Library/Audio/Plug-Ins/VST3/`
4. Find **"LH Thunderforge"** in the plugin list
5. Star it (click the star icon) to add it to Favorites

#### Logic Pro

1. Open Logic Pro
2. Logic automatically scans `~/Library/Audio/Plug-Ins/` — no setup needed
3. Open the **Audio FX** slot on a channel strip
4. Look under **Audio Units** or the **VST3** section for "LH Thunderforge"

#### Ableton Live

1. Open Ableton Live
2. Go to **Preferences** → **Plug-Ins**
3. Enable **"Use VST3 Plug-In System Folders"**
4. Click **"Rescan"**
5. Find it in the **Plug-ins** browser

#### GarageBand

> GarageBand supports Audio Units (`.component`) format, not VST3. For GarageBand, you can try the standalone app (see below) or consider upgrading to Logic Pro.

---

### Step 3 — Connect Your Guitar

**Option A: Audio Interface** `RECOMMENDED for best quality`
- Plug your guitar into an audio interface (Focusrite Scarlett, Universal Audio, etc.)
- Connect the interface to your Mac via USB/USB-C
- In your DAW, set the input to your interface

**Option B: Direct to Mac** `Works but lower quality`
- Get a 1/4" to 3.5mm adapter
- Plug guitar into the Mac's headphone jack (it works as a combo input on most Macs)
- In your DAW, set input to "Built-in Microphone" or "Line In"

---

### Step 4 — Set Up a Track

#### In FL Studio:

1. Press **F9** to open the Mixer
2. Click an empty mixer insert (track)
3. Click the effects slot on the right side
4. Choose **"LH Thunderforge"** from the plugin list
5. Click the **IN** button on the mixer insert to enable input monitoring
6. Set **Input** to your guitar/interface channel

#### In Logic / Ableton:

1. Create an **Audio Track**
2. Set the input to your guitar input
3. Click an **Audio FX** slot
4. Insert **LH Thunderforge**
5. Enable input monitoring (the circle/I button on the track)

---

## Controls Explained

### Top Bar — Effect Toggles

Each button is an LED that you click to turn an effect on or off:

| Button | What It Does | When to Turn It ON |
|--------|-------------|-------------------|
| **GATE** | Noise Gate — silences silence | Always for high-gain tones |
| **TS** | Tube Screamer — adds overdrive | Blues, rock, or solo boost |
| **COMP** | Compressor — levels dynamics | Clean tones, funk, country |
| **DLY** | Delay — echo/repeat effect | Solos, ambient, U2-style |
| **REV** | Reverb — room/hall ambience | Everything except tight metal |
| **CHO** | Chorus — shimmery, wide effect | Cleans, 80s tones |

### Main Knobs

| Knob | Range | What It Does |
|------|-------|-------------|
| **INPUT** | -12 to +12 dB | Adjusts how hot the signal hits the amp. If it sounds too distorted at low GAIN, turn this down. |
| **GAIN** | 0 – 10 | The amp's drive/distortion amount. 0–3 = clean, 4–6 = crunch, 7–10 = high gain |
| **BASS** | 0 – 10 | Low-end fullness. 5 is neutral. |
| **MID** | 0 – 10 | The "honk" or "scoop" of the tone. High = present, low = scooped metal |
| **TREBLE** | 0 – 10 | Brightness and cut. |
| **PRESENCE** | 0 – 10 | Ultra-high frequency air and sparkle |
| **MASTER** | -60 to +12 dB | Final output volume. Reduce this if clipping. |

### Selectors

**AMP MODEL** — Click to cycle through 8 amp simulations (see below)

**CABINET** — Click to choose a speaker cabinet IR (5 options + Custom)

### Preset Navigator

Click the **◀** and **▶** arrows to step through the 8 built-in presets. The preset name is shown in the center display.

### VU Meter (right side)

Real-time output level display:
- **Green** — Perfect level
- **Yellow** — Getting loud, but fine
- **Red** — Clipping! Turn down MASTER or INPUT

---

## 8 Amp Models

| Model | Based On | Character | Best For |
|-------|----------|-----------|----------|
| **Plexi '68** | Marshall JTM45 | Warm, round British clean | Jazz, clean rhythm, blues |
| **Plexi Bright** | Marshall 1959 Super Lead | Bright, chimey British crunch | Classic rock, Hendrix, Who |
| **JCM800** | Marshall JCM800 2203 | Aggressive British high-gain | Hard rock, AC/DC, metal |
| **Recto Modern** | Mesa/Boogie Dual Rectifier | Massive, scooped American gain | Modern metal, djent, chug |
| **AC30 Top Boost** | Vox AC30 Top Boost | Chimey, jangly British sparkle | Beatles, Radiohead, indie |
| **Twin Clean** | Fender Twin Reverb | Pristine, sparkly American clean | Country, funk, R&B, jazz |
| **5150 Red** | Peavey 5150 Red channel | Brutal, tight modern metal | Metal, hardcore, thrash |
| **Soldano** | Soldano SLO-100 | Smooth, creamy high-gain lead | Lead tones, fusion, rock solos |

---

## 5 Cabinet IRs

Cabinet IRs (Impulse Responses) simulate the sound of a real speaker cabinet captured with a microphone. They transform the amp sound into something that sounds like it was mic'd in a real studio.

| Cabinet | Based On | Character |
|---------|----------|-----------|
| **4x12 Greenback** | Marshall 1960A with Celestion G12M | Vintage warmth, classic rock |
| **4x12 V30** | Marshall with Celestion V30 | Modern rock standard, versatile |
| **2x12 Blue** | Fender-style with Celestion Blue | Sparkle, jangle, clean tones |
| **1x12 Open Back** | Vintage combo speaker | Warm, open, bloom |
| **4x12 T75** | Marshall with Celestion T75 | Punchy, tight, high-gain ready |

> **Custom** — Load your own IR file (coming in a future update)

---

## 8 Built-in Presets

These are starting points — tweak them to taste after loading.

### 1. Clean Sparkle
*Jazz, country, clean rhythm*
```
INPUT: 0 dB  |  GAIN: 3.0  |  MASTER: -6 dB
BASS: 5  |  MID: 6  |  TREBLE: 7  |  PRESENCE: 6
AMP: Twin Clean  |  CAB: 2x12 Blue
EFFECTS ON: Chorus, Reverb
```

### 2. Highway Crunch
*Classic rock rhythm — AC/DC, Stones, Zeppelin*
```
INPUT: 0 dB  |  GAIN: 6.5  |  MASTER: -6 dB
BASS: 6  |  MID: 7  |  TREBLE: 6  |  PRESENCE: 5
AMP: Plexi Bright  |  CAB: 4x12 Greenback
EFFECTS ON: TS (for solo boost)
```

### 3. British Invasion
*British hard rock, raw amp tone*
```
INPUT: 0 dB  |  GAIN: 7.5  |  MASTER: -6 dB
BASS: 5  |  MID: 8  |  TREBLE: 5.5  |  PRESENCE: 6
AMP: JCM800  |  CAB: 4x12 T75
EFFECTS ON: (none — pure amp)
```

### 4. Metal Thunder
*Modern metal — tight, heavy, aggressive*
```
INPUT: 0 dB  |  GAIN: 8.5  |  MASTER: -6 dB
BASS: 4  |  MID: 6  |  TREBLE: 7  |  PRESENCE: 7
AMP: 5150 Red  |  CAB: 4x12 V30
EFFECTS ON: Gate (essential — kills pick noise between riffs)
```

### 5. Ambient Shimmer
*Post-rock, shoegaze, atmospheric*
```
INPUT: 0 dB  |  GAIN: 4.0  |  MASTER: -6 dB
BASS: 4  |  MID: 5  |  TREBLE: 7  |  PRESENCE: 8
AMP: AC30 Top Boost  |  CAB: 1x12 Open Back
EFFECTS ON: Chorus, Delay (375ms), Reverb (large)
```

### 6. Smooth Lead
*Creamy sustain for solos — SRV, Clapton, Bonamassa*
```
INPUT: 0 dB  |  GAIN: 7.0  |  MASTER: -6 dB
BASS: 5  |  MID: 7  |  TREBLE: 5  |  PRESENCE: 4
AMP: Soldano  |  CAB: 4x12 Greenback
EFFECTS ON: Delay (375ms, 40% feedback), Reverb (small room)
```

### 7. Nu Metal Chunk
*Drop-tuned djent, tight rhythms*
```
INPUT: 0 dB  |  GAIN: 8.0  |  MASTER: -6 dB
BASS: 5  |  MID: 5  |  TREBLE: 7  |  PRESENCE: 6
AMP: Recto Modern  |  CAB: 4x12 V30
EFFECTS ON: Gate, Compressor
```

### 8. Vox Jangle
*Chimey Britpop and indie cleans — Oasis, Blur, Smiths*
```
INPUT: 0 dB  |  GAIN: 5.0  |  MASTER: -6 dB
BASS: 4  |  MID: 6  |  TREBLE: 8  |  PRESENCE: 7
AMP: AC30 Top Boost  |  CAB: 2x12 Blue
EFFECTS ON: Chorus (subtle), Reverb (medium)
```

---

## All Commands

### Install
```bash
# Recommended: Homebrew
brew install Tehuti01/threwfpr/lh-thunderforge

# Alternative: Auto-install script
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/install.sh | bash
```

### Update
```bash
# Homebrew
brew update && brew upgrade lh-thunderforge

# Script
curl -fsSL https://raw.githubusercontent.com/Tehuti01/thunderfpr/main/scripts/update.sh | bash
```

### Uninstall
```bash
# Homebrew
brew uninstall lh-thunderforge

# Manual
rm -rf ~/Library/Audio/Plug-Ins/VST3/"LH Thunderforge.vst3"
rm -rf ~/Library/Audio/Plug-Ins/CLAP/"LH Thunderforge.clap"
```

### Build from Source `OPTIONAL — for developers`

> Only needed if you want to modify the plugin or build it yourself. Requires Rust installed.

```bash
# 1. Install Rust (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone the repository
git clone https://github.com/Tehuti01/thunderfpr.git
cd thunderfpr

# 3. Build release version
cargo build --release

# 4. Package into VST3/CLAP bundles
./scripts/build.sh

# 5. Install the bundles
cp -r build/LH\ Thunderforge.vst3 ~/Library/Audio/Plug-Ins/VST3/
cp -r build/LH\ Thunderforge.clap ~/Library/Audio/Plug-Ins/CLAP/
```

---

## Troubleshooting

### "Plugin not showing up in my DAW"

**Check the plugin is installed:**
```bash
ls ~/Library/Audio/Plug-Ins/VST3/
# You should see: LH Thunderforge.vst3
```

**Then rescan:**
- FL Studio: Options → Manage plugins → Start scan
- Logic: Should auto-detect, try restarting Logic
- Ableton: Preferences → Plug-Ins → Rescan

---

### "No sound when I play guitar"

Go through this checklist:

1. Is your guitar plugged in?
2. Is your audio interface connected and selected in your DAW's audio settings?
3. Is input monitoring enabled? (The "IN" or monitor button on your DAW track)
4. Is the **MASTER** knob turned up? (Default is -6 dB — that's normal, but check it's not at -60)
5. Are your DAW's output and monitoring turned on?

---

### "It sounds really distorted even on low gain"

Your input signal is probably too hot. Try:
- Turn the **INPUT** knob down (towards -12 dB)
- Lower the gain on your audio interface
- Turn **GAIN** to 2 or 3 first to find the clean sweet spot

---

### "I hear a hum or buzz"

Turn on the **GATE** effect and adjust the **Threshold** slider upward until the hum cuts off between notes. This is completely normal with electric guitars — all real amp rigs use a noise gate.

---

### "High latency / audio glitches"

- Lower your buffer size: DAW Audio Settings → Buffer Size → try **128** or **256**
- Close other apps (browsers, especially)
- Use an audio interface instead of the Mac's built-in audio
- Make sure your Mac is plugged in (power saving reduces CPU performance)

---

### "Plugin crashes my DAW"

1. Update to the latest version first
2. Try a larger buffer size (512 or 1024)
3. Make sure your DAW is 64-bit
4. [Report the issue](https://github.com/Tehuti01/thunderfpr/issues) with your macOS version and DAW version

---

## Features at a Glance

- **8 amp models** — from pristine clean to face-melting metal
- **5 speaker cabinet IRs** — real captured impulse responses
- **Full effects chain** — 6 studio-quality effects
- **8 built-in presets** — playable out of the box
- **VST3 + CLAP formats** — works in all major DAWs
- **Apple Silicon native** — built for M1/M2/M3, no Rosetta needed
- **Zero latency mode** — < 5ms at 128 samples
- **Thread-safe parameter system** — no glitches when tweaking knobs
- **Real-time VU metering** — know your levels at a glance
- **Free and open source** — GPL-3.0, modify and share freely

---

## Contributing

Found a bug? Want a new amp model or feature?

- **Bug reports:** [Open an issue](https://github.com/Tehuti01/thunderfpr/issues)
- **Feature requests:** [Start a discussion](https://github.com/Tehuti01/thunderfpr/discussions)
- **Pull requests:** Fork the repo and submit — all contributions welcome

---

## Credits

**Developer:** Lukas Hansen / Stradego Capital
**Plugin Framework:** [nih-plug](https://github.com/robbert-vdh/nih-plug)
**DSP:** Custom Rust implementation
**Language:** Rust

---

## License

GPL-3.0-or-later — see [LICENSE](LICENSE)

This software is free to use, modify, and distribute under the terms of the GPL license. If you distribute modified versions, they must also be GPL-licensed and open source.

---

<div align="center">

**LH Thunderforge** — Professional amp simulation, zero cost.

[⬇ Download](https://github.com/Tehuti01/thunderfpr/releases) · [Issues](https://github.com/Tehuti01/thunderfpr/issues) · [Discussions](https://github.com/Tehuti01/thunderfpr/discussions)

</div>
