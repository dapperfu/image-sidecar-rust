/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T10:30:00Z
 * Context: Benchmark suite for JSON validation performance
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: criterion, tempfile
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sportball_sidecar_rust::SportballSidecar;
use std::path::Path;
use tempfile::TempDir;
use std::fs;

fn create_test_json_files(count: usize, temp_dir: &TempDir) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    
    for i in 0..count {
        let file_path = temp_dir.path().join(format!("test_{:06}.json", i));
        let json_data = serde_json::json!({
            "sidecar_info": {
                "operation_type": "face_detection",
                "created_at": "2024-12-19T10:30:00Z",
                "image_path": format!("/path/to/image_{}.jpg", i)
            },
            "face_detection": {
                "success": true,
                "faces": [
                    {
                        "face_id": 0,
                        "bbox": {"x": 0.1, "y": 0.2, "width": 0.3, "height": 0.4},
                        "confidence": 0.95,
                        "landmarks": [[0.1, 0.2], [0.3, 0.4], [0.5, 0.6]],
                        "encoding": vec![0.1; 128]
                    }
                ],
                "metadata": {
                    "image_path": format!("/path/to/image_{}.jpg", i),
                    "image_width": 1920,
                    "image_height": 1080,
                    "faces_found": 1,
                    "processing_time": 0.123,
                    "extractor": "insightface",
                    "model_name": "buffalo_l"
                }
            }
        });
        
        fs::write(&file_path, serde_json::to_string_pretty(&json_data).unwrap()).unwrap();
        files.push(file_path);
    }
    
    files
}

fn benchmark_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_validation");
    
    for file_count in [10, 100, 1000, 5000].iter() {
        let temp_dir = TempDir::new().unwrap();
        let _files = create_test_json_files(*file_count, &temp_dir);
        
        group.bench_with_input(
            BenchmarkId::new("validate_sidecars", file_count),
            file_count,
            |b, _| {
                b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
                    let sidecar = SportballSidecar::new(Some(16));
                    let result = sidecar.validate_sidecars(black_box(temp_dir.path())).await;
                    black_box(result)
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistics");
    
    for file_count in [10, 100, 1000, 5000].iter() {
        let temp_dir = TempDir::new().unwrap();
        let _files = create_test_json_files(*file_count, &temp_dir);
        
        group.bench_with_input(
            BenchmarkId::new("get_statistics", file_count),
            file_count,
            |b, _| {
                b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
                    let sidecar = SportballSidecar::new(None);
                    let result = sidecar.get_statistics(black_box(temp_dir.path())).await;
                    black_box(result)
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_validation, benchmark_statistics);
criterion_main!(benches);
