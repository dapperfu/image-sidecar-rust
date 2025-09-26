/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: JSON utilities for sportball-sidecar-rust
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: serde, anyhow
 */

use anyhow::Result;
use serde_json::Value;

/// JSON utilities for sidecar operations
pub struct JsonUtils;

impl JsonUtils {
    /// Make a value JSON-serializable by converting non-serializable types
    pub fn make_serializable(value: &Value) -> Value {
        match value {
            Value::Object(map) => {
                let mut serializable_map = serde_json::Map::new();
                for (key, val) in map {
                    serializable_map.insert(key.clone(), Self::make_serializable(val));
                }
                Value::Object(serializable_map)
            }
            Value::Array(arr) => {
                let serializable_arr: Vec<Value> = arr
                    .iter()
                    .map(|val| Self::make_serializable(val))
                    .collect();
                Value::Array(serializable_arr)
            }
            _ => value.clone(),
        }
    }

    /// Validate JSON structure for sidecar files
    pub fn validate_sidecar_structure(value: &Value) -> Result<()> {
        // Check for required fields or structure
        if value.is_object() {
            // Basic validation - can be extended
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid JSON structure: expected object"))
        }
    }

    /// Extract metadata from sidecar JSON
    pub fn extract_metadata(value: &Value) -> Option<Value> {
        value.get("metadata").cloned()
    }

    /// Extract sidecar info from JSON
    pub fn extract_sidecar_info(value: &Value) -> Option<Value> {
        value.get("sidecar_info").cloned()
    }

    /// Check if JSON contains detection data
    pub fn has_detection_data(value: &Value) -> bool {
        // Check for common detection keys
        let detection_keys = ["faces", "objects", "detections", "data"];
        detection_keys.iter().any(|key| value.get(key).is_some())
    }

    /// Get file size estimate for JSON value
    pub fn estimate_size(value: &Value) -> usize {
        serde_json::to_string(value)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    /// Merge two JSON values, with the second taking precedence
    pub fn merge_values(base: &Value, overlay: &Value) -> Value {
        match (base, overlay) {
            (Value::Object(base_map), Value::Object(overlay_map)) => {
                let mut result = base_map.clone();
                for (key, value) in overlay_map {
                    result.insert(key.clone(), Self::merge_values(
                        result.get(key).unwrap_or(&Value::Null),
                        value
                    ));
                }
                Value::Object(result)
            }
            _ => overlay.clone(),
        }
    }

    /// Pretty print JSON with consistent formatting
    pub fn pretty_print(value: &Value) -> Result<String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| anyhow::anyhow!("Failed to pretty print JSON: {}", e))
    }

    /// Compact print JSON
    pub fn compact_print(value: &Value) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| anyhow::anyhow!("Failed to compact print JSON: {}", e))
    }
}
