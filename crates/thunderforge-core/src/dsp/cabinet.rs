use rustfft::{Fft, FftPlanner, num_complex::Complex};
use std::sync::Arc;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::path::Path;

/// Cabinet IR convolution using partitioned FFT (overlap-add method).
#[derive(Clone)]
pub struct CabinetSim {
    ir_data: Vec<f32>,
    partition_size: usize,
    num_partitions: usize,
    // FFT plans
    fft_forward: Arc<dyn Fft<f32>>,
    fft_inverse: Arc<dyn Fft<f32>>,
    // Frequency domain IR (pre-computed)
    ir_freq: Vec<Complex<f32>>,
    // Input buffer for FFT (Complex samples)
    fft_buffer: Vec<Complex<f32>>,
    // Overlap-add buffers
    overlap_buffers: [Vec<f32>; 2], // Stereo
    overlap_pos: usize,
    // Input history for overlap-add
    input_history: Vec<f32>,
    // Bypass and mix
    mix: f32,
    bypass: bool,
    // Current model index
    current_model: usize,
}

impl CabinetSim {
    pub fn new(sample_rate: f32, max_ir_len: usize) -> Self {
        let partition_size = max_ir_len.next_power_of_two().max(1024);
        let fft_size = partition_size * 2;

        let mut planner = FftPlanner::new();
        let fft_forward = planner.plan_fft_forward(fft_size);
        let fft_inverse = planner.plan_fft_inverse(fft_size);

        let mut g = Self {
            ir_data: Vec::new(),
            partition_size,
            num_partitions: 0,
            fft_forward,
            fft_inverse,
            ir_freq: Vec::new(),
            fft_buffer: vec![Complex::new(0.0, 0.0); fft_size],
            overlap_buffers: [vec![0.0; partition_size], vec![0.0; partition_size]],
            overlap_pos: 0,
            input_history: vec![0.0; partition_size],
            mix: 1.0,
            bypass: false,
            current_model: 0,
        };
        g.set_sample_rate(sample_rate);
        g
    }

    pub fn set_sample_rate(&mut self, _sample_rate: f32) {
        // IR is already at target sample rate
        // Could resample here if needed
    }

    /// Load IR from WAV file
    pub fn load_ir_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::new(file);
        
        // Read WAV header
        let mut header = [0u8; 44];
        reader.read_exact(&mut header)?;
        
        // Verify WAV format
        if &header[0..4] != b"RIFF" || &header[8..16] != b"WAVEfmt " {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid WAV file",
            ));
        }
        
        // Get format info
        let channels = u16::from_le_bytes([header[22], header[23]]) as usize;
        let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
        let bits_per_sample = u16::from_le_bytes([header[34], header[35]]);
        
        // Read data chunk
        let mut chunk_header = [0u8; 8];
        reader.read_exact(&mut chunk_header)?;
        
        while &chunk_header[0..4] != b"data" {
            let chunk_size = u32::from_le_bytes([
                chunk_header[4],
                chunk_header[5],
                chunk_header[6],
                chunk_header[7],
            ]) as usize;
            reader.consume(chunk_size);
            reader.read_exact(&mut chunk_header)?;
        }
        
        let data_size = u32::from_le_bytes([
            chunk_header[4],
            chunk_header[5],
            chunk_header[6],
            chunk_header[7],
        ]) as usize;
        
        // Read audio data
        let num_samples = data_size / (bits_per_sample as usize / 8);
        let mut ir_samples = Vec::with_capacity(num_samples);
        
        if bits_per_sample == 16 {
            let mut buffer = vec![0u8; data_size];
            reader.read_exact(&mut buffer)?;
            
            for i in 0..num_samples {
                let sample = i16::from_le_bytes([
                    buffer[i * 2],
                    buffer[i * 2 + 1],
                ]);
                ir_samples.push(sample as f32 / 32768.0);
            }
        } else if bits_per_sample == 24 {
            let mut buffer = vec![0u8; data_size];
            reader.read_exact(&mut buffer)?;
            
            for i in 0..num_samples {
                let b0 = buffer[i * 3] as i8 as i32;
                let b1 = buffer[i * 3 + 1] as i8 as i32;
                let b2 = buffer[i * 3 + 2] as i8 as i32;
                let sample = (b0 | (b1 << 8) | (b2 << 16)) as f32 / 8388608.0;
                ir_samples.push(sample);
            }
        } else if bits_per_sample == 32 {
            let mut buffer = vec![0u8; data_size];
            reader.read_exact(&mut buffer)?;
            
            for i in 0..num_samples {
                let sample = i32::from_le_bytes([
                    buffer[i * 4],
                    buffer[i * 4 + 1],
                    buffer[i * 4 + 2],
                    buffer[i * 4 + 3],
                ]);
                ir_samples.push(sample as f32 / 2147483648.0);
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported bit depth: {}", bits_per_sample),
            ));
        }
        
        // Convert to mono if stereo
        if channels == 2 {
            let mono_len = ir_samples.len() / 2;
            for i in 0..mono_len {
                ir_samples[i] = (ir_samples[i * 2] + ir_samples[i * 2 + 1]) * 0.5;
            }
            ir_samples.truncate(mono_len);
        }
        
        log::info!("Loaded IR: {:?} ({} samples, {} bits, {} ch)", 
                   path.as_ref(), ir_samples.len(), bits_per_sample, channels);
        
        self.load_ir(&ir_samples);
        Ok(())
    }

    /// Load IR from samples
    pub fn load_ir(&mut self, ir_samples: &[f32]) {
        if ir_samples.is_empty() {
            return;
        }

        self.ir_data = ir_samples.to_vec();
        self.num_partitions = (ir_samples.len() + self.partition_size - 1) / self.partition_size;

        // Compute frequency-domain IR (first partition)
        let fft_size = self.partition_size * 2;
        self.ir_freq = Vec::with_capacity(fft_size);

        // Zero-pad first partition and convert to complex
        self.fft_buffer.fill(Complex::new(0.0, 0.0));
        let copy_len = ir_samples.len().min(self.partition_size);
        for (i, &sample) in ir_samples[..copy_len].iter().enumerate() {
            self.fft_buffer[i] = Complex::new(sample, 0.0);
        }

        // FFT
        self.fft_forward.process(&mut self.fft_buffer);

        // Store frequency response
        self.ir_freq = self.fft_buffer.clone();
    }

    pub fn set_mix(&mut self, mix_pct: f32) {
        self.mix = (mix_pct / 100.0).clamp(0.0, 1.0);
    }

    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    pub fn reset(&mut self) {
        self.fft_buffer.fill(Complex::new(0.0, 0.0));
        for buf in &mut self.overlap_buffers {
            buf.fill(0.0);
        }
        self.input_history.fill(0.0);
        self.overlap_pos = 0;
    }

    /// Process mono input → mono output (simplified single-partition)
    #[inline(always)]
    pub fn process_mono(&mut self, input: &[f32], output: &mut [f32]) {
        if self.bypass || self.ir_data.is_empty() {
            output.copy_from_slice(input);
            return;
        }

        let partition_size = self.partition_size;

        for (in_sample, out_sample) in input.iter().zip(output.iter_mut()) {
            // Input buffer
            self.input_history[self.overlap_pos] = *in_sample;

            // When buffer is full, process
            if self.overlap_pos == partition_size - 1 {
                // Prepare FFT buffer
                self.fft_buffer.fill(Complex::new(0.0, 0.0));
                for (i, &sample) in self.input_history.iter().enumerate() {
                    self.fft_buffer[i] = Complex::new(sample, 0.0);
                }

                // FFT
                self.fft_forward.process(&mut self.fft_buffer);

                // Frequency-domain multiplication
                for (buf, ir) in self.fft_buffer.iter_mut().zip(self.ir_freq.iter()) {
                    *buf *= ir;
                }

                // IFFT
                self.fft_inverse.process(&mut self.fft_buffer);

                // Overlap-add (first half contains output)
                for i in 0..partition_size {
                    self.overlap_buffers[0][i] = self.fft_buffer[i].re;
                }
                self.overlap_pos = 0;
            }

            *out_sample = self.overlap_buffers[0][self.overlap_pos] * self.mix
                + *in_sample * (1.0 - self.mix);
            self.overlap_pos += 1;
        }
    }

    /// Process mono input → stereo output
    pub fn process_mono_to_stereo(&mut self, input: &[f32], output: &mut [f32; 2]) {
        // For now, simple mono → stereo (same signal both channels)
        let mut mono_out = vec![0.0; input.len()];
        self.process_mono(input, &mut mono_out);

        for (_i, &sample) in mono_out.iter().enumerate() {
            output[0] = sample;
            output[1] = sample;
        }
    }
}

// Lazy static for FFT planning could be added for optimization
