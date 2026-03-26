use super::utils::{ms_to_samples, lerp};

/// Chorus effect using modulated delay lines with stereo spread.
#[derive(Clone, Debug)]
pub struct Chorus {
    // Delay buffers
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    // LFO
    lfo_phase: f32,
    // Parameters
    rate: f32,
    depth: f32,
    mix: f32,
    bypass: bool,
    // Sample rate
    sample_rate: f32,
    max_delay_ms: f32,
}

impl Chorus {
    pub fn new(sample_rate: f32, max_delay_ms: f32) -> Self {
        let max_samples = ms_to_samples(max_delay_ms, sample_rate) + 1;
        let mut c = Self {
            buffer_l: vec![0.0; max_samples],
            buffer_r: vec![0.0; max_samples],
            write_pos: 0,
            lfo_phase: 0.0,
            rate: 1.0,
            depth: 0.5,
            mix: 0.5,
            bypass: false,
            sample_rate,
            max_delay_ms,
        };
        c.set_sample_rate(sample_rate);
        c
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        let max_samples = ms_to_samples(self.max_delay_ms, sr) + 1;
        self.buffer_l = vec![0.0; max_samples];
        self.buffer_r = vec![0.0; max_samples];
    }

    pub fn set_rate(&mut self, rate_hz: f32) {
        self.rate = rate_hz.clamp(0.1, 10.0);
    }

    pub fn set_depth(&mut self, depth_pct: f32) {
        self.depth = (depth_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_mix(&mut self, mix_pct: f32) {
        self.mix = (mix_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    pub fn reset(&mut self) {
        self.buffer_l.fill(0.0);
        self.buffer_r.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }

    #[inline(always)]
    fn lfo_sine(&self) -> f32 {
        self.lfo_phase.sin()
    }

    #[inline(always)]
    fn read_interpolated(&self, buffer: &[f32], mut pos: f32) -> f32 {
        // Wrap position
        let len = buffer.len() as f32;
        while pos < 0.0 {
            pos += len;
        }
        while pos >= len {
            pos -= len;
        }

        // Linear interpolation
        let pos_floor = pos.floor() as usize;
        let pos_ceil = (pos_floor + 1) % buffer.len();
        let frac = pos - pos.floor();

        buffer[pos_floor] * (1.0 - frac) + buffer[pos_ceil] * frac
    }

    #[inline(always)]
    pub fn process(&mut self, input_l: f32, input_r: f32) -> [f32; 2] {
        if self.bypass {
            return [input_l, input_r];
        }

        // Update LFO
        let lfo_value = self.lfo_sine();
        self.lfo_phase += 2.0 * std::f32::consts::PI * self.rate / self.sample_rate;
        if self.lfo_phase > 2.0 * std::f32::consts::PI {
            self.lfo_phase -= 2.0 * std::f32::consts::PI;
        }

        // Delay modulation (1ms to 25ms base, modulated by LFO)
        let base_delay_ms = 10.0;
        let modulation_ms = self.depth * 15.0 * lfo_value; // +/- 15ms
        let delay_ms_l = base_delay_ms + modulation_ms;
        let delay_ms_r = base_delay_ms + modulation_ms * 0.99; // Slight difference for stereo

        let delay_samples_l = ms_to_samples(delay_ms_l, self.sample_rate) as f32;
        let delay_samples_r = ms_to_samples(delay_ms_r, self.sample_rate) as f32;

        // Calculate read positions
        let mut read_l = self.write_pos as f32 - delay_samples_l;
        let mut read_r = self.write_pos as f32 - delay_samples_r;

        // Wrap
        let len = self.buffer_l.len() as f32;
        while read_l < 0.0 {
            read_l += len;
        }
        while read_r < 0.0 {
            read_r += len;
        }

        // Read delayed signal
        let delayed_l = self.read_interpolated(&self.buffer_l, read_l);
        let delayed_r = self.read_interpolated(&self.buffer_r, read_r);

        // Write to buffer
        self.buffer_l[self.write_pos] = input_l;
        self.buffer_r[self.write_pos] = input_r;

        // Increment write position
        self.write_pos = (self.write_pos + 1) % self.buffer_l.len();

        // Mix
        let out_l = lerp(input_l, delayed_l, self.mix);
        let out_r = lerp(input_r, delayed_r, self.mix);

        [out_l, out_r]
    }

    pub fn process_buffer(&mut self, input: &[f32; 2], output: &mut [f32; 2]) {
        let out = self.process(input[0], input[1]);
        output[0] = out[0];
        output[1] = out[1];
    }
}
