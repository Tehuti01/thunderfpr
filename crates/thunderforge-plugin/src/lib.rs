use nih_plug::prelude::*;
use std::sync::{Arc, Mutex};

mod params;
mod processor;
mod editor;

use params::ThunderforgeParams;
use processor::ThunderforgeProcessor;

/// Main plugin structure
pub struct Thunderforge {
    params: Arc<ThunderforgeParams>,
    processor: Arc<Mutex<ThunderforgeProcessor>>,
}

impl Default for Thunderforge {
    fn default() -> Self {
        let params = Arc::new(ThunderforgeParams::default());
        let processor = Arc::new(Mutex::new(ThunderforgeProcessor::new(params.clone())));
        Self { params, processor }
    }
}

impl Plugin for Thunderforge {
    const NAME: &'static str = "LH Thunderforge";
    const VENDOR: &'static str = "Lukas Hansen Audio";
    const URL: &'static str = "https://lukashansen.audio";
    const EMAIL: &'static str = "lukas@stradego.capital";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Mono input → Stereo output (standard for guitar plugins)
    // Also supports Stereo → Stereo for re-amping
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create_egui(self.params.clone(), self.processor.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Prepare processor with sample rate and max buffer size
        if let Ok(mut proc) = self.processor.lock() {
            proc.prepare(
                buffer_config.sample_rate,
                buffer_config.max_buffer_size as usize,
            );
        }
        true
    }

    fn reset(&mut self) {
        if let Ok(mut proc) = self.processor.lock() {
            proc.reset();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Process audio through the signal chain
        if let Ok(mut proc) = self.processor.lock() {
            proc.process(buffer, &self.params);
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Thunderforge {
    const CLAP_ID: &'static str = "capital.stradego.thunderforge";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Neural Amp Modeler Guitar Plugin with AI Tone Engine");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Distortion,
        ClapFeature::Mono,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for Thunderforge {
    const VST3_CLASS_ID: [u8; 16] = *b"LHThunderforge01";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Distortion,
    ];
}

// Export plugin for VST3 and CLAP formats
nih_export_clap!(Thunderforge);
nih_export_vst3!(Thunderforge);
