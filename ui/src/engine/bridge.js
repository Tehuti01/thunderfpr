/**
 * LH Thunderforge - Rust ↔ WebView Bridge
 * Handles communication between the plugin UI and Rust DSP
 */

class PluginBridge {
    constructor() {
        this.paramCallbacks = new Map();
        this.initialized = false;
        
        // Listen for messages from Rust
        window.addEventListener('message', (event) => {
            this.handleMessage(event.data);
        });
    }

    /**
     * Send parameter change to Rust
     * @param {string} paramId - Parameter ID
     * @param {number|boolean} value - Normalized value (0-1) or boolean
     */
    setParam(paramId, value) {
        const message = {
            type: 'param_change',
            paramId: paramId,
            value: value
        };
        
        // In a real implementation, this would call into Rust
        // For now, we'll use postMessage as a placeholder
        console.log('[Bridge] Set Param:', paramId, '=', value);
        
        // If running in a real WebView, call the Rust bridge
        if (window.nih_plug_bridge) {
            window.nih_plug_bridge.postMessage(JSON.stringify(message));
        }
    }

    /**
     * Request current parameter value from Rust
     * @param {string} paramId - Parameter ID
     */
    getParam(paramId) {
        const message = {
            type: 'get_param',
            paramId: paramId
        };
        
        if (window.nih_plug_bridge) {
            window.nih_plug_bridge.postMessage(JSON.stringify(message));
        }
    }

    /**
     * Handle incoming messages from Rust
     * @param {Object} data - Message data
     */
    handleMessage(data) {
        try {
            const message = typeof data === 'string' ? JSON.parse(data) : data;
            
            switch (message.type) {
                case 'param_update':
                    this.handleParamUpdate(message.paramId, message.value);
                    break;
                    
                case 'preset_loaded':
                    this.handlePresetLoaded(message.preset);
                    break;
                    
                case 'meter_update':
                    this.handleMeterUpdate(message.input, message.output);
                    break;
                    
                case 'init_complete':
                    this.handleInitComplete(message.params);
                    break;
            }
        } catch (error) {
            console.error('[Bridge] Error handling message:', error);
        }
    }

    /**
     * Handle parameter update from Rust
     * @param {string} paramId - Parameter ID
     * @param {number} value - Normalized value
     */
    handleParamUpdate(paramId, value) {
        // Notify all registered callbacks
        const callbacks = this.paramCallbacks.get(paramId);
        if (callbacks) {
            callbacks.forEach(cb => cb(value));
        }

        // Update UI elements
        this.updateUIForParam(paramId, value);
    }

    /**
     * Update UI elements for a parameter change
     */
    updateUIForParam(paramId, value) {
        // Update knobs
        const knob = document.querySelector(`[data-param="${paramId}"]`);
        if (knob) {
            const knobElement = knob.querySelector('.knob');
            if (knobElement) {
                const min = parseFloat(knob.dataset.min) || 0;
                const max = parseFloat(knob.dataset.max) || 1;
                const actualValue = min + (max - min) * value;
                
                // Update knob rotation
                const angle = value * 270 - 135; // 270 degree range
                const indicator = knobElement.querySelector('.knob-indicator');
                if (indicator) {
                    indicator.style.transform = `translate(-50%, -100%) rotate(${angle}deg)`;
                }
                
                // Update arc fill
                const arcFill = knobElement.querySelector('.knob-arc-fill');
                if (arcFill) {
                    arcFill.style.setProperty('--knob-angle', `${value * 270}deg`);
                }
                
                // Update value display
                const valueDisplay = knob.querySelector('.knob-value');
                if (valueDisplay) {
                    valueDisplay.textContent = actualValue.toFixed(1);
                }
            }
        }

        // Update switches/LEDs
        const switchEl = document.querySelector(`[data-param="${paramId}"]`);
        if (switchEl && switchEl.classList.contains('stomp-switch')) {
            if (value > 0.5) {
                switchEl.classList.add('active');
            } else {
                switchEl.classList.remove('active');
            }
        }

        // Update selects
        const select = document.querySelector(`[data-param="${paramId}"]`);
        if (select && select.tagName === 'SELECT') {
            select.value = Math.round(value * (select.options.length - 1)).toString();
        }
    }

    /**
     * Register callback for parameter changes
     * @param {string} paramId - Parameter ID
     * @param {Function} callback - Callback function
     */
    onParamChange(paramId, callback) {
        if (!this.paramCallbacks.has(paramId)) {
            this.paramCallbacks.set(paramId, []);
        }
        this.paramCallbacks.get(paramId).push(callback);
    }

    /**
     * Handle preset loaded
     */
    handlePresetLoaded(preset) {
        console.log('[Bridge] Preset loaded:', preset);
        // Update preset browser UI
        const presetItems = document.querySelectorAll('.preset-item');
        presetItems.forEach(item => {
            item.classList.remove('selected');
            if (parseInt(item.dataset.preset) === preset.index) {
                item.classList.add('selected');
            }
        });
    }

    /**
     * Handle meter update
     */
    handleMeterUpdate(input, output) {
        // Update waveform display
        if (window.waveform) {
            window.waveform.update(input, output);
        }
        
        // Update LED meters
        const inputLed = document.getElementById('gate-led');
        const outputLed = document.getElementById('power-led');
        
        if (inputLed) {
            this.updateLed(inputLed, input);
        }
        if (outputLed) {
            this.updateLed(outputLed, output);
        }
    }

    /**
     * Update LED based on level
     */
    updateLed(led, level) {
        led.classList.remove('on', 'hot', 'clip');
        
        if (level > 0.95) {
            led.classList.add('clip');
        } else if (level > 0.7) {
            led.classList.add('hot');
        } else if (level > 0.1) {
            led.classList.add('on');
        }
    }

    /**
     * Handle initialization complete
     */
    handleInitComplete(params) {
        this.initialized = true;
        console.log('[Bridge] Initialization complete');
        
        // Initialize all parameters
        for (const [paramId, value] of Object.entries(params)) {
            this.updateUIForParam(paramId, value);
        }
    }
}

// Create global bridge instance
window.pluginBridge = new PluginBridge();
