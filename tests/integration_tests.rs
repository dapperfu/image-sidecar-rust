/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Integration tests for image-sidecar-rust
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: tempfile, tokio
 */

use image_sidecar_rust::ImageSidecar;
use image_sidecar_rust::sidecar::OperationType;
use tempfile::TempDir;
use std::fs;
use serde_json::json;
use bincode;

#[tokio::test]
async fn test_sidecar_creation_and_validation() {
    let temp_dir = TempDir::new().unwrap();
    let image_path = temp_dir.path().join("test.jpg");
    fs::write(&image_path, b"fake image data").unwrap();
    
    let sidecar = ImageSidecar::new(None);
    let data = json!({
        "faces": [
            {
                "bbox": [100, 100, 50, 50],
                "confidence": 0.95
            }
        ],
        "face_count": 1
    });
    
    // Create sidecar
    let result = sidecar.create_sidecar(&image_path, OperationType::FaceDetection, data).await;
    assert!(result.is_ok());
    
    let sidecar_info = result.unwrap();
    assert!(sidecar_info.sidecar_path.exists());
    assert_eq!(sidecar_info.operation, OperationType::FaceDetection);
    
    // Validate sidecar
    let validation_results = sidecar.validate_sidecars(temp_dir.path()).await.unwrap();
    assert_eq!(validation_results.len(), 1);
    assert!(validation_results[0].is_valid);
}

#[tokio::test]
async fn test_statistics_generation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test images and sidecars
    for i in 0..5 {
        let image_path = temp_dir.path().join(format!("test_{}.jpg", i));
        fs::write(&image_path, b"fake image data").unwrap();
        
        let sidecar_path = temp_dir.path().join(format!("test_{}.bin", i));
        let sidecar_data = json!({
            "sidecar_info": {
                "operation_type": "face_detection",
                "created_at": "2024-12-19T10:30:00Z"
            },
            "face_detection": {
                "success": true,
                "faces": [],
                "face_count": 0
            }
        });
        // Create binary sidecar data using the same format as the serializer
        let json_str = serde_json::to_string(&sidecar_data).unwrap();
        let binary_data = bincode::serialize(&json_str).unwrap();
        fs::write(&sidecar_path, binary_data).unwrap();
    }
    
    let sidecar = ImageSidecar::new(None);
    let stats = sidecar.get_statistics(temp_dir.path()).await.unwrap();
    
    assert_eq!(stats.total_images, 5);
    assert_eq!(stats.total_sidecars, 5);
    assert_eq!(stats.coverage_percentage, 100.0);
    assert!(stats.operation_counts.contains_key("face_detection"));
}

#[tokio::test]
async fn test_orphaned_sidecar_cleanup() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create orphaned sidecar (no corresponding image)
    let orphaned_sidecar = temp_dir.path().join("orphaned.json");
    let sidecar_data = json!({
        "sidecar_info": {
            "operation_type": "face_detection",
            "created_at": "2024-12-19T10:30:00Z"
        },
        "face_detection": {
            "success": true,
            "faces": []
        }
    });
    fs::write(&orphaned_sidecar, serde_json::to_string_pretty(&sidecar_data).unwrap()).unwrap();
    
    // Create valid sidecar with corresponding image
    let image_path = temp_dir.path().join("valid.jpg");
    fs::write(&image_path, b"fake image data").unwrap();
    
    let valid_sidecar = temp_dir.path().join("valid.json");
    fs::write(&valid_sidecar, serde_json::to_string_pretty(&sidecar_data).unwrap()).unwrap();
    
    let sidecar = ImageSidecar::new(None);
    let removed_count = sidecar.cleanup_orphaned(temp_dir.path()).await.unwrap();
    
    assert_eq!(removed_count, 1);
    assert!(!orphaned_sidecar.exists());
    assert!(valid_sidecar.exists());
}

#[tokio::test]
async fn test_parallel_processing() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple test files
    for i in 0..20 {
        let image_path = temp_dir.path().join(format!("test_{}.jpg", i));
        fs::write(&image_path, b"fake image data").unwrap();
        
        let sidecar_path = temp_dir.path().join(format!("test_{}.json", i));
        let sidecar_data = json!({
            "sidecar_info": {
                "operation_type": "object_detection",
                "created_at": "2024-12-19T10:30:00Z"
            },
            "object_detection": {
                "success": true,
                "objects": [
                    {
                        "class": "person",
                        "confidence": 0.9,
                        "bbox": [100, 100, 200, 300]
                    }
                ]
            }
        });
        fs::write(&sidecar_path, serde_json::to_string_pretty(&sidecar_data).unwrap()).unwrap();
    }
    
    let sidecar = ImageSidecar::new(Some(8)); // Use 8 workers
    let validation_results = sidecar.validate_sidecars(temp_dir.path()).await.unwrap();
    
    assert_eq!(validation_results.len(), 20);
    assert!(validation_results.iter().all(|r| r.is_valid));
}

#[tokio::test]
async fn test_symlink_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create actual image
    let actual_image = temp_dir.path().join("actual.jpg");
    fs::write(&actual_image, b"fake image data").unwrap();
    
    // Create symlink to image
    let symlink_path = temp_dir.path().join("symlink.jpg");
    std::os::unix::fs::symlink(&actual_image, &symlink_path).unwrap();
    
    // Create sidecar next to actual image
    let sidecar_path = temp_dir.path().join("actual.json");
    let sidecar_data = json!({
        "sidecar_info": {
            "operation_type": "quality_assessment",
            "created_at": "2024-12-19T10:30:00Z"
        },
        "quality_assessment": {
            "success": true,
            "score": 0.85
        }
    });
    fs::write(&sidecar_path, serde_json::to_string_pretty(&sidecar_data).unwrap()).unwrap();
    
    let sidecar = ImageSidecar::new(None);
    let sidecars = sidecar.find_sidecars(temp_dir.path()).await.unwrap();
    
    // Should find sidecar for the actual image
    assert_eq!(sidecars.len(), 1);
    let info = &sidecars[0];
    assert_eq!(info.operation, OperationType::QualityAssessment);
    // The sidecar is associated with the actual image, not the symlink
    assert!(info.symlink_info.is_none());
}
