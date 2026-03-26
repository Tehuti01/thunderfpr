use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use std::sync::Arc;
use thunderforge_core::dsp::*;
use thunderforge_core::dsp::waveshaper::PowerAmp;
use std::path::PathBuf;

use crate::params::ThunderforgeParams;

/// Get the IR file path for a cabinet model index
fn get_ir_path(model_index: i32) -> Option<PathBuf> {
    let ir_names = [
        "4x12_greenback.wav",
        "4x12_v30.wav",
        "2x12_blue.wav",
        "1x12_openback.wav",
        "4x12_t75.wav",
    ];
    
    if model_index < 0 || model_index >= ir_names.len() as i32 {
        return None;
    }
    
    // Look for IRs in common locations
    let possible_paths = [
        // Relative to plugin location (would need executable path)
        PathBuf::from(format!("./cabinets/{}", ir_names[model_index as usize])),
        // User's plugin data folder
        PathBuf::from(format!(
            "{}/Tehuti-vst-rust/cabinets/{}",
            std::env::var("HOME").unwrap_or_default(),
            ir_names[model_index as usize]
        )),
        // Absolute path for development
        PathBuf::from(format!(
            "/Users/tehuti01/Desktop/Tehuti-vst-rust/cabinets/{}",
            ir_names[model_index as usize]
        )),
    ];
    
    for path in &possible_paths {
        if path.exists() {
            return Some(path.clone());
        }
    }
    
    None
}

/// Maximum delay time for effects (ms)
const MAX_DELAY_MS: f32 = 2000.0;
/// Maximum IR length (samples)
const MAX_IR_LEN: usize = 4096;

/// Audio processor - chains all DSP modules in signal flow order
pub struct ThunderforgeProcessor {
    // Input stage
    input_gain: f32,
    // Noise gate
    noise_gate: NoiseGate,
    // Overdrive/Tube Screamer
    overdrive: TubeScreamer,
    // Compressor
    compressor: Compressor,
    // Amp section (fallback waveshaper or NAM)
    preamp_waveshaper: WaveShaper,
    power_amp: PowerAmp,
    // Tone stack (EQ)
    tone_stack: ToneStack,
    // Cabinet simulation
    cabinet: CabinetSim,
    // Effects
    delay: StereoDelay,
    reverb: FdnReverb,
    chorus: Chorus,
    // Master
    master_gain: f32,
    // State
    sample_rate: f32,
    prepared: bool,
    // Parameters (arc for editor access)
    #[allow(dead_code)]
    params: Arc<ThunderforgeParams>,
    // Metering (atomic for thread-safe GUI access)
    pub input_level: AtomicF32,
    pub output_level: AtomicF32,
    // Cabinet model tracking
    last_cabinet_model: i32,
}

impl ThunderforgeProcessor {
    pub fn new(params: Arc<ThunderforgeParams>) -> Self {
        Self {
            input_gain: 1.0,
            noise_gate: NoiseGate::new(48000.0),
            overdrive: TubeScreamer::new(48000.0),
            compressor: Compressor::new(48000.0),
            preamp_waveshaper: WaveShaper::new(48000.0),
            power_amp: PowerAmp::new(48000.0),
            tone_stack: ToneStack::new(48000.0),
            cabinet: CabinetSim::new(48000.0, MAX_IR_LEN),
            delay: StereoDelay::new(48000.0, MAX_DELAY_MS),
            reverb: FdnReverb::new(48000.0),
            chorus: Chorus::new(48000.0, MAX_DELAY_MS),
            master_gain: 1.0,
            sample_rate: 48000.0,
            prepared: false,
            params,
            input_level: AtomicF32::new(0.0),
            output_level: AtomicF32::new(0.0),
            last_cabinet_model: -1, // Force load on first process
        }
    }

    /// Prepare processor for playback
    pub fn prepare(&mut self, sample_rate: f32, _max_buffer_size: usize) {
        self.sample_rate = sample_rate;

        // Initialize all DSP modules
        self.noise_gate.set_sample_rate(sample_rate);
        self.overdrive.set_sample_rate(sample_rate);
        self.compressor.set_sample_rate(sample_rate);
        self.preamp_waveshaper.set_sample_rate(sample_rate);
        self.power_amp.set_sample_rate(sample_rate);
        self.tone_stack.set_sample_rate(sample_rate);
        self.cabinet.set_sample_rate(sample_rate);
        self.delay.set_sample_rate(sample_rate);
        self.reverb.set_sample_rate(sample_rate);
        self.chorus.set_sample_rate(sample_rate);

        self.prepared = true;
    }

    /// Reset all DSP states
    pub fn reset(&mut self) {
        self.noise_gate.reset();
        self.overdrive.reset();
        self.compressor.reset();
        self.preamp_waveshaper.reset();
        self.power_amp.reset();
        self.tone_stack.reset();
        self.cabinet.reset();
        self.delay.reset();
        self.reverb.reset();
        self.chorus.reset();
    }

    /// Update all parameters from the params struct
    fn update_parameters(&mut self, params: &ThunderforgeParams) {
        // Input gain
        self.input_gain = utils::db_to_linear(params.amp.input_gain.value());

        // Noise gate
        self.noise_gate.set_params(
            params.gate.threshold.value(),
            params.gate.attack.value(),
            params.gate.hold.value(),
            params.gate.release.value(),
        );

        // Overdrive
        self.overdrive.set_drive(params.overdrive.drive.value());
        self.overdrive.set_tone(params.overdrive.tone.value());
        self.overdrive.set_level(params.overdrive.level.value());

        // Compressor
        self.compressor.set_params(
            params.compressor.threshold.value(),
            params.compressor.ratio.value(),
            params.compressor.attack.value(),
            params.compressor.release.value(),
            params.compressor.makeup.value(),
        );

        // Amp gain (fallback)
        self.preamp_waveshaper.set_gain(params.amp.gain.value());

        // Tone stack
        self.tone_stack.set_params(
            params.eq.bass.value(),
            params.eq.mid.value(),
            params.eq.treble.value(),
            params.eq.presence.value(),
        );

        // Cabinet - load IR if model changed
        let cabinet_model = params.cabinet.model.value();
        if cabinet_model != self.last_cabinet_model {
            if let Some(ir_path) = get_ir_path(cabinet_model) {
                if let Err(e) = self.cabinet.load_ir_file(&ir_path) {
                    log::error!("Failed to load cabinet IR: {:?}", e);
                } else {
                    log::info!("Loaded cabinet IR: {:?}", ir_path);
                    self.last_cabinet_model = cabinet_model;
                }
            } else {
                log::warn!("Cabinet IR not found for model {}", cabinet_model);
                self.last_cabinet_model = cabinet_model;
            }
        }
        
        self.cabinet.set_mix(params.cabinet.mix.value());

        // Delay
        self.delay.set_time_ms(params.delay.time.value());
        self.delay.set_feedback(params.delay.feedback.value());
        self.delay.set_mix(params.delay.mix.value());
        self.delay.set_sync(params.delay.sync.value());

        // Reverb
        self.reverb.set_size(params.reverb.size.value());
        self.reverb.set_decay(params.reverb.decay.value());
        self.reverb.set_damping(params.reverb.damping.value());
        self.reverb.set_predelay_ms(params.reverb.predelay.value());
        self.reverb.set_mix(params.reverb.mix.value());

        // Chorus
        self.chorus.set_rate(params.chorus.rate.value());
        self.chorus.set_depth(params.chorus.depth.value());
        self.chorus.set_mix(params.chorus.mix.value());

        // Master
        self.master_gain = utils::db_to_linear(params.amp.master.value());
    }

    /// Process audio buffer
    pub fn process(&mut self, buffer: &mut Buffer, params: &ThunderforgeParams) {
        if !self.prepared {
            return;
        }

        // Update parameters (in real implementation, use smoothed values)
        self.update_parameters(params);

        let num_channels = buffer.channels();
        let num_samples = buffer.samples();

        // Get raw buffer data as slices
        let channels = buffer.as_slice();
        
        if channels.is_empty() {
            return;
        }

        let input_left = &channels[0];
        let input_right = if num_channels > 1 { &channels[1] } else { &channels[0] };

        // Working buffers
        let mut work_l = input_left.to_vec();
        let mut work_r = input_right.to_vec();

        // Apply input gain
        for sample in &mut work_l {
            *sample *= self.input_gain;
        }
        for sample in &mut work_r {
            *sample *= self.input_gain;
        }

        // Metering (RMS)
        let input_rms = utils::rms(&work_l);
        self.input_level.store(input_rms, std::sync::atomic::Ordering::Relaxed);

        // === SIGNAL CHAIN ===

        // 1. Noise Gate (mono processing)
        if !params.gate.bypass.value() {
            self.noise_gate.process_buffer(&mut work_l);
            self.noise_gate.process_buffer(&mut work_r);
        }

        // 2. Overdrive/Tube Screamer
        if !params.overdrive.bypass.value() {
            self.overdrive.process_buffer(&mut work_l);
            self.overdrive.process_buffer(&mut work_r);
        }

        // 3. Compressor
        if !params.compressor.bypass.value() {
            self.compressor.process_buffer(&mut work_l);
            self.compressor.process_buffer(&mut work_r);
        }

        // 4. Preamp (fallback waveshaper - NAM would go here)
        for i in 0..num_samples {
            work_l[i] = self.preamp_waveshaper.process(work_l[i]);
            work_r[i] = self.preamp_waveshaper.process(work_r[i]);
        }

        // 5. Tone Stack (EQ)
        self.tone_stack.process_buffer(&mut work_l);
        self.tone_stack.process_buffer(&mut work_r);

        // 6. Power Amp (fallback)
        for i in 0..num_samples {
            work_l[i] = self.power_amp.process(work_l[i]);
            work_r[i] = self.power_amp.process(work_r[i]);
        }

        // 7. Cabinet Simulation (IR)
        if !params.cabinet.bypass.value() {
            // Process mono to stereo
            for i in 0..num_samples {
                let mut stereo_out = [0.0, 0.0];
                self.cabinet.process_mono_to_stereo(
                    &[(work_l[i] + work_r[i]) * 0.5],
                    &mut stereo_out,
                );
                work_l[i] = stereo_out[0];
                work_r[i] = stereo_out[1];
            }
        }

        // 8. Effects: Chorus → Delay → Reverb
        for i in 0..num_samples {
            let mut stereo_in = [work_l[i], work_r[i]];
            let mut stereo_out = [0.0, 0.0];

            // Chorus
            if !params.chorus.bypass.value() {
                self.chorus.process_buffer(&stereo_in, &mut stereo_out);
                stereo_in = stereo_out;
            }

            // Delay
            if !params.delay.bypass.value() {
                self.delay.process_buffer(&stereo_in, &mut stereo_out);
                stereo_in = stereo_out;
            }

            // Reverb
            if !params.reverb.bypass.value() {
                self.reverb.process_buffer(&stereo_in, &mut stereo_out);
            } else {
                stereo_out = stereo_in;
            }

            work_l[i] = stereo_out[0];
            work_r[i] = stereo_out[1];
        }

        // 9. Master volume
        for i in 0..num_samples {
            work_l[i] *= self.master_gain;
            work_r[i] *= self.master_gain;
        }

        // Copy back to output buffer
        // as_slice() returns &mut [&mut [f32]], so we can write directly
        let out_channels = buffer.as_slice();
        out_channels[0].copy_from_slice(&work_l);
        if num_channels > 1 {
            out_channels[1].copy_from_slice(&work_r);
        }

        // Output metering
        let output_rms = utils::rms(&work_l);
        self.output_level.store(output_rms, std::sync::atomic::Ordering::Relaxed);
    }
}
