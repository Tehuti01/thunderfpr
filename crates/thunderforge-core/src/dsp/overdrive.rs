use super::biquad::Biquad;

/// Tube Screamer-style overdrive with asymmetric soft-clipping.
#[derive(Clone, Debug)]
pub struct TubeScreamer {
    drive: f32,
    tone: f32,
    level: f32,
    hp_filter: Biquad,   // Input high-pass (removes mud below 720Hz)
    tone_filter: Biquad, // Output tone control (LP sweep 1kHz-5kHz)
    sample_rate: f32,
}

impl TubeScreamer {
    pub fn new(sample_rate: f32) -> Self {
        let mut ts = Self {
            drive: 0.5,
            tone: 0.5,
            level: 0.5,
            hp_filter: Biquad::highpass(sample_rate, 720.0),
            tone_filter: Biquad::lowpass(sample_rate, 3000.0),
            sample_rate,
        };
        ts.update_tone();
        ts
    }

    pub fn set_drive(&mut self, drive_pct: f32) {
        self.drive = (drive_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_tone(&mut self, tone_pct: f32) {
        self.tone = (tone_pct / 100.0).clamp(0.0, 1.0);
        self.update_tone();
    }

    pub fn set_level(&mut self, level_pct: f32) {
        self.level = (level_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.hp_filter = Biquad::highpass(sr, 720.0);
        self.update_tone();
    }

    fn update_tone(&mut self) {
        // Tone sweep: 1kHz to 5kHz
        let cutoff = 1000.0 + self.tone * 4000.0;
        self.tone_filter = Biquad::lowpass(self.sample_rate, cutoff);
    }

    pub fn reset(&mut self) {
        self.hp_filter.reset();
        self.tone_filter.reset();
    }

    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        // 1. Remove low-end mud
        let hp = self.hp_filter.process(sample);

        // 2. Gain staging
        let gain = 1.0 + self.drive * 49.0; // 1x to 50x

        // 3. Asymmetric soft-clip (tanh waveshaper)
        let driven = hp * gain;
        let clipped = if driven >= 0.0 {
            driven.tanh() * 1.0 // positive half — normal
        } else {
            driven.tanh() * 0.95 // negative half — slight asymmetry for even harmonics
        };

        // Normalize tanh output by gain saturation
        let gain_tanh = (gain).tanh().max(1e-6);
        let normalized = clipped / gain_tanh;

        // 4. Tone filter
        let toned = self.tone_filter.process(normalized);

        // 5. Output level
        toned * self.level
    }

    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }
}
