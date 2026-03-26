/**
 * LH Thunderforge - Model Browser & Preset Manager
 */

class ModelBrowser {
    constructor() {
        this.currentModel = 0;
        this.currentCabinet = 0;
        this.currentPreset = 0;
        
        this.init();
    }

    init() {
        // Model select
        const ampModelSelect = document.getElementById('amp-model');
        if (ampModelSelect) {
            ampModelSelect.addEventListener('change', (e) => {
                this.currentModel = parseInt(e.target.value);
                this.updateModelDisplay();
                this.sendModelChange();
            });
        }

        // Cabinet select
        const cabModelSelect = document.getElementById('cab-model');
        if (cabModelSelect) {
            cabModelSelect.addEventListener('change', (e) => {
                this.currentCabinet = parseInt(e.target.value);
                this.updateCabDisplay();
                this.sendCabChange();
            });
        }

        // Preset list
        const presetItems = document.querySelectorAll('.preset-item');
        presetItems.forEach((item, index) => {
            item.addEventListener('click', () => {
                this.loadPreset(index);
            });
        });

        // Initialize displays
        this.updateModelDisplay();
        this.updateCabDisplay();
    }

    updateModelDisplay() {
        const modelNames = [
            "PLEXI '68",
            "PLEXI BRT",
            "JCM800",
            "RECTO MOD",
            "AC30 TOP",
            "TWIN CLN",
            "5150 RED",
            "SOLDANO"
        ];
        
        const display = document.getElementById('model-display');
        if (display) {
            const text = display.querySelector('.lcd-text');
            if (text) {
                text.textContent = modelNames[this.currentModel] || "UNKNOWN";
            }
        }
        
        // Update info panel
        this.updateInfoPanel();
    }

    updateCabDisplay() {
        const cabNames = [
            "4x12 GB",
            "4x12 V30",
            "2x12 BLU",
            "1x12 OPN",
            "4x12 T75",
            "CUSTOM"
        ];
        
        const display = document.getElementById('cab-display');
        if (display) {
            const text = display.querySelector('.lcd-text');
            if (text) {
                text.textContent = cabNames[this.currentCabinet] || "UNKNOWN";
            }
        }
    }

    updateInfoPanel() {
        const modelInfo = [
            { type: 'British', gain: 'Medium', character: 'Warm' },
            { type: 'British', gain: 'High', character: 'Bright' },
            { type: 'British', gain: 'High', character: 'Aggressive' },
            { type: 'American', gain: 'Extreme', character: 'Tight' },
            { type: 'British', gain: 'Low-Med', character: 'Chimey' },
            { type: 'American', gain: 'Clean', character: 'Pristine' },
            { type: 'American', gain: 'Extreme', character: 'Modern' },
            { type: 'American', gain: 'High', character: 'Smooth' }
        ];
        
        const info = modelInfo[this.currentModel];
        if (info) {
            const typeEl = document.getElementById('info-type');
            const gainEl = document.getElementById('info-gain');
            const charEl = document.getElementById('info-character');
            
            if (typeEl) typeEl.textContent = info.type;
            if (gainEl) gainEl.textContent = info.gain;
            if (charEl) charEl.textContent = info.character;
        }
    }

    sendModelChange() {
        if (window.pluginBridge) {
            window.pluginBridge.setParam('nam_model', this.currentModel / 7);
        }
    }

    sendCabChange() {
        if (window.pluginBridge) {
            window.pluginBridge.setParam('cab_model', this.currentCabinet / 5);
        }
    }

    loadPreset(index) {
        this.currentPreset = index;
        
        // Update UI
        const presetItems = document.querySelectorAll('.preset-item');
        presetItems.forEach((item, i) => {
            if (i === index) {
                item.classList.add('selected');
            } else {
                item.classList.remove('selected');
            }
        });
        
        // Send to Rust
        if (window.pluginBridge) {
            window.pluginBridge.setParam('load_preset', index);
        }
        
        // Load preset parameters (would come from Rust)
        this.loadPresetParams(index);
    }

    loadPresetParams(index) {
        // Factory preset configurations
        const presets = [
            { // Clean Sparkle
                amp_gain: 0.3,
                eq_bass: 0.5,
                eq_mid: 0.6,
                eq_treble: 0.7,
                nam_model: 5/7,
                cab_model: 2/5
            },
            { // Highway Crunch
                amp_gain: 0.65,
                eq_bass: 0.6,
                eq_mid: 0.7,
                eq_treble: 0.6,
                nam_model: 1/7,
                cab_model: 0
            },
            { // British Invasion
                amp_gain: 0.75,
                eq_bass: 0.5,
                eq_mid: 0.8,
                eq_treble: 0.55,
                nam_model: 2/7,
                cab_model: 4/5
            },
            { // Metal Thunder
                amp_gain: 0.85,
                eq_bass: 0.4,
                eq_mid: 0.6,
                eq_treble: 0.65,
                nam_model: 6/7,
                cab_model: 1/5
            },
            { // Ambient Shimmer
                amp_gain: 0.4,
                eq_bass: 0.4,
                eq_mid: 0.5,
                eq_treble: 0.7,
                nam_model: 4/7,
                cab_model: 3/5
            },
            { // Smooth Lead
                amp_gain: 0.7,
                eq_bass: 0.5,
                eq_mid: 0.7,
                eq_treble: 0.5,
                nam_model: 7/7,
                cab_model: 0
            },
            { // Nu Metal Chunk
                amp_gain: 0.8,
                eq_bass: 0.5,
                eq_mid: 0.5,
                eq_treble: 0.7,
                nam_model: 3/7,
                cab_model: 1/5
            },
            { // Vox Jangle
                amp_gain: 0.5,
                eq_bass: 0.4,
                eq_mid: 0.6,
                eq_treble: 0.8,
                nam_model: 4/7,
                cab_model: 2/5
            }
        ];
        
        const preset = presets[index];
        if (preset && window.pluginBridge) {
            // Send all preset parameters
            for (const [paramId, value] of Object.entries(preset)) {
                window.pluginBridge.setParam(paramId, value);
            }
        }
    }
}

// Initialize model browser
document.addEventListener('DOMContentLoaded', () => {
    window.modelBrowser = new ModelBrowser();
});
