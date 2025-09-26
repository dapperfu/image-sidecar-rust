/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Sidecar operations implementation
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: tokio, serde, anyhow
 */

use crate::sidecar::types::{OperationType, SidecarError, ValidationResult};
use anyhow::Result;
use std::path::Path;
use tokio::fs;
use serde_json::Value;

/// Sidecar operations for CRUD operations
pub struct SidecarOperations;

impl SidecarOperations {
    /// Load data from a sidecar file
    pub async fn load_data(sidecar_path: &Path) -> Result<Value> {
        if !sidecar_path.exists() {
            return Err(SidecarError::SidecarNotFound(sidecar_path.to_path_buf()).into());
        }

        let content = fs::read_to_string(sidecar_path).await?;
        let data: Value = serde_json::from_str(&content)?;
        Ok(data)
    }

    /// Save data to a sidecar file
    pub async fn save_data(sidecar_path: &Path, data: &Value) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = sidecar_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(data)?;
        fs::write(sidecar_path, content).await?;
        Ok(())
    }

    /// Merge data with existing sidecar file
    pub async fn merge_data(
        sidecar_path: &Path,
        operation_type: &OperationType,
        new_data: &Value,
    ) -> Result<()> {
        let mut existing_data = if sidecar_path.exists() {
            Self::load_data(sidecar_path).await.unwrap_or_else(|_| Value::Object(serde_json::Map::new()))
        } else {
            Value::Object(serde_json::Map::new())
        };

        // Merge the new data
        if let Some(obj) = existing_data.as_object_mut() {
            obj.insert(operation_type.as_str().to_string(), new_data.clone());

            // Update sidecar_info if it exists, otherwise create new
            if let Some(sidecar_info) = obj.get_mut("sidecar_info") {
                if let Some(sidecar_obj) = sidecar_info.as_object_mut() {
                    sidecar_obj.insert("last_updated".to_string(), 
                        serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
                    sidecar_obj.insert("last_operation".to_string(), 
                        serde_json::Value::String(operation_type.as_str().to_string()));
                }
            } else {
                let mut sidecar_info = serde_json::Map::new();
                sidecar_info.insert("operation_type".to_string(), 
                    serde_json::Value::String(operation_type.as_str().to_string()));
                sidecar_info.insert("created_at".to_string(), 
                    serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
                sidecar_info.insert("last_updated".to_string(), 
                    serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
                sidecar_info.insert("last_operation".to_string(), 
                    serde_json::Value::String(operation_type.as_str().to_string()));
                
                obj.insert("sidecar_info".to_string(), Value::Object(sidecar_info));
            }
        }

        Self::save_data(sidecar_path, &existing_data).await
    }

    /// Validate a sidecar file
    pub async fn validate_sidecar(sidecar_path: &Path) -> ValidationResult {
        let start_time = std::time::Instant::now();

        if !sidecar_path.exists() {
            return ValidationResult::error(
                sidecar_path.to_path_buf(),
                "File does not exist".to_string(),
                start_time.elapsed().as_secs_f64(),
            );
        }

        match fs::metadata(sidecar_path).await {
            Ok(metadata) => {
                let file_size = metadata.len();

                match Self::load_data(sidecar_path).await {
                    Ok(data) => {
                        let processing_time = start_time.elapsed().as_secs_f64();
                        let detection_count = Self::extract_detection_count(&data);
                        let tool_name = Self::extract_tool_name(&data);
                        let operation_type = Self::extract_operation_type(&data);

                        let mut result = ValidationResult::success(
                            sidecar_path.to_path_buf(),
                            processing_time,
                            file_size,
                        );
                        result.detection_count = detection_count;
                        result.tool_name = tool_name;
                        result.operation_type = operation_type;

                        result
                    }
                    Err(e) => ValidationResult::error(
                        sidecar_path.to_path_buf(),
                        format!("JSON decode error: {}", e),
                        start_time.elapsed().as_secs_f64(),
                    ),
                }
            }
            Err(e) => ValidationResult::error(
                sidecar_path.to_path_buf(),
                format!("File read error: {}", e),
                start_time.elapsed().as_secs_f64(),
            ),
        }
    }

    /// Extract detection count from JSON data
    fn extract_detection_count(data: &Value) -> u32 {
        // Try common detection count fields
        if let Some(count) = data.get("count").and_then(|v| v.as_u64()) {
            return count as u32;
        }

        // Check for arrays of detections
        for key in &["faces", "objects", "detections"] {
            if let Some(array) = data.get(key).and_then(|v| v.as_array()) {
                return array.len() as u32;
            }
        }

        // Check nested structures
        for key in &["data", "result", "detection"] {
            if let Some(nested) = data.get(key) {
                let nested_count = Self::extract_detection_count(nested);
                if nested_count > 0 {
                    return nested_count;
                }
            }
        }

        0
    }

    /// Extract tool name from JSON data
    fn extract_tool_name(data: &Value) -> Option<String> {
        // Try common tool name fields
        for key in &["tool_name", "detector", "model", "algorithm"] {
            if let Some(name) = data.get(key).and_then(|v| v.as_str()) {
                return Some(name.to_string());
            }
        }

        // Check nested structures
        for key in &["data", "result", "metadata"] {
            if let Some(nested) = data.get(key) {
                if let Some(name) = Self::extract_tool_name(nested) {
                    return Some(name);
                }
            }
        }

        None
    }

    /// Extract operation type from JSON data
    fn extract_operation_type(data: &Value) -> Option<OperationType> {
        // Check sidecar_info structure
        if let Some(sidecar_info) = data.get("sidecar_info") {
            if let Some(operation_str) = sidecar_info.get("operation_type").and_then(|v| v.as_str()) {
                return Some(OperationType::from_str(operation_str));
            }
        }

        // Check for detector-specific keys
        let operation_mapping = [
            ("Face_detector", OperationType::FaceDetection),
            ("Object_detector", OperationType::ObjectDetection),
            ("Ball_detector", OperationType::BallDetection),
            ("Quality_assessor", OperationType::QualityAssessment),
            ("Game_detector", OperationType::GameDetection),
            ("yolov8", OperationType::Yolov8),
        ];

        if let Some(obj) = data.as_object() {
            for (key, operation_type) in &operation_mapping {
                if obj.contains_key(*key) {
                    return Some(operation_type.clone());
                }
            }
        }

        None
    }

    /// Check if JSON data contains a specific operation type
    pub fn contains_operation_type(data: &Value, operation_type: &str) -> bool {
        // Check direct keys
        if data.get(operation_type).is_some() {
            return true;
        }

        // Check sidecar_info structure
        if let Some(sidecar_info) = data.get("sidecar_info") {
            if let Some(op_type) = sidecar_info.get("operation_type").and_then(|v| v.as_str()) {
                if op_type == operation_type {
                    return true;
                }
            }
        }

        // Check nested structures
        for key in &["data", "result"] {
            if let Some(nested) = data.get(key) {
                if Self::contains_operation_type(nested, operation_type) {
                    return true;
                }
            }
        }

        false
    }
}
