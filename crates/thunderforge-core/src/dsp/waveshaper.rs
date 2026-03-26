/// Multi-stage tube preamp waveshaper fallback (used when no NAM model is loaded).
#[derive(Clone, Debug)]
pub struct WaveShaper {
    gain: f32,
    bias: f32,
    stages: usize,
    // Simple interstage filter state (one-pole lowpass at 8kHz)
    lp_state: f32,
    lp_coeff: f32,
    hp_state: f32,
    hp_coeff: f32,
}

impl WaveShaper {
    pub fn new(sample_rate: f32) -> Self {
        let mut ws = Self {
            gain: 5.0,
            bias: 0.02,
            stages: 2,
            lp_state: 0.0,
            lp_coeff: 0.0,
            hp_state: 0.0,
            hp_coeff: 0.0,
        };
        ws.set_sample_rate(sample_rate);
        ws
    }

    pub fn set_gain(&mut self, gain_0_to_10: f32) {
        self.gain = 1.0 + gain_0_to_10 * 4.0; // 1x to 41x
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        // One-pole lowpass at 8kHz (interstage)
        let omega_lp = 2.0 * std::f32::consts::PI * 8000.0 / sr;
        self.lp_coeff = 1.0 - omega_lp.min(1.0);
        // One-pole highpass at 80Hz (bass rolloff between stages)
        let omega_hp = 2.0 * std::f32::consts::PI * 80.0 / sr;
        self.hp_coeff = 1.0 - omega_hp.min(1.0);
    }

    pub fn reset(&mut self) {
        self.lp_state = 0.0;
        self.hp_state = 0.0;
    }

    /// Asymmetric tube-style saturation — even harmonics via bias shift
    #[inline(always)]
    fn tube_sat(x: f32, gain: f32, bias: f32) -> f32 {
        let biased = x + bias;
        if biased >= 0.0 {
            biased.tanh() / (gain * bias + 1e-6).tanh().max(1e-6)
        } else {
            biased.tanh() * 0.95 / (gain * bias + 1e-6).tanh().max(1e-6)
        }
    }

    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        let mut s = sample * self.gain;

        // Stage 1
        s = Self::tube_sat(s, self.gain, self.bias);

        // Interstage filter (lowpass + highpass)
        self.lp_state = self.lp_state * self.lp_coeff + s * (1.0 - self.lp_coeff);
        let lp_out = self.lp_state;
        let hp_out = lp_out - self.hp_state;
        self.hp_state = self.hp_state * self.hp_coeff + lp_out * (1.0 - self.hp_coeff);
        s = hp_out;

        // Stage 2 (softer)
        if self.stages >= 2 {
            s = Self::tube_sat(s * 0.7, self.gain * 0.5, self.bias * 0.5);
        }

        s
    }

    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }
}

/// Power amp emulation (sag + soft saturation)
#[derive(Clone, Debug)]
pub struct PowerAmp {
    sag_depth: f32,
    sag_envelope: f32,
    sag_coeff: f32,
    output_hp: f32,
    output_hp_coeff: f32,
    output_lp: f32,
    output_lp_coeff: f32,
}

impl PowerAmp {
    pub fn new(sample_rate: f32) -> Self {
        let mut pa = Self {
            sag_depth: 0.1,
            sag_envelope: 0.0,
            sag_coeff: 0.0,
            output_hp: 0.0,
            output_hp_coeff: 0.0,
            output_lp: 0.0,
            output_lp_coeff: 0.0,
        };
        pa.set_sample_rate(sample_rate);
        pa
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        // Sag envelope: ~50ms release
        let omega_sag = 2.0 * std::f32::consts::PI * 20.0 / sr;
        self.sag_coeff = 1.0 - omega_sag.min(1.0);
        // Output transformer: bass rolloff at 50Hz, treble rolloff at 7kHz
        let omega_hp = 2.0 * std::f32::consts::PI * 50.0 / sr;
        self.output_hp_coeff = 1.0 - omega_hp.min(1.0);
        let omega_lp = 2.0 * std::f32::consts::PI * 7000.0 / sr;
        self.output_lp_coeff = 1.0 - omega_lp.min(1.0);
    }

    pub fn reset(&mut self) {
        self.sag_envelope = 0.0;
        self.output_hp = 0.0;
        self.output_lp = 0.0;
    }

    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        // Power supply sag
        let abs_s = sample.abs();
        if abs_s > self.sag_envelope {
            self.sag_envelope = abs_s;
        } else {
            self.sag_envelope *= self.sag_coeff;
        }
        let sag_gain = 1.0 - self.sag_envelope * self.sag_depth;

        // Soft saturation (lower threshold than preamp)
        let s = (sample * sag_gain * 0.9).tanh();

        // Output transformer coloring
        self.output_lp = self.output_lp * self.output_lp_coeff + s * (1.0 - self.output_lp_coeff);
        let hp_out = self.output_lp - self.output_hp;
        self.output_hp = self.output_hp * self.output_hp_coeff
            + self.output_lp * (1.0 - self.output_hp_coeff);

        hp_out
    }

    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }
}
