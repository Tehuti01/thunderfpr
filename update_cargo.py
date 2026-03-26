import os
cargo_toml = "thunderfpr/Cargo.toml"
with open(cargo_toml, "r") as f:
    lines = f.readlines()
with open(cargo_toml, "w") as f:
    for line in lines:
        if "nih_plug = " in line:
            f.write('nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git", features = ["assert_process_allocs", "simd"] }\n')
        else:
            f.write(line)
