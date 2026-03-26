use super::utils::ms_to_samples;

/// Feedback Delay Network (FDN) reverb with 8 comb filters + 4 allpass filters.
#[derive(Clone, Debug)]
pub struct FdnReverb {
    // Comb filters (parallel)
    comb_buffers: [Vec<f32>; 8],
    comb_positions: [usize; 8],
    comb_gains: [f32; 8],
    // Allpass filters (series)
    ap_buffers: [Vec<f32>; 4],
    ap_positions: [usize; 4],
    ap_gain: f32,
    // Parameters
    size: f32,      // 0-100 (controls delay times)
    decay: f32,     // 0.1-10.0 seconds
    damping: f32,   // 0-100 (lowpass in feedback)
    predelay_ms: f32,
    mix: f32,
    bypass: bool,
    // Damping filters
    damping_state: [f32; 8],
    damping_coeff: f32,
    // Predelay
    predelay_buffer: Vec<f32>,
    predelay_write: usize,
    predelay_read: usize,
    // Sample rate
    sample_rate: f32,
    // Hadamard matrix coefficients (simplified)
    hadamard: [[f32; 8]; 8],
}

impl FdnReverb {
    pub fn new(sample_rate: f32) -> Self {
        // Comb delay times in ms (prime numbers for density)
        let comb_times = [29.7, 37.1, 41.3, 47.9, 53.2, 59.8, 67.3, 73.1];
        // Allpass delay times
        let ap_times = [5.0, 7.0, 9.0, 11.0];

        let mut comb_buffers: [Vec<f32>; 8] = Default::default();
        let comb_positions: [usize; 8] = [0; 8];
        let mut ap_buffers: [Vec<f32>; 4] = Default::default();
        let ap_positions: [usize; 4] = [0; 4];

        for i in 0..8 {
            let len = ms_to_samples(comb_times[i], sample_rate).max(1);
            comb_buffers[i] = vec![0.0; len];
        }
        for i in 0..4 {
            let len = ms_to_samples(ap_times[i], sample_rate).max(1);
            ap_buffers[i] = vec![0.0; len];
        }

        let mut r = Self {
            comb_buffers,
            comb_positions,
            comb_gains: [0.0; 8],
            ap_buffers,
            ap_positions,
            ap_gain: 0.5,
            size: 50.0,
            decay: 2.0,
            damping: 50.0,
            predelay_ms: 20.0,
            mix: 0.2,
            bypass: false,
            damping_state: [0.0; 8],
            damping_coeff: 0.0,
            predelay_buffer: Vec::new(),
            predelay_write: 0,
            predelay_read: 0,
            sample_rate,
            hadamard: [
                [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
                [1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0],
                [1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
                [1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0],
                [1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0],
                [1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0],
            ],
        };
        r.set_sample_rate(sample_rate);
        r.update_comb_gains();
        r
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        // Recreate buffers
        let comb_times = [29.7, 37.1, 41.3, 47.9, 53.2, 59.8, 67.3, 73.1];
        let ap_times = [5.0, 7.0, 9.0, 11.0];

        for (i, &t) in comb_times.iter().enumerate() {
            let len = ms_to_samples(t * (self.size / 50.0), sr).max(1);
            self.comb_buffers[i] = vec![0.0; len];
            self.comb_positions[i] = 0;
        }
        for (i, &t) in ap_times.iter().enumerate() {
            let len = ms_to_samples(t, sr).max(1);
            self.ap_buffers[i] = vec![0.0; len];
            self.ap_positions[i] = 0;
        }

        // Predelay
        let pd_len = ms_to_samples(self.predelay_ms, sr).max(1);
        self.predelay_buffer = vec![0.0; pd_len];
        self.predelay_write = 0;
        self.predelay_read = 0;

        // Damping coefficient
        let omega = 2.0 * std::f32::consts::PI * (1000.0 + self.damping * 100.0) / sr;
        self.damping_coeff = 1.0 - omega.min(1.0);
    }

    pub fn set_size(&mut self, size_pct: f32) {
        self.size = size_pct.clamp(0.0, 100.0);
        // Could resize buffers here for real-time size changes
    }

    pub fn set_decay(&mut self, decay_sec: f32) {
        self.decay = decay_sec.clamp(0.1, 10.0);
        self.update_comb_gains();
    }

    pub fn set_damping(&mut self, damping_pct: f32) {
        self.damping = damping_pct.clamp(0.0, 100.0);
        let omega = 2.0 * std::f32::consts::PI * (1000.0 + self.damping * 100.0) / self.sample_rate;
        self.damping_coeff = 1.0 - omega.min(1.0);
    }

    pub fn set_predelay_ms(&mut self, ms: f32) {
        self.predelay_ms = ms.clamp(0.0, 200.0);
        let len = ms_to_samples(self.predelay_ms, self.sample_rate).max(1);
        self.predelay_buffer = vec![0.0; len];
        self.predelay_write = 0;
        self.predelay_read = 0;
    }

    pub fn set_mix(&mut self, mix_pct: f32) {
        self.mix = (mix_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    fn update_comb_gains(&mut self) {
        // Calculate feedback gain from decay time
        // RT60 = -3.0 * delay / ln(gain)
        // gain = exp(-3.0 * delay / RT60)
        let comb_times = [29.7, 37.1, 41.3, 47.9, 53.2, 59.8, 67.3, 73.1];
        for (i, &t) in comb_times.iter().enumerate() {
            let delay_sec = t / 1000.0 * (self.size / 50.0);
            self.comb_gains[i] = (-3.0 * delay_sec / self.decay).exp().clamp(0.0, 0.98);
        }
    }

    pub fn reset(&mut self) {
        for buf in &mut self.comb_buffers {
            buf.fill(0.0);
        }
        for buf in &mut self.ap_buffers {
            buf.fill(0.0);
        }
        self.predelay_buffer.fill(0.0);
        self.damping_state.fill(0.0);
        self.comb_positions = [0; 8];
        self.ap_positions = [0; 4];
        self.predelay_write = 0;
        self.predelay_read = 0;
    }

    #[inline(always)]
    fn comb_process(&mut self, input: f32, idx: usize) -> f32 {
        let pos = self.comb_positions[idx];
        let output = self.comb_buffers[idx][pos];

        // Damping filter
        let filtered = self.damping_state[idx] * self.damping_coeff + output * (1.0 - self.damping_coeff);
        self.damping_state[idx] = filtered;

        // Write
        self.comb_buffers[idx][pos] = input + filtered * self.comb_gains[idx];
        self.comb_positions[idx] = (pos + 1) % self.comb_buffers[idx].len();

        output
    }

    #[inline(always)]
    fn allpass_process(&mut self, input: f32, idx: usize) -> f32 {
        let pos = self.ap_positions[idx];
        let buffered = self.ap_buffers[idx][pos];
        let output = -input + buffered;
        self.ap_buffers[idx][pos] = input + self.ap_gain * buffered;
        self.ap_positions[idx] = (pos + 1) % self.ap_buffers[idx].len();
        output
    }

    #[inline(always)]
    fn predelay_push(&mut self, sample: f32) -> f32 {
        self.predelay_buffer[self.predelay_write] = sample;
        let out = self.predelay_buffer[self.predelay_read];
        self.predelay_write = (self.predelay_write + 1) % self.predelay_buffer.len();
        self.predelay_read = (self.predelay_read + 1) % self.predelay_buffer.len();
        out
    }

    pub fn process(&mut self, input_l: f32, input_r: f32) -> [f32; 2] {
        if self.bypass {
            return [input_l, input_r];
        }

        // Predelay
        let in_l = self.predelay_push(input_l);
        let in_r = self.predelay_push(input_r);

        // Sum input to all comb filters
        let comb_sum = in_l + in_r;

        // Process comb filters (parallel)
        let mut comb_outputs = [0.0; 8];
        for (i, out) in comb_outputs.iter_mut().enumerate() {
            *out = self.comb_process(comb_sum * 0.125, i);
        }

        // Hadamard mixing (simplified - just sum/difference)
        let mut mixed = [0.0; 8];
        for (i, m) in mixed.iter_mut().enumerate() {
            for (j, &co) in comb_outputs.iter().enumerate() {
                *m += self.hadamard[i][j] * co;
            }
            *m *= 0.354; // 1/sqrt(8) for normalization
        }

        // Process allpass filters (series) on first two outputs for L/R
        let mut wet_l = mixed[0];
        let mut wet_r = mixed[1];

        for i in 0..4 {
            wet_l = self.allpass_process(wet_l, i);
            wet_r = self.allpass_process(wet_r, i);
        }

        // Mix
        let out_l = input_l * (1.0 - self.mix) + wet_l * self.mix;
        let out_r = input_r * (1.0 - self.mix) + wet_r * self.mix;

        [out_l, out_r]
    }

    pub fn process_buffer(&mut self, input: &[f32; 2], output: &mut [f32; 2]) {
        let out = self.process(input[0], input[1]);
        output[0] = out[0];
        output[1] = out[1];
    }
}
