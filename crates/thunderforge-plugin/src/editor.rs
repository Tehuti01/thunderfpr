use nih_plug::prelude::*;
use nih_plug_egui::egui::{self, Color32, FontId, Margin, RichText, Stroke, Vec2};
use std::sync::Arc;

use crate::params::ThunderforgeParams;
use crate::processor::ThunderforgeProcessor;

/// Main editor struct
pub struct ThunderforgeEditor {
    params: Arc<ThunderforgeParams>,
    processor: Arc<ThunderforgeProcessor>,
    preset_names: Vec<String>,
    current_preset: usize,
}

impl ThunderforgeEditor {
    pub fn new(params: Arc<ThunderforgeParams>, processor: Arc<ThunderforgeProcessor>) -> Self {
        Self {
            params,
            processor,
            preset_names: vec![
                "Clean Sparkle",
                "Highway Crunch",
                "British Invasion",
                "Metal Thunder",
                "Ambient Shimmer",
                "Smooth Lead",
                "Nu Metal Chunk",
                "Vox Jangle",
            ],
            current_preset: 0,
        }
    }

    /// Draw a rotary knob with value display
    fn draw_knob(ui: &mut egui::Ui, param: &impl nih_plug::prelude::Param, label: &str, size: f32) {
        ui.vertical(|ui| {
            let value = param.normalized_value();
            let angle = value * 270.0 - 135.0;
            
            // Allocate space for knob
            let (rect, response) = ui.allocate_exact_size(Vec2::new(size, size), egui::Sense::drag());
            let center = rect.center();
            let radius = size / 2.0 - 2.0;
            
            // Knob background
            ui.painter().circle_filled(center, radius, Color32::from_rgb(42, 45, 53));
            ui.painter().circle_stroke(center, radius, Stroke::new(2.0, Color32::from_rgb(70, 75, 85)));
            
            // Value arc
            let start_angle = -135.0 * std::f32::consts::PI / 180.0;
            let end_angle = (value * 270.0 - 135.0) * std::f32::consts::PI / 180.0;
            let arc_points: Vec<egui::Pos2> = (0..=30)
                .map(|i| {
                    let t = i as f32 / 30.0;
                    let a = start_angle + t * (end_angle - start_angle);
                    center + (radius - 5.0) * egui::vec2(a.cos(), a.sin())
                })
                .collect();
            
            if arc_points.len() > 1 {
                ui.painter().add(egui::Shape::line(arc_points, Stroke::new(3.0, Color32::from_rgb(255, 176, 0))));
            }
            
            // Indicator line
            let indicator_end = center + (radius * 0.75) * egui::vec2(angle.to_radians().sin(), -angle.to_radians().cos());
            ui.painter().line_segment([center, indicator_end], Stroke::new(2.5, Color32::WHITE));
            
            // Center cap
            ui.painter().circle_filled(center, radius * 0.4, Color32::from_rgb(35, 38, 45));
            
            // Handle drag
            if response.dragged() {
                let delta = -response.drag_delta().y * 0.005;
                param.set_normalized_value((value + delta).clamp(0.0, 1.0));
            }
            
            // Label
            ui.label(RichText::new(label).monospace().size(10.0).color(Color32::from_rgb(120, 140, 160)));
            
            // Value tooltip
            if response.hovered() {
                let actual = param.value_to_string();
                egui::show_tooltip(ui.ctx().screen_rect(), egui::Id::new(label), |ui| {
                    ui.label(RichText::new(actual).monospace().color(Color32::from_rgb(255, 176, 0)));
                });
            }
        });
    }

    /// Draw toggle button with LED
    fn draw_toggle(ui: &mut egui::Ui, param: &impl nih_plug::prelude::Param, label: &str, color: Color32) {
        let value = param.normalized_value() > 0.5;
        
        ui.vertical(|ui| {
            // LED indicator
            let led_color = if value { color } else { Color32::from_rgb(40, 0, 0) };
            let (rect, _) = ui.allocate_exact_size(Vec2::new(10, 10), egui::Sense::hover());
            ui.painter().circle_filled(rect.center(), 5.0, led_color);
            
            if value {
                ui.painter().circle_stroke(rect.center(), 6.0, Stroke::new(1.0, color));
            }
            
            // Button
            let (_, response) = ui.allocate_exact_size(Vec2::new(55, 24), egui::Sense::click());
            let btn_rect = rect.translate(Vec2::new(0, 15));
            
            let bg = if value { color } else { Color32::from_rgb(40, 45, 55) };
            ui.painter().rect_filled(btn_rect, 4.0, bg);
            
            ui.painter().text(
                btn_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                FontId::monospace(9.0),
                if value { Color32::BLACK } else { Color32::from_rgb(120, 130, 140) },
            );
            
            if response.clicked() {
                param.set_normalized_value(if value { 0.0 } else { 1.0 });
            }
        });
    }
}

// Implement egui renderer
impl nih_plug_egui::EguiUserInterface for ThunderforgeEditor {
    fn render(&mut self, ctx: &egui::Context, _frame: &mut nih_plug_egui::EguiState) {
        // Dark theme
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        ctx.set_style(style);

        // Top panel - Signal chain
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.set_min_height(55.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("⚡ SIGNAL CHAIN").monospace().size(12.0).color(Color32::from_rgb(100, 180, 255)));
                ui.separator();
                
                self.draw_toggle(ui, &self.params.gate.bypass, "GATE", Color32::from_rgb(0, 255, 120));
                self.draw_toggle(ui, &self.params.overdrive.bypass, "TS", Color32::from_rgb(255, 136, 0));
                self.draw_toggle(ui, &self.params.compressor.bypass, "COMP", Color32::from_rgb(0, 180, 255));
                
                ui.label(RichText::new("🎸 AMP").monospace().size(11.0).color(Color32::from_rgb(200, 168, 78)));
                ui.label(RichText::new("📦 CAB").monospace().size(11.0).color(Color32::from_rgb(200, 168, 78)));
                
                self.draw_toggle(ui, &self.params.delay.bypass, "DLY", Color32::from_rgb(180, 100, 255));
                self.draw_toggle(ui, &self.params.reverb.bypass, "REV", Color32::from_rgb(0, 200, 200));
                self.draw_toggle(ui, &self.params.chorus.bypass, "CHO", Color32::from_rgb(255, 100, 180));
            });
        });

        // Bottom panel - Status
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.set_min_height(26.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔹 48kHz/128").monospace().size(10.0).color(Color32::from_rgb(0, 255, 120)));
                ui.label(RichText::new("📊 CPU: 8%").monospace().size(10.0).color(Color32::from_rgb(0, 255, 120)));
                ui.label(RichText::new("⏱️ 2.9ms").monospace().size(10.0).color(Color32::from_rgb(0, 255, 120)));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("🎸 LH Thunderforge v1.0").monospace().size(10.0).color(Color32::from_rgb(0, 255, 120)));
                });
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Amp face
                egui::Frame::none()
                    .fill(Color32::from_rgb(13, 13, 15))
                    .inner_margin(Margin::same(16))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Preset nav
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("PRESET").monospace().size(10.0).color(Color32::from_rgb(120, 140, 160)));
                                
                                if ui.button(RichText::new("◀").size(12.0)).clicked() {
                                    self.current_preset = (self.current_preset + self.preset_names.len() - 1) % self.preset_names.len();
                                }
                                
                                ui.label(RichText::new(&self.preset_names[self.current_preset])
                                    .monospace().size(12.0).color(Color32::from_rgb(255, 176, 0)));
                                
                                if ui.button(RichText::new("▶").size(12.0)).clicked() {
                                    self.current_preset = (self.current_preset + 1) % self.preset_names.len();
                                }
                            });
                            
                            ui.add_space(12.0);
                            
                            // Row 1 - Gain + EQ
                            ui.horizontal(|ui| {
                                Self::draw_knob(ui, &self.params.amp.gain, "GAIN", 60.0);
                                ui.separator();
                                Self::draw_knob(ui, &self.params.eq.bass, "BASS", 45.0);
                                Self::draw_knob(ui, &self.params.eq.mid, "MID", 45.0);
                                Self::draw_knob(ui, &self.params.eq.treble, "TREBLE", 45.0);
                                Self::draw_knob(ui, &self.params.eq.presence, "PRESENCE", 45.0);
                            });
                            
                            ui.add_space(12.0);
                            
                            // Row 2 - Input, Model, Cab, Master
                            ui.horizontal(|ui| {
                                Self::draw_knob(ui, &self.params.amp.input_gain, "INPUT", 35.0);
                                
                                ui.separator();
                                
                                // Amp model
                                ui.vertical(|ui| {
                                    ui.label(RichText::new("AMP").monospace().size(9.0).color(Color32::from_rgb(120, 140, 160)));
            egui::ComboBox::from_id_salt("amp")
                                        .selected_text(RichText::new(format!("{}", self.params.amp.model.value())).monospace().size(10.0))
                                        .width(100.0)
                                        .show_ui(ui, |ui| {
                                            let models = ["Plexi '68", "Plexi Bright", "JCM800", "Recto", "AC30", "Twin", "5150", "Soldano"];
                                            for (i, m) in models.iter().enumerate() {
                                                ui.selectable_value(&mut self.params.amp.model.value(), i as i32, *m);
                                            }
                                        });
                                });
                                
                                ui.separator();
                                
                                // Cab
                                ui.vertical(|ui| {
                                    ui.label(RichText::new("CAB").monospace().size(9.0).color(Color32::from_rgb(120, 140, 160)));
                                    egui::ComboBox::from_id_salt("cab")
                                        .selected_text(RichText::new(format!("{}", self.params.cabinet.model.value())).monospace().size(10.0))
                                        .width(100.0)
                                        .show_ui(ui, |ui| {
                                            let cabs = ["4x12 GB", "4x12 V30", "2x12 Blue", "1x12 Open", "4x12 T75", "Custom"];
                                            for (i, c) in cabs.iter().enumerate() {
                                                ui.selectable_value(&mut self.params.cabinet.model.value(), i as i32, *c);
                                            }
                                        });
                                });
                                
                                ui.separator();
                                
                                Self::draw_knob(ui, &self.params.amp.master, "MASTER", 45.0);
                            });
                        });
                    });
                
                // Side panel - VU meter
                egui::SidePanel::right("side").min_width(100.0).show_inside(ui, |ui| {
                    egui::Frame::none()
                        .fill(Color32::from_rgb(25, 30, 40))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new("OUT").monospace().size(10.0).color(Color32::from_rgb(120, 140, 160)));
                                
                                let level = self.processor.output_level.load(std::sync::atomic::Ordering::Relaxed);
                                let norm = (20.0 * level.log10().clamp(-3.0, 0.0) + 60.0) / 60.0;
                                
                                let (rect, _) = ui.allocate_exact_size(Vec2::new(30.0, 100.0), egui::Sense::hover());
                                ui.painter().rect_filled(rect, 3.0, Color32::from_rgb(20, 25, 35));
                                
                                let h = rect.height() * norm.clamp(0.0, 1.0);
                                let color = if norm > 0.95 { Color32::RED } else if norm > 0.75 { Color32::YELLOW } else { Color32::from_rgb(0, 255, 120) };
                                ui.painter().rect_filled(egui::Rect::from_min_size(rect.min + egui::vec2(3.0, rect.height() - h - 3.0), Vec2::new(24.0, h)), 2.0, color);
                            });
                        });
                });
            });
        });
    }
}

/// Create egui editor
pub fn create_egui(
    params: Arc<ThunderforgeParams>,
    processor: Arc<ThunderforgeProcessor>,
) -> Option<Box<dyn Editor>> {
    nih_plug_egui::create_egui_editor(
        (950, 500),
        "LH Thunderforge",
        move |ctx, frame| {
            let mut editor = ThunderforgeEditor::new(params.clone(), processor.clone());
            editor.render(ctx, frame);
        },
    )
}
