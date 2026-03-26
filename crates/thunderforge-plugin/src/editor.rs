use nih_plug::prelude::*;

/// Simple editor - provides basic window for FL Studio
pub struct ThunderforgeEditor {
    _params: std::sync::Arc<crate::params::ThunderforgeParams>,
}

impl ThunderforgeEditor {
    pub fn new(params: std::sync::Arc<crate::params::ThunderforgeParams>) -> Self {
        Self {
            _params: params,
        }
    }
}

impl Editor for ThunderforgeEditor {
    fn spawn(
        &self,
        _window: ParentWindowHandle,
        _context: std::sync::Arc<dyn nih_plug::prelude::GuiContext>,
    ) -> Box<dyn std::any::Any + Send + 'static> {
        Box::new(())
    }

    fn size(&self) -> (u32, u32) {
        (800, 400)
    }

    fn set_scale_factor(&self, _factor: f32) -> bool {
        false
    }

    fn param_value_changed(&self, _id: &str, _value: f32) {
    }

    fn param_modulation_changed(&self, _id: &str, _value: f32) {
    }

    fn param_values_changed(&self) {
    }
}

/// Create editor
pub fn create(
    params: std::sync::Arc<crate::params::ThunderforgeParams>,
) -> Option<Box<dyn Editor>> {
    Some(Box::new(ThunderforgeEditor::new(params)))
}
