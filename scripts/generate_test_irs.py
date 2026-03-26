#!/usr/bin/env python3
"""
Generate simple test IR files for cabinet simulation testing.
Creates impulse responses with different frequency characteristics.
"""

import wave
import struct
import math
import os

def generate_ir(filename, duration_ms=100, sample_rate=48000, ir_type='lowpass'):
    """Generate a test IR file with specified characteristics."""
    
    num_samples = int(sample_rate * duration_ms / 1000)
    samples = []
    
    for i in range(num_samples):
        t = i / sample_rate
        
        if ir_type == 'lowpass':
            # Simulate guitar cab frequency response (lowpass around 5kHz)
            decay = math.exp(-t * 30)  # 30Hz decay
            value = math.sin(2 * math.pi * 1000 * t) * decay
            # Add some coloration
            value += 0.3 * math.sin(2 * math.pi * 2000 * t) * decay
            value += 0.1 * math.sin(2 * math.pi * 5000 * t) * decay
            
        elif ir_type == 'bright':
            # Brighter response (more highs)
            decay = math.exp(-t * 25)
            value = math.sin(2 * math.pi * 1000 * t) * decay
            value += 0.5 * math.sin(2 * math.pi * 3000 * t) * decay
            value += 0.3 * math.sin(2 * math.pi * 6000 * t) * decay
            
        elif ir_type == 'warm':
            # Warm vintage response (rolled off highs)
            decay = math.exp(-t * 35)
            value = math.sin(2 * math.pi * 500 * t) * decay
            value += 0.4 * math.sin(2 * math.pi * 1500 * t) * decay
            value += 0.1 * math.sin(2 * math.pi * 3000 * t) * decay
            
        elif ir_type == 'modern':
            # Modern high-gain cab (tight lows, present mids)
            decay = math.exp(-t * 40)
            value = math.sin(2 * math.pi * 800 * t) * decay
            value += 0.6 * math.sin(2 * math.pi * 2500 * t) * decay
            value += 0.2 * math.sin(2 * math.pi * 4000 * t) * decay
        else:
            # Simple impulse
            value = 1.0 if i == 0 else 0.0
        
        # Normalize to prevent clipping
        value = value * 0.5
        # Clamp to valid range
        value = max(-1.0, min(1.0, value))
        samples.append(value)
    
    # Write WAV file
    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1)  # Mono
        wav_file.setsampwidth(2)  # 16-bit
        wav_file.setframerate(sample_rate)
        
        for sample in samples:
            # Convert to 16-bit integer
            value = int(sample * 32767)
            wav_file.writeframes(struct.pack('<h', value))
    
    print(f"Generated: {filename} ({num_samples} samples, {duration_ms}ms)")

# Create cabinets directory
os.makedirs('cabinets', exist_ok=True)

# Generate test IRs
generate_ir('cabinets/4x12_greenback.wav', duration_ms=100, ir_type='warm')
generate_ir('cabinets/4x12_v30.wav', duration_ms=100, ir_type='bright')
generate_ir('cabinets/2x12_blue.wav', duration_ms=80, ir_type='bright')
generate_ir('cabinets/1x12_openback.wav', duration_ms=90, ir_type='warm')
generate_ir('cabinets/4x12_t75.wav', duration_ms=100, ir_type='modern')

print("\nTest IR files generated successfully!")
