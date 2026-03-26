use super::biquad::Biquad;

/// Marshall-style 3-band tone stack (James topology) plus Presence control.
/// Bass/Mid/Treble/Presence all on 0–10 scale.
#[derive(Clone, Debug)]
pub struct ToneStack {
    bass: f32,
    mid: f32,
    treble: f32,
    presence: f32,
    bass_filter: Biquad,
    mid_filter: Biquad,
    treble_filter: Biquad,
    presence_filter: Biquad,
    sample_rate: f32,
}

impl ToneStack {
    pub fn new(sample_rate: f32) -> Self {
        let mut ts = Self {
            bass: 5.0,
            mid: 5.0,
            treble: 5.0,
            presence: 5.0,
            bass_filter: Biquad::passthrough(),
            mid_filter: Biquad::passthrough(),
            treble_filter: Biquad::passthrough(),
            presence_filter: Biquad::passthrough(),
            sample_rate,
        };
        ts.recalculate();
        ts
    }

    /// All values on 0–10 scale
    pub fn set_params(&mut self, bass: f32, mid: f32, treble: f32, presence: f32) {
        self.bass = bass.clamp(0.0, 10.0);
        self.mid = mid.clamp(0.0, 10.0);
        self.treble = treble.clamp(0.0, 10.0);
        self.presence = presence.clamp(0.0, 10.0);
        self.recalculate();
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.recalculate();
    }

    fn knob_to_db(value: f32, range_db: f32) -> f32 {
        // 0-10 → -range_db to +range_db (center at 5 = 0 dB)
        (value - 5.0) * (range_db / 5.0)
    }

    fn recalculate(&mut self) {
        let sr = self.sample_rate;
        // Bass: low shelf at 100Hz, ±12dB
        let bass_db = Self::knob_to_db(self.bass, 12.0);
        self.bass_filter = Biquad::low_shelf(sr, 100.0, bass_db);

        // Mid: peaking at 800Hz, ±12dB, Q=0.7
        let mid_db = Self::knob_to_db(self.mid, 12.0);
        self.mid_filter = Biquad::peaking(sr, 800.0, mid_db, 0.7);

        // Treble: high shelf at 3.2kHz, ±12dB
        let treble_db = Self::knob_to_db(self.treble, 12.0);
        self.treble_filter = Biquad::high_shelf(sr, 3200.0, treble_db);

        // Presence: high shelf at 5kHz, ±6dB
        let presence_db = Self::knob_to_db(self.presence, 6.0);
        self.presence_filter = Biquad::high_shelf(sr, 5000.0, presence_db);
    }

    pub fn reset(&mut self) {
        self.bass_filter.reset();
        self.mid_filter.reset();
        self.treble_filter.reset();
        self.presence_filter.reset();
    }

    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        let s = self.bass_filter.process(sample);
        let s = self.mid_filter.process(s);
        let s = self.treble_filter.process(s);
        self.presence_filter.process(s)
    }

    pub fn process_buffer(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }
}
