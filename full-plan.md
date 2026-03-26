╔══════════════════════════════════════════════════════════════════════════════════════════════════════╗
║                                                                                                    ║
║        THE SYMBIOSIS PROTOCOL — UNIVERSAL AI CONSTRUCTION PROMPT v3.0                              ║
║        Professional-Grade Guitar Amp Simulator & Neural Modeler                                    ║
║        Dual-Implementation: Rust (nih-plug) + C++ (JUCE)                                           ║
║                                                                                                    ║
║        Codename: THUNDERFORGE                                                                      ║
║        Author: Lukas Hansen / Stradego Capital                                                     ║
║        Classification: ENGINEERING SPEC — Fully Implementable                                      ║
║        Target: VST3 + CLAP Plugin with Hyper-Realistic Skeuomorphic UI                             ║
║                                                                                                    ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TABLE OF CONTENTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PART I      SYSTEM IDENTITY & APEX DEVELOPER DIRECTIVES          §1–§5
PART II     VISUAL DESIGN SYSTEM — THE HARDWARE AESTHETIC        §6–§12
PART III    AUDIO DSP ARCHITECTURE — THE SOUND ENGINE            §13–§20
PART IV     NEURAL AMP MODELING — AI TONE ENGINE                 §21–§28
PART V      IMPLEMENTATION A: RUST / NIH-PLUG                   §29–§42
PART VI     IMPLEMENTATION B: C++ / JUCE                         §43–§56
PART VII    PRESET LIBRARY — LEGENDARY TONE PROFILES             §57–§64
PART VIII   UI COMPONENT SPECIFICATIONS                          §65–§78
PART IX     BUILD, TEST & DISTRIBUTION PIPELINE                  §79–§86
PART X      CROSS-PLATFORM DEPLOYMENT                            §87–§92
PART XI     ADVANCED TOPICS & OPTIMIZATION                       §93–§98
PART XII    FILE STRUCTURE & CODE ARCHITECTURE                   §99–§106
APPENDIX A  QUICK-START COMMANDS
APPENDIX B  DSP ALGORITHM REFERENCE
APPENDIX C  PARAMETER ID REGISTRY
APPENDIX D  NEURAL MODEL FORMAT SPECIFICATION
APPENDIX E  CSS/UI TOKEN REFERENCE
APPENDIX F  TESTING CHECKLIST

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
HOW TO USE THIS PROMPT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

This document is a UNIVERSAL CONSTRUCTION PROMPT. Feed it to any capable AI coding assistant
(Claude, GPT-4, Gemini, etc.) and instruct it to build:

OPTION A → "Build the Rust/nih-plug implementation" (uses PART V)
OPTION B → "Build the C++/JUCE implementation" (uses PART VI)
OPTION C → "Build both implementations" (uses PART V + VI)

All other parts (I–IV, VII–XII, Appendices) are SHARED between both implementations.
The AI must read ALL shared parts first, then execute the language-specific implementation.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART I — SYSTEM IDENTITY & APEX DEVELOPER DIRECTIVES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 1. PROJECT IDENTITY CARD
─────────────────────────────────────────────────────────────

Field                   | Value
------------------------|------------------------------------------------------
Project Name            | The Symbiosis Protocol
Codename                | THUNDERFORGE
Plugin Name             | LH Thunderforge
Vendor                  | Lukas Hansen Audio / Stradego Capital
Version                 | 1.0.0
Plugin Formats          | VST3, CLAP (Rust) / VST3, AU, AAX (JUCE)
Plugin Category         | Guitar Amp Simulator / Effects Processor
UI Dimensions           | 1200 x 800 pixels (default), resizable
Sample Rates            | 44100, 48000, 88200, 96000, 176400, 192000 Hz
Buffer Sizes            | 32 to 4096 samples
Latency Target          | < 5ms at 44.1kHz with 128-sample buffer
Bit Depth               | 32-bit float internal, 64-bit double optional
Channels                | Mono-in → Stereo-out (guitar input)
OS Targets              | Windows 10+, macOS 11+, Linux (Ubuntu 22.04+)
Brand Tagline           | "Minimalist Code. Maximalist Sound."
Design Philosophy       | Hardware-grade skeuomorphism meets silicon precision
UI Philosophy           | Every pixel must feel machined from titanium

§ 2. THE APEX DEVELOPER SYSTEM PROMPT — CORE DIRECTIVES
─────────────────────────────────────────────────────────────

You are an Apex-Tier Audio Software Architect. You do not build standard plugins;
you engineer "Digital Hardware." Your aesthetic baseline is the intersection of
premium audio software (FabFilter, Neural DSP, Universal Audio) and elite
Silicon Valley design (Vercel, Anthropic, Linear). You design with mathematical
precision, optical depth, and absolute confidence.

CRITICAL: These directives MUST be followed for ALL visual output in this project.
They apply equally to JUCE's custom paint routines AND Rust's webview/iced/egui rendering.

§ 3. CORE AESTHETIC DIRECTIVES
─────────────────────────────────────────────────────────────

DIRECTIVE 1: SPATIAL PRESSURE AND THE PHI GRID

Never use standard spacing. Calculate ALL margins, padding, and layout grids
using the Golden Ratio (φ = 1.618033988749895).

Base Unit:     8px
Scale:         8, 13, 21, 34, 55, 89, 144, 233, 377, 610 (Fibonacci)
Micro Scale:   1, 2, 3, 5, 8 px (for fine details)

Layout Principle: Create extreme spatial tension — balance microscopic,
high-density data clusters (parameter readouts, LED meters) with massive,
intentional voids of negative space (between major sections).

Panel Proportions: Primary panels use φ ratio splits:
- Main display area: 61.8% width
- Side panel:        38.2% width
- Top controls:      38.2% height
- Bottom amp face:   61.8% height

Implementation (JUCE):
constexpr float PHI = 1.618033988749895f;
constexpr float PHI_INV = 0.618033988749895f;
int mainWidth = static_cast<int>(totalWidth * PHI_INV);
int sideWidth = totalWidth - mainWidth;

Implementation (Rust/CSS):
:root {
--phi: 1.618033988749895;
--phi-inv: 0.618033988749895;
--phi-0: 3.77px;   --phi-1: 6.10px;   --phi-2: 9.87px;
--phi-3: 15.97px;  --phi-4: 25.83px;   --phi-5: 41.78px;
--phi-6: 67.58px;  --phi-7: 109.31px;  --phi-8: 176.81px;
--phi-9: 286.00px; --phi-10: 462.62px; --phi-11: 748.31px;
--phi-12: 1210.42px;
}

DIRECTIVE 2: OPTICAL MATERIALS AND SPECULAR LIGHTING

Eradicate all flat, solid colors. Treat UI components as PHYSICAL MATERIALS.

Surface Types:
┌─────────────────┬──────────────────────────────────────────────────────┐
│ Material        │ Implementation                                       │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Brushed Metal   │ Multi-stop linear gradient (min 5 stops) with       │
│                 │ noise overlay at 0.03 opacity. Simulate directional  │
│                 │ brushing via repeating-linear-gradient at 0.5px      │
│                 │ intervals with alternating 2% lightness variance.    │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Machined        │ Radial gradient simulating chamfered edges.          │
│ Titanium        │ Inner highlight at 12 o'clock position.              │
│                 │ Ambient occlusion shadow on bottom-right.            │
│                 │ Base: rgb(45, 50, 58). Highlight: rgb(120, 130, 145)│
├─────────────────┼──────────────────────────────────────────────────────┤
│ Frosted Glass   │ backdrop-filter: blur(12px) saturate(180%).          │
│                 │ Background: rgba(10, 14, 26, 0.75).                  │
│                 │ Border: 1px solid rgba(255, 255, 255, 0.06).         │
│                 │ Inner shadow: inset 0 1px 0 rgba(255,255,255,0.04). │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Amp Tolex       │ Repeating SVG pattern simulating leather/vinyl       │
│                 │ texture. Dark base with subtle diamond pattern.      │
│                 │ Emboss effect via dual shadow layers.                │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Chrome Bezel    │ 3-stop gradient: dark → bright highlight → dark.    │
│                 │ Sharp specular highlight at 1px width.               │
│                 │ Reflection simulation via gradient angle matching    │
│                 │ global light source at 315 degrees (top-left).      │
├─────────────────┼──────────────────────────────────────────────────────┤
│ LCD Display     │ Background: rgb(8, 12, 8). Text: rgb(0, 255, 120). │
│                 │ Monospace font. Subtle scanline overlay.             │
│                 │ Text-shadow glow: 0 0 8px rgba(0,255,120,0.6).     │
│                 │ Pixel grid overlay at 0.02 opacity.                  │
└─────────────────┴──────────────────────────────────────────────────────┘

Shadows: Every elevated element uses MULTI-LAYER ambient occlusion:
Layer 1 (close):   0 1px 2px rgba(0,0,0,0.3)
Layer 2 (medium):  0 4px 8px rgba(0,0,0,0.2)
Layer 3 (far):     0 12px 24px rgba(0,0,0,0.15)
Layer 4 (ambient): 0 24px 48px rgba(0,0,0,0.08)

Global Noise Grain:
Apply a fixed, microscopic fractal noise texture to the global background:
- Opacity: 0.03 to 0.05
- Blend mode: overlay
- This kills digital flatness and adds physical grit
- Implementation: SVG <feTurbulence> filter or pre-rendered PNG tile

JUCE Implementation:
void paintSurface(juce::Graphics& g, juce::Rectangle<int> bounds, SurfaceType type) {
// Example: Brushed Metal
juce::ColourGradient gradient(
juce::Colour(55, 60, 70), bounds.getX(), bounds.getY(),
juce::Colour(35, 40, 48), bounds.getRight(), bounds.getBottom(), false);
gradient.addColour(0.3, juce::Colour(65, 72, 82));
gradient.addColour(0.5, juce::Colour(50, 56, 65));
gradient.addColour(0.7, juce::Colour(58, 64, 74));
g.setGradientFill(gradient);
g.fillRect(bounds);
// Add noise overlay
paintNoiseOverlay(g, bounds, 0.04f);
}

DIRECTIVE 3: MONUMENTAL TYPOGRAPHY

Maximize typographic contrast. The plugin name and section headers are
colossal. Technical readouts are microscopic.

Type Scale (φ-derived):
--type-3xs:  7.64px    (micro labels, version numbers)
--type-2xs:  9.33px    (parameter units: "dB", "Hz", "ms")
--type-xs:   12.36px   (secondary labels)
--type-sm:   15.10px   (control labels)
--type-base: 20.00px   (body text, parameter names)
--type-lg:   24.36px   (section headers)
--type-xl:   32.36px   (panel titles)
--type-2xl:  39.42px   (major headers)
--type-3xl:  52.36px   (hero elements)
--type-4xl:  63.82px   (plugin name)
--type-5xl:  84.72px   (display numbers — gain readout)
--type-6xl:  103.27px  (monumental — splash screen only)

Font Families:
Display/Headlines: "DM Sans", "Satoshi", "Cabinet Grotesk" — bold, tight tracking
Technical/Labels:  "JetBrains Mono", "IBM Plex Mono", "Berkeley Mono" — monospaced
Parameter Values:  "Tabular Nums" variant of mono font — aligned decimal points

Headline Treatment:
- letter-spacing: -0.04em (negative tracking for compression)
- line-height: 0.85 (extremely tight leading)
- font-weight: 800 or 900
- For chrome/metallic finish: background-clip: text with metallic gradient

Technical Label Treatment:
- letter-spacing: 0.25em to 0.4em (wide tracking)
- text-transform: uppercase
- font-weight: 400 or 500
- font-size: --type-3xs to --type-xs
- color: muted steel blue or amber LCD

DIRECTIVE 4: KINETIC PHYSICS AND TACTILE WEIGHT

Every interactive element must respond with PHYSICAL WEIGHT.

Spring Physics Configuration:
Standard:    { stiffness: 400, damping: 30, mass: 1.0 }
Heavy:       { stiffness: 200, damping: 40, mass: 2.0 }
Snappy:      { stiffness: 600, damping: 25, mass: 0.8 }
Elastic:     { stiffness: 300, damping: 15, mass: 1.2 }

Knob Interaction Physics:
- Rotation follows mouse delta with momentum
- Release triggers spring-back to nearest detent (if snapping enabled)
- Double-click resets to default with elastic spring animation
- Visual feedback: specular highlight shifts with rotation angle
- Shadow recalculates based on rotation (3D depth illusion)

Button Interaction Physics:
- Press: translateY(2px) with shadow reduction (surface depression)
- Active state maintains depression for toggle buttons
- LED indicators pulse with eased glow on state change
- Spring release animation on mouse-up (bounces back)

3D Tilt Parallax (Cards/Panels):
- Track mouse position relative to element center
- Apply CSS transform: perspective(800px) rotateX(Xdeg) rotateY(Ydeg)
- Maximum tilt: 3 degrees (subtle, not gimmicky)
- Specular highlight follows tilt angle
- Transition: 150ms ease-out on mouse-move, 400ms spring on mouse-leave

JUCE Implementation:
// In knob mouse drag handler:
float delta = event.getDistanceFromDragStartY() * sensitivity;
float springForce = -stiffness * (currentValue - targetValue);
float dampingForce = -damping * currentVelocity;
currentVelocity += (springForce + dampingForce) / mass * deltaTime;
currentValue += currentVelocity * deltaTime;

DIRECTIVE 5: ARCHITECTURAL COMPONENT PATTERNS

THE MONOLITH:
- The central amp display. Massive, centralized hub that draws the eye
- Contains: amp model visualization, primary gain/EQ knobs, master level
- Dimensions: minimum 60% of total UI width, centered
- Visual weight: heaviest shadows, brightest highlights, most detail
- The amp face is the HERO — everything else orbits around it

THE ASYMMETRIC BENTO:
- Surrounding panels use 12-column grids with UNEVEN card widths
- Example layout: [3col][5col][4col] or [7col][5col]
- Cards bounded by glowing 1px borders (fiber-optic aesthetic)
- Border color: rgba(100, 180, 255, 0.08) default
- Border glow on hover: box-shadow 0 0 12px rgba(100, 180, 255, 0.15)

THE TERMINAL FORM:
- All text inputs styled as high-end developer consoles
- No standard white boxes — dark background, bottom-border only
- Monospaced placeholder text in muted color
- Active state: bottom-border glows amber/cyan
- Caret: block cursor, not line cursor
- Font: JetBrains Mono or equivalent

THE STATUS BAR:
- Bottom strip showing real-time engine metrics
- CPU load, sample rate, buffer size, latency, model name
- LCD-style green text on near-black background
- Monospaced, wide-tracked, uppercase

§ 4. COLOR SYSTEM — THE THUNDERFORGE PALETTE
─────────────────────────────────────────────────────────────

Role               | Color Name        | Hex/RGB                  | Usage
-------------------|-------------------|--------------------------|----------------------------------
Background Deep    | Abyss             | #050507                  | Deepest background layer
Background Base    | Deep Navy         | #0a0e1a                  | Main window background
Surface 1          | Charcoal Panel    | #141820                  | Primary panels
Surface 2          | Steel Panel       | #1e232e                  | Elevated panels
Surface 3          | Slate             | #2a303d                  | Highest elevation
Amp Face Base      | Tolex Black       | #0d0d0f                  | Amplifier face background
Amp Face Accent    | Marshall Gold     | #c8a84e                  | Amp panel trim, piping
Chrome Highlight   | Bright Chrome     | #e8edf5                  | Specular highlights on metal
Chrome Mid         | Brushed Steel     | #8892a4                  | Mid-tone on metal surfaces
Chrome Shadow      | Dark Steel        | #3a4050                  | Shadow on metal surfaces
Primary Accent     | Amber Gold        | #ffb000                  | Primary buttons, active states
Secondary Accent   | Signal Cyan       | #06b6d4                  | Secondary highlights, meters
Success / On       | Tube Green        | #00ff78                  | LED on-state, positive values
Danger / Clip      | Redline           | #ff3344                  | Clipping indicator, warnings
Warm Indicator     | Valve Orange      | #ff8800                  | Warm/driven indicators
Text Primary       | Soft White        | #dce6f0                  | Primary readable text
Text Secondary     | Steel Blue        | #6488b4                  | Secondary labels
Text Muted         | Dim Steel         | #3e4a5c                  | Disabled / tertiary text
Text LCD           | LCD Green         | #00ff78                  | LCD display text
Text LCD Amber     | LCD Amber         | #ffb000                  | Amber LCD variant
Border Default     | Ghost Line        | rgba(255,255,255,0.06)   | Panel borders
Border Active      | Glow Line         | rgba(255,176,0,0.3)     | Active/focused borders
Knob Body          | Dark Aluminum     | #2a2d35                  | Knob base color
Knob Indicator     | Bright Notch      | #ffffff                  | Knob position indicator line
Knob Ring          | Ring Shadow       | #1a1c22                  | Recessed ring around knob

§ 5. GLOBAL LIGHT SOURCE MODEL
─────────────────────────────────────────────────────────────

All visual elements in the UI share a SINGLE CONSISTENT LIGHT SOURCE:

Light Position:   Top-left, 315 degrees (northwest)
Light Elevation:  45 degrees above the surface plane
Light Type:       Soft directional (not point light)
Ambient Light:    20% fill (prevents pure black shadows)

This means:
- ALL gradients flow from top-left (light) to bottom-right (shadow)
- ALL shadows cast toward bottom-right
- ALL specular highlights appear on top-left edges
- ALL bevels have bright top-left edge, dark bottom-right edge
- ALL knob specular dots are positioned at 10-11 o'clock

Consistency in lighting is what makes the UI feel like a REAL PHYSICAL OBJECT
rather than a flat graphic. Every single element must respect this light model.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART II — VISUAL DESIGN SYSTEM — THE HARDWARE AESTHETIC
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 6. THE AMP FACE — CENTRAL MONOLITH
─────────────────────────────────────────────────────────────

The amp face is the absolute visual center of the plugin. It occupies the
lower 61.8% of the UI height and represents a photorealistic vintage
Marshall-style amplifier front panel.

Visual Layers (bottom to top):
Layer 0: Tolex texture background (dark leather/vinyl pattern)
Layer 1: Chrome chassis plate (brushed metal, raised 4px with shadow)
Layer 2: Control panel (dark faceplate with silk-screened labels)
Layer 3: Knobs (3D cylindrical, with indicator lines and shadows)
Layer 4: LED indicators (recessed, with glow halos)
Layer 5: LCD/OLED displays (for digital readouts)

Amp Face Layout (left to right):
┌──────────────────────────────────────────────────────────────────────┐
│  ╔═══════════════════════════════════════════════════════════════╗   │
│  ║  [INPUT]  [GAIN]──[BASS]──[MID]──[TREBLE]──[PRESENCE]       ║   │
│  ║           ◉       ◉       ◉       ◉         ◉               ║   │
│  ║                                                               ║   │
│  ║  [VOLUME]──[MODEL SELECT ▼]──[CAB SELECT ▼]──[MASTER]       ║   │
│  ║   ◉         ╔══LCD══╗        ╔══LCD══╗        ◉             ║   │
│  ║              ║PLEXI ║        ║4x12GB║        [POWER LED]    ║   │
│  ║              ╚══════╝        ╚══════╝                        ║   │
│  ╚═══════════════════════════════════════════════════════════════╝   │
│              L  H      T H U N D E R F O R G E                      │
└──────────────────────────────────────────────────────────────────────┘

§ 7. KNOB DESIGN SPECIFICATION — THE CROWN JEWEL
─────────────────────────────────────────────────────────────

Every knob is a multi-layered visual construction simulating a real
3D control. There are NO flat circles. Each knob type has specific
visual properties:

KNOB TYPE A — "Marshall Style" (Primary Controls: Gain, Volume, Master)
Diameter: 55px (large), 42px (medium), 30px (small)
Layers:
1. Shadow Ring:  Inset shadow simulating recessed mounting hole
box-shadow: inset 0 2px 6px rgba(0,0,0,0.8)
2. Skirt:        Dark metallic ring, gradient from #1a1c22 to #0e0f12
Subtle radial gradient for 3D curvature
3. Body:         Main knob surface — conical gradient
Center: #4a4e58, Edge: #2a2d35
Specular highlight: radial gradient at 10 o'clock
Size: 70% of total knob diameter
4. Cap:          Flat top with embossed number or dot
Background: #35393f
Border: 1px solid rgba(255,255,255,0.05)
5. Indicator:    White line from center to edge
Width: 2px, Color: #ffffff
Extends from 60% to 95% of radius
Casts micro-shadow: 0 1px 2px rgba(0,0,0,0.5)
6. Value Arc:    270-degree arc around the knob
Background track: rgba(255,255,255,0.04)
Active fill: amber gradient (#ff8800 → #ffb000)
Width: 3px, rounded caps
7. Detent Marks: 11 tick marks around the 270-degree arc
Tiny rectangles: 1px x 4px, color rgba(255,255,255,0.08)
Every other mark slightly longer (6px) for major steps

Value Display: Appears on hover/drag
Position: Centered below knob
Style: LCD amber text, JetBrains Mono, --type-xs
Format: "7.5" or "3.2 kHz" (context-dependent)
Animation: Fade in 100ms on hover, fade out 200ms on leave

KNOB TYPE B — "Chicken Head" (Model Select, Cab Select)
Shape: Pointer/arrow style instead of round
Steps: Discrete positions (click-stops), no smooth rotation
Visual: Arrow-shaped indicator instead of line
Detents: Visible notch positions around the dial

KNOB TYPE C — "Small Trim" (Fine Adjustments)
Diameter: 24px
Simplified: 2 layers only (body + indicator)
No value arc, no detent marks
Used for: fine-tune, bias, sag

§ 8. LED INDICATOR DESIGN
─────────────────────────────────────────────────────────────

LED Type A — "Power LED" (Single Status)
Diameter: 8px
Off State:   Background: #1a0000, border: 1px solid #300000
On State:    Background: radial-gradient(#ff3344, #cc0000)
box-shadow: 0 0 4px #ff3344, 0 0 12px rgba(255,51,68,0.4)
Transition:  150ms ease-in-out

LED Type B — "Signal LED" (Level-Reactive)
Same as Type A but color varies with signal level:
Silent:    Off (dark)
Signal:    Green (#00ff78)
Hot:       Amber (#ffb000)
Clipping:  Red (#ff3344), plus rapid pulse animation

LED Type C — "VU Segment" (Meter Bar)
Array of 12 rectangular segments, 4px x 8px each
Colors by position: Green (1-7), Amber (8-10), Red (11-12)
Segments illuminate from bottom to top
Off segments: rgba(255,255,255,0.03)
On segments:  Full color with 0 0 6px glow

§ 9. LCD / OLED DISPLAY PANELS
─────────────────────────────────────────────────────────────

Two display types exist in the UI:

LCD TYPE — "Green Phosphor" (Status Displays)
Background:   #080c08
Text Color:   #00ff78
Text Shadow:  0 0 8px rgba(0,255,120,0.6)
Font:         JetBrains Mono, 700 weight
Border:       2px solid #1a2a1a (dark green frame)
Bezel:        3px inset shadow (recessed screen)
Scanlines:    Overlay of 1px horizontal lines at 0.03 opacity
Pixel Grid:   2x2 px grid overlay at 0.02 opacity (simulates dot matrix)
Corner Radius: 2px (nearly square, industrial look)

OLED TYPE — "High-Res" (Model Info, Waveform Display)
Background:   #000000 (true black)
Text Color:   Variable — white for text, colored for data
Font:         DM Sans for labels, JetBrains Mono for values
Border:       1px solid rgba(255,255,255,0.04) (barely visible)
Bezel:        Thin chrome frame (1px bright line top-left, dark bottom-right)
Corner Radius: 6px (modern, rounded)
Content:      Can display waveform visualizations, frequency curves, model info

§ 10. SWITCH DESIGN SPECIFICATION
─────────────────────────────────────────────────────────────

SWITCH TYPE A — "Toggle" (Channel Select, Boost On/Off)
Physical Style: Bat-handle toggle switch
Visual: Metal lever with chrome finish, pivot point visible
States: 2-position or 3-position
Animation: Spring-physics flip between positions (stiffness: 500, damping: 28)
Sound: Optional UI click sound on toggle
Shadow: Lever casts shadow that shifts with position

SWITCH TYPE B — "Stomp" (Effects Bypass)
Physical Style: Guitar pedal footswitch
Visual: Round metal button, recessed into a circular housing
Diameter: 36px
States: On (illuminated LED ring) / Off (dark)
LED Ring: 2px ring around switch, glows in effect color when active
Depression: 2px translateY on press

SWITCH TYPE C — "Rocker" (Power Switch)
Physical Style: IEC-standard rocker switch
Visual: Rectangular, pivots on center axis
States: ON (I) / OFF (O) marked on switch face
Adjacent LED: Red power LED illuminates when ON

§ 11. EFFECT PEDAL CHAIN — TOP SECTION UI
─────────────────────────────────────────────────────────────

The top 38.2% of the UI contains the signal chain view:

Layout:
┌────────────────────────────────────────────────────────────────────┐
│  SIGNAL CHAIN                                                      │
│  ┌────┐   ┌────┐   ┌────┐   ┌────┐   ┌────────┐   ┌────┐       │
│  │GATE│──▶│TS  │──▶│COMP│──▶│ AMP │──▶│  CAB   │──▶│REV │       │
│  │    │   │    │   │    │   │MODEL│   │ SIM    │   │    │       │
│  └────┘   └────┘   └────┘   └────┘   └────────┘   └────┘       │
│   [ON]     [ON]     [OFF]    [ALWAYS]   [ALWAYS]    [ON]         │
└────────────────────────────────────────────────────────────────────┘

Each pedal block:
- 89px x 89px (Fibonacci square)
- Rounded corners: 8px
- Surface: Frosted glass material
- Active border glow when enabled
- Stomp switch (Type B) below each block
- Connection lines: 1px dashed, rgba(255,255,255,0.08)
- Arrow indicators at connection points

Signal Flow Visualization:
- Animated signal particles flow along connection lines when audio is present
- Particles speed correlates to signal amplitude
- Color: amber/cyan gradient
- 2px diameter circles, 0.6 opacity, motion blur

§ 12. WAVEFORM & SPECTRUM VISUALIZER
─────────────────────────────────────────────────────────────

Located in the top-right OLED display panel:

Waveform Mode:
- Real-time audio waveform, 60fps update
- Line width: 1.5px
- Color: Gradient from cyan (#06b6d4) to green (#00ff78)
- Background: True black with subtle grid lines
- Grid: 0.02 opacity, major divisions every 25%
- Scroll mode: Oscilloscope-style, triggered on zero-crossing

Spectrum Mode:
- FFT visualization, 2048-point
- Bar display or smooth curve
- Color: Gradient matching signal chain position
- Logarithmic frequency axis (20Hz — 20kHz)
- dB scale: -60dB to 0dB
- Peak hold: 1.5 second decay

Toggle: Click display to switch between Waveform and Spectrum modes.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART III — AUDIO DSP ARCHITECTURE — THE SOUND ENGINE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 13. SIGNAL CHAIN ARCHITECTURE
─────────────────────────────────────────────────────────────

The complete audio signal chain, processed per-sample or per-block:

INPUT → [Noise Gate] → [Drive/TS Pedal] → [Compressor]
→ [PRE-AMP / NEURAL MODEL] → [TONE STACK (EQ)]
→ [POWER AMP / NEURAL MODEL] → [CABINET SIM (IR Loader)]
→ [Effects Loop: Delay → Reverb → Chorus]
→ [Master Volume] → OUTPUT

Processing Order is CRITICAL. Guitar signal chains are NOT commutative.
The order above matches real-world amplifier topology.

§ 14. INPUT STAGE
─────────────────────────────────────────────────────────────

Input Gain: -12dB to +12dB trim
Purpose: Match guitar output levels (single-coil vs humbucker)
Implementation: Simple multiplication by gain coefficient
Parameter ID: "input_gain"
Default: 0.0 dB

Input Buffer:
- Circular buffer for lookahead processing (noise gate)
- Size: 256 samples at current sample rate
- Thread-safe ring buffer implementation

§ 15. NOISE GATE
─────────────────────────────────────────────────────────────

Purpose: Silence signal below threshold to eliminate hum/noise

Parameters:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Threshold         | gate_threshold  | -80 to 0     | -45      | dB
Attack            | gate_attack     | 0.1 to 50    | 1.0      | ms
Hold              | gate_hold       | 0 to 500     | 50       | ms
Release           | gate_release    | 1 to 2000    | 100      | ms
Bypass            | gate_bypass     | 0 or 1       | 0        | bool

Algorithm: Envelope follower with hysteresis
1. Compute RMS level over 1ms window
2. Compare against threshold with 3dB hysteresis band
3. On open: Apply attack envelope (linear ramp)
4. On close: Hold for gate_hold ms, then apply release envelope
5. Multiply signal by envelope value (0.0 = closed, 1.0 = open)

§ 16. OVERDRIVE / TUBE SCREAMER STAGE
─────────────────────────────────────────────────────────────

Purpose: Pre-amp drive pedal emulation (Tube Screamer topology)

Parameters:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Drive             | ts_drive        | 0 to 100     | 50       | %
Tone              | ts_tone         | 0 to 100     | 50       | %
Level             | ts_level        | 0 to 100     | 50       | %
Bypass            | ts_bypass       | 0 or 1       | 0        | bool

Algorithm: Asymmetric soft-clipping with tone filter
1. Apply input highpass filter at 720 Hz (removes low-end mud)
2. Apply gain staging: gain = 1.0 + (drive * 5.0)
3. Soft-clip using tanh waveshaper:
   output = tanh(input * gain) / tanh(gain)
4. Apply asymmetry: positive peaks clip slightly harder
   if (sample > 0) sample *= 1.1;
5. Apply tone control: first-order lowpass filter
   Cutoff: 1000 Hz + (tone * 4000 Hz)  // 1kHz to 5kHz sweep
6. Apply output level scaling

§ 17. COMPRESSOR STAGE
─────────────────────────────────────────────────────────────

Purpose: Dynamic range compression for sustain and punch

Parameters:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Threshold         | comp_threshold  | -60 to 0     | -20      | dB
Ratio             | comp_ratio      | 1.0 to 20.0  | 4.0      | :1
Attack            | comp_attack     | 0.1 to 100   | 10       | ms
Release           | comp_release    | 10 to 1000   | 100      | ms
Makeup Gain       | comp_makeup     | 0 to 24      | 0        | dB
Bypass            | comp_bypass     | 0 or 1       | 1        | bool

Algorithm: Feed-forward compressor with RMS detection
1. Compute RMS level (1ms window)
2. Convert to dB: level_db = 20 * log10(rms)
3. Compute gain reduction:
   if (level_db > threshold)
   reduction = (level_db - threshold) * (1.0 - 1.0/ratio)
4. Smooth gain with attack/release envelope follower
5. Apply gain: output = input * db_to_linear(-reduction + makeup)

§ 18. TONE STACK — 3-BAND EQ
─────────────────────────────────────────────────────────────

Purpose: Classic Marshall/Fender tone stack EQ

Parameters:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Bass              | eq_bass         | 0 to 10      | 5        | (0-10 scale)
Mid               | eq_mid          | 0 to 10      | 5        | (0-10 scale)
Treble            | eq_treble       | 0 to 10      | 5        | (0-10 scale)
Presence          | eq_presence     | 0 to 10      | 5        | (0-10 scale)

Algorithm: Marshall-type tone stack (James topology)
This is a passive RC network emulation using:
- Bass:    Shelving filter centered at 100 Hz, +/- 12dB range
- Mid:     Peaking filter centered at 800 Hz, +/- 12dB range, Q=0.7
- Treble:  Shelving filter centered at 3.2 kHz, +/- 12dB range
- Presence: High shelf at 5 kHz, +/- 6dB range

Each band uses a biquad filter (second-order IIR):
y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]

Coefficients recalculated when any parameter changes.
Use Robert Bristow-Johnson's Audio EQ Cookbook formulas.

§ 19. CABINET SIMULATION — IR LOADER
─────────────────────────────────────────────────────────────

Purpose: Convolution-based cabinet impulse response loading

Parameters:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Cabinet Model     | cab_model       | 0 to N       | 0        | enum
Mix               | cab_mix         | 0 to 100     | 100      | %
Bypass            | cab_bypass      | 0 or 1       | 0        | bool

Built-in Cabinet IRs:
ID  | Name              | Description
----|-------------------|--------------------------------------------------
0   | 4x12 Greenback    | Marshall 1960A with Celestion G12M Greenback
1   | 4x12 V30          | Marshall 1960B with Celestion Vintage 30
2   | 2x12 Blue         | Fender Twin with Jensen C12Q
3   | 1x12 Open Back    | Fender Deluxe Reverb style
4   | 4x12 T75          | Marshall JCM800 with Celestion G12T-75
5   | Custom IR          | User-loaded WAV file

Algorithm: Partitioned Convolution (FFT-based)
- IR Length: Up to 4096 samples (supports 44.1kHz-192kHz)
- Partition Size: Match host buffer size
- Use overlap-add or overlap-save method
- FFT library: rustfft (Rust) or juce::dsp::Convolution (JUCE)
- Minimum phase option: Convert IR to minimum phase for lower latency

§ 20. EFFECTS SECTION — POST-AMP
─────────────────────────────────────────────────────────────

DELAY:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Time              | delay_time      | 1 to 2000    | 375      | ms
Feedback          | delay_feedback  | 0 to 95      | 40       | %
Mix               | delay_mix       | 0 to 100     | 30       | %
Sync              | delay_sync      | 0 or 1       | 0        | bool
Bypass            | delay_bypass    | 0 or 1       | 0        | bool

Algorithm: Interpolating delay line
- Circular buffer with fractional read position
- Linear or cubic interpolation for non-integer delay times
- Feedback with soft-clip saturation to prevent blowup
- Optional tempo sync (1/4, 1/8, 1/8 dotted, 1/4 triplet)
- Lowpass filter in feedback path (2kHz-12kHz) for tape delay character

REVERB:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Size              | reverb_size     | 0 to 100     | 50       | %
Decay             | reverb_decay    | 0.1 to 10.0  | 2.0      | seconds
Damping           | reverb_damping  | 0 to 100     | 50       | %
Pre-Delay         | reverb_predelay | 0 to 200     | 20       | ms
Mix               | reverb_mix      | 0 to 100     | 20       | %
Bypass            | reverb_bypass   | 0 or 1       | 0        | bool

Algorithm: Feedback Delay Network (FDN) or Schroeder/Moorer
- 8 parallel comb filters + 4 series allpass filters
- Hadamard mixing matrix for dense reflections
- Damping via lowpass filters in feedback paths
- Pre-delay via simple delay line before reverb input
- Stereo output: decorrelate via offset delays

CHORUS:
Parameter         | ID              | Range        | Default  | Unit
------------------|-----------------|------------- |----------|------
Rate              | chorus_rate     | 0.1 to 10.0  | 1.0      | Hz
Depth             | chorus_depth    | 0 to 100     | 50       | %
Mix               | chorus_mix      | 0 to 100     | 50       | %
Bypass            | chorus_bypass   | 0 or 1       | 1        | bool

Algorithm: Modulated delay line
- LFO: Sine wave at chorus_rate
- Delay range: 1ms to 25ms, modulated by LFO * depth
- Stereo: Two voices with 90-degree phase offset

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART IV — NEURAL AMP MODELING — AI TONE ENGINE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 21. WHAT IS NEURAL AMP MODELING (NAM)?
─────────────────────────────────────────────────────────────

Neural Amp Modeler (NAM) uses deep neural networks trained on audio recordings
of real amplifiers to replicate their exact sonic characteristics. Unlike
traditional DSP (which approximates circuit behavior with math), NAM captures
the ACTUAL transfer function of physical hardware — including all nonlinear
saturation, harmonic generation, frequency-dependent compression, and
"feel" characteristics that make vintage amps special.

Key Technology: NeuralAmpModelerCore
Repository: github.com/sdatkinson/NeuralAmpModelerCore
Language: C++17
License: MIT
Purpose: Core DSP library for loading and running NAM model files
Model Types Supported: WaveNet, LSTM, Feedforward
Model Format: .nam (JSON metadata + binary weights)

Alternative for Rust: RTNeural
Repository: github.com/jatinchowdhury18/RTNeural
Language: C++17 with Rust FFI possible
Purpose: Optimized real-time neural network inference
Supported Architectures: GRU, LSTM, Dense, Conv1D
Optimizations: SIMD (SSE2/AVX/NEON), Eigen backend

§ 22. NAM INTEGRATION ARCHITECTURE
─────────────────────────────────────────────────────────────

For C++ / JUCE:
- Include NeuralAmpModelerCore as a submodule or CMake dependency
- Link against the NAMCore library
- Use DSP::NAMProcessor class to load .nam model files
- Process audio through model in processBlock()

For Rust / nih-plug:
OPTION A — FFI Bridge to NeuralAmpModelerCore:
- Build NeuralAmpModelerCore as a static library (.a / .lib)
- Create Rust bindings using bindgen or manual extern "C" declarations
- Wrap in a safe Rust API: NamModel::load(path) → NamModel
- Process: model.process(&input_buffer, &mut output_buffer)

OPTION B — Pure Rust Implementation:
- Port the WaveNet inference to Rust using ndarray + SIMD intrinsics
- Load .nam JSON metadata with serde_json
- Load binary weights into ndarray arrays
- Implement forward pass with optimized matrix multiplication
- Use packed_simd2 or std::simd (nightly) for vectorization

OPTION C — RTNeural via FFI:
- Similar to Option A but using RTNeural C++ library
- Provides optimized GRU/LSTM/Dense layers
- Can load pre-trained models in custom format

RECOMMENDED: Option A for fastest time-to-market, Option B for pure Rust purity.

§ 23. NAM MODEL LOADING PIPELINE
─────────────────────────────────────────────────────────────

Step 1: Parse .nam file header (JSON)
{
"version": "0.5.4",
"architecture": "WaveNet",
"config": {
"input_size": 1,
"condition_size": 0,
"head_size": 1,
"channels": 16,
"layers": 18,
"kernel_size": 3,
"dilation_depth": 9,
"head_channels": 8
},
"sample_rate": 48000
}

Step 2: Load binary weights into memory-aligned arrays
- Weights stored as 32-bit floats in binary blob
- Align to 32-byte boundaries for SIMD access
- Total model size: typically 200KB to 2MB

Step 3: Initialize inference engine
- Allocate intermediate buffers for each layer
- Pre-compute dilated convolution offsets
- Warm up with 1 second of silence to stabilize recurrent states

Step 4: Real-time inference
- Per-sample or per-block processing
- Input: single float (mono guitar signal)
- Output: single float (processed signal)
- Latency: 0 samples (causal architecture)

§ 24. PREAMP + POWERAMP DUAL-MODEL ARCHITECTURE
─────────────────────────────────────────────────────────────

For maximum flexibility, the THUNDERFORGE supports loading TWO NAM models:

Model 1: PREAMP MODEL
- Captures the preamp section of the amplifier
- Parameters: Input Gain affects signal level going INTO this model
- Tone stack sits BETWEEN preamp and poweramp models

Model 2: POWERAMP MODEL
- Captures the power amplifier section
- Parameters: Master Volume affects signal level going INTO this model
- Cabinet IR sits AFTER the poweramp model

Signal Flow with Dual Models:
INPUT → Gate → TS → Comp → [NAM Preamp] → Tone Stack → [NAM Poweramp] → [IR Cab] → FX → OUT

Single Model Mode:
When only one model is loaded, it captures the complete amp (pre + power):
INPUT → Gate → TS → Comp → [NAM Full Amp] → Tone Stack → [IR Cab] → FX → OUT

§ 25. FALLBACK DSP — WHEN NO NAM MODEL IS LOADED
─────────────────────────────────────────────────────────────

If no neural model is loaded, the plugin falls back to traditional DSP:

Preamp Emulation (No NAM):
1. Input gain staging (Gain knob)
2. Asymmetric waveshaping:
   Algorithm: Modified tanh with tube-like asymmetry
   if (x >= 0) y = tanh(x * 1.2)
   else        y = tanh(x * 0.8) * 0.95
3. Bias shift: Add small DC offset (0.02) before waveshaper for even harmonics
4. Multi-stage: Run 2-3 gain stages in series for cascaded distortion
5. Interstage filtering: Bandpass 80Hz-8kHz between stages

Power Amp Emulation (No NAM):
1. Softer saturation than preamp (lower gain, wider headroom)
2. Power supply sag simulation: Dynamic compression based on signal envelope
   sag_amount = 1.0 - (envelope * sag_depth * 0.1)
   Reduces gain during loud passages, compresses dynamics
3. Negative feedback loop simulation: Reduces distortion at low volumes
4. Output transformer coloring: Subtle lowpass at 7kHz + bass rolloff at 50Hz

§ 26. SAMPLE RATE HANDLING FOR NEURAL MODELS
─────────────────────────────────────────────────────────────

NAM models are trained at a specific sample rate (typically 48kHz).
If the host runs at a different rate:

Host Rate == Model Rate: Direct processing (ideal)

Host Rate > Model Rate: Downsample → Process → Upsample
1. Apply anti-aliasing lowpass filter (Nyquist/2 of model rate)
2. Decimate to model rate
3. Run NAM model
4. Upsample back to host rate (zero-fill + interpolation filter)

Host Rate < Model Rate: Generally acceptable quality loss
1. Run model at host rate (slight tonal difference)
2. Compensate with tone stack adjustment

Implementation: Use high-quality resampling (sinc interpolation, 64 taps)

§ 27. MODEL BROWSER UI
─────────────────────────────────────────────────────────────

The model browser allows users to select from built-in and custom NAM models:

UI Design:
- Slide-out panel from right side (233px wide, full height)
- Surface: Frosted glass material
- Header: "AMP MODELS" in wide-tracked mono uppercase
- List: Scrollable list of model entries
- Each entry: Model name, type badge, sample rate, file size
- Selected entry: Amber border glow + checkmark icon
- Load button: Chrome button with amber text "LOAD MODEL"
- Custom: "Import .nam File" button at bottom

Categories:
[ALL] [CLEAN] [CRUNCH] [HIGH GAIN] [METAL] [CUSTOM]

Built-in Model Names:
- "Plexi '68 Normal"        (Clean to crunch)
- "Plexi '68 Bright"        (Bright channel, more gain)
- "JCM800 Hot-Rod"          (Classic British high-gain)
- "Recto Modern"            (American high-gain)
- "AC30 Top Boost"          (Vox chimey cleans)
- "Twin Reverb Clean"       (Fender pristine cleans)
- "5150 III Red"             (Modern metal saturation)
- "Soldano SLO Crunch"      (Smooth lead tone)

§ 28. NAM PERFORMANCE OPTIMIZATION
─────────────────────────────────────────────────────────────

Target: < 25% CPU on single core at 48kHz / 128 buffer

Optimization Strategies:
1. SIMD: Use SSE2/AVX2 (x86) or NEON (ARM) for matrix multiply
2. Memory layout: Interleave weights for cache-friendly access
3. Branch prediction: Avoid conditionals in inner processing loop
4. Buffer size: Process in chunks matching CPU cache line size (64 bytes)
5. Threading: NEVER use threads for real-time audio. Single-thread only.
6. Allocation: ZERO allocations in the audio callback. Pre-allocate all buffers.
7. Profiling: Use `std::time::Instant` (Rust) or `juce::PerformanceCounter` (JUCE)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART V — IMPLEMENTATION A: RUST / NIH-PLUG
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 29. TECHNOLOGY STACK — RUST IMPLEMENTATION
─────────────────────────────────────────────────────────────

Layer               | Technology        | Version  | Rationale
--------------------|-------------------|----------|-----------------------------------------------
Language            | Rust              | 2024 ed  | Memory safety, zero-cost abstractions
Plugin Framework    | nih-plug          | latest   | Rust-native VST3/CLAP, no C++ SDK dependency
GUI Option A        | nih-plug-webview  | latest   | WebView-based UI (HTML/CSS/JS for skeuomorphism)
GUI Option B        | nih-plug-iced     | latest   | Pure Rust GUI via iced framework
GUI Option C        | nih-plug-egui     | latest   | Immediate-mode GUI via egui
DSP Math            | std::f32          | stable   | Built-in floating point operations
FFT                 | rustfft           | latest   | Fast Fourier Transform for convolution
SIMD                | std::simd         | nightly  | SIMD intrinsics for neural model inference
NAM Integration     | cc + bindgen      | latest   | FFI bridge to NeuralAmpModelerCore (C++)
Serialization       | serde + serde_json| latest   | Preset and model metadata serialization
Audio DSP           | synfx-dsp         | latest   | DSP utilities (filters, oscillators)
Concurrency         | crossbeam         | latest   | Lock-free channels for GUI↔DSP communication

RECOMMENDED GUI: nih-plug-webview (Option A) for maximum visual fidelity.
The skeuomorphic UI described in PART II is most naturally implemented
in HTML/CSS/JavaScript, with Rust handling all DSP and parameter management.

§ 30. PROJECT STRUCTURE — RUST
─────────────────────────────────────────────────────────────

thunderforge-rs/
├── Cargo.toml                          # Workspace root
├── Cargo.lock
├── .cargo/
│   └── config.toml                     # Build configuration
├── crates/
│   ├── thunderforge-core/              # Core DSP library (no plugin dependency)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # Public API
│   │       ├── dsp/
│   │       │   ├── mod.rs
│   │       │   ├── noise_gate.rs       # Noise gate processor
│   │       │   ├── overdrive.rs        # Tube Screamer emulation
│   │       │   ├── compressor.rs       # Dynamics compressor
│   │       │   ├── tone_stack.rs       # 3-band EQ (Marshall topology)
│   │       │   ├── waveshaper.rs       # Tube saturation curves
│   │       │   ├── delay.rs            # Stereo delay with feedback
│   │       │   ├── reverb.rs           # FDN reverb
│   │       │   ├── chorus.rs           # Modulated delay chorus
│   │       │   ├── cabinet.rs          # IR convolution engine
│   │       │   ├── biquad.rs           # Biquad filter implementation
│   │       │   └── utils.rs            # db_to_linear, linear_to_db, etc.
│   │       ├── nam/
│   │       │   ├── mod.rs
│   │       │   ├── model.rs            # NAM model loader and inference
│   │       │   ├── wavenet.rs          # WaveNet architecture in Rust
│   │       │   ├── lstm.rs             # LSTM architecture (optional)
│   │       │   └── weights.rs          # Binary weight loading
│   │       └── presets/
│   │           ├── mod.rs
│   │           ├── preset.rs           # Preset data structure
│   │           └── factory.rs          # Factory preset definitions
│   │
│   ├── thunderforge-plugin/            # nih-plug plugin wrapper
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # Plugin entry point, nih_export_vst3!
│   │       ├── params.rs               # All parameter definitions
│   │       ├── editor.rs               # GUI editor (webview bridge)
│   │       └── processor.rs            # Audio processing callback
│   │
│   └── thunderforge-ui/                # WebView UI (HTML/CSS/JS)
│       ├── package.json
│       ├── src/
│       │   ├── index.html              # Main UI HTML
│       │   ├── styles/
│       │   │   ├── main.css            # All visual styles
│       │   │   ├── knobs.css           # Knob component styles
│       │   │   ├── leds.css            # LED indicator styles
│       │   │   ├── displays.css        # LCD/OLED display styles
│       │   │   └── materials.css       # Material textures
│       │   ├── components/
│       │   │   ├── Knob.js             # Interactive knob component
│       │   │   ├── Switch.js           # Toggle switch component
│       │   │   ├── LED.js              # LED indicator component
│       │   │   ├── Display.js          # LCD/OLED display component
│       │   │   ├── Meter.js            # VU meter component
│       │   │   ├── Waveform.js         # Waveform visualizer
│       │   │   └── ModelBrowser.js     # NAM model browser panel
│       │   ├── engine/
│       │   │   ├── bridge.js           # Rust↔WebView communication bridge
│       │   │   ├── params.js           # Parameter state management
│       │   │   └── animation.js        # Spring physics animation engine
│       │   └── assets/
│       │       ├── textures/           # Tolex, metal, noise textures (PNG/SVG)
│       │       ├── fonts/              # JetBrains Mono, DM Sans
│       │       └── icons/              # SVG icons for UI elements
│       └── dist/                       # Built UI output (embedded in plugin)
│
├── models/                             # Built-in NAM model files
│   ├── plexi_68_normal.nam
│   ├── plexi_68_bright.nam
│   ├── jcm800_hotrod.nam
│   ├── recto_modern.nam
│   ├── ac30_top_boost.nam
│   ├── twin_reverb_clean.nam
│   ├── 5150_iii_red.nam
│   └── soldano_slo_crunch.nam
│
├── cabinets/                           # Built-in IR files
│   ├── 4x12_greenback.wav
│   ├── 4x12_v30.wav
│   ├── 2x12_blue.wav
│   ├── 1x12_openback.wav
│   └── 4x12_t75.wav
│
├── presets/                            # Factory preset JSON files
│   ├── clean_sparkle.json
│   ├── highway_crunch.json
│   ├── british_invasion.json
│   ├── metal_thunder.json
│   └── ambient_shimmer.json
│
└── scripts/
├── build.sh                        # Build script (all platforms)
├── bundle.sh                       # Bundle VST3/CLAP for distribution
└── install.sh                      # Install to system plugin directory

§ 31. CARGO.TOML — WORKSPACE ROOT
─────────────────────────────────────────────────────────────

[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "1.0.0"
authors = ["Lukas Hansen <lukas@stradego.capital>"]
edition = "2024"
license = "GPL-3.0-or-later"

[workspace.dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git", features = ["assert_process_allocs"] }
nih_plug_webview = { git = "https://github.com/robbert-vdh/nih-plug.git" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rustfft = "6"
crossbeam-channel = "0.5"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = "symbols"

[profile.release.build-override]
opt-level = 3

§ 32. CORE PLUGIN STRUCTURE — lib.rs
─────────────────────────────────────────────────────────────

The main plugin entry point defines the nih-plug plugin:

// crates/thunderforge-plugin/src/lib.rs

use nih_plug::prelude::*;
use std::sync::Arc;

mod params;
mod processor;
mod editor;

use params::ThunderforgeParams;
use processor::ThunderforgeProcessor;

pub struct Thunderforge {
params: Arc<ThunderforgeParams>,
processor: ThunderforgeProcessor,
}

impl Default for Thunderforge {
fn default() -> Self {
let params = Arc::new(ThunderforgeParams::default());
Self {
params: params.clone(),
processor: ThunderforgeProcessor::new(params),
}
}
}

impl Plugin for Thunderforge {
const NAME: &'static str = "LH Thunderforge";
const VENDOR: &'static str = "Lukas Hansen Audio";
const URL: &'static str = "https://lukashansen.audio";
const EMAIL: &'static str = "lukas@stradego.capital";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
// Mono in → Stereo out (standard guitar plugin)
AudioIOLayout {
main_input_channels: NonZeroU32::new(1),
main_output_channels: NonZeroU32::new(2),
..AudioIOLayout::const_default()
},
// Stereo in → Stereo out (for re-amping stereo sources)
AudioIOLayout {
main_input_channels: NonZeroU32::new(2),
main_output_channels: NonZeroU32::new(2),
..AudioIOLayout::const_default()
},
];
const MIDI_INPUT: MidiConfig = MidiConfig::None;
const SAMPLE_ACCURATE_AUTOMATION: bool = true;
type SysExMessage = ();
type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.processor.prepare(buffer_config.sample_rate, buffer_config.max_buffer_size as usize);
        true
    }

    fn reset(&mut self) {
        self.processor.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.processor.process(buffer);
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Thunderforge {
const CLAP_ID: &'static str = "capital.stradego.thunderforge";
const CLAP_DESCRIPTION: Option<&'static str> = Some("Neural Amp Modeler Guitar Plugin");
const CLAP_MANUAL_URL: Option<&'static str> = None;
const CLAP_SUPPORT_URL: Option<&'static str> = None;
const CLAP_FEATURES: &'static [ClapFeature] = &[
ClapFeature::AudioEffect,
ClapFeature::Distortion,
ClapFeature::Mono,
ClapFeature::Stereo,
];
}

impl Vst3Plugin for Thunderforge {
const VST3_CLASS_ID: [u8; 16] = *b"LHThunderforge01";
const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
Vst3SubCategory::Fx,
Vst3SubCategory::Distortion,
Vst3SubCategory::Guitar,
];
}

nih_export_clap!(Thunderforge);
nih_export_vst3!(Thunderforge);

§ 33-42. [ADDITIONAL RUST SECTIONS — PARAMS, DSP MODULES, WEBVIEW BRIDGE]
─────────────────────────────────────────────────────────────

[These sections detail the complete Rust implementation of every module
listed in the project structure. Due to the massive scope, the AI building
this project should generate each file following the architecture above
and the DSP algorithms in PART III. Key implementation notes:]

PARAMS (params.rs):
- Every parameter from §14-§20 defined as nih_plug FloatParam, BoolParam, or EnumParam
- Grouped into nested structs: GateParams, OverdriveParams, CompParams, etc.
- All parameters derive Params
- Smoothing: Use SmoothingStyle::Logarithmic(50.0) for gain params
- Use SmoothingStyle::Linear(5.0) for time params

PROCESSOR (processor.rs):
- Owns instances of every DSP module from thunderforge-core
- process() method chains modules in signal chain order
- Sample-accurate parameter automation via per-sample smoothing
- Mono-to-stereo split happens after cabinet sim (for stereo FX)

WEBVIEW BRIDGE (editor.rs):
- Uses nih_plug_webview to create an editor
- Embeds the HTML/CSS/JS from thunderforge-ui/dist/
- Parameter changes from UI → Rust: JS calls bridge function
- Parameter changes from Rust → UI: Rust sends JSON update to JS
- 60fps update loop for meter data and waveform visualization

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART VI — IMPLEMENTATION B: C++ / JUCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 43. TECHNOLOGY STACK — JUCE IMPLEMENTATION
─────────────────────────────────────────────────────────────

Layer               | Technology            | Version  | Rationale
--------------------|-----------------------|----------|------------------------------------------
Language            | C++20                 | latest   | Industry standard for audio plugins
Framework           | JUCE                  | 8.x      | Mature, cross-platform plugin framework
Plugin Formats      | VST3, AU, AAX         | all      | Maximum DAW compatibility
GUI                 | JUCE Graphics + OpenGL| built-in | Hardware-accelerated custom painting
NAM Engine          | NeuralAmpModelerCore  | latest   | Direct C++ integration (no FFI needed)
DSP Utilities       | juce::dsp             | built-in | Filters, convolution, FFT, gain, etc.
Build System        | CMake                 | 3.22+    | Cross-platform build configuration
IR Loading          | juce::dsp::Convolution| built-in | Partitioned convolution for cabinet IRs
Threading           | juce::Thread          | built-in | Background model loading
State               | juce::AudioProcessorValueTreeState | built-in | Parameter management

§ 44. PROJECT STRUCTURE — JUCE / CMAKE
─────────────────────────────────────────────────────────────

thunderforge-juce/
├── CMakeLists.txt                        # Root CMake configuration
├── JUCE/                                 # JUCE framework (submodule)
├── modules/
│   └── NeuralAmpModelerCore/             # NAM core library (submodule)
├── Source/
│   ├── PluginProcessor.h                 # AudioProcessor declaration
│   ├── PluginProcessor.cpp               # Audio processing implementation
│   ├── PluginEditor.h                    # AudioProcessorEditor declaration
│   ├── PluginEditor.cpp                  # GUI main editor
│   ├── Parameters.h                      # Parameter ID constants and layout
│   ├── Parameters.cpp                    # APVTS parameter creation
│   ├── DSP/
│   │   ├── SignalChain.h                 # Complete signal chain processor
│   │   ├── SignalChain.cpp
│   │   ├── NoiseGate.h / .cpp           # Noise gate
│   │   ├── TubeScreamer.h / .cpp        # Overdrive emulation
│   │   ├── Compressor.h / .cpp          # Dynamics compressor
│   │   ├── ToneStack.h / .cpp           # 3-band EQ
│   │   ├── WaveShaper.h / .cpp          # Tube saturation curves
│   │   ├── CabinetSim.h / .cpp          # IR convolution wrapper
│   │   ├── StereoDelay.h / .cpp         # Delay effect
│   │   ├── PlateReverb.h / .cpp         # Reverb effect
│   │   ├── Chorus.h / .cpp              # Chorus effect
│   │   └── NAMProcessor.h / .cpp        # NAM model wrapper
│   ├── GUI/
│   │   ├── LookAndFeel/
│   │   │   ├── ThunderforgeLookAndFeel.h
│   │   │   ├── ThunderforgeLookAndFeel.cpp  # Custom JUCE LookAndFeel
│   │   │   ├── KnobPainter.h / .cpp    # Skeuomorphic knob rendering
│   │   │   ├── SwitchPainter.h / .cpp   # Toggle switch rendering
│   │   │   ├── LEDPainter.h / .cpp      # LED indicator rendering
│   │   │   └── Materials.h / .cpp       # Metal, glass, tolex textures
│   │   ├── Components/
│   │   │   ├── AmpFaceComponent.h / .cpp     # Main amp face panel
│   │   │   ├── SignalChainComponent.h / .cpp  # Top signal chain view
│   │   │   ├── KnobComponent.h / .cpp        # Interactive knob
│   │   │   ├── LCDDisplayComponent.h / .cpp   # LCD readout
│   │   │   ├── VUMeterComponent.h / .cpp      # VU meter
│   │   │   ├── WaveformComponent.h / .cpp     # Waveform visualizer
│   │   │   ├── ModelBrowserComponent.h / .cpp # NAM model browser
│   │   │   └── StatusBarComponent.h / .cpp    # Bottom status strip
│   │   └── Textures/                    # Embedded texture resources
│   │       ├── tolex_tile.png
│   │       ├── brushed_metal.png
│   │       ├── noise_grain.png
│   │       └── amp_logo.svg
│   └── Presets/
│       ├── PresetManager.h / .cpp       # Preset save/load/browse
│       └── FactoryPresets.h             # Embedded factory presets
│
├── Resources/
│   ├── Models/                          # Built-in .nam files
│   ├── Cabinets/                        # Built-in IR .wav files
│   └── Fonts/                           # Embedded fonts
│
└── Installers/
├── Windows/                         # InnoSetup installer script
├── macOS/                           # .pkg installer script
└── Linux/                           # .deb packaging script

§ 45. CMAKE CONFIGURATION
─────────────────────────────────────────────────────────────

cmake_minimum_required(VERSION 3.22)
project(Thunderforge VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(JUCE)
add_subdirectory(modules/NeuralAmpModelerCore)

juce_add_plugin(Thunderforge
COMPANY_NAME "Lukas Hansen Audio"
IS_SYNTH FALSE
NEEDS_MIDI_INPUT FALSE
NEEDS_MIDI_OUTPUT FALSE
IS_MIDI_EFFECT FALSE
EDITOR_WANTS_KEYBOARD_FOCUS FALSE
COPY_PLUGIN_AFTER_BUILD TRUE
PLUGIN_MANUFACTURER_CODE LkHn
PLUGIN_CODE Thfg
FORMATS VST3 AU Standalone  # AAX requires iLok account
PRODUCT_NAME "LH Thunderforge"
VST3_CATEGORIES "Fx Distortion Guitar"
)

target_sources(Thunderforge PRIVATE
Source/PluginProcessor.cpp
Source/PluginEditor.cpp
Source/Parameters.cpp
Source/DSP/SignalChain.cpp
Source/DSP/NoiseGate.cpp
Source/DSP/TubeScreamer.cpp
Source/DSP/Compressor.cpp
Source/DSP/ToneStack.cpp
Source/DSP/WaveShaper.cpp
Source/DSP/CabinetSim.cpp
Source/DSP/StereoDelay.cpp
Source/DSP/PlateReverb.cpp
Source/DSP/Chorus.cpp
Source/DSP/NAMProcessor.cpp
Source/GUI/LookAndFeel/ThunderforgeLookAndFeel.cpp
Source/GUI/LookAndFeel/KnobPainter.cpp
Source/GUI/LookAndFeel/SwitchPainter.cpp
Source/GUI/LookAndFeel/LEDPainter.cpp
Source/GUI/LookAndFeel/Materials.cpp
Source/GUI/Components/AmpFaceComponent.cpp
Source/GUI/Components/SignalChainComponent.cpp
Source/GUI/Components/KnobComponent.cpp
Source/GUI/Components/LCDDisplayComponent.cpp
Source/GUI/Components/VUMeterComponent.cpp
Source/GUI/Components/WaveformComponent.cpp
Source/GUI/Components/ModelBrowserComponent.cpp
Source/GUI/Components/StatusBarComponent.cpp
Source/Presets/PresetManager.cpp
)

target_link_libraries(Thunderforge PRIVATE
juce::juce_audio_utils
juce::juce_dsp
juce::juce_opengl
NeuralAmpModelerCore
)

juce_generate_juce_header(Thunderforge)

§ 46-56. [ADDITIONAL JUCE SECTIONS — PROCESSOR, EDITOR, LOOKANDFEEL]
─────────────────────────────────────────────────────────────

[These sections detail the complete JUCE implementation. Key notes:]

PluginProcessor:
- Inherits juce::AudioProcessor
- Uses AudioProcessorValueTreeState for all parameters
- processBlock() chains DSP modules in signal chain order
- prepareToPlay() initializes all DSP with sample rate and block size
- State save/load via getStateInformation() / setStateInformation()

PluginEditor:
- Inherits juce::AudioProcessorEditor
- Sets size to 1200 x 800, resizable with constraints
- Owns AmpFaceComponent, SignalChainComponent, StatusBarComponent
- Uses OpenGLContext for hardware-accelerated rendering
- Timer callback at 60fps for meter/waveform updates

ThunderforgeLookAndFeel:
- Custom drawRotarySlider() for all knob types
- Custom drawToggleButton() for switches
- Custom drawLinearSlider() for faders
- All drawing uses the materials and colors from §4 and §6-§9
- Every drawXxx method implements the multi-layer rendering described
  in the knob/LED/LCD specifications

KnobComponent:
- Inherits juce::Slider (Rotary style)
- Custom paint() that renders:
    1. Shadow ring (inset shadow)
    2. Metal skirt (radial gradient)
    3. Knob body (conical gradient with specular highlight)
    4. Indicator line (rotated to match value)
    5. Value arc (270-degree arc, filled to current value)
    6. Detent marks (11 tick marks)
- mouseDrag() with spring physics momentum
- mouseDoubleClick() resets to default with animation
- Value tooltip on hover

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART VII — PRESET LIBRARY — LEGENDARY TONE PROFILES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 57. PRESET FORMAT
─────────────────────────────────────────────────────────────

{
"name": "Highway Crunch",
"author": "LH Factory",
"category": "Classic Rock",
"description": "The sound of classic rock highway anthems. Crunchy, open chords.",
"version": "1.0",
"parameters": {
"input_gain": 3.0,
"gate_bypass": true,
"gate_threshold": -50.0,
"ts_bypass": true,
"ts_drive": 30,
"ts_tone": 55,
"ts_level": 60,
"comp_bypass": true,
"nam_model": "plexi_68_bright",
"eq_bass": 6,
"eq_mid": 7,
"eq_treble": 6,
"eq_presence": 5,
"cab_model": "4x12_greenback",
"cab_mix": 100,
"delay_bypass": true,
"delay_time": 340,
"delay_feedback": 25,
"delay_mix": 15,
"reverb_bypass": false,
"reverb_size": 40,
"reverb_decay": 1.5,
"reverb_mix": 10,
"chorus_bypass": true,
"master_volume": -6.0
}
}

§ 58. FACTORY PRESET DEFINITIONS
─────────────────────────────────────────────────────────────

PRESET 1: "Clean Sparkle"
Category:    Clean
NAM Model:   twin_reverb_clean
Cabinet:     2x12_blue
Description: Pristine Fender-style cleans with shimmer reverb
Key Settings: Gain 3, Bass 5, Mid 6, Treble 7, Presence 6
FX: Light chorus (rate 0.8, depth 30), reverb (size 60, decay 3.0, mix 30)
Use Case: Jazz, country, clean rhythm

PRESET 2: "Highway Crunch"
Category:    Classic Rock
NAM Model:   plexi_68_bright
Cabinet:     4x12_greenback
Description: The sound of Australian rock highway anthems
Key Settings: Gain 6.5, Bass 6, Mid 7, Treble 6, Presence 5
FX: Slapback delay (time 120ms, feedback 15, mix 12)
Tone Targets: Open power chords with chimey top end, tight low end
Use Case: Classic rock rhythm, blues-rock

PRESET 3: "British Invasion"
Category:    Classic Rock
NAM Model:   jcm800_hotrod
Cabinet:     4x12_t75
Description: Saturated British high-gain for aggressive rock
Key Settings: Gain 7.5, Bass 5, Mid 8, Treble 5.5, Presence 6
FX: None (dry and raw)
Tone Targets: Thick midrange bark, articulate palm mutes
Use Case: Hard rock, punk, aggressive rhythm

PRESET 4: "Metal Thunder"
Category:    Metal / High Gain
NAM Model:   5150_iii_red
Cabinet:     4x12_v30
Description: Extreme saturation for modern metal
Key Settings: Gain 8.5, Bass 4, Mid 6, Treble 6.5, Presence 7
FX: Noise gate (threshold -40dB), tight settings
Tone Targets: Djent-ready tightness, scooped but not thin
Use Case: Metal rhythm, breakdown chugs, technical riffing

PRESET 5: "Ambient Shimmer"
Category:    Atmospheric
NAM Model:   ac30_top_boost
Cabinet:     1x12_openback
Description: Lush, spacious tones for ambient playing
Key Settings: Gain 4, Bass 4, Mid 5, Treble 7, Presence 8
FX: Chorus (rate 0.5, depth 60), delay (time 500ms, feedback 50, mix 35),
reverb (size 80, decay 5.0, mix 45)
Use Case: Ambient, post-rock, shoegaze, worship

PRESET 6: "Smooth Lead"
Category:    Lead
NAM Model:   soldano_slo_crunch
Cabinet:     4x12_greenback
Description: Creamy sustain for vocal-like lead lines
Key Settings: Gain 7, Bass 5, Mid 7, Treble 5, Presence 4
FX: Delay (time 375ms dotted 8th, feedback 35, mix 25),
reverb (size 50, decay 2.0, mix 15)
Use Case: Blues-rock leads, classic rock solos

PRESET 7: "Nu Metal Chunk"
Category:    Metal / Drop Tuning
NAM Model:   recto_modern
Cabinet:     4x12_v30
Description: American high-gain for drop-tuned aggression
Key Settings: Gain 8, Bass 5, Mid 5, Treble 7, Presence 6
FX: Gate (threshold -38dB), comp (ratio 3:1, subtle)
Tone Targets: Massive low-end with cutting highs, mid scoop
Use Case: Nu-metal, modern rock, drop-C/B tuning

PRESET 8: "Vox Jangle"
Category:    Clean/Edge of Breakup
NAM Model:   ac30_top_boost
Cabinet:     2x12_blue
Description: Chimey British cleans with edge-of-breakup character
Key Settings: Gain 5, Bass 4, Mid 6, Treble 8, Presence 7
FX: Tremolo-chorus (rate 3.0, depth 20), spring reverb (size 35, decay 1.0)
Use Case: Indie rock, jangle pop, Britpop

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART VIII — UI COMPONENT SPECIFICATIONS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 65. MASTER LAYOUT — THE GOLDEN RATIO GRID
─────────────────────────────────────────────────────────────

Plugin Window: 1200 x 800 pixels

┌──────────────────────────────────────────────────────────────────────┐
│ SIGNAL CHAIN BAR (height: 144px = φ^7 scaled)                      │
│ [Gate] → [TS] → [Comp] → [AMP] → [CAB] → [Delay] → [Rev] → [Ch]  │
├────────────────────────────────────────┬─────────────────────────────┤
│                                        │  OLED DISPLAY (WAVEFORM)   │
│                                        │  233px x 233px             │
│    AMP FACE — THE MONOLITH             │  ─────────────────────     │
│    (741px x 495px)                     │  MODEL BROWSER / INFO      │
│                                        │  233px x 262px             │
│    [GAIN] [BASS] [MID] [TREBLE] [PRES] │                           │
│    [VOL]  [MODEL▼]   [CAB▼]  [MASTER]  │  [PRESET BROWSER]        │
│                                        │                            │
├──────────────────────────────────────────────────────────────────────┤
│ STATUS BAR (height: 34px)                                           │
│ CPU: 12% | 48kHz / 128 | Latency: 2.9ms | Model: Plexi '68 Bright │
└──────────────────────────────────────────────────────────────────────┘

Width Split: 741px (61.8%) | 459px (38.2%) — Golden Ratio
Height Split: 144px top | 622px middle | 34px bottom

§ 66-78. [DETAILED COMPONENT SPECS]
─────────────────────────────────────────────────────────────

[Each component from §6-§12 is fully specified here with exact pixel
dimensions, color values, animation curves, and interaction behaviors.
The AI should implement these using the language-specific rendering
system (JUCE Graphics for C++, HTML/CSS for Rust/WebView).]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART IX — BUILD, TEST & DISTRIBUTION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 79. BUILD COMMANDS
─────────────────────────────────────────────────────────────

RUST BUILD:
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Bundle VST3 plugin (creates .vst3 bundle)
cargo xtask bundle thunderforge-plugin --release

# Bundle CLAP plugin
cargo xtask bundle thunderforge-plugin --release --format clap

JUCE BUILD:
# Configure
cmake -B build -DCMAKE_BUILD_TYPE=Release

# Build
cmake --build build --config Release

# The VST3/AU/AAX will be in build/Thunderforge_artefacts/Release/

§ 80. TESTING WITHOUT A DAW
─────────────────────────────────────────────────────────────

Standalone Testing:
JUCE: Built-in Standalone format (add "Standalone" to FORMATS in CMake)
Rust: Use a minimal standalone host wrapper

VST3 Test Host:
- Steinberg VST3PluginTestHost (official, free)
- Download from: steinbergmedia.github.io
- Allows loading VST3, testing parameters, processing audio

Plugin Buddy by Modalics:
- Free VST3/CLAP host for testing
- Simple interface, good for quick validation

Audio Test Signals:
- Guitar DI recordings (clean, direct guitar signal)
- Sine sweep (20Hz-20kHz) for frequency response analysis
- Impulse signal for latency measurement
- Noise signal for noise gate testing

§ 81. DISTRIBUTION — TERMINAL UPDATE SYSTEM
─────────────────────────────────────────────────────────────

For sharing builds with external testers via GitHub Releases:

Install Script (macOS/Linux):
#!/bin/bash
# thunderforge-install.sh
REPO="lukashansen/thunderforge"
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep tag_name | cut -d'"' -f4)
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
URL="https://github.com/$REPO/releases/download/$LATEST/thunderforge-${PLATFORM}-${ARCH}.zip"

echo "Installing LH Thunderforge $LATEST..."
curl -L -o /tmp/thunderforge.zip "$URL"
unzip -o /tmp/thunderforge.zip -d /tmp/thunderforge

# Install VST3
if [ "$PLATFORM" = "darwin" ]; then
cp -r /tmp/thunderforge/*.vst3 ~/Library/Audio/Plug-Ins/VST3/
else
mkdir -p ~/.vst3
cp -r /tmp/thunderforge/*.vst3 ~/.vst3/
fi

echo "Done! Restart your DAW to load LH Thunderforge."

Install Script (Windows PowerShell):
$repo = "lukashansen/thunderforge"
$release = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
$asset = $release.assets | Where-Object { $_.name -like "*windows*" }
Invoke-WebRequest $asset.browser_download_url -OutFile "$env:TEMP\thunderforge.zip"
Expand-Archive "$env:TEMP\thunderforge.zip" -DestinationPath "$env:TEMP\thunderforge" -Force
Copy-Item "$env:TEMP\thunderforge\*.vst3" -Destination "$env:CommonProgramFiles\VST3\" -Recurse -Force
Write-Host "Done! Restart your DAW."

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART X — CROSS-PLATFORM DEPLOYMENT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 87. PLATFORM-SPECIFIC INSTALLATION PATHS
─────────────────────────────────────────────────────────────

Platform  | VST3 Path                              | CLAP Path
----------|----------------------------------------|----------------------------------
Windows   | C:\Program Files\Common Files\VST3\    | C:\Program Files\Common Files\CLAP\
macOS     | ~/Library/Audio/Plug-Ins/VST3/         | ~/Library/Audio/Plug-Ins/CLAP/
Linux     | ~/.vst3/                               | ~/.clap/

§ 88. VST3 BUNDLE STRUCTURE
─────────────────────────────────────────────────────────────

LH Thunderforge.vst3/
├── Contents/
│   ├── x86_64-win/            # Windows 64-bit binary
│   │   └── LH Thunderforge.vst3
│   ├── x86_64-linux/          # Linux 64-bit binary
│   │   └── LH Thunderforge.so
│   ├── MacOS/                 # macOS universal binary
│   │   └── LH Thunderforge
│   ├── Resources/
│   │   ├── Models/            # NAM model files
│   │   ├── Cabinets/          # IR wav files
│   │   ├── Presets/           # Factory presets
│   │   └── moduleinfo.json    # VST3 module information
│   └── Info.plist             # macOS bundle info (macOS only)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PART XI — ADVANCED TOPICS & OPTIMIZATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

§ 93. REAL-TIME AUDIO RULES — THE SACRED LAWS
─────────────────────────────────────────────────────────────

These rules are ABSOLUTE. Violating ANY of them causes audio glitches.

1. ZERO ALLOCATIONS in the audio callback. No new, no malloc, no Vec::push.
2. ZERO LOCKS in the audio callback. No mutex, no rwlock, no critical section.
3. ZERO SYSTEM CALLS in the audio callback. No file I/O, no network, no print.
4. ZERO BLOCKING in the audio callback. No sleep, no wait, no condition variable.
5. ALL buffers pre-allocated in prepare/initialize.
6. GUI↔DSP communication via lock-free ring buffers or atomic variables.
7. Model loading happens on background thread, then swapped atomically.
8. Parameter smoothing prevents zipper noise on value changes.
9. Denormal protection: flush denormals to zero (FTZ/DAZ flags).
10. NEVER divide by zero. Guard all divisions with epsilon checks.

§ 94. CPU OPTIMIZATION TARGETS
─────────────────────────────────────────────────────────────

Scenario                  | Target CPU Usage | Notes
--------------------------|------------------|---------------------------------------
Simple preset, no NAM     | < 5%             | Pure DSP, minimal effects
Full preset with NAM      | < 25%            | Single NAM model + effects
Dual NAM models + full FX | < 40%            | Pre + power amp models + all effects
Measured on: Intel i7-12700K, 48kHz, 128 buffer

§ 95. OVERSAMPLING STRATEGY
─────────────────────────────────────────────────────────────

Purpose: Reduce aliasing from nonlinear waveshaping

When to oversample:
- The overdrive/tube screamer stage: 2x oversampling
- The waveshaper (fallback preamp): 4x oversampling
- NAM models: Generally NOT oversampled (trained at target rate)

Implementation:
1. Upsample input buffer (zero-stuff + lowpass filter)
2. Process at higher rate
3. Downsample output (lowpass filter + decimate)
4. Filter: Half-band FIR (steep transition, linear phase)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
APPENDIX A — QUICK-START COMMANDS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RUST QUICK START:
git clone https://github.com/lukashansen/thunderforge-rs.git
cd thunderforge-rs
cargo xtask bundle thunderforge-plugin --release
# Plugin binary at: target/bundled/LH Thunderforge.vst3

JUCE QUICK START:
git clone --recursive https://github.com/lukashansen/thunderforge-juce.git
cd thunderforge-juce
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
# Plugin binary at: build/Thunderforge_artefacts/Release/VST3/

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
APPENDIX B — DSP ALGORITHM REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

BIQUAD FILTER COEFFICIENTS (Robert Bristow-Johnson Audio EQ Cookbook):

Low Shelf:
A  = sqrt(10^(dBgain/20))
w0 = 2*pi*f0/Fs
alpha = sin(w0)/2 * sqrt((A + 1/A)*(1/S - 1) + 2)
b0 =    A*( (A+1) - (A-1)*cos(w0) + 2*sqrt(A)*alpha )
b1 =  2*A*( (A-1) - (A+1)*cos(w0)                   )
b2 =    A*( (A+1) - (A-1)*cos(w0) - 2*sqrt(A)*alpha )
a0 =        (A+1) + (A-1)*cos(w0) + 2*sqrt(A)*alpha
a1 =   -2*( (A-1) + (A+1)*cos(w0)                   )
a2 =        (A+1) + (A-1)*cos(w0) - 2*sqrt(A)*alpha

High Shelf:
(Similar, with cos(w0) terms swapped)

Peaking EQ:
b0 =   1 + alpha*A
b1 =  -2*cos(w0)
b2 =   1 - alpha*A
a0 =   1 + alpha/A
a1 =  -2*cos(w0)
a2 =   1 - alpha/A

WAVESHAPER FUNCTIONS:

Soft Clip (tanh):     y = tanh(x * gain)
Hard Clip:            y = max(-1, min(1, x * gain))
Tube Asymmetric:      y = (x >= 0) ? tanh(x * 1.2) : tanh(x * 0.8) * 0.95
Fuzz:                 y = sign(x) * (1 - exp(-abs(x * gain)))

UTILITY FUNCTIONS:

dB to Linear:   linear = 10^(dB / 20)
Linear to dB:   dB = 20 * log10(linear)
Frequency to MIDI: midi = 69 + 12 * log2(freq / 440)
MIDI to Frequency:  freq = 440 * 2^((midi - 69) / 12)
ms to Samples:      samples = ms * sampleRate / 1000
Samples to ms:      ms = samples * 1000 / sampleRate

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
APPENDIX C — PARAMETER ID REGISTRY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Complete registry of all automatable parameters:

ID                  | Type    | Min     | Max     | Default | Unit   | Smooth
--------------------|---------|---------|---------|---------|--------|--------
input_gain          | float   | -12.0   | 12.0    | 0.0     | dB     | Log 50ms
gate_threshold      | float   | -80.0   | 0.0     | -45.0   | dB     | Lin 5ms
gate_attack         | float   | 0.1     | 50.0    | 1.0     | ms     | Lin 5ms
gate_hold           | float   | 0.0     | 500.0   | 50.0    | ms     | Lin 5ms
gate_release        | float   | 1.0     | 2000.0  | 100.0   | ms     | Lin 5ms
gate_bypass         | bool    | 0       | 1       | 0       | —      | —
ts_drive            | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
ts_tone             | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
ts_level            | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
ts_bypass           | bool    | 0       | 1       | 0       | —      | —
comp_threshold      | float   | -60.0   | 0.0     | -20.0   | dB     | Log 50ms
comp_ratio          | float   | 1.0     | 20.0    | 4.0     | :1     | Lin 10ms
comp_attack         | float   | 0.1     | 100.0   | 10.0    | ms     | Lin 5ms
comp_release        | float   | 10.0    | 1000.0  | 100.0   | ms     | Lin 5ms
comp_makeup         | float   | 0.0     | 24.0    | 0.0     | dB     | Log 50ms
comp_bypass         | bool    | 0       | 1       | 1       | —      | —
amp_gain            | float   | 0.0     | 10.0    | 5.0     | —      | Log 50ms
eq_bass             | float   | 0.0     | 10.0    | 5.0     | —      | Log 50ms
eq_mid              | float   | 0.0     | 10.0    | 5.0     | —      | Log 50ms
eq_treble           | float   | 0.0     | 10.0    | 5.0     | —      | Log 50ms
eq_presence         | float   | 0.0     | 10.0    | 5.0     | —      | Log 50ms
nam_model           | enum    | 0       | 255     | 0       | —      | —
cab_model           | enum    | 0       | 255     | 0       | —      | —
cab_mix             | float   | 0.0     | 100.0   | 100.0   | %      | Log 50ms
delay_time          | float   | 1.0     | 2000.0  | 375.0   | ms     | Lin 10ms
delay_feedback      | float   | 0.0     | 95.0    | 40.0    | %      | Log 50ms
delay_mix           | float   | 0.0     | 100.0   | 30.0    | %      | Log 50ms
delay_sync          | bool    | 0       | 1       | 0       | —      | —
delay_bypass        | bool    | 0       | 1       | 0       | —      | —
reverb_size         | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
reverb_decay        | float   | 0.1     | 10.0    | 2.0     | s      | Lin 10ms
reverb_damping      | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
reverb_predelay     | float   | 0.0     | 200.0   | 20.0    | ms     | Lin 5ms
reverb_mix          | float   | 0.0     | 100.0   | 20.0    | %      | Log 50ms
reverb_bypass       | bool    | 0       | 1       | 0       | —      | —
chorus_rate         | float   | 0.1     | 10.0    | 1.0     | Hz     | Lin 10ms
chorus_depth        | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
chorus_mix          | float   | 0.0     | 100.0   | 50.0    | %      | Log 50ms
chorus_bypass       | bool    | 0       | 1       | 1       | —      | —
master_volume       | float   | -60.0   | 12.0    | -6.0    | dB     | Log 50ms

Total: 38 automatable parameters

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
APPENDIX F — TESTING CHECKLIST
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[ ] Plugin loads in DAW without crash
[ ] Audio passes through when all effects bypassed
[ ] Noise gate opens/closes correctly
[ ] Overdrive produces distortion that increases with drive knob
[ ] Compressor reduces dynamic range measurably
[ ] Tone stack EQ changes frequency response
[ ] NAM model loads and produces different tone than fallback DSP
[ ] Cabinet IR changes the sound character
[ ] Delay produces echoes at correct time interval
[ ] Reverb produces spacious tail
[ ] All knobs respond to mouse drag
[ ] All knobs reset on double-click
[ ] Preset save/load works correctly
[ ] Parameter automation from DAW works
[ ] CPU usage stays under 25% with full preset
[ ] No audio glitches at any buffer size (32-4096)
[ ] Plugin state saves and restores correctly on DAW reload
[ ] UI renders at 60fps with no visual glitches
[ ] All text is readable and correctly positioned
[ ] Knob specular highlights match global light direction

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

END OF UNIVERSAL CONSTRUCTION PROMPT
THE SYMBIOSIS PROTOCOL — THUNDERFORGE v1.0

"Minimalist Code. Maximalist Sound."
— Lukas Hansen, Stradego Capital

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━