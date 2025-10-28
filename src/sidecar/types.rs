/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Type definitions for sidecar operations
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: serde, chrono, uuid
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationType {
    FaceDetection,
    ObjectDetection,
    BallDetection,
    QualityAssessment,
    GameDetection,
    Yolov8,
    Unified,
    Unknown,
}

impl OperationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationType::FaceDetection => "face_detection",
            OperationType::ObjectDetection => "object_detection",
            OperationType::BallDetection => "ball_detection",
            OperationType::QualityAssessment => "quality_assessment",
            OperationType::GameDetection => "game_detection",
            OperationType::Yolov8 => "yolov8",
            OperationType::Unified => "unified",
            OperationType::Unknown => "unknown",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "face_detection" => OperationType::FaceDetection,
            "object_detection" => OperationType::ObjectDetection,
            "ball_detection" => OperationType::BallDetection,
            "quality_assessment" => OperationType::QualityAssessment,
            "game_detection" => OperationType::GameDetection,
            "yolov8" => OperationType::Yolov8,
            "unified" => OperationType::Unified,
            _ => OperationType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymlinkInfo {
    pub symlink_path: PathBuf,
    pub target_path: PathBuf,
    pub is_symlink: bool,
    pub broken: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarInfo {
    pub id: Uuid,
    pub image_path: PathBuf,
    pub sidecar_path: PathBuf,
    pub operation: OperationType,
    pub symlink_info: Option<SymlinkInfo>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub data_size: u64,
    pub is_valid: bool,
}

impl SidecarInfo {
    pub fn new(
        image_path: PathBuf,
        sidecar_path: PathBuf,
        operation: OperationType,
        symlink_info: Option<SymlinkInfo>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            image_path,
            sidecar_path,
            operation,
            symlink_info,
            created_at: now,
            last_updated: now,
            data_size: 0,
            is_valid: false,
        }
    }
    
    pub fn get_processing_time(&self) -> Option<f64> {
        // This would be extracted from the sidecar data
        // For now, return None as placeholder
        None
    }
    
    pub fn get_success_status(&self) -> bool {
        self.is_valid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub file_path: PathBuf,
    pub is_valid: bool,
    pub error: Option<String>,
    pub processing_time: f64,
    pub file_size: u64,
    pub detection_count: u32,
    pub tool_name: Option<String>,
    pub operation_type: Option<OperationType>,
}

impl ValidationResult {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            is_valid: false,
            error: None,
            processing_time: 0.0,
            file_size: 0,
            detection_count: 0,
            tool_name: None,
            operation_type: None,
        }
    }
    
    pub fn success(file_path: PathBuf, processing_time: f64, file_size: u64) -> Self {
        Self {
            file_path,
            is_valid: true,
            error: None,
            processing_time,
            file_size,
            detection_count: 0,
            tool_name: None,
            operation_type: None,
        }
    }
    
    pub fn error(file_path: PathBuf, error: String, processing_time: f64) -> Self {
        Self {
            file_path,
            is_valid: false,
            error: Some(error),
            processing_time,
            file_size: 0,
            detection_count: 0,
            tool_name: None,
            operation_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsResult {
    pub directory: PathBuf,
    pub total_images: u32,
    pub symlink_count: u32,
    pub broken_symlinks: u32,
    pub total_sidecars: u32,
    pub coverage_percentage: f64,
    pub operation_counts: HashMap<String, u32>,
    pub avg_processing_times: HashMap<String, f64>,
    pub success_rate_percentages: HashMap<String, f64>,
    pub avg_data_sizes: HashMap<String, f64>,
    pub filter_applied: Option<String>,
    pub sidecars: Vec<SidecarInfo>,
}

impl StatisticsResult {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            total_images: 0,
            symlink_count: 0,
            broken_symlinks: 0,
            total_sidecars: 0,
            coverage_percentage: 0.0,
            operation_counts: HashMap::new(),
            avg_processing_times: HashMap::new(),
            success_rate_percentages: HashMap::new(),
            avg_data_sizes: HashMap::new(),
            filter_applied: None,
            sidecars: Vec::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SidecarError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Invalid operation type: {0}")]
    InvalidOperationType(String),
    
    #[error("Sidecar file not found: {0}")]
    SidecarNotFound(PathBuf),
    
    #[error("Image file not found: {0}")]
    ImageNotFound(PathBuf),
    
    #[error("Symlink resolution failed: {0}")]
    SymlinkResolutionFailed(PathBuf),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, SidecarError>;
