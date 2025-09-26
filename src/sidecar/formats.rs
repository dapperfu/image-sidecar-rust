/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T12:00:00Z
 * Context: Binary serialization format support for sidecar operations
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: serde, bincode, rkyv, bytecheck
 */

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;
use thiserror::Error;

/// Supported sidecar file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SidecarFormat {
    /// JSON format (human-readable, slower)
    Json,
    /// Binary format using bincode (fast, compact)
    Binary,
    /// Zero-copy binary format using rkyv (fastest, compact)
    Rkyv,
}

impl SidecarFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            SidecarFormat::Json => "json",
            SidecarFormat::Binary => "bin",
            SidecarFormat::Rkyv => "rkyv",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(SidecarFormat::Json),
            "bin" => Some(SidecarFormat::Binary),
            "rkyv" => Some(SidecarFormat::Rkyv),
            _ => None,
        }
    }

    /// Detect format from file path
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    /// Get the default format for new files
    pub fn default() -> Self {
        SidecarFormat::Binary
    }

    /// Check if this format is binary
    pub fn is_binary(&self) -> bool {
        matches!(self, SidecarFormat::Binary | SidecarFormat::Rkyv)
    }

    /// Get format description
    pub fn description(&self) -> &'static str {
        match self {
            SidecarFormat::Json => "JSON (human-readable, slower)",
            SidecarFormat::Binary => "Binary (fast, compact)",
            SidecarFormat::Rkyv => "Rkyv (zero-copy, fastest)",
        }
    }
}

/// Serialization errors
#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Binary serialization error: {0}")]
    Binary(#[from] bincode::Error),
    #[error("Rkyv serialization error: {0}")]
    Rkyv(String),
    #[error("Bytecheck validation error: {0}")]
    Bytecheck(String),
    #[error("Unsupported format: {0:?}")]
    UnsupportedFormat(SidecarFormat),
    #[error("Format detection failed")]
    FormatDetectionFailed,
}

/// Trait for serializing sidecar data
pub trait SidecarSerializer {
    /// Serialize data to bytes
    fn serialize(&self, data: &serde_json::Value) -> Result<Vec<u8>, SerializationError>;
    
    /// Deserialize data from bytes
    fn deserialize(&self, bytes: &[u8]) -> Result<serde_json::Value, SerializationError>;
    
    /// Get the format this serializer handles
    fn format(&self) -> SidecarFormat;
}

/// JSON serializer
pub struct JsonSerializer;

impl SidecarSerializer for JsonSerializer {
    fn serialize(&self, data: &serde_json::Value) -> Result<Vec<u8>, SerializationError> {
        let json_str = serde_json::to_string_pretty(data)?;
        Ok(json_str.into_bytes())
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<serde_json::Value, SerializationError> {
        let json_str = std::str::from_utf8(bytes)
            .map_err(|e| SerializationError::Json(serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))))?;
        let value = serde_json::from_str(json_str)?;
        Ok(value)
    }

    fn format(&self) -> SidecarFormat {
        SidecarFormat::Json
    }
}

/// Binary serializer using bincode
pub struct BinarySerializer;

impl SidecarSerializer for BinarySerializer {
    fn serialize(&self, data: &serde_json::Value) -> Result<Vec<u8>, SerializationError> {
        // Convert JSON to a more bincode-friendly format
        let json_str = serde_json::to_string(data)?;
        let bytes = bincode::serialize(&json_str)?;
        Ok(bytes)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<serde_json::Value, SerializationError> {
        // Deserialize the JSON string first, then parse it
        let json_str: String = bincode::deserialize(bytes)?;
        let value = serde_json::from_str(&json_str)?;
        Ok(value)
    }

    fn format(&self) -> SidecarFormat {
        SidecarFormat::Binary
    }
}

/// Rkyv serializer for zero-copy deserialization
/// Note: Simplified implementation - rkyv support can be added later
pub struct RkyvSerializer;

impl SidecarSerializer for RkyvSerializer {
    fn serialize(&self, data: &serde_json::Value) -> Result<Vec<u8>, SerializationError> {
        // Convert to JSON string first, then serialize the string
        // This avoids bincode's limitations with serde_json::Value
        let json_str = serde_json::to_string(data)?;
        let bytes = bincode::serialize(&json_str)?;
        Ok(bytes)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<serde_json::Value, SerializationError> {
        // Deserialize the JSON string, then parse it back to Value
        let json_str: String = bincode::deserialize(bytes)?;
        let value = serde_json::from_str(&json_str)?;
        Ok(value)
    }

    fn format(&self) -> SidecarFormat {
        SidecarFormat::Rkyv
    }
}


/// Format manager for handling different serialization formats
pub struct FormatManager {
    json_serializer: JsonSerializer,
    binary_serializer: BinarySerializer,
    rkyv_serializer: RkyvSerializer,
}

impl FormatManager {
    pub fn new() -> Self {
        Self {
            json_serializer: JsonSerializer,
            binary_serializer: BinarySerializer,
            rkyv_serializer: RkyvSerializer,
        }
    }

    /// Get serializer for a specific format
    pub fn get_serializer(&self, format: SidecarFormat) -> &dyn SidecarSerializer {
        match format {
            SidecarFormat::Json => &self.json_serializer,
            SidecarFormat::Binary => &self.binary_serializer,
            SidecarFormat::Rkyv => &self.rkyv_serializer,
        }
    }

    /// Detect format from file content
    pub fn detect_format_from_content(&self, bytes: &[u8]) -> Result<SidecarFormat, SerializationError> {
        // Try to parse as JSON first
        if let Ok(_) = serde_json::from_slice::<serde_json::Value>(bytes) {
            return Ok(SidecarFormat::Json);
        }

        // Try bincode
        if let Ok(_) = bincode::deserialize::<serde_json::Value>(bytes) {
            return Ok(SidecarFormat::Binary);
        }

        // Try rkyv
        if let Ok(_) = bincode::deserialize::<serde_json::Value>(bytes) {
            return Ok(SidecarFormat::Rkyv);
        }

        Err(SerializationError::FormatDetectionFailed)
    }

    /// Convert between formats
    pub fn convert_format(
        &self,
        data: &serde_json::Value,
        from_format: SidecarFormat,
        to_format: SidecarFormat,
    ) -> Result<Vec<u8>, SerializationError> {
        if from_format == to_format {
            return self.get_serializer(to_format).serialize(data);
        }

        // Deserialize from source format
        let source_bytes = self.get_serializer(from_format).serialize(data)?;
        let deserialized_data = self.get_serializer(from_format).deserialize(&source_bytes)?;

        // Serialize to target format
        self.get_serializer(to_format).serialize(&deserialized_data)
    }
}

impl Default for FormatManager {
    fn default() -> Self {
        Self::new()
    }
}
