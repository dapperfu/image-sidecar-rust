/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Parallel processor implementation for sportball-sidecar-rust
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: tokio, rayon, anyhow
 */

use crate::sidecar::types::{ValidationResult, OperationType};
use crate::sidecar::formats::{SidecarFormat, FormatManager};
use anyhow::Result;
use rayon::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use walkdir::WalkDir;

/// Parallel processor for high-performance sidecar operations
pub struct ParallelProcessor {
    max_workers: usize,
}

impl ParallelProcessor {
    /// Create a new ParallelProcessor instance
    pub fn new(max_workers: usize) -> Self {
        Self { max_workers }
    }

    /// Validate all sidecar files in a directory in parallel
    pub async fn validate_directory(&self, directory: &Path) -> Result<Vec<ValidationResult>> {
        let sidecar_files = self.find_sidecar_files(directory).await?;
        self.validate_files_parallel(&sidecar_files).await
    }

    /// Validate multiple sidecar files in parallel
    pub async fn validate_files_parallel(&self, file_paths: &[std::path::PathBuf]) -> Result<Vec<ValidationResult>> {
        if file_paths.is_empty() {
            return Ok(Vec::new());
        }

        // Use rayon for parallel processing
        let results: Vec<ValidationResult> = file_paths
            .par_iter()
            .map(|path| {
                let start_time = std::time::Instant::now();
                
                if !path.exists() {
                    return ValidationResult::error(
                        path.clone(),
                        "File does not exist".to_string(),
                        start_time.elapsed().as_secs_f64(),
                    );
                }

                match std::fs::metadata(path) {
                    Ok(metadata) => {
                        let file_size = metadata.len();

                        match std::fs::read(path) {
                            Ok(content_bytes) => {
                                // Use format manager to deserialize
                                let format_manager = FormatManager::new();
                                
                                // Detect format from file extension first
                                let format = SidecarFormat::from_path(path)
                                    .unwrap_or(SidecarFormat::Json);
                                
                                match format_manager.get_serializer(format).deserialize(&content_bytes) {
                                    Ok(data) => {
                                        let processing_time = start_time.elapsed().as_secs_f64();
                                        let detection_count = self.extract_detection_count(&data);
                                        let tool_name = self.extract_tool_name(&data);
                                        let operation_type = self.extract_operation_type(&data);

                                        let mut result = ValidationResult::success(
                                            path.clone(),
                                            processing_time,
                                            file_size,
                                        );
                                        result.detection_count = detection_count;
                                        result.tool_name = tool_name;
                                        result.operation_type = operation_type;

                                        result
                                    }
                                    Err(e) => ValidationResult::error(
                                        path.clone(),
                                        format!("Deserialization error: {}", e),
                                        start_time.elapsed().as_secs_f64(),
                                    ),
                                }
                            }
                            Err(e) => ValidationResult::error(
                                path.clone(),
                                format!("File read error: {}", e),
                                start_time.elapsed().as_secs_f64(),
                            ),
                        }
                    }
                    Err(e) => ValidationResult::error(
                        path.clone(),
                        format!("File metadata error: {}", e),
                        start_time.elapsed().as_secs_f64(),
                    ),
                }
            })
            .collect();

        Ok(results)
    }

    /// Filter sidecar files by operation type in parallel
    pub async fn filter_by_operation_type(
        &self,
        file_paths: &[std::path::PathBuf],
        operation_type: &str,
    ) -> Result<Vec<std::path::PathBuf>> {
        let filtered: Vec<std::path::PathBuf> = file_paths
            .par_iter()
            .filter(|path| {
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        match serde_json::from_str::<serde_json::Value>(&content) {
                            Ok(data) => self.contains_operation_type(&data, operation_type),
                            Err(_) => true, // Include files that can't be parsed for validation
                        }
                    }
                    Err(_) => true, // Include files that can't be read for validation
                }
            })
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Get validation statistics from results
    pub fn get_validation_statistics(&self, results: &[ValidationResult]) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        
        let total_files = results.len();
        let valid_files = results.iter().filter(|r| r.is_valid).count();
        let invalid_files = total_files - valid_files;
        
        let total_processing_time: f64 = results.iter().map(|r| r.processing_time).sum();
        let total_file_size: u64 = results.iter().map(|r| r.file_size).sum();
        
        let avg_processing_time = if total_files > 0 {
            total_processing_time / total_files as f64
        } else {
            0.0
        };
        
        let avg_file_size = if total_files > 0 {
            total_file_size as f64 / total_files as f64
        } else {
            0.0
        };

        stats.insert("total_files".to_string(), serde_json::Value::Number(serde_json::Number::from(total_files)));
        stats.insert("valid_files".to_string(), serde_json::Value::Number(serde_json::Number::from(valid_files)));
        stats.insert("invalid_files".to_string(), serde_json::Value::Number(serde_json::Number::from(invalid_files)));
        stats.insert("valid_percentage".to_string(), serde_json::Value::Number(serde_json::Number::from_f64((valid_files as f64 / total_files as f64) * 100.0).unwrap()));
        stats.insert("invalid_percentage".to_string(), serde_json::Value::Number(serde_json::Number::from_f64((invalid_files as f64 / total_files as f64) * 100.0).unwrap()));
        stats.insert("total_processing_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(total_processing_time).unwrap()));
        stats.insert("avg_processing_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(avg_processing_time).unwrap()));
        stats.insert("total_file_size".to_string(), serde_json::Value::Number(serde_json::Number::from(total_file_size)));
        stats.insert("avg_file_size".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(avg_file_size).unwrap()));

        // Group by operation type
        let mut operation_stats = HashMap::new();
        for result in results {
            if let Some(op_type) = &result.operation_type {
                let op_str = op_type.as_str();
                let entry = operation_stats.entry(op_str.to_string()).or_insert((0, 0, 0.0, 0u64));
                entry.0 += 1; // count
                if result.is_valid {
                    entry.1 += 1; // valid count
                }
                entry.2 += result.processing_time; // total time
                entry.3 += result.file_size; // total size
            }
        }

        let mut operation_stats_json = serde_json::Map::new();
        for (op_type, (count, valid_count, total_time, total_size)) in operation_stats {
            let mut op_stat = serde_json::Map::new();
            op_stat.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(count)));
            op_stat.insert("valid_count".to_string(), serde_json::Value::Number(serde_json::Number::from(valid_count)));
            op_stat.insert("success_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64((valid_count as f64 / count as f64) * 100.0).unwrap()));
            op_stat.insert("avg_processing_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(total_time / count as f64).unwrap()));
            op_stat.insert("avg_file_size".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(total_size as f64 / count as f64).unwrap()));
            
            operation_stats_json.insert(op_type, serde_json::Value::Object(op_stat));
        }

        stats.insert("operation_stats".to_string(), serde_json::Value::Object(operation_stats_json));

        stats
    }

    // Private helper methods

    async fn find_sidecar_files(&self, directory: &Path) -> Result<Vec<std::path::PathBuf>> {
        let mut sidecar_files = Vec::new();

        for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy().to_lowercase();
                    // Look for all supported sidecar formats
                    if matches!(ext_str.as_str(), "json" | "bin" | "rkyv") {
                        sidecar_files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(sidecar_files)
    }

    fn extract_detection_count(&self, data: &serde_json::Value) -> u32 {
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
                let nested_count = self.extract_detection_count(nested);
                if nested_count > 0 {
                    return nested_count;
                }
            }
        }

        0
    }

    fn extract_tool_name(&self, data: &serde_json::Value) -> Option<String> {
        // Try common tool name fields
        for key in &["tool_name", "detector", "model", "algorithm"] {
            if let Some(name) = data.get(key).and_then(|v| v.as_str()) {
                return Some(name.to_string());
            }
        }

        // Check nested structures
        for key in &["data", "result", "metadata"] {
            if let Some(nested) = data.get(key) {
                if let Some(name) = self.extract_tool_name(nested) {
                    return Some(name);
                }
            }
        }

        None
    }

    fn extract_operation_type(&self, data: &serde_json::Value) -> Option<OperationType> {
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

    fn contains_operation_type(&self, data: &serde_json::Value, operation_type: &str) -> bool {
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
                if self.contains_operation_type(nested, operation_type) {
                    return true;
                }
            }
        }

        false
    }
}
