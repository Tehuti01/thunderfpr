/// Second-order IIR biquad filter.
/// Coefficients via Robert Bristow-Johnson Audio EQ Cookbook.
#[derive(Clone, Debug, Default)]
pub struct Biquad {
    // Coefficients
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    // State
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl Biquad {
    pub fn new() -> Self {
        Self::passthrough()
    }

    pub fn passthrough() -> Self {
        Self { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0, ..Default::default() }
    }

    /// Low shelf filter
    pub fn low_shelf(sample_rate: f32, freq: f32, db_gain: f32) -> Self {
        let a = (10.0_f64).powf(db_gain as f64 / 40.0);
        let w0 = 2.0 * std::f64::consts::PI * freq as f64 / sample_rate as f64;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / 2.0 * ((a + 1.0 / a) * (1.0 / 1.0 - 1.0) + 2.0).sqrt();

        let b0 =   a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 =   a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha);
        let a0 =       (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;
        let a1 =  -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
        let a2 =       (a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            ..Default::default()
        }
    }

    /// High shelf filter
    pub fn high_shelf(sample_rate: f32, freq: f32, db_gain: f32) -> Self {
        let a = (10.0_f64).powf(db_gain as f64 / 40.0);
        let w0 = 2.0 * std::f64::consts::PI * freq as f64 / sample_rate as f64;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / 2.0 * ((a + 1.0 / a) * (1.0 / 1.0 - 1.0) + 2.0).sqrt();

        let b0 =   a * ((a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
        let b2 =   a * ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha);
        let a0 =       (a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;
        let a1 =   2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
        let a2 =       (a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            ..Default::default()
        }
    }

    /// Peaking EQ filter
    pub fn peaking(sample_rate: f32, freq: f32, db_gain: f32, q: f32) -> Self {
        let a = (10.0_f64).powf(db_gain as f64 / 40.0);
        let w0 = 2.0 * std::f64::consts::PI * freq as f64 / sample_rate as f64;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q as f64);

        let b0 =  1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 =  1.0 - alpha * a;
        let a0 =  1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 =  1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            ..Default::default()
        }
    }

    /// First-order lowpass filter (biquad form)
    pub fn lowpass(sample_rate: f32, freq: f32) -> Self {
        let w0 = 2.0 * std::f64::consts::PI * freq as f64 / sample_rate as f64;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * 0.707_f64);

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 =  1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 =  1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 =  1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            ..Default::default()
        }
    }

    /// First-order highpass filter
    pub fn highpass(sample_rate: f32, freq: f32) -> Self {
        let w0 = 2.0 * std::f64::consts::PI * freq as f64 / sample_rate as f64;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * 0.707_f64);

        let b0 =  (1.0 + cos_w0) / 2.0;
        let b1 = -(1.0 + cos_w0);
        let b2 =  (1.0 + cos_w0) / 2.0;
        let a0 =   1.0 + alpha;
        let a1 =  -2.0 * cos_w0;
        let a2 =   1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            ..Default::default()
        }
    }

    /// Process a single sample
    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let xd = x as f64;
        let y = self.b0 * xd + self.b1 * self.x1 + self.b2 * self.x2
              - self.a1 * self.y1 - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = xd;
        self.y2 = self.y1;
        self.y1 = y;

        y as f32
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}
