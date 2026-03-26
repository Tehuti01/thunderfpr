#!/usr/bin/env python3
"""
LH Thunderforge - Audio Processing Demo
Generates a test guitar-like signal and processes it through the DSP
"""

import wave
import struct
import math
import os

def generate_guitar_like_signal(duration_sec=2.0, sample_rate=48000):
    """Generate a test signal similar to a guitar note"""
    num_samples = int(sample_rate * duration_sec)
    samples = []
    
    # Fundamental frequency (E2 = 82.41 Hz - low E string)
    f0 = 82.41
    
    for i in range(num_samples):
        t = i / sample_rate
        
        # Fundamental + harmonics (guitar-like spectrum)
        signal = 0.0
        signal += 1.0 * math.sin(2 * math.pi * f0 * t)      # Fundamental
        signal += 0.5 * math.sin(2 * math.pi * 2 * f0 * t)  # 2nd harmonic
        signal += 0.3 * math.sin(2 * math.pi * 3 * f0 * t)  # 3rd harmonic
        signal += 0.2 * math.sin(2 * math.pi * 4 * f0 * t)  # 4th harmonic
        signal += 0.1 * math.sin(2 * math.pi * 5 * f0 * t)  # 5th harmonic
        
        # Envelope (pluck decay)
        envelope = math.exp(-t * 2.0)
        
        # Apply envelope
        sample = signal * envelope
        
        samples.append(sample * 0.5)
    
    return samples

def apply_distortion(samples, drive=0.7):
    """Simple distortion effect (like the amp gain stage)"""
    output = []
    gain = 1.0 + drive * 4.0
    
    for sample in samples:
        # Apply gain
        driven = sample * gain
        
        # Soft clipping (tube-like)
        if driven > 1.0:
            driven = 1.0
        elif driven < -1.0:
            driven = -1.0
        else:
            driven = math.tanh(driven * 2.0) / math.tanh(2.0)
        
        output.append(driven * 0.5)  # Output level
    
    return output

def apply_eq(samples, bass=0.5, mid=0.5, treble=0.5):
    """Simple EQ simulation"""
    output = []
    
    # Simple tone control approximation
    for i, sample in enumerate(samples):
        # Bass boost/cut
        if i > 10:
            bass_component = (sample - samples[i-10]) * bass
        else:
            bass_component = 0
        
        # Treble boost/cut  
        treble_component = sample * (treble - 0.5) * 0.5
        
        out = sample + bass_component * 0.3 + treble_component
        output.append(out)
    
    return output

def save_wav(filename, samples, sample_rate=48000):
    """Save samples to WAV file"""
    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        
        for sample in samples:
            # Clamp and convert to 16-bit
            sample = max(-1.0, min(1.0, sample))
            value = int(sample * 32767)
            wav_file.writeframes(struct.pack('<h', value))

def main():
    print("🎸 LH Thunderforge - Audio Processing Demo\n")
    
    # Create demo directory
    demo_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "demo_output")
    os.makedirs(demo_dir, exist_ok=True)
    
    # Generate test signal
    print("Generating test guitar signal...")
    clean_signal = generate_guitar_like_signal(2.0, 48000)
    
    # Save clean signal
    clean_file = os.path.join(demo_dir, "01_clean_guitar.wav")
    save_wav(clean_file, clean_signal)
    print(f"✅ Saved clean guitar: {clean_file}")
    
    # Apply distortion (amp gain)
    print("\nApplying amp distortion...")
    distorted = apply_distortion(clean_signal, drive=0.7)
    distorted_file = os.path.join(demo_dir, "02_with_distortion.wav")
    save_wav(distorted_file, distorted)
    print(f"✅ Saved with distortion: {distorted_file}")
    
    # Apply EQ
    print("\nApplying EQ (Bass: 6, Mid: 7, Treble: 6)...")
    eq_signal = apply_eq(distorted, bass=0.6, mid=0.7, treble=0.6)
    eq_file = os.path.join(demo_dir, "03_with_eq.wav")
    save_wav(eq_file, eq_signal)
    print(f"✅ Saved with EQ: {eq_file}")
    
    # High gain setting
    print("\nApplying high gain (metal tone)...")
    high_gain = apply_distortion(clean_signal, drive=0.9)
    high_gain_file = os.path.join(demo_dir, "04_high_gain_metal.wav")
    save_wav(high_gain_file, high_gain)
    print(f"✅ Saved high gain: {high_gain_file}")
    
    print("\n" + "="*60)
    print("✅ Audio demo files generated successfully!")
    print("="*60)
    print(f"\nDemo files saved to: {demo_dir}")
    print("\nFiles created:")
    print("  1. 01_clean_guitar.wav - Clean guitar signal")
    print("  2. 02_with_distortion.wav - With amp distortion")
    print("  3. 03_with_eq.wav - With EQ applied")
    print("  4. 04_high_gain_metal.wav - High gain metal tone")
    print("\n🎵 Open these files to hear the processing!")
    print("\nTo test the actual plugin:")
    print("  1. Open FL Studio / Logic Pro / GarageBand")
    print("  2. Load 'LH Thunderforge' plugin")
    print("  3. Play guitar through it!")

if __name__ == "__main__":
    main()
