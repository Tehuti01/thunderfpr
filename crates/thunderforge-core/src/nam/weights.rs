//! Model Weights Loading and Management
//! 
//! Handles loading binary weights from .nam files and managing
//! memory layout for efficient SIMD access.

/// Model weights container
pub struct ModelWeights {
    /// Weight tensors (flattened for now)
    pub data: Vec<f32>,
    /// Weight metadata (shapes, layer assignments)
    pub layers: Vec<LayerInfo>,
}

/// Information about a weight layer
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub name: String,
    pub shape: Vec<usize>,
    pub offset: usize,
    pub size: usize,
}

impl ModelWeights {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            layers: Vec::new(),
        }
    }

    /// Create from raw weight data
    pub fn from_vec(data: Vec<f32>) -> Self {
        Self {
            data,
            layers: Vec::new(),
        }
    }

    /// Get total number of weights
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get weights as slice
    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }

    /// Get weights as mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.data
    }

    /// Add layer information
    pub fn add_layer(&mut self, name: &str, shape: Vec<usize>, offset: usize, size: usize) {
        self.layers.push(LayerInfo {
            name: name.to_string(),
            shape,
            offset,
            size,
        });
    }

    /// Get weights for a specific layer
    pub fn get_layer_weights(&self, layer_idx: usize) -> Option<&[f32]> {
        if layer_idx >= self.layers.len() {
            return None;
        }
        let layer = &self.layers[layer_idx];
        Some(&self.data[layer.offset..layer.offset + layer.size])
    }

    /// Align weights for SIMD access (32-byte alignment)
    pub fn align_for_simd(&mut self) {
        // In a real implementation, this would ensure proper memory alignment
        // for SIMD operations (AVX2, NEON, etc.)
        // For now, Vec<f32> is typically already well-aligned
    }

    /// Clear all weights
    pub fn clear(&mut self) {
        self.data.clear();
        self.layers.clear();
    }
}

impl Default for ModelWeights {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse binary weights from .nam file
/// 
/// .nam files store weights as 32-bit floats in little-endian format
pub fn parse_weights_bytes(bytes: &[u8]) -> Vec<f32> {
    if !bytes.len().is_multiple_of(4) {
        log::warn!("Weight data length is not a multiple of 4 bytes");
        return Vec::new();
    }

    let num_weights = bytes.len() / 4;
    let mut weights = Vec::with_capacity(num_weights);

    for chunk in bytes.chunks_exact(4) {
        let weight = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        weights.push(weight);
    }

    weights
}

/// Convert weights to bytes for serialization
pub fn weights_to_bytes(weights: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(weights.len() * 4);
    for &weight in weights {
        bytes.extend_from_slice(&weight.to_le_bytes());
    }
    bytes
}
