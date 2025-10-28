/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Core sidecar manager implementation
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: tokio, serde, rayon, anyhow
 */

use crate::sidecar::types::{
    SidecarInfo, OperationType, SidecarError, StatisticsResult, SymlinkInfo
};
use crate::sidecar::formats::{SidecarFormat, FormatManager};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;
use chrono::Utc;
use serde_json::Value;

/// Core sidecar manager for handling sidecar files in multiple formats
pub struct SidecarManager {
    image_extensions: Vec<String>,
    operation_mapping: HashMap<String, OperationType>,
    format_manager: FormatManager,
    default_format: SidecarFormat,
}

impl SidecarManager {
    /// Create a new SidecarManager instance
    pub fn new() -> Self {
        let mut operation_mapping = HashMap::new();
        operation_mapping.insert("Face_detector".to_string(), OperationType::FaceDetection);
        operation_mapping.insert("Object_detector".to_string(), OperationType::ObjectDetection);
        operation_mapping.insert("Ball_detector".to_string(), OperationType::BallDetection);
        operation_mapping.insert("Quality_assessor".to_string(), OperationType::QualityAssessment);
        operation_mapping.insert("Game_detector".to_string(), OperationType::GameDetection);
        operation_mapping.insert("yolov8".to_string(), OperationType::Yolov8);

        Self {
            image_extensions: vec![
                "jpg".to_string(), "jpeg".to_string(), "png".to_string(),
                "tiff".to_string(), "bmp".to_string(), "webp".to_string()
            ],
            operation_mapping,
            format_manager: FormatManager::new(),
            default_format: SidecarFormat::default(),
        }
    }

    /// Find sidecar file for a given image path
    /// Priority: .bin -> .rkyv -> .json (most efficient to least efficient)
    pub async fn find_sidecar_for_image(&self, image_path: &Path) -> Result<Option<SidecarInfo>> {
        if !image_path.exists() {
            return Ok(None);
        }

        // Resolve symlink if needed
        let (actual_image_path, symlink_info) = self.resolve_symlink(image_path).await?;

        // Try formats in order of efficiency: bin -> rkyv -> json
        let formats_to_try = [SidecarFormat::Binary, SidecarFormat::Rkyv, SidecarFormat::Json];
        
        for format in &formats_to_try {
            let sidecar_path = actual_image_path.with_extension(format.extension());
            
            if sidecar_path.exists() {
                let operation = self.detect_operation_type(&sidecar_path).await?;
                let mut sidecar_info = SidecarInfo::new(
                    image_path.to_path_buf(),
                    sidecar_path,
                    operation,
                    symlink_info,
                );
                
                // Load and validate the sidecar
                if let Ok(data) = self.load_sidecar_data(&sidecar_info.sidecar_path).await {
                    sidecar_info.data_size = data.to_string().len() as u64;
                    sidecar_info.is_valid = true;
                }

                return Ok(Some(sidecar_info));
            }
        }

        Ok(None)
    }

    /// Find all sidecar files in a directory
    pub async fn find_all_sidecars(&self, directory: &Path) -> Result<Vec<SidecarInfo>> {
        let mut sidecars = Vec::new();
        let mut processed_sidecars = std::collections::HashSet::new();

        // Find all image files
        let image_files = self.find_image_files(directory).await?;

        // Process each image file
        for image_file in image_files {
            if let Some(sidecar_info) = self.find_sidecar_for_image(&image_file).await? {
                if processed_sidecars.insert(sidecar_info.sidecar_path.clone()) {
                    sidecars.push(sidecar_info);
                }
            }
        }

        // Also look for pattern-based sidecars
        let pattern_sidecars = self.find_pattern_sidecars(directory).await?;
        for sidecar_info in pattern_sidecars {
            if processed_sidecars.insert(sidecar_info.sidecar_path.clone()) {
                sidecars.push(sidecar_info);
            }
        }

        Ok(sidecars)
    }

    /// Create a new sidecar file for an image using the default format
    pub async fn create_sidecar(
        &self,
        image_path: &Path,
        operation: OperationType,
        data: Value,
    ) -> Result<SidecarInfo> {
        self.create_sidecar_with_format(image_path, operation, data, self.default_format).await
    }

    /// Save data to a sidecar file, merging with existing data if present
    /// This is the primary method expected by sportball Python code
    pub async fn save_data(
        &self,
        image_path: &Path,
        operation: OperationType,
        data: Value,
    ) -> Result<SidecarInfo> {
        // Resolve symlink if needed
        let (actual_image_path, symlink_info) = self.resolve_symlink(image_path).await?;

        // Create sidecar path next to actual image with binary format
        let sidecar_path = actual_image_path.with_extension("bin");

        // Load existing data if sidecar exists, otherwise start with empty
        let mut existing_data = if sidecar_path.exists() {
            self.load_sidecar_data(&sidecar_path).await.unwrap_or_else(|_| Value::Object(serde_json::Map::new()))
        } else {
            Value::Object(serde_json::Map::new())
        };

        // Merge the new data into existing data
        if let Some(obj) = existing_data.as_object_mut() {
            // Insert or update the operation data
            obj.insert(operation.as_str().to_string(), data);

            // Update sidecar_info if it exists, otherwise create new
            if let Some(sidecar_info) = obj.get_mut("sidecar_info") {
                if let Some(sidecar_obj) = sidecar_info.as_object_mut() {
                    sidecar_obj.insert("last_updated".to_string(), 
                        serde_json::Value::String(Utc::now().to_rfc3339()));
                    sidecar_obj.insert("last_operation".to_string(), 
                        serde_json::Value::String(operation.as_str().to_string()));
                }
            } else {
                let mut sidecar_info = serde_json::Map::new();
                sidecar_info.insert("created_at".to_string(), 
                    serde_json::Value::String(Utc::now().to_rfc3339()));
                sidecar_info.insert("last_updated".to_string(), 
                    serde_json::Value::String(Utc::now().to_rfc3339()));
                sidecar_info.insert("last_operation".to_string(), 
                    serde_json::Value::String(operation.as_str().to_string()));
                sidecar_info.insert("image_path".to_string(), 
                    serde_json::Value::String(actual_image_path.to_string_lossy().to_string()));
                sidecar_info.insert("symlink_path".to_string(), 
                    serde_json::Value::String(image_path.to_string_lossy().to_string()));
                
                // Serialize symlink_info if present
                if let Some(symlink) = &symlink_info {
                    sidecar_info.insert("symlink_info".to_string(), serde_json::json!({
                        "symlink_path": symlink.symlink_path.to_string_lossy(),
                        "target_path": symlink.target_path.to_string_lossy(),
                        "is_symlink": symlink.is_symlink,
                        "broken": symlink.broken
                    }));
                }
                
                obj.insert("sidecar_info".to_string(), Value::Object(sidecar_info));
            }
        }

        // Serialize using binary format
        let serializer = self.format_manager.get_serializer(SidecarFormat::Binary);
        let content_bytes = serializer.serialize(&existing_data)
            .map_err(|e| SidecarError::SerializationError(e.to_string()))?;
        
        fs::write(&sidecar_path, &content_bytes).await?;

        let mut sidecar_info = SidecarInfo::new(
            image_path.to_path_buf(),
            sidecar_path.clone(),
            operation,
            symlink_info,
        );
        sidecar_info.data_size = content_bytes.len() as u64;
        sidecar_info.is_valid = true;

        Ok(sidecar_info)
    }

    /// Create a new sidecar file for an image with a specific format
    pub async fn create_sidecar_with_format(
        &self,
        image_path: &Path,
        operation: OperationType,
        data: Value,
        format: SidecarFormat,
    ) -> Result<SidecarInfo> {
        // Resolve symlink if needed
        let (actual_image_path, symlink_info) = self.resolve_symlink(image_path).await?;

        // Create sidecar path next to actual image with the specified format
        let sidecar_path = actual_image_path.with_extension(format.extension());

        // Add metadata to data
        let mut enhanced_data = serde_json::Map::new();
        enhanced_data.insert("sidecar_info".to_string(), serde_json::json!({
            "operation_type": operation.as_str(),
            "created_at": Utc::now().to_rfc3339(),
            "image_path": actual_image_path.to_string_lossy(),
            "symlink_path": image_path.to_string_lossy(),
            "symlink_info": symlink_info
        }));
        enhanced_data.insert("data".to_string(), data);

        // Serialize using the specified format
        let serializer = self.format_manager.get_serializer(format);
        let content_bytes = serializer.serialize(&serde_json::Value::Object(enhanced_data))
            .map_err(|e| SidecarError::SerializationError(e.to_string()))?;
        
        fs::write(&sidecar_path, &content_bytes).await?;

        let mut sidecar_info = SidecarInfo::new(
            image_path.to_path_buf(),
            sidecar_path.clone(),
            operation,
            symlink_info,
        );
        sidecar_info.data_size = content_bytes.len() as u64;
        sidecar_info.is_valid = true;

        Ok(sidecar_info)
    }

    /// Get comprehensive statistics about sidecar files in a directory
    pub async fn get_statistics(&self, directory: &Path) -> Result<StatisticsResult> {
        let mut stats = StatisticsResult::new(directory.to_path_buf());
        let sidecars = self.find_all_sidecars(directory).await?;

        // Count images (including symlinks)
        let image_files = self.find_image_files(directory).await?;
        let mut symlink_count = 0;
        let mut broken_symlinks = 0;

        for image_file in &image_files {
            if image_file.is_symlink() {
                symlink_count += 1;
                if let Ok(metadata) = fs::symlink_metadata(image_file).await {
                    if metadata.file_type().is_symlink() {
                        if !image_file.exists() {
                            broken_symlinks += 1;
                        }
                    }
                }
            }
        }

        // Analyze sidecars
        let mut operation_counts = HashMap::new();
        let mut processing_times = HashMap::new();
        let mut success_rates = HashMap::new();
        let mut data_sizes = HashMap::new();

        for sidecar in &sidecars {
            let operation = sidecar.operation.as_str().to_string();

            // Count operations
            *operation_counts.entry(operation.clone()).or_insert(0) += 1;

            // Collect processing times
            if let Some(proc_time) = sidecar.get_processing_time() {
                processing_times.entry(operation.clone()).or_insert_with(Vec::new).push(proc_time);
            }

            // Collect success rates
            let success = sidecar.get_success_status();
            let rates = success_rates.entry(operation.clone()).or_insert((0, 0));
            rates.1 += 1;
            if success {
                rates.0 += 1;
            }

            // Collect data sizes
            data_sizes.entry(operation.clone()).or_insert_with(Vec::new).push(sidecar.data_size);
        }

        // Calculate averages
        let mut avg_processing_times = HashMap::new();
        for (operation, times) in processing_times {
            if !times.is_empty() {
                let avg = times.iter().sum::<f64>() / times.len() as f64;
                avg_processing_times.insert(operation, avg);
            }
        }

        let mut success_rate_percentages = HashMap::new();
        for (operation, (success, total)) in success_rates {
            if total > 0 {
                let percentage = (success as f64 / total as f64) * 100.0;
                success_rate_percentages.insert(operation, percentage);
            }
        }

        let mut avg_data_sizes = HashMap::new();
        for (operation, sizes) in data_sizes {
            if !sizes.is_empty() {
                let avg = sizes.iter().sum::<u64>() as f64 / sizes.len() as f64;
                avg_data_sizes.insert(operation, avg);
            }
        }

        // Populate statistics
        stats.total_images = image_files.len() as u32;
        stats.symlink_count = symlink_count;
        stats.broken_symlinks = broken_symlinks;
        stats.total_sidecars = sidecars.len() as u32;
        stats.coverage_percentage = if stats.total_images > 0 {
            (stats.total_sidecars as f64 / stats.total_images as f64) * 100.0
        } else {
            0.0
        };
        stats.operation_counts = operation_counts;
        stats.avg_processing_times = avg_processing_times;
        stats.success_rate_percentages = success_rate_percentages;
        stats.avg_data_sizes = avg_data_sizes;
        stats.sidecars = sidecars;

        Ok(stats)
    }

    /// Clean up orphaned sidecar files
    pub async fn cleanup_orphaned_sidecars(&self, directory: &Path) -> Result<usize> {
        let mut removed_count = 0;

        // Find all sidecar files
        let sidecar_files = self.find_sidecar_files(directory).await?;

        for sidecar_path in sidecar_files {
            // Check if corresponding image exists
            let image_name = sidecar_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .rsplit('_')
                .next()
                .unwrap_or("");

            let mut image_exists = false;
            for ext in &self.image_extensions {
                let potential_image = directory.join(format!("{}.{}", image_name, ext));
                if potential_image.exists() {
                    image_exists = true;
                    break;
                }
            }

            if !image_exists {
                fs::remove_file(&sidecar_path).await?;
                removed_count += 1;
                tracing::info!("Removed orphaned sidecar: {:?}", sidecar_path);
            }
        }

        Ok(removed_count)
    }

    // Private helper methods

    async fn resolve_symlink(&self, path: &Path) -> Result<(PathBuf, Option<SymlinkInfo>)> {
        if path.is_symlink() {
            match fs::read_link(path).await {
                Ok(target_path) => {
                    let broken = !target_path.exists();
                    Ok((target_path, Some(SymlinkInfo {
                        symlink_path: path.to_path_buf(),
                        target_path: path.canonicalize().unwrap_or_else(|_| path.to_path_buf()),
                        is_symlink: true,
                        broken,
                    })))
                }
                Err(_e) => Err(SidecarError::SymlinkResolutionFailed(path.to_path_buf()).into()),
            }
        } else {
            Ok((path.to_path_buf(), None))
        }
    }

    async fn detect_operation_type(&self, sidecar_path: &Path) -> Result<OperationType> {
        match self.load_sidecar_data(sidecar_path).await {
            Ok(data) => {
                // Check for sidecar_info structure
                if let Some(sidecar_info) = data.get("sidecar_info") {
                    if let Some(operation_str) = sidecar_info.get("operation_type").and_then(|v| v.as_str()) {
                        return Ok(OperationType::from_str(operation_str));
                    }
                }

                // Check for detector-specific keys
                if let Some(obj) = data.as_object() {
                    for (key, operation_type) in &self.operation_mapping {
                        if obj.contains_key(key) {
                            return Ok(operation_type.clone());
                        }
                    }
                }

                Ok(OperationType::Unknown)
            }
            Err(_) => Ok(OperationType::Unknown),
        }
    }

    async fn load_sidecar_data(&self, sidecar_path: &Path) -> Result<Value> {
        let content_bytes = fs::read(sidecar_path).await?;
        
        // Detect format from file extension first
        if let Some(format) = SidecarFormat::from_path(sidecar_path) {
            let serializer = self.format_manager.get_serializer(format);
            return serializer.deserialize(&content_bytes)
                .map_err(|e| SidecarError::SerializationError(e.to_string()).into());
        }
        
        // Fallback: try to detect format from content
        match self.format_manager.detect_format_from_content(&content_bytes) {
            Ok(format) => {
                let serializer = self.format_manager.get_serializer(format);
                serializer.deserialize(&content_bytes)
                    .map_err(|e| SidecarError::SerializationError(e.to_string()).into())
            }
            Err(_) => {
                // Final fallback: try as JSON
                let content_str = std::str::from_utf8(&content_bytes)
                    .map_err(|e| SidecarError::SerializationError(format!("Invalid UTF-8: {}", e)))?;
                let data: Value = serde_json::from_str(content_str)?;
                Ok(data)
            }
        }
    }

    async fn find_image_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
        let mut image_files = Vec::new();

        for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy().to_lowercase();
                    if self.image_extensions.iter().any(|ext| ext == &ext_str) {
                        image_files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(image_files)
    }

    async fn find_pattern_sidecars(&self, directory: &Path) -> Result<Vec<SidecarInfo>> {
        let mut sidecars = Vec::new();
        let sidecar_files = self.find_sidecar_files(directory).await?;

        for sidecar_path in sidecar_files {
            // Try to find corresponding image
            let image_name = sidecar_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .rsplit('_')
                .next()
                .unwrap_or("");

            for ext in &self.image_extensions {
                let potential_image = directory.join(format!("{}.{}", image_name, ext));
                if potential_image.exists() {
                    let operation = self.detect_operation_type(&sidecar_path).await?;
                    let mut sidecar_info = SidecarInfo::new(
                        potential_image,
                        sidecar_path.clone(),
                        operation,
                        None,
                    );
                    
                    // Load and validate the sidecar
                    if let Ok(data) = self.load_sidecar_data(&sidecar_path).await {
                        sidecar_info.data_size = data.to_string().len() as u64;
                        sidecar_info.is_valid = true;
                    }

                    sidecars.push(sidecar_info);
                    break;
                }
            }
        }

        Ok(sidecars)
    }

    async fn find_sidecar_files(&self, directory: &Path) -> Result<Vec<PathBuf>> {
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

    /// Convert a sidecar file from one format to another
    pub async fn convert_sidecar_format(
        &self,
        sidecar_path: &Path,
        target_format: SidecarFormat,
    ) -> Result<PathBuf> {
        // Load the existing sidecar data
        let data = self.load_sidecar_data(sidecar_path).await?;
        
        // Determine the current format
        let current_format = SidecarFormat::from_path(sidecar_path)
            .unwrap_or(SidecarFormat::Json);
        
        if current_format == target_format {
            return Ok(sidecar_path.to_path_buf());
        }
        
        // Create new path with target format extension
        let target_path = sidecar_path.with_extension(target_format.extension());
        
        // Serialize to new format
        let serializer = self.format_manager.get_serializer(target_format);
        let content_bytes = serializer.serialize(&data)
            .map_err(|e| SidecarError::SerializationError(e.to_string()))?;
        
        // Write the new file
        fs::write(&target_path, content_bytes).await?;
        
        // Remove the old file
        fs::remove_file(sidecar_path).await?;
        
        Ok(target_path)
    }

    /// Convert all sidecar files in a directory to a target format
    pub async fn convert_directory_format(
        &self,
        directory: &Path,
        target_format: SidecarFormat,
    ) -> Result<u32> {
        let sidecar_files = self.find_sidecar_files(directory).await?;
        let mut converted_count = 0;
        
        for sidecar_path in sidecar_files {
            let current_format = SidecarFormat::from_path(&sidecar_path)
                .unwrap_or(SidecarFormat::Json);
            
            if current_format != target_format {
                match self.convert_sidecar_format(&sidecar_path, target_format).await {
                    Ok(_) => {
                        converted_count += 1;
                        tracing::info!("Converted {:?} to {:?}", sidecar_path, target_format);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to convert {:?}: {}", sidecar_path, e);
                    }
                }
            }
        }
        
        Ok(converted_count)
    }

    /// Set the default format for new sidecar files
    pub fn set_default_format(&mut self, format: SidecarFormat) {
        self.default_format = format;
    }

    /// Get the current default format
    pub fn get_default_format(&self) -> SidecarFormat {
        self.default_format
    }

    /// Get format statistics for a directory
    pub async fn get_format_statistics(&self, directory: &Path) -> Result<HashMap<SidecarFormat, u32>> {
        let sidecar_files = self.find_sidecar_files(directory).await?;
        let mut format_counts = HashMap::new();
        
        for sidecar_path in sidecar_files {
            let format = SidecarFormat::from_path(&sidecar_path)
                .unwrap_or(SidecarFormat::Json);
            *format_counts.entry(format).or_insert(0) += 1;
        }
        
        Ok(format_counts)
    }
}

impl Default for SidecarManager {
    fn default() -> Self {
        Self::new()
    }
}
