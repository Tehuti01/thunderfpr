use super::utils::ms_to_samples;

/// Stereo delay with interpolation and feedback.
#[derive(Clone, Debug)]
pub struct StereoDelay {
    // Buffer
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    read_pos_l: f32,
    read_pos_r: f32,
    // Parameters
    delay_time_ms: f32,
    feedback: f32,
    mix: f32,
    sync: bool,
    bypass: bool,
    // Filters
    fb_lpf_state: f32,
    fb_lpf_coeff: f32,
    // Sample rate
    sample_rate: f32,
    max_delay_ms: f32,
}

impl StereoDelay {
    pub fn new(sample_rate: f32, max_delay_ms: f32) -> Self {
        let max_samples = ms_to_samples(max_delay_ms, sample_rate) + 1;
        let mut d = Self {
            buffer_l: vec![0.0; max_samples],
            buffer_r: vec![0.0; max_samples],
            write_pos: 0,
            read_pos_l: 0.0,
            read_pos_r: 0.0,
            delay_time_ms: 375.0,
            feedback: 0.4,
            mix: 0.3,
            sync: false,
            bypass: false,
            fb_lpf_state: 0.0,
            fb_lpf_coeff: 0.0,
            sample_rate,
            max_delay_ms,
        };
        d.set_sample_rate(sample_rate);
        d
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        let max_samples = ms_to_samples(self.max_delay_ms, sr) + 1;
        self.buffer_l = vec![0.0; max_samples];
        self.buffer_r = vec![0.0; max_samples];
        // Lowpass in feedback path at ~2kHz for tape delay character
        let omega = 2.0 * std::f32::consts::PI * 2000.0 / sr;
        self.fb_lpf_coeff = 1.0 - omega.min(1.0);
    }

    pub fn set_time_ms(&mut self, time_ms: f32) {
        self.delay_time_ms = time_ms.clamp(1.0, self.max_delay_ms);
    }

    pub fn set_feedback(&mut self, fb_pct: f32) {
        self.feedback = (fb_pct / 100.0).clamp(0.0, 0.95);
    }

    pub fn set_mix(&mut self, mix_pct: f32) {
        self.mix = (mix_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_sync(&mut self, sync: bool) {
        self.sync = sync;
        // Could sync to host tempo here
    }

    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    pub fn reset(&mut self) {
        self.buffer_l.fill(0.0);
        self.buffer_r.fill(0.0);
        self.write_pos = 0;
        self.read_pos_l = 0.0;
        self.read_pos_r = 0.0;
        self.fb_lpf_state = 0.0;
    }

    #[inline(always)]
    fn read_interpolated(&self, buffer: &[f32], mut pos: f32) -> f32 {
        // Wrap position
        let len = buffer.len() as f32;
        if pos >= len {
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

        let delay_samples = ms_to_samples(self.delay_time_ms, self.sample_rate) as f32;

        // Calculate read positions (stereo spread)
        let mut read_l = self.write_pos as f32 - delay_samples;
        let mut read_r = self.write_pos as f32 - delay_samples * 1.01; // Slight detune

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

        // Feedback with lowpass
        let fb_in_l = delayed_l * self.feedback;
        let _fb_in_r = delayed_r * self.feedback;

        self.fb_lpf_state = self.fb_lpf_state * self.fb_lpf_coeff + fb_in_l * (1.0 - self.fb_lpf_coeff);

        // Write to buffer (input + feedback)
        self.buffer_l[self.write_pos] = input_l + self.fb_lpf_state;
        self.buffer_r[self.write_pos] = input_r + self.fb_lpf_state * 0.99; // Slight difference for stereo

        // Increment write position
        self.write_pos = (self.write_pos + 1) % self.buffer_l.len();

        // Update read positions
        self.read_pos_l = read_l + 1.0;
        self.read_pos_r = read_r + 1.0;

        // Mix
        let out_l = input_l * (1.0 - self.mix) + delayed_l * self.mix;
        let out_r = input_r * (1.0 - self.mix) + delayed_r * self.mix;

        [out_l, out_r]
    }

    pub fn process_buffer(&mut self, input: &[f32; 2], output: &mut [f32; 2]) {
        let out = self.process(input[0], input[1]);
        output[0] = out[0];
        output[1] = out[1];
    }
}
