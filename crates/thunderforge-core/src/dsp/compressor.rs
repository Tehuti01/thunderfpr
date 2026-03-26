use super::utils::{db_to_linear, linear_to_db, ms_to_samples};

/// Feed-forward RMS compressor.
#[derive(Clone, Debug)]
pub struct Compressor {
    threshold_db: f32,
    ratio: f32,
    attack_coeff: f32,
    release_coeff: f32,
    makeup_gain: f32,
    // State
    envelope: f32,
    gain_db: f32,
    sample_rate: f32,
    // RMS window
    rms_buffer: Vec<f32>,
    rms_pos: usize,
    rms_sum_sq: f32,
}

impl Compressor {
    pub fn new(sample_rate: f32) -> Self {
        let rms_len = ((sample_rate * 0.001) as usize).max(1); // 1ms window
        let mut c = Self {
            threshold_db: -20.0,
            ratio: 4.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            makeup_gain: 1.0,
            envelope: 0.0,
            gain_db: 0.0,
            sample_rate,
            rms_buffer: vec![0.0; rms_len],
            rms_pos: 0,
            rms_sum_sq: 0.0,
        };
        c.set_params(-20.0, 4.0, 10.0, 100.0, 0.0);
        c
    }

    pub fn set_params(
        &mut self,
        threshold_db: f32,
        ratio: f32,
        attack_ms: f32,
        release_ms: f32,
        makeup_db: f32,
    ) {
        self.threshold_db = threshold_db;
        self.ratio = ratio.max(1.0);
        let sr = self.sample_rate;
        self.attack_coeff = (-1.0 / (ms_to_samples(attack_ms, sr).max(1) as f32)).exp();
        self.release_coeff = (-1.0 / (ms_to_samples(release_ms, sr).max(1) as f32)).exp();
        self.makeup_gain = db_to_linear(makeup_db);
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        let rms_len = ((sr * 0.001) as usize).max(1);
        self.rms_buffer = vec![0.0; rms_len];
        self.rms_pos = 0;
        self.rms_sum_sq = 0.0;
    }

    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.gain_db = 0.0;
        self.rms_buffer.fill(0.0);
        self.rms_pos = 0;
        self.rms_sum_sq = 0.0;
    }

    #[inline(always)]
    fn rms_push(&mut self, sample: f32) -> f32 {
        let old = self.rms_buffer[self.rms_pos];
        self.rms_sum_sq = (self.rms_sum_sq - old * old + sample * sample).max(0.0);
        self.rms_buffer[self.rms_pos] = sample;
        self.rms_pos = (self.rms_pos + 1) % self.rms_buffer.len();
        (self.rms_sum_sq / self.rms_buffer.len() as f32).sqrt()
    }

    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        // 1. Compute RMS level
        let rms = self.rms_push(sample);
        let level_db = linear_to_db(rms.max(1e-10));

        // 2. Compute target gain reduction
        let target_reduction = if level_db > self.threshold_db {
            (level_db - self.threshold_db) * (1.0 - 1.0 / self.ratio)
        } else {
            0.0
        };

        // 3. Smooth gain reduction with attack/release
        let coeff = if target_reduction > self.gain_db {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        self.gain_db = self.gain_db * coeff + target_reduction * (1.0 - coeff);

        // 4. Apply gain
        let gain = db_to_linear(-self.gain_db) * self.makeup_gain;
        sample * gain
    }

    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }

    pub fn gain_reduction_db(&self) -> f32 {
        -self.gain_db
    }
}
