// Simple GUI Test App - No audio processing, just visual testing
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "🎸 LH Thunderforge - GUI Test",
        options,
        Box::new(|_cc| Box::new(ThunderforgeGUI::new())),
    )
}

struct ThunderforgeGUI {
    gain: f32, bass: f32, mid: f32, treble: f32, presence: f32,
    input: f32, master: f32,
    gate: bool, ts: bool, comp: bool, delay: bool, rev: bool, chorus: bool,
    amp_model: i32, cab_model: i32,
    preset: usize,
    preset_names: Vec<String>,
}

impl ThunderforgeGUI {
    fn new() -> Self {
        Self {
            gain: 5.0, bass: 5.0, mid: 5.0, treble: 5.0, presence: 5.0,
            input: 0.0, master: -6.0,
            gate: false, ts: false, comp: false, delay: false, rev: false, chorus: false,
            amp_model: 0, cab_model: 0,
            preset: 0,
            preset_names: vec![
                "Clean Sparkle".to_string(), "Highway Crunch".to_string(),
                "British Invasion".to_string(), "Metal Thunder".to_string(),
                "Ambient Shimmer".to_string(), "Smooth Lead".to_string(),
                "Nu Metal Chunk".to_string(), "Vox Jangle".to_string(),
            ],
        }
    }
}

fn draw_knob(ui: &mut egui::Ui, label: &str, value: &mut f32, min: f32, max: f32, size: f32) {
    ui.vertical(|ui| {
        let norm = (*value - min) / (max - min);
        let angle = norm * 270.0 - 135.0;
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::drag());
        let center = rect.center();
        let r = size / 2.0 - 2.0;

        ui.painter().circle_filled(center, r, egui::Color32::from_rgb(42, 45, 53));
        ui.painter().circle_stroke(center, r, egui::Stroke::new(2.0, egui::Color32::from_rgb(70, 75, 85)));

        let pts: Vec<egui::Pos2> = (0..=20).map(|i| {
            let a = (-135.0 + norm * 270.0 * (i as f32 / 20.0)) * std::f32::consts::PI / 180.0;
            center + (r - 5.0) * egui::vec2(a.cos(), a.sin())
        }).collect();
        if pts.len() > 1 {
            ui.painter().add(egui::Shape::line(pts, egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 176, 0))));
        }

        let ind = center + (r * 0.75) * egui::vec2(angle.to_radians().sin(), -angle.to_radians().cos());
        ui.painter().line_segment([center, ind], egui::Stroke::new(2.5, egui::Color32::WHITE));
        ui.painter().circle_filled(center, r * 0.4, egui::Color32::from_rgb(35, 38, 45));

        if resp.dragged() {
            *value = (*value + -resp.drag_delta().y * 0.01).clamp(min, max);
        }
        ui.label(egui::RichText::new(label).monospace().size(10.0).color(egui::Color32::from_rgb(120, 140, 160)));
        if resp.hovered() {
            egui::show_tooltip(ui.ctx(), egui::Id::new(label), |ui| {
                ui.label(egui::RichText::new(format!("{:.1}", *value)).monospace().color(egui::Color32::from_rgb(255, 176, 0)));
            });
        }
    });
}

fn draw_toggle(ui: &mut egui::Ui, label: &str, val: &mut bool, col: egui::Color32) {
    ui.vertical(|ui| {
        let led = if *val { col } else { egui::Color32::from_rgb(40, 0, 0) };
        let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 5.0, led);
        let (_, resp) = ui.allocate_exact_size(egui::vec2(55.0, 24.0), egui::Sense::click());
        let bg = if *val { col } else { egui::Color32::from_rgb(40, 45, 55) };
        ui.painter().rect_filled(rect.translate(egui::vec2(0.0, 15.0)), 4.0, bg);
        ui.painter().text(
            rect.translate(egui::vec2(0.0, 15.0)).center(), egui::Align2::CENTER_CENTER,
            label, egui::FontId::monospace(9.0),
            if *val { egui::Color32::BLACK } else { egui::Color32::from_rgb(120, 130, 140) },
        );
        if resp.clicked() { *val = !*val; }
    });
}

impl eframe::App for ThunderforgeGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.set_min_height(55.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚡ SIGNAL CHAIN").monospace().size(12.0).color(egui::Color32::from_rgb(100, 180, 255)));
                ui.separator();
                draw_toggle(ui, "GATE", &mut self.gate, egui::Color32::from_rgb(0, 255, 120));
                draw_toggle(ui, "TS", &mut self.ts, egui::Color32::from_rgb(255, 136, 0));
                draw_toggle(ui, "COMP", &mut self.comp, egui::Color32::from_rgb(0, 180, 255));
                ui.label(egui::RichText::new("🎸 AMP").monospace().size(11.0).color(egui::Color32::from_rgb(200, 168, 78)));
                ui.label(egui::RichText::new("📦 CAB").monospace().size(11.0).color(egui::Color32::from_rgb(200, 168, 78)));
                draw_toggle(ui, "DLY", &mut self.delay, egui::Color32::from_rgb(180, 100, 255));
                draw_toggle(ui, "REV", &mut self.rev, egui::Color32::from_rgb(0, 200, 200));
                draw_toggle(ui, "CHO", &mut self.chorus, egui::Color32::from_rgb(255, 100, 180));
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.set_min_height(26.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔹 48kHz / 512").monospace().size(10.0).color(egui::Color32::from_rgb(0, 255, 120)));
                ui.label(egui::RichText::new("📊 CPU: 12%").monospace().size(10.0).color(egui::Color32::from_rgb(0, 255, 120)));
                ui.label(egui::RichText::new("⏱️ 10.7ms").monospace().size(10.0).color(egui::Color32::from_rgb(0, 255, 120)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("🎸 LH Thunderforge GUI Test").monospace().size(10.0).color(egui::Color32::from_rgb(0, 255, 120)));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::Frame::none().fill(egui::Color32::from_rgb(13, 13, 15)).inner_margin(egui::Margin::same(16.0)).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("PRESET").monospace().size(10.0).color(egui::Color32::from_rgb(120, 140, 160)));
                            if ui.button(egui::RichText::new("◀").size(12.0)).clicked() {
                                self.preset = (self.preset + self.preset_names.len() - 1) % self.preset_names.len();
                            }
                            ui.label(egui::RichText::new(&self.preset_names[self.preset]).monospace().size(12.0).color(egui::Color32::from_rgb(255, 176, 0)));
                            if ui.button(egui::RichText::new("▶").size(12.0)).clicked() {
                                self.preset = (self.preset + 1) % self.preset_names.len();
                            }
                        });
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            draw_knob(ui, "GAIN", &mut self.gain, 0.0, 10.0, 60.0); ui.separator();
                            draw_knob(ui, "BASS", &mut self.bass, 0.0, 10.0, 45.0);
                            draw_knob(ui, "MID", &mut self.mid, 0.0, 10.0, 45.0);
                            draw_knob(ui, "TREBLE", &mut self.treble, 0.0, 10.0, 45.0);
                            draw_knob(ui, "PRESENCE", &mut self.presence, 0.0, 10.0, 45.0);
                        });
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            draw_knob(ui, "INPUT", &mut self.input, -12.0, 12.0, 40.0); ui.separator();
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("AMP").monospace().size(9.0).color(egui::Color32::from_rgb(120, 140, 160)));
                                egui::ComboBox::from_id_source("amp")
                                    .selected_text(format!("{}", self.amp_model))
                                    .width(100.0)
                                    .show_ui(ui, |ui: &mut egui::Ui| {
                                        let models = ["Plexi", "JCM800", "Recto", "AC30", "Twin", "5150", "Soldano"];
                                        for (i, m) in models.iter().enumerate() {
                                            ui.selectable_value(&mut self.amp_model, i as i32, *m);
                                        }
                                    });
                            }); ui.separator();
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("CAB").monospace().size(9.0).color(egui::Color32::from_rgb(120, 140, 160)));
                                egui::ComboBox::from_id_source("cab")
                                    .selected_text(format!("{}", self.cab_model))
                                    .width(100.0)
                                    .show_ui(ui, |ui: &mut egui::Ui| {
                                        let cabs = ["4x12 GB", "4x12 V30", "2x12 Blue", "1x12", "4x12 T75"];
                                        for (i, c) in cabs.iter().enumerate() {
                                            ui.selectable_value(&mut self.cab_model, i as i32, *c);
                                        }
                                    });
                            }); ui.separator();
                            draw_knob(ui, "MASTER", &mut self.master, -60.0, 12.0, 45.0);
                        });
                    });
                });
                egui::SidePanel::right("side").min_width(100.0).show_inside(ui, |ui| {
                    egui::Frame::none().fill(egui::Color32::from_rgb(25, 30, 40)).inner_margin(egui::Margin::same(12.0)).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("OUT").monospace().size(10.0).color(egui::Color32::from_rgb(120, 140, 160)));
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(30.0, 100.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 3.0, egui::Color32::from_rgb(20, 25, 35));
                            let h = rect.height() * 0.5;
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(rect.min + egui::vec2(3.0, rect.height() - h - 3.0), egui::vec2(24.0, h)),
                                2.0, egui::Color32::from_rgb(0, 255, 120),
                            );
                        });
                    });
                });
            });
        });
        ctx.request_repaint();
    }
}
