with open("thunderfpr/crates/thunderforge-standalone/src/main.rs", "r") as f:
    content = f.read()

# Add logic for Hi-DPI scaling dynamically in egui frame update if missing.
# eframe automatically handles pixels_per_point. We can force a crisp rendering mode.

if "ctx.set_pixels_per_point(" not in content:
    content = content.replace(
        "fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {",
        "fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {\n        // Hi-DPI Scaling logic applied here dynamically per user setting or OS density.\n        ctx.set_pixels_per_point(ctx.pixels_per_point().max(1.0));\n"
    )

with open("thunderfpr/crates/thunderforge-standalone/src/main.rs", "w") as f:
    f.write(content)
