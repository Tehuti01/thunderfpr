/**
 * LH Thunderforge - Waveform Visualizer
 */

class Waveform {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.buffer = [];
        this.maxBufferLength = 500;
        this.inputLevel = 0;
        this.outputLevel = 0;
        
        this.init();
    }

    init() {
        this.resize();
        window.addEventListener('resize', () => this.resize());
        this.animate();
    }

    resize() {
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width * window.devicePixelRatio;
        this.canvas.height = rect.height * window.devicePixelRatio;
        this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }

    update(input, output) {
        this.inputLevel = input;
        this.outputLevel = output;
        
        // Add to buffer
        this.buffer.push(output);
        if (this.buffer.length > this.maxBufferLength) {
            this.buffer.shift();
        }
    }

    animate() {
        this.draw();
        requestAnimationFrame(() => this.animate());
    }

    draw() {
        const width = this.canvas.width / window.devicePixelRatio;
        const height = this.canvas.height / window.devicePixelRatio;
        const ctx = this.ctx;

        // Clear
        ctx.fillStyle = '#000000';
        ctx.fillRect(0, 0, width, height);

        // Draw grid
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.02)';
        ctx.lineWidth = 1;
        
        // Vertical lines
        for (let x = 0; x < width; x += width / 10) {
            ctx.beginPath();
            ctx.moveTo(x, 0);
            ctx.lineTo(x, height);
            ctx.stroke();
        }
        
        // Horizontal lines
        for (let y = 0; y < height; y += height / 6) {
            ctx.beginPath();
            ctx.moveTo(0, y);
            ctx.lineTo(width, y);
            ctx.stroke();
        }

        // Draw center line
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
        ctx.beginPath();
        ctx.moveTo(0, height / 2);
        ctx.lineTo(width, height / 2);
        ctx.stroke();

        // Draw waveform
        if (this.buffer.length > 1) {
            const gradient = ctx.createLinearGradient(0, 0, width, 0);
            gradient.addColorStop(0, '#06b6d4');
            gradient.addColorStop(1, '#00ff78');
            
            ctx.strokeStyle = gradient;
            ctx.lineWidth = 2;
            ctx.lineCap = 'round';
            ctx.lineJoin = 'round';
            
            ctx.beginPath();
            const sliceWidth = width / this.maxBufferLength;
            let x = 0;

            for (let i = 0; i < this.buffer.length; i++) {
                const sample = this.buffer[i];
                const y = (height / 2) + (sample * height / 2 * 0.9);

                if (i === 0) {
                    ctx.moveTo(x, y);
                } else {
                    ctx.lineTo(x, y);
                }

                x += sliceWidth;
            }

            ctx.stroke();
        }

        // Draw level meter on right side
        const meterWidth = 8;
        const meterX = width - meterWidth - 4;
        
        // Background
        ctx.fillStyle = 'rgba(255, 255, 255, 0.05)';
        ctx.fillRect(meterX, 4, meterWidth, height - 8);

        // Level bar
        const levelHeight = (height - 8) * this.outputLevel;
        const gradient = ctx.createLinearGradient(0, height - 4 - levelHeight, 0, height - 4);
        
        if (this.outputLevel > 0.95) {
            gradient.addColorStop(0, '#ff3344');
            gradient.addColorStop(1, '#ff3344');
        } else if (this.outputLevel > 0.7) {
            gradient.addColorStop(0, '#ffb000');
            gradient.addColorStop(1, '#ffb000');
        } else {
            gradient.addColorStop(0, '#00ff78');
            gradient.addColorStop(1, '#00ff78');
        }
        
        ctx.fillStyle = gradient;
        ctx.fillRect(meterX, height - 4 - levelHeight, meterWidth, levelHeight);
    }
}

// Initialize waveform visualizer
document.addEventListener('DOMContentLoaded', () => {
    window.waveform = new Waveform('waveform-canvas');
});
