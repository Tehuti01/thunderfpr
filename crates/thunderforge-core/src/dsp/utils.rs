/// Convert dB value to linear amplitude
#[inline(always)]
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Convert linear amplitude to dB
#[inline(always)]
pub fn linear_to_db(linear: f32) -> f32 {
    if linear <= 1e-10 {
        -200.0
    } else {
        20.0 * linear.log10()
    }
}

/// Convert milliseconds to sample count
#[inline(always)]
pub fn ms_to_samples(ms: f32, sample_rate: f32) -> usize {
    ((ms * sample_rate) / 1000.0).round() as usize
}

/// Convert sample count to milliseconds
#[inline(always)]
pub fn samples_to_ms(samples: usize, sample_rate: f32) -> f32 {
    (samples as f32) * 1000.0 / sample_rate
}

/// Clamp value between min and max
#[inline(always)]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Soft clip using tanh
#[inline(always)]
pub fn soft_clip(x: f32) -> f32 {
    x.tanh()
}

/// Hard clip
#[inline(always)]
pub fn hard_clip(x: f32) -> f32 {
    clamp(x, -1.0, 1.0)
}

/// Denormal protection — flush values near zero to zero
#[inline(always)]
pub fn denormal_guard(x: f32) -> f32 {
    if x.abs() < 1e-15 { 0.0 } else { x }
}

/// One-pole lowpass filter coefficient from cutoff frequency
#[inline(always)]
pub fn one_pole_coeff(cutoff_hz: f32, sample_rate: f32) -> f32 {
    let omega = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate;
    1.0 - omega.min(1.0)
}

/// Simple linear interpolation
#[inline(always)]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// RMS of a buffer slice
pub fn rms(buf: &[f32]) -> f32 {
    if buf.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = buf.iter().map(|&x| x * x).sum();
    (sum_sq / buf.len() as f32).sqrt()
}
