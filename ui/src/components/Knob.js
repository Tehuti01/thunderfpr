/**
 * LH Thunderforge - Interactive Knob Component
 * Implements spring physics and tactile interaction
 */

class Knob {
    constructor(element, options = {}) {
        this.element = element;
        this.paramId = element.dataset.param;
        this.min = parseFloat(element.dataset.min) || 0;
        this.max = parseFloat(element.dataset.max) || 1;
        this.value = 0.5; // Normalized 0-1
        this.defaultValue = 0.5;
        
        // Spring physics
        this.springStiffness = options.stiffness || 400;
        this.springDamping = options.damping || 30;
        this.springMass = options.mass || 1.0;
        this.velocity = 0;
        this.targetValue = 0.5;
        
        // Interaction state
        this.isDragging = false;
        this.lastY = 0;
        this.sensitivity = 0.005;
        
        // Detents
        this.hasDetents = options.detents || false;
        this.detentSteps = options.steps || 11;
        this.detentSnap = options.snap || false;
        
        // Initialize
        this.init();
    }

    init() {
        // Create tooltip
        this.tooltip = document.createElement('div');
        this.tooltip.className = 'knob-tooltip';
        this.element.appendChild(this.tooltip);

        // Bind events
        this.element.addEventListener('mousedown', (e) => this.onMouseDown(e));
        document.addEventListener('mousemove', (e) => this.onMouseMove(e));
        document.addEventListener('mouseup', () => this.onMouseUp());
        
        // Touch support
        this.element.addEventListener('touchstart', (e) => this.onTouchStart(e));
        document.addEventListener('touchmove', (e) => this.onTouchMove(e));
        document.addEventListener('touchend', () => this.onTouchEnd());
        
        // Double-click to reset
        this.element.addEventListener('dblclick', () => this.reset());
        
        // Mouse wheel
        this.element.addEventListener('wheel', (e) => this.onWheel(e));
        
        // Initial render
        this.updateDisplay();
    }

    onMouseDown(e) {
        if (e.button !== 0) return; // Only left click
        
        e.preventDefault();
        this.isDragging = true;
        this.lastY = e.clientY;
        this.element.classList.add('dragging');
        
        // Capture pointer for smooth dragging
        if (this.element.setPointerCapture) {
            this.element.setPointerCapture(e.pointerId || 0);
        }
    }

    onMouseMove(e) {
        if (!this.isDragging) return;
        
        e.preventDefault();
        const deltaY = this.lastY - e.clientY;
        this.lastY = e.clientY;
        
        // Update target value with spring physics
        const deltaValue = deltaY * this.sensitivity;
        this.targetValue = Math.max(0, Math.min(1, this.targetValue + deltaValue));
        
        this.updateDisplay();
        this.sendValue();
    }

    onMouseUp() {
        this.isDragging = false;
        this.element.classList.remove('dragging');
        
        // Snap to detent if enabled
        if (this.detentSnap && this.hasDetents) {
            this.snapToDetent();
        }
        
        // Release pointer capture
        if (this.element.releasePointerCapture) {
            this.element.releasePointerCapture(0);
        }
    }

    onTouchStart(e) {
        if (e.touches.length === 1) {
            e.preventDefault();
            this.isDragging = true;
            this.lastY = e.touches[0].clientY;
            this.element.classList.add('dragging');
        }
    }

    onTouchMove(e) {
        if (!this.isDragging || e.touches.length !== 1) return;
        
        e.preventDefault();
        const deltaY = this.lastY - e.touches[0].clientY;
        this.lastY = e.touches[0].clientY;
        
        const deltaValue = deltaY * this.sensitivity;
        this.targetValue = Math.max(0, Math.min(1, this.targetValue + deltaValue));
        
        this.updateDisplay();
        this.sendValue();
    }

    onTouchEnd() {
        this.onMouseUp();
    }

    onWheel(e) {
        e.preventDefault();
        
        const delta = e.deltaY > 0 ? -1 : 1;
        const deltaValue = delta * this.sensitivity * 10;
        this.targetValue = Math.max(0, Math.min(1, this.targetValue + deltaValue));
        
        this.updateDisplay();
        this.sendValue();
    }

    /**
     * Apply spring physics animation
     */
    updatePhysics() {
        if (Math.abs(this.targetValue - this.value) < 0.001) {
            this.value = this.targetValue;
            return;
        }
        
        // Spring force
        const springForce = -this.springStiffness * (this.value - this.targetValue);
        const dampingForce = -this.springDamping * this.velocity;
        const acceleration = (springForce + dampingForce) / this.springMass;
        
        this.velocity += acceleration * 0.016; // Assume 60fps
        this.value += this.velocity * 0.016;
        
        // Clamp
        this.value = Math.max(0, Math.min(1, this.value));
        
        this.updateDisplay();
    }

    /**
     * Update visual display
     */
    updateDisplay() {
        // Calculate actual value
        const actualValue = this.min + (this.max - this.min) * this.value;
        
        // Update indicator rotation
        const indicator = this.element.querySelector('.knob-indicator');
        if (indicator) {
            const angle = this.value * 270 - 135; // 270 degree range
            indicator.style.transform = `translate(-50%, -100%) rotate(${angle}deg)`;
        }
        
        // Update arc fill
        const arcFill = this.element.querySelector('.knob-arc-fill');
        if (arcFill) {
            arcFill.style.setProperty('--knob-angle', `${this.value * 270}deg`);
        }
        
        // Update tooltip
        if (this.tooltip) {
            this.tooltip.textContent = actualValue.toFixed(1);
        }
        
        // Update value display
        const valueDisplay = this.element.querySelector('.knob-value');
        if (valueDisplay) {
            valueDisplay.textContent = actualValue.toFixed(1);
        }
    }

    /**
     * Send value to Rust via bridge
     */
    sendValue() {
        if (window.pluginBridge && this.paramId) {
            window.pluginBridge.setParam(this.paramId, this.value);
        }
    }

    /**
     * Snap to nearest detent
     */
    snapToDetent() {
        const stepSize = 1 / (this.detentSteps - 1);
        const nearestStep = Math.round(this.value / stepSize);
        this.targetValue = nearestStep * stepSize;
        this.updateDisplay();
        this.sendValue();
    }

    /**
     * Reset to default value
     */
    reset() {
        this.targetValue = this.defaultValue;
        this.velocity = -0.5; // Add some bounce
        this.updateDisplay();
        this.sendValue();
    }

    /**
     * Set value from external source (Rust)
     */
    setValue(normalizedValue) {
        this.targetValue = normalizedValue;
        this.updateDisplay();
    }

    /**
     * Start animation loop
     */
    startAnimation() {
        const animate = () => {
            this.updatePhysics();
            requestAnimationFrame(animate);
        };
        animate();
    }
}

// Initialize all knobs on page load
document.addEventListener('DOMContentLoaded', () => {
    const knobElements = document.querySelectorAll('.knob-container .knob');
    const knobs = [];
    
    knobElements.forEach(el => {
        const knob = new Knob(el);
        knobs.push(knob);
        knob.startAnimation();
    });
    
    window.knobs = knobs;
});
