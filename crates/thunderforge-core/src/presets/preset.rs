//! Preset Data Structure
//! 
//! Defines the preset format for saving and loading plugin states.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Preset data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// Preset name
    pub name: String,
    /// Preset author
    #[serde(default)]
    pub author: String,
    /// Category (Clean, Crunch, High Gain, etc.)
    #[serde(default)]
    pub category: String,
    /// Description
    #[serde(default)]
    pub description: String,
    /// Version
    #[serde(default)]
    pub version: String,
    /// Parameter values
    pub parameters: HashMap<String, f32>,
    /// Boolean parameter values
    #[serde(default)]
    pub bool_parameters: HashMap<String, bool>,
    /// Integer parameter values (model selections)
    #[serde(default)]
    pub int_parameters: HashMap<String, i32>,
}

impl Preset {
    /// Create a new preset
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            author: String::new(),
            category: String::new(),
            description: String::new(),
            version: "1.0".to_string(),
            parameters: HashMap::new(),
            bool_parameters: HashMap::new(),
            int_parameters: HashMap::new(),
        }
    }

    /// Create from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get a float parameter value
    pub fn get_float(&self, id: &str, default: f32) -> f32 {
        *self.parameters.get(id).unwrap_or(&default)
    }

    /// Get a boolean parameter value
    pub fn get_bool(&self, id: &str, default: bool) -> bool {
        *self.bool_parameters.get(id).unwrap_or(&default)
    }

    /// Get an integer parameter value
    pub fn get_int(&self, id: &str, default: i32) -> i32 {
        *self.int_parameters.get(id).unwrap_or(&default)
    }

    /// Set a float parameter
    pub fn set_float(&mut self, id: &str, value: f32) {
        self.parameters.insert(id.to_string(), value);
    }

    /// Set a boolean parameter
    pub fn set_bool(&mut self, id: &str, value: bool) {
        self.bool_parameters.insert(id.to_string(), value);
    }

    /// Set an integer parameter
    pub fn set_int(&mut self, id: &str, value: i32) {
        self.int_parameters.insert(id.to_string(), value);
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self::new("Untitled")
    }
}
