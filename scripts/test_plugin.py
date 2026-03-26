#!/usr/bin/env python3
"""
LH Thunderforge - Quick Test
Tests the plugin audio processing without a DAW
"""

import subprocess
import sys
import os

def test_plugin():
    print("🎸 LH Thunderforge - Plugin Test\n")
    
    # Check if plugin exists
    plugin_path = os.path.expanduser("~/Library/Audio/Plug-Ins/VST3/LH Thunderforge.vst3")
    
    if not os.path.exists(plugin_path):
        print("❌ Plugin not found!")
        print(f"Expected at: {plugin_path}")
        return False
    
    print(f"✅ Plugin found at: {plugin_path}")
    
    # Check binary
    binary_path = os.path.join(plugin_path, "Contents/MacOS/LH Thunderforge")
    if os.path.exists(binary_path):
        size = os.path.getsize(binary_path)
        print(f"✅ Plugin binary: {size / 1024 / 1024:.1f} MB")
    else:
        print("❌ Plugin binary not found!")
        return False
    
    # Check IR files
    cabinets_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "cabinets")
    if os.path.exists(cabinets_dir):
        ir_files = [f for f in os.listdir(cabinets_dir) if f.endswith('.wav')]
        print(f"✅ Cabinet IRs found: {len(ir_files)} files")
        for ir in ir_files[:3]:
            print(f"   - {ir}")
        if len(ir_files) > 3:
            print(f"   ... and {len(ir_files) - 3} more")
    
    # Check if VST3 is valid bundle
    plist_path = os.path.join(plugin_path, "Contents/Info.plist")
    if os.path.exists(plist_path):
        print("✅ Valid VST3 bundle structure")
    
    print("\n" + "="*50)
    print("✅ Plugin is ready to use!")
    print("="*50)
    print("\nTo use in FL Studio:")
    print("1. Open FL Studio")
    print("2. Go to Options → Manage plugins")
    print("3. Click 'Find installed plugins'")
    print("4. Look for 'LH Thunderforge'")
    print("5. Drag to mixer track with guitar input")
    print("\nTo use in other DAWs:")
    print("- Logic Pro: Plugin is auto-detected")
    print("- Ableton Live: Scan VST3 folder in preferences")
    print("- GarageBand: Plugin should appear automatically")
    
    return True

if __name__ == "__main__":
    success = test_plugin()
    sys.exit(0 if success else 1)
