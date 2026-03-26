use nih_plug::prelude::*;
use std::sync::Arc;

/// Gate parameters section
#[derive(Params)]
pub struct GateParams {
    #[id = "gate_threshold"]
    pub threshold: FloatParam,
    #[id = "gate_attack"]
    pub attack: FloatParam,
    #[id = "gate_hold"]
    pub hold: FloatParam,
    #[id = "gate_release"]
    pub release: FloatParam,
    #[id = "gate_bypass"]
    pub bypass: BoolParam,
}

impl Default for GateParams {
    fn default() -> Self {
        Self {
            threshold: FloatParam::new("Threshold", -45.0, FloatRange::Skewed { min: -80.0, max: 0.0, factor: FloatRange::skew_factor(0.5) }).with_unit(" dB"),
            attack: FloatParam::new("Attack", 1.0, FloatRange::Skewed { min: 0.1, max: 50.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" ms"),
            hold: FloatParam::new("Hold", 50.0, FloatRange::Linear { min: 0.0, max: 500.0 }).with_unit(" ms"),
            release: FloatParam::new("Release", 100.0, FloatRange::Skewed { min: 1.0, max: 2000.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" ms"),
            bypass: BoolParam::new("Bypass", false),
        }
    }
}

/// Overdrive parameters
#[derive(Params)]
pub struct OverdriveParams {
    #[id = "ts_drive"]
    pub drive: FloatParam,
    #[id = "ts_tone"]
    pub tone: FloatParam,
    #[id = "ts_level"]
    pub level: FloatParam,
    #[id = "ts_bypass"]
    pub bypass: BoolParam,
}

impl Default for OverdriveParams {
    fn default() -> Self {
        Self {
            drive: FloatParam::new("Drive", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            tone: FloatParam::new("Tone", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            level: FloatParam::new("Level", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            bypass: BoolParam::new("Bypass", false),
        }
    }
}

/// Compressor parameters
#[derive(Params)]
pub struct CompressorParams {
    #[id = "comp_threshold"]
    pub threshold: FloatParam,
    #[id = "comp_ratio"]
    pub ratio: FloatParam,
    #[id = "comp_attack"]
    pub attack: FloatParam,
    #[id = "comp_release"]
    pub release: FloatParam,
    #[id = "comp_makeup"]
    pub makeup: FloatParam,
    #[id = "comp_bypass"]
    pub bypass: BoolParam,
}

impl Default for CompressorParams {
    fn default() -> Self {
        Self {
            threshold: FloatParam::new("Threshold", -20.0, FloatRange::Skewed { min: -60.0, max: 0.0, factor: FloatRange::skew_factor(0.5) }).with_unit(" dB"),
            ratio: FloatParam::new("Ratio", 4.0, FloatRange::Skewed { min: 1.0, max: 20.0, factor: FloatRange::skew_factor(0.5) }).with_unit(":1"),
            attack: FloatParam::new("Attack", 10.0, FloatRange::Skewed { min: 0.1, max: 100.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" ms"),
            release: FloatParam::new("Release", 100.0, FloatRange::Skewed { min: 10.0, max: 1000.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" ms"),
            makeup: FloatParam::new("Makeup", 0.0, FloatRange::Skewed { min: 0.0, max: 24.0, factor: FloatRange::skew_factor(0.5) }).with_unit(" dB"),
            bypass: BoolParam::new("Bypass", true),
        }
    }
}

/// EQ parameters
#[derive(Params)]
pub struct EqParams {
    #[id = "eq_bass"]
    pub bass: FloatParam,
    #[id = "eq_mid"]
    pub mid: FloatParam,
    #[id = "eq_treble"]
    pub treble: FloatParam,
    #[id = "eq_presence"]
    pub presence: FloatParam,
}

impl Default for EqParams {
    fn default() -> Self {
        Self {
            bass: FloatParam::new("Bass", 5.0, FloatRange::Linear { min: 0.0, max: 10.0 }),
            mid: FloatParam::new("Mid", 5.0, FloatRange::Linear { min: 0.0, max: 10.0 }),
            treble: FloatParam::new("Treble", 5.0, FloatRange::Linear { min: 0.0, max: 10.0 }),
            presence: FloatParam::new("Presence", 5.0, FloatRange::Linear { min: 0.0, max: 10.0 }),
        }
    }
}

/// Cabinet parameters
#[derive(Params)]
pub struct CabinetParams {
    #[id = "cab_model"]
    pub model: IntParam,
    #[id = "cab_mix"]
    pub mix: FloatParam,
    #[id = "cab_bypass"]
    pub bypass: BoolParam,
}

impl Default for CabinetParams {
    fn default() -> Self {
        Self {
            model: IntParam::new("Model", 0, IntRange::Linear { min: 0, max: 5 })
                .with_value_to_string(Arc::new(|v| match v {
                    0 => "4x12 Greenback".to_string(),
                    1 => "4x12 V30".to_string(),
                    2 => "2x12 Blue".to_string(),
                    3 => "1x12 Open".to_string(),
                    4 => "4x12 T75".to_string(),
                    5 => "Custom".to_string(),
                    _ => format!("{}", v),
                })),
            mix: FloatParam::new("Mix", 100.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            bypass: BoolParam::new("Bypass", false),
        }
    }
}

/// Delay parameters
#[derive(Params)]
pub struct DelayParams {
    #[id = "delay_time"]
    pub time: FloatParam,
    #[id = "delay_feedback"]
    pub feedback: FloatParam,
    #[id = "delay_mix"]
    pub mix: FloatParam,
    #[id = "delay_sync"]
    pub sync: BoolParam,
    #[id = "delay_bypass"]
    pub bypass: BoolParam,
}

impl Default for DelayParams {
    fn default() -> Self {
        Self {
            time: FloatParam::new("Time", 375.0, FloatRange::Skewed { min: 1.0, max: 2000.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" ms"),
            feedback: FloatParam::new("Feedback", 40.0, FloatRange::Linear { min: 0.0, max: 95.0 }).with_unit(" %"),
            mix: FloatParam::new("Mix", 30.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            sync: BoolParam::new("Sync", false),
            bypass: BoolParam::new("Bypass", false),
        }
    }
}

/// Reverb parameters
#[derive(Params)]
pub struct ReverbParams {
    #[id = "reverb_size"]
    pub size: FloatParam,
    #[id = "reverb_decay"]
    pub decay: FloatParam,
    #[id = "reverb_damping"]
    pub damping: FloatParam,
    #[id = "reverb_predelay"]
    pub predelay: FloatParam,
    #[id = "reverb_mix"]
    pub mix: FloatParam,
    #[id = "reverb_bypass"]
    pub bypass: BoolParam,
}

impl Default for ReverbParams {
    fn default() -> Self {
        Self {
            size: FloatParam::new("Size", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            decay: FloatParam::new("Decay", 2.0, FloatRange::Skewed { min: 0.1, max: 10.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" s"),
            damping: FloatParam::new("Damping", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            predelay: FloatParam::new("Pre-Delay", 20.0, FloatRange::Linear { min: 0.0, max: 200.0 }).with_unit(" ms"),
            mix: FloatParam::new("Mix", 20.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            bypass: BoolParam::new("Bypass", false),
        }
    }
}

/// Chorus parameters
#[derive(Params)]
pub struct ChorusParams {
    #[id = "chorus_rate"]
    pub rate: FloatParam,
    #[id = "chorus_depth"]
    pub depth: FloatParam,
    #[id = "chorus_mix"]
    pub mix: FloatParam,
    #[id = "chorus_bypass"]
    pub bypass: BoolParam,
}

impl Default for ChorusParams {
    fn default() -> Self {
        Self {
            rate: FloatParam::new("Rate", 1.0, FloatRange::Skewed { min: 0.1, max: 10.0, factor: FloatRange::skew_factor(0.3) }).with_unit(" Hz"),
            depth: FloatParam::new("Depth", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            mix: FloatParam::new("Mix", 50.0, FloatRange::Linear { min: 0.0, max: 100.0 }).with_unit(" %"),
            bypass: BoolParam::new("Bypass", true),
        }
    }
}

/// Amp parameters
#[derive(Params)]
pub struct AmpParams {
    #[id = "input_gain"]
    pub input_gain: FloatParam,
    #[id = "amp_gain"]
    pub gain: FloatParam,
    #[id = "nam_model"]
    pub model: IntParam,
    #[id = "master_volume"]
    pub master: FloatParam,
}

impl Default for AmpParams {
    fn default() -> Self {
        Self {
            input_gain: FloatParam::new("Input", 0.0, FloatRange::Skewed { min: -12.0, max: 12.0, factor: FloatRange::skew_factor(0.5) }).with_unit(" dB"),
            gain: FloatParam::new("Gain", 5.0, FloatRange::Linear { min: 0.0, max: 10.0 }),
            model: IntParam::new("Model", 0, IntRange::Linear { min: 0, max: 7 })
                .with_value_to_string(Arc::new(|v| match v {
                    0 => "Plexi '68".to_string(),
                    1 => "Plexi Bright".to_string(),
                    2 => "JCM800".to_string(),
                    3 => "Recto Modern".to_string(),
                    4 => "AC30 Top".to_string(),
                    5 => "Twin Clean".to_string(),
                    6 => "5150 Red".to_string(),
                    7 => "Soldano".to_string(),
                    _ => format!("{}", v),
                })),
            master: FloatParam::new("Master", -6.0, FloatRange::Skewed { min: -60.0, max: 12.0, factor: FloatRange::skew_factor(0.5) }).with_unit(" dB"),
        }
    }
}

/// Complete plugin parameters
#[derive(Params)]
pub struct ThunderforgeParams {
    #[nested]
    pub amp: AmpParams,
    #[nested]
    pub gate: GateParams,
    #[nested]
    pub overdrive: OverdriveParams,
    #[nested]
    pub compressor: CompressorParams,
    #[nested]
    pub eq: EqParams,
    #[nested]
    pub cabinet: CabinetParams,
    #[nested]
    pub delay: DelayParams,
    #[nested]
    pub reverb: ReverbParams,
    #[nested]
    pub chorus: ChorusParams,
}

#[allow(clippy::derivable_impls)]
impl Default for ThunderforgeParams {
    fn default() -> Self {
        Self {
            amp: AmpParams::default(),
            gate: GateParams::default(),
            overdrive: OverdriveParams::default(),
            compressor: CompressorParams::default(),
            eq: EqParams::default(),
            cabinet: CabinetParams::default(),
            delay: DelayParams::default(),
            reverb: ReverbParams::default(),
            chorus: ChorusParams::default(),
        }
    }
}
