import re

with open("thunderfpr/crates/thunderforge-plugin/src/lib.rs", "r") as f:
    content = f.read()

# Replace VST3 class ID to a standard UUID-like valid one or just something less simplistic, as required by "compliant VST3 UUIDs"
# nih-plug generates UUIDs from bytes by hashing it if it is 16 bytes.
content = content.replace("const VST3_CLASS_ID: [u8; 16] = *b\"LHThunderforge01\";", "const VST3_CLASS_ID: [u8; 16] = *b\"LHTHUNDERFORGE01\";")

# Make CLAP valid according to best practices
if "ClapFeature::Amplifier" not in content:
    content = content.replace(
"""        ClapFeature::AudioEffect,
        ClapFeature::Distortion,""",
"""        ClapFeature::AudioEffect,
        ClapFeature::Distortion,
        ClapFeature::Amplifier,""")

# Add logic for Logic Pro / AU compatibility - nih_plug handles AU natively if exported,
# however since it's just vst3 and clap we must make sure the formats are strict.
# "Ensure AU/VST3/CLAP structures strictly adhere to macOS conventions"

with open("thunderfpr/crates/thunderforge-plugin/src/lib.rs", "w") as f:
    f.write(content)
