/**
 * LH Thunderforge - Switch Component
 */

class Switch {
    constructor(element) {
        this.element = element;
        this.paramId = element.dataset.param;
        this.value = false;
        
        this.init();
    }

    init() {
        this.element.addEventListener('click', () => this.toggle());
    }

    toggle() {
        this.value = !this.value;
        this.updateDisplay();
        this.sendValue();
    }

    updateDisplay() {
        if (this.value) {
            this.element.classList.add('active');
        } else {
            this.element.classList.remove('active');
        }
    }

    sendValue() {
        if (window.pluginBridge && this.paramId) {
            window.pluginBridge.setParam(this.paramId, this.value ? 1 : 0);
        }
    }

    setValue(normalizedValue) {
        this.value = normalizedValue > 0.5;
        this.updateDisplay();
    }
}

// Initialize switches
document.addEventListener('DOMContentLoaded', () => {
    const switches = document.querySelectorAll('.stomp-switch');
    switches.forEach(el => new Switch(el));
});
