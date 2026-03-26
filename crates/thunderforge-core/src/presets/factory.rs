//! Factory Presets
//! 
//! Built-in preset definitions as specified in the design document.

use crate::presets::Preset;

/// Factory presets collection
pub struct FactoryPresets;

impl FactoryPresets {
    /// Get all factory presets
    pub fn all() -> Vec<Preset> {
        vec![
            Self::clean_sparkle(),
            Self::highway_crunch(),
            Self::british_invasion(),
            Self::metal_thunder(),
            Self::ambient_shimmer(),
            Self::smooth_lead(),
            Self::nu_metal_chunk(),
            Self::vox_jangle(),
        ]
    }

    /// Preset 1: Clean Sparkle
    /// Pristine Fender-style cleans with shimmer reverb
    pub fn clean_sparkle() -> Preset {
        let mut p = Preset::new("Clean Sparkle");
        p.author = "LH Factory".to_string();
        p.category = "Clean".to_string();
        p.description = "Pristine Fender-style cleans with shimmer reverb".to_string();

        // Amp
        p.set_int("nam_model", 5); // Twin Clean
        p.set_float("input_gain", 3.0);
        p.set_float("amp_gain", 3.0);

        // EQ
        p.set_float("eq_bass", 5.0);
        p.set_float("eq_mid", 6.0);
        p.set_float("eq_treble", 7.0);
        p.set_float("eq_presence", 6.0);

        // Cabinet
        p.set_int("cab_model", 2); // 2x12 Blue
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", true);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", false);
        p.set_float("chorus_rate", 0.8);
        p.set_float("chorus_depth", 30.0);
        p.set_float("chorus_mix", 20.0);
        p.set_bool("delay_bypass", true);
        p.set_bool("reverb_bypass", false);
        p.set_float("reverb_size", 60.0);
        p.set_float("reverb_decay", 3.0);
        p.set_float("reverb_mix", 30.0);

        // Master
        p.set_float("master_volume", -6.0);

        p
    }

    /// Preset 2: Highway Crunch
    /// The sound of classic rock highway anthems
    pub fn highway_crunch() -> Preset {
        let mut p = Preset::new("Highway Crunch");
        p.author = "LH Factory".to_string();
        p.category = "Classic Rock".to_string();
        p.description = "The sound of classic rock highway anthems. Crunchy, open chords.".to_string();

        // Amp
        p.set_int("nam_model", 1); // Plexi Bright
        p.set_float("input_gain", 3.0);
        p.set_float("amp_gain", 6.5);

        // EQ
        p.set_float("eq_bass", 6.0);
        p.set_float("eq_mid", 7.0);
        p.set_float("eq_treble", 6.0);
        p.set_float("eq_presence", 5.0);

        // Cabinet
        p.set_int("cab_model", 0); // 4x12 Greenback
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", true);
        p.set_bool("ts_bypass", true);
        p.set_float("ts_drive", 30.0);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", true);
        p.set_bool("delay_bypass", false);
        p.set_float("delay_time", 120.0);
        p.set_float("delay_feedback", 15.0);
        p.set_float("delay_mix", 12.0);
        p.set_bool("reverb_bypass", true);

        // Master
        p.set_float("master_volume", -6.0);

        p
    }

    /// Preset 3: British Invasion
    /// Saturated British high-gain for aggressive rock
    pub fn british_invasion() -> Preset {
        let mut p = Preset::new("British Invasion");
        p.author = "LH Factory".to_string();
        p.category = "Classic Rock".to_string();
        p.description = "Saturated British high-gain for aggressive rock".to_string();

        // Amp
        p.set_int("nam_model", 2); // JCM800
        p.set_float("input_gain", 0.0);
        p.set_float("amp_gain", 7.5);

        // EQ
        p.set_float("eq_bass", 5.0);
        p.set_float("eq_mid", 8.0);
        p.set_float("eq_treble", 5.5);
        p.set_float("eq_presence", 6.0);

        // Cabinet
        p.set_int("cab_model", 4); // 4x12 T75
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", true);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", true);
        p.set_bool("delay_bypass", true);
        p.set_bool("reverb_bypass", true);

        // Master
        p.set_float("master_volume", -4.0);

        p
    }

    /// Preset 4: Metal Thunder
    /// Extreme saturation for modern metal
    pub fn metal_thunder() -> Preset {
        let mut p = Preset::new("Metal Thunder");
        p.author = "LH Factory".to_string();
        p.category = "Metal".to_string();
        p.description = "Extreme saturation for modern metal".to_string();

        // Amp
        p.set_int("nam_model", 6); // 5150 Red
        p.set_float("input_gain", 0.0);
        p.set_float("amp_gain", 8.5);

        // EQ
        p.set_float("eq_bass", 4.0);
        p.set_float("eq_mid", 6.0);
        p.set_float("eq_treble", 6.5);
        p.set_float("eq_presence", 7.0);

        // Cabinet
        p.set_int("cab_model", 1); // 4x12 V30
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", false);
        p.set_float("gate_threshold", -40.0);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", true);
        p.set_bool("delay_bypass", true);
        p.set_bool("reverb_bypass", true);

        // Master
        p.set_float("master_volume", -3.0);

        p
    }

    /// Preset 5: Ambient Shimmer
    /// Lush, spacious tones for ambient playing
    pub fn ambient_shimmer() -> Preset {
        let mut p = Preset::new("Ambient Shimmer");
        p.author = "LH Factory".to_string();
        p.category = "Atmospheric".to_string();
        p.description = "Lush, spacious tones for ambient playing".to_string();

        // Amp
        p.set_int("nam_model", 4); // AC30 Top Boost
        p.set_float("input_gain", 0.0);
        p.set_float("amp_gain", 4.0);

        // EQ
        p.set_float("eq_bass", 4.0);
        p.set_float("eq_mid", 5.0);
        p.set_float("eq_treble", 7.0);
        p.set_float("eq_presence", 8.0);

        // Cabinet
        p.set_int("cab_model", 3); // 1x12 Open
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", true);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", false);
        p.set_float("chorus_rate", 0.5);
        p.set_float("chorus_depth", 60.0);
        p.set_float("chorus_mix", 40.0);
        p.set_bool("delay_bypass", false);
        p.set_float("delay_time", 500.0);
        p.set_float("delay_feedback", 50.0);
        p.set_float("delay_mix", 35.0);
        p.set_bool("reverb_bypass", false);
        p.set_float("reverb_size", 80.0);
        p.set_float("reverb_decay", 5.0);
        p.set_float("reverb_mix", 45.0);

        // Master
        p.set_float("master_volume", -8.0);

        p
    }

    /// Preset 6: Smooth Lead
    /// Creamy sustain for vocal-like lead lines
    pub fn smooth_lead() -> Preset {
        let mut p = Preset::new("Smooth Lead");
        p.author = "LH Factory".to_string();
        p.category = "Lead".to_string();
        p.description = "Creamy sustain for vocal-like lead lines".to_string();

        // Amp
        p.set_int("nam_model", 7); // Soldano
        p.set_float("input_gain", 0.0);
        p.set_float("amp_gain", 7.0);

        // EQ
        p.set_float("eq_bass", 5.0);
        p.set_float("eq_mid", 7.0);
        p.set_float("eq_treble", 5.0);
        p.set_float("eq_presence", 4.0);

        // Cabinet
        p.set_int("cab_model", 0); // 4x12 Greenback
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", true);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", true);
        p.set_bool("delay_bypass", false);
        p.set_float("delay_time", 375.0);
        p.set_float("delay_feedback", 35.0);
        p.set_float("delay_mix", 25.0);
        p.set_bool("reverb_bypass", false);
        p.set_float("reverb_size", 50.0);
        p.set_float("reverb_decay", 2.0);
        p.set_float("reverb_mix", 15.0);

        // Master
        p.set_float("master_volume", -5.0);

        p
    }

    /// Preset 7: Nu Metal Chunk
    /// American high-gain for drop-tuned aggression
    pub fn nu_metal_chunk() -> Preset {
        let mut p = Preset::new("Nu Metal Chunk");
        p.author = "LH Factory".to_string();
        p.category = "Metal".to_string();
        p.description = "American high-gain for drop-tuned aggression".to_string();

        // Amp
        p.set_int("nam_model", 3); // Recto Modern
        p.set_float("input_gain", 0.0);
        p.set_float("amp_gain", 8.0);

        // EQ
        p.set_float("eq_bass", 5.0);
        p.set_float("eq_mid", 5.0);
        p.set_float("eq_treble", 7.0);
        p.set_float("eq_presence", 6.0);

        // Cabinet
        p.set_int("cab_model", 1); // 4x12 V30
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", false);
        p.set_float("gate_threshold", -38.0);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", false);
        p.set_float("comp_ratio", 3.0);
        p.set_bool("chorus_bypass", true);
        p.set_bool("delay_bypass", true);
        p.set_bool("reverb_bypass", true);

        // Master
        p.set_float("master_volume", -3.0);

        p
    }

    /// Preset 8: Vox Jangle
    /// Chimey British cleans with edge-of-breakup character
    pub fn vox_jangle() -> Preset {
        let mut p = Preset::new("Vox Jangle");
        p.author = "LH Factory".to_string();
        p.category = "Clean".to_string();
        p.description = "Chimey British cleans with edge-of-breakup character".to_string();

        // Amp
        p.set_int("nam_model", 4); // AC30 Top Boost
        p.set_float("input_gain", 0.0);
        p.set_float("amp_gain", 5.0);

        // EQ
        p.set_float("eq_bass", 4.0);
        p.set_float("eq_mid", 6.0);
        p.set_float("eq_treble", 8.0);
        p.set_float("eq_presence", 7.0);

        // Cabinet
        p.set_int("cab_model", 2); // 2x12 Blue
        p.set_float("cab_mix", 100.0);

        // FX
        p.set_bool("gate_bypass", true);
        p.set_bool("ts_bypass", true);
        p.set_bool("comp_bypass", true);
        p.set_bool("chorus_bypass", false);
        p.set_float("chorus_rate", 3.0);
        p.set_float("chorus_depth", 20.0);
        p.set_float("chorus_mix", 25.0);
        p.set_bool("delay_bypass", true);
        p.set_bool("reverb_bypass", false);
        p.set_float("reverb_size", 35.0);
        p.set_float("reverb_decay", 1.0);
        p.set_float("reverb_mix", 20.0);

        // Master
        p.set_float("master_volume", -7.0);

        p
    }
}
