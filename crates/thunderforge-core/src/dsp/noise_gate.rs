use super::utils::{db_to_linear, ms_to_samples};

#[derive(Clone, Debug)]
pub struct NoiseGate {
    threshold: f32,
    attack_coeff: f32,
    hold_samples: usize,
    release_coeff: f32,
    // State
    envelope: f32,
    gain: f32,
    hold_counter: usize,
    open: bool,
    sample_rate: f32,
}

impl NoiseGate {
    pub fn new(sample_rate: f32) -> Self {
        let mut g = Self {
            threshold: 0.0,
            attack_coeff: 0.0,
            hold_samples: 0,
            release_coeff: 0.0,
            envelope: 0.0,
            gain: 0.0,
            hold_counter: 0,
            open: false,
            sample_rate,
        };
        g.set_params(-45.0, 1.0, 50.0, 100.0);
        g
    }

    pub fn set_params(&mut self, threshold_db: f32, attack_ms: f32, hold_ms: f32, release_ms: f32) {
        self.threshold = db_to_linear(threshold_db);
        let sr = self.sample_rate;
        self.attack_coeff = (-1.0 / (ms_to_samples(attack_ms, sr).max(1) as f32)).exp();
        self.hold_samples = ms_to_samples(hold_ms, sr);
        self.release_coeff = (-1.0 / (ms_to_samples(release_ms, sr).max(1) as f32)).exp();
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.gain = 0.0;
        self.hold_counter = 0;
        self.open = false;
    }

    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        // Envelope follower (peak detector with smoothing)
        let abs_sample = sample.abs();
        if abs_sample > self.envelope {
            self.envelope = abs_sample;
        } else {
            self.envelope = self.envelope * 0.9999 + abs_sample * 0.0001;
        }

        // Gate logic with hysteresis (3dB band)
        let open_threshold = self.threshold;
        let close_threshold = self.threshold * 0.707; // -3dB hysteresis

        if self.envelope >= open_threshold {
            self.open = true;
            self.hold_counter = self.hold_samples;
        } else if self.open && self.envelope < close_threshold {
            if self.hold_counter > 0 {
                self.hold_counter -= 1;
            } else {
                self.open = false;
            }
        }

        // Gain smoothing (attack/release)
        let target_gain = if self.open { 1.0 } else { 0.0 };
        let coeff = if target_gain > self.gain {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        self.gain = self.gain * coeff + target_gain * (1.0 - coeff);

        sample * self.gain
    }

    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }
}
