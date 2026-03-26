// Simple editor placeholder
use nih_plug::prelude::*;

#[allow(dead_code)]
pub struct ThunderforgeEditor;

impl Editor for ThunderforgeEditor {
    fn spawn(&self, _window: ParentWindowHandle, _context: std::sync::Arc<dyn GuiContext>) -> Box<dyn std::any::Any + Send + 'static> {
        Box::new(())
    }
    fn size(&self) -> (u32, u32) { (1000, 600) }
    fn set_scale_factor(&self, _factor: f32) -> bool { false }
    fn param_value_changed(&self, _id: &str, _value: f32) {}
    fn param_modulation_changed(&self, _id: &str, _value: f32) {}
    fn param_values_changed(&self) {}
}

#[allow(dead_code)]
pub fn create_egui(
    _params: std::sync::Arc<crate::params::ThunderforgeParams>,
    _processor: std::sync::Arc<std::sync::Mutex<crate::processor::ThunderforgeProcessor>>,
) -> Option<Box<dyn Editor>> {
    Some(Box::new(ThunderforgeEditor))
}
