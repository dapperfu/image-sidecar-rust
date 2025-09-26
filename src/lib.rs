/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Core library interface for sportball-sidecar-rust
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: tokio, serde, rayon, clap, anyhow, pyo3
 */

pub mod sidecar;
pub mod parallel;
pub mod utils;

#[cfg(feature = "python")]
pub mod python;

pub use sidecar::{
    SidecarManager, SidecarInfo, OperationType, SidecarError,
    ValidationResult, StatisticsResult, SidecarFormat, FormatManager
};
pub use parallel::ParallelProcessor;
pub use utils::json::JsonUtils;

use anyhow::Result;
use std::path::Path;

/// Main entry point for sidecar operations
pub struct SportballSidecar {
    manager: SidecarManager,
    processor: ParallelProcessor,
}

impl SportballSidecar {
    /// Create a new SportballSidecar instance
    pub fn new(max_workers: Option<usize>) -> Self {
        let manager = SidecarManager::new();
        let processor = ParallelProcessor::new(max_workers.unwrap_or_else(|| {
            std::thread::available_parallelism().map(|n| n.get()).unwrap_or(16)
        }));
        
        Self { manager, processor }
    }
    
    /// Validate JSON sidecar files in parallel
    pub async fn validate_sidecars(&self, directory: &Path) -> Result<Vec<ValidationResult>> {
        self.processor.validate_directory(directory).await
    }
    
    /// Get comprehensive statistics about sidecar files
    pub async fn get_statistics(&self, directory: &Path) -> Result<StatisticsResult> {
        self.manager.get_statistics(directory).await
    }
    
    /// Find all sidecar files in a directory
    pub async fn find_sidecars(&self, directory: &Path) -> Result<Vec<SidecarInfo>> {
        self.manager.find_all_sidecars(directory).await
    }
    
    /// Create a new sidecar file
    pub async fn create_sidecar(
        &self,
        image_path: &Path,
        operation: OperationType,
        data: serde_json::Value,
    ) -> Result<SidecarInfo> {
        self.manager.create_sidecar(image_path, operation, data).await
    }
    
    /// Clean up orphaned sidecar files
    pub async fn cleanup_orphaned(&self, directory: &Path) -> Result<usize> {
        self.manager.cleanup_orphaned_sidecars(directory).await
    }
    
    /// Convert sidecar files between formats
    pub async fn convert_directory_format(&self, directory: &Path, target_format: SidecarFormat) -> Result<u32> {
        self.manager.convert_directory_format(directory, target_format).await
    }
    
    /// Get format statistics for a directory
    pub async fn get_format_statistics(&self, directory: &Path) -> Result<std::collections::HashMap<SidecarFormat, u32>> {
        self.manager.get_format_statistics(directory).await
    }
    
    /// Set the default format for new sidecar files
    pub fn set_default_format(&mut self, format: SidecarFormat) {
        self.manager.set_default_format(format);
    }
    
    /// Get the current default format
    pub fn get_default_format(&self) -> SidecarFormat {
        self.manager.get_default_format()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[tokio::test]
    async fn test_sidecar_creation() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test.jpg");
        fs::write(&image_path, b"fake image data").unwrap();
        
        let sidecar = SportballSidecar::new(None);
        let data = serde_json::json!({"test": "data"});
        
        let result = sidecar.create_sidecar(&image_path, OperationType::FaceDetection, data).await;
        assert!(result.is_ok());
        
        let sidecar_info = result.unwrap();
        assert!(sidecar_info.sidecar_path.exists());
    }
    
    #[tokio::test]
    async fn test_binary_format_support() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test.jpg");
        fs::write(&image_path, b"fake image data").unwrap();
        
        let mut sidecar = SportballSidecar::new(None);
        
        // Test creating sidecar with binary format
        sidecar.set_default_format(SidecarFormat::Binary);
        let data = serde_json::json!({
            "face_detection": {
                "success": true,
                "faces": [{"confidence": 0.95}]
            }
        });
        
        let result = sidecar.create_sidecar(&image_path, OperationType::FaceDetection, data).await;
        assert!(result.is_ok());
        
        let sidecar_info = result.unwrap();
        assert!(sidecar_info.sidecar_path.exists());
        assert_eq!(sidecar_info.sidecar_path.extension().unwrap(), "bin");
        
        // Test finding the sidecar (should find .bin file)
        let found_sidecar = sidecar.find_sidecars(temp_dir.path()).await.unwrap();
        assert_eq!(found_sidecar.len(), 1);
        assert_eq!(found_sidecar[0].sidecar_path.extension().unwrap(), "bin");
    }
    
    #[tokio::test]
    async fn test_format_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test.jpg");
        fs::write(&image_path, b"fake image data").unwrap();
        
        let mut sidecar = SportballSidecar::new(None);
        
        // Set default format to JSON for this test
        sidecar.set_default_format(SidecarFormat::Json);
        let data = serde_json::json!({"test": "data"});
        
        // Create JSON sidecar
        let json_result = sidecar.create_sidecar(&image_path, OperationType::FaceDetection, data).await;
        assert!(json_result.is_ok());
        
        let json_sidecar = json_result.unwrap();
        assert_eq!(json_sidecar.sidecar_path.extension().unwrap(), "json");
        
        // Convert to binary format
        let converted_count = sidecar.convert_directory_format(temp_dir.path(), SidecarFormat::Binary).await.unwrap();
        assert_eq!(converted_count, 1);
        
        // Verify conversion
        let found_sidecars = sidecar.find_sidecars(temp_dir.path()).await.unwrap();
        assert_eq!(found_sidecars.len(), 1);
        assert_eq!(found_sidecars[0].sidecar_path.extension().unwrap(), "bin");
    }
}
