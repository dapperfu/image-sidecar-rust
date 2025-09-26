/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Python bindings for sportball-sidecar-rust using PyO3
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with PyO3 integration
 * - Dependencies: pyo3, tokio, serde, rayon, anyhow
 */

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::exceptions::PyRuntimeError;
use std::path::Path;
use std::collections::HashMap;
use serde_json::Value;
use tokio::runtime::Runtime;

use crate::{
    SportballSidecar, SidecarFormat, OperationType, SidecarInfo,
    ValidationResult, StatisticsResult
};

/// Python wrapper for SportballSidecar
#[pyclass]
pub struct PySportballSidecar {
    inner: SportballSidecar,
    runtime: Runtime,
}

#[pymethods]
impl PySportballSidecar {
    /// Create a new SportballSidecar instance
    #[new]
    pub fn new(max_workers: Option<usize>) -> PyResult<Self> {
        let runtime = Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        
        let inner = SportballSidecar::new(max_workers);
        
        Ok(Self { inner, runtime })
    }
    
    /// Validate JSON sidecar files in parallel
    pub fn validate_sidecars(&self, directory: &str) -> PyResult<Vec<PyValidationResult>> {
        let path = Path::new(directory);
        let results = self.runtime.block_on(async {
            self.inner.validate_sidecars(path).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Validation failed: {}", e)))?;
        
        Ok(results.into_iter().map(PyValidationResult::from).collect())
    }
    
    /// Get comprehensive statistics about sidecar files
    pub fn get_statistics(&self, directory: &str) -> PyResult<PyStatisticsResult> {
        let path = Path::new(directory);
        let stats = self.runtime.block_on(async {
            self.inner.get_statistics(path).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Statistics collection failed: {}", e)))?;
        
        Ok(PyStatisticsResult::from(stats))
    }
    
    /// Find all sidecar files in a directory
    pub fn find_sidecars(&self, directory: &str) -> PyResult<Vec<PySidecarInfo>> {
        let path = Path::new(directory);
        let sidecars = self.runtime.block_on(async {
            self.inner.find_sidecars(path).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Sidecar search failed: {}", e)))?;
        
        Ok(sidecars.into_iter().map(PySidecarInfo::from).collect())
    }
    
    /// Create a new sidecar file
    pub fn create_sidecar(
        &self,
        image_path: &str,
        operation: PyOperationType,
        data: &PyDict,
    ) -> PyResult<PySidecarInfo> {
        let path = Path::new(image_path);
        
        // Convert PyDict to JSON string manually
        let json_str = Python::with_gil(|py| {
            let json_module = py.import("json")?;
            let json_str = json_module.call_method1("dumps", (data,))?;
            json_str.extract::<String>()
        }).map_err(|e| PyRuntimeError::new_err(format!("Failed to convert data to JSON: {}", e)))?;
        
        let json_value: Value = serde_json::from_str(&json_str)
            .map_err(|e| PyRuntimeError::new_err(format!("Invalid JSON: {}", e)))?;
        
        let sidecar_info = self.runtime.block_on(async {
            self.inner.create_sidecar(path, operation.into(), json_value).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Sidecar creation failed: {}", e)))?;
        
        Ok(PySidecarInfo::from(sidecar_info))
    }
    
    /// Clean up orphaned sidecar files
    pub fn cleanup_orphaned(&self, directory: &str) -> PyResult<usize> {
        let path = Path::new(directory);
        let count = self.runtime.block_on(async {
            self.inner.cleanup_orphaned(path).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Cleanup failed: {}", e)))?;
        
        Ok(count)
    }
    
    /// Convert sidecar files between formats
    pub fn convert_directory_format(&self, directory: &str, target_format: PySidecarFormat) -> PyResult<u32> {
        let path = Path::new(directory);
        let count = self.runtime.block_on(async {
            self.inner.convert_directory_format(path, target_format.into()).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Format conversion failed: {}", e)))?;
        
        Ok(count)
    }
    
    /// Get format statistics for a directory
    pub fn get_format_statistics(&self, directory: &str) -> PyResult<HashMap<String, u32>> {
        let path = Path::new(directory);
        let stats = self.runtime.block_on(async {
            self.inner.get_format_statistics(path).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Format statistics failed: {}", e)))?;
        
        Ok(stats.into_iter().map(|(k, v)| (k.extension().to_string(), v)).collect())
    }
    
    /// Set the default format for new sidecar files
    pub fn set_default_format(&mut self, format: PySidecarFormat) {
        self.inner.set_default_format(format.into());
    }
    
    /// Get the current default format
    pub fn get_default_format(&self) -> PySidecarFormat {
        PySidecarFormat::from(self.inner.get_default_format())
    }
}

/// Python wrapper for SidecarFormat
#[pyclass]
#[derive(Clone, Copy)]
pub struct PySidecarFormat {
    inner: SidecarFormat,
}

impl From<SidecarFormat> for PySidecarFormat {
    fn from(format: SidecarFormat) -> Self {
        Self { inner: format }
    }
}

impl From<PySidecarFormat> for SidecarFormat {
    fn from(format: PySidecarFormat) -> Self {
        format.inner
    }
}

#[pymethods]
impl PySidecarFormat {
    #[new]
    pub fn new(format_str: &str) -> PyResult<Self> {
        let format = match format_str.to_lowercase().as_str() {
            "json" => SidecarFormat::Json,
            "bin" | "binary" => SidecarFormat::Binary,
            "rkyv" => SidecarFormat::Rkyv,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown format: {}", format_str))),
        };
        Ok(Self { inner: format })
    }
    
    fn __str__(&self) -> String {
        self.inner.extension().to_string()
    }
    
    fn __repr__(&self) -> String {
        format!("SidecarFormat('{}')", self.__str__())
    }
}

/// Python wrapper for OperationType
#[pyclass]
#[derive(Clone)]
pub struct PyOperationType {
    inner: OperationType,
}

impl From<OperationType> for PyOperationType {
    fn from(op: OperationType) -> Self {
        Self { inner: op }
    }
}

impl From<PyOperationType> for OperationType {
    fn from(op: PyOperationType) -> Self {
        op.inner
    }
}

#[pymethods]
impl PyOperationType {
    #[new]
    pub fn new(op_str: &str) -> PyResult<Self> {
        let op = match op_str.to_lowercase().as_str() {
            "face_detection" => OperationType::FaceDetection,
            "object_detection" => OperationType::ObjectDetection,
            "ball_detection" => OperationType::BallDetection,
            "quality_assessment" => OperationType::QualityAssessment,
            "game_detection" => OperationType::GameDetection,
            "yolov8" => OperationType::Yolov8,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown operation: {}", op_str))),
        };
        Ok(Self { inner: op })
    }
    
    fn __str__(&self) -> String {
        self.inner.as_str().to_string()
    }
    
    fn __repr__(&self) -> String {
        format!("OperationType('{}')", self.__str__())
    }
}

/// Python wrapper for SidecarInfo
#[pyclass]
pub struct PySidecarInfo {
    #[pyo3(get)]
    pub image_path: String,
    #[pyo3(get)]
    pub sidecar_path: String,
    #[pyo3(get)]
    pub operation: PyOperationType,
    #[pyo3(get)]
    pub data_size: u64,
    #[pyo3(get)]
    pub created_at: String,
    #[pyo3(get)]
    pub is_valid: bool,
}

impl From<SidecarInfo> for PySidecarInfo {
    fn from(info: SidecarInfo) -> Self {
        Self {
            image_path: info.image_path.to_string_lossy().to_string(),
            sidecar_path: info.sidecar_path.to_string_lossy().to_string(),
            operation: PyOperationType::from(info.operation),
            data_size: info.data_size,
            created_at: info.created_at.to_rfc3339(),
            is_valid: info.is_valid,
        }
    }
}

/// Python wrapper for ValidationResult
#[pyclass]
pub struct PyValidationResult {
    #[pyo3(get)]
    pub file_path: String,
    #[pyo3(get)]
    pub is_valid: bool,
    #[pyo3(get)]
    pub error: Option<String>,
    #[pyo3(get)]
    pub processing_time: f64,
    #[pyo3(get)]
    pub file_size: u64,
}

impl From<ValidationResult> for PyValidationResult {
    fn from(result: ValidationResult) -> Self {
        Self {
            file_path: result.file_path.to_string_lossy().to_string(),
            is_valid: result.is_valid,
            error: result.error,
            processing_time: result.processing_time,
            file_size: result.file_size,
        }
    }
}

/// Python wrapper for StatisticsResult
#[pyclass]
pub struct PyStatisticsResult {
    #[pyo3(get)]
    pub total_images: u32,
    #[pyo3(get)]
    pub total_sidecars: u32,
    #[pyo3(get)]
    pub coverage_percentage: f64,
    #[pyo3(get)]
    pub operation_counts: HashMap<String, u32>,
    #[pyo3(get)]
    pub avg_processing_times: HashMap<String, f64>,
    #[pyo3(get)]
    pub success_rate_percentages: HashMap<String, f64>,
    #[pyo3(get)]
    pub avg_data_sizes: HashMap<String, f64>,
}

impl From<StatisticsResult> for PyStatisticsResult {
    fn from(stats: StatisticsResult) -> Self {
        Self {
            total_images: stats.total_images,
            total_sidecars: stats.total_sidecars,
            coverage_percentage: stats.coverage_percentage,
            operation_counts: stats.operation_counts,
            avg_processing_times: stats.avg_processing_times,
            success_rate_percentages: stats.success_rate_percentages,
            avg_data_sizes: stats.avg_data_sizes,
        }
    }
}

/// Python module definition
#[pymodule]
fn sportball_sidecar_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySportballSidecar>()?;
    m.add_class::<PySidecarFormat>()?;
    m.add_class::<PyOperationType>()?;
    m.add_class::<PySidecarInfo>()?;
    m.add_class::<PyValidationResult>()?;
    m.add_class::<PyStatisticsResult>()?;
    
    m.add("__version__", "0.1.0")?;
    
    Ok(())
}
