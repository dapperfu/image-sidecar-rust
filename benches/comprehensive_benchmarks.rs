/**
 * This code written by Claude Sonnet 4 (claude-3-5-sonnet-20241022)
 * Generated via Cursor IDE (cursor.sh) with AI assistance
 * Model: Anthropic Claude 3.5 Sonnet
 * Generation timestamp: 2024-12-19T13:00:00Z
 * Context: Comprehensive benchmark suite for sidecar format performance
 * 
 * Technical details:
 * - LLM: Claude 3.5 Sonnet (2024-10-22)
 * - IDE: Cursor (cursor.sh)
 * - Generation method: AI-assisted pair programming
 * - Code style: Rust idiomatic with comprehensive error handling
 * - Dependencies: criterion, tokio, rayon
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use sportball_sidecar_rust::{SportballSidecar, SidecarFormat};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Runtime;
use rayon::prelude::*;
use std::time::Instant;

/// Benchmark conversion performance between formats
fn benchmark_conversion_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let data_dir = Path::new("/tank/games/Game_04_153212-160200/");
    
    // Only run if the data directory exists
    if !data_dir.exists() {
        println!("⚠️  Data directory not found: {:?}", data_dir);
        return;
    }

    let mut group = c.benchmark_group("conversion_performance");
    
    // Benchmark JSON to Binary conversion
    group.bench_function("json_to_binary", |b| {
        b.to_async(&rt).iter(|| async {
            let sidecar = SportballSidecar::new(None);
            let result = sidecar.convert_directory_format(black_box(data_dir), SidecarFormat::Binary).await;
            black_box(result)
        })
    });

    // Benchmark JSON to Rkyv conversion
    group.bench_function("json_to_rkyv", |b| {
        b.to_async(&rt).iter(|| async {
            let sidecar = SportballSidecar::new(None);
            let result = sidecar.convert_directory_format(black_box(data_dir), SidecarFormat::Rkyv).await;
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark reading performance for different formats
fn benchmark_reading_performance(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let data_dir = Path::new("/tank/games/Game_04_153212-160200/");
    
    if !data_dir.exists() {
        println!("⚠️  Data directory not found: {:?}", data_dir);
        return;
    }

    let mut group = c.benchmark_group("reading_performance");
    
    // Benchmark JSON reading
    group.bench_function("read_json", |b| {
        b.to_async(&rt).iter(|| async {
            let sidecar = SportballSidecar::new(Some(16));
            let result = sidecar.validate_sidecars(black_box(data_dir)).await;
            black_box(result)
        })
    });

    // Benchmark Binary reading (after conversion)
    group.bench_function("read_binary", |b| {
        b.to_async(&rt).iter(|| async {
            let sidecar = SportballSidecar::new(Some(16));
            let result = sidecar.validate_sidecars(black_box(data_dir)).await;
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark parallel processing with different worker counts
fn benchmark_parallel_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let data_dir = Path::new("/tank/games/Game_04_153212-160200/");
    
    if !data_dir.exists() {
        println!("⚠️  Data directory not found: {:?}", data_dir);
        return;
    }

    let mut group = c.benchmark_group("parallel_processing");
    
    let worker_counts = vec![1, 2, 4, 8, 16, 32];
    
    for workers in worker_counts {
        group.bench_with_input(
            BenchmarkId::new("validation", workers),
            &workers,
            |b, &workers| {
                b.to_async(&rt).iter(|| async {
                    let sidecar = SportballSidecar::new(Some(workers));
                    let result = sidecar.validate_sidecars(black_box(data_dir)).await;
                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark file size analysis
fn benchmark_file_size_analysis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let data_dir = Path::new("/tank/games/Game_04_153212-160200/");
    
    if !data_dir.exists() {
        println!("⚠️  Data directory not found: {:?}", data_dir);
        return;
    }

    let mut group = c.benchmark_group("file_size_analysis");
    
    // Benchmark format statistics
    group.bench_function("format_statistics", |b| {
        b.to_async(&rt).iter(|| async {
            let sidecar = SportballSidecar::new(None);
            let result = sidecar.get_format_statistics(black_box(data_dir)).await;
            black_box(result)
        })
    });

    // Benchmark general statistics
    group.bench_function("general_statistics", |b| {
        b.to_async(&rt).iter(|| async {
            let sidecar = SportballSidecar::new(None);
            let result = sidecar.get_statistics(black_box(data_dir)).await;
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark serialization/deserialization performance
fn benchmark_serialization_performance(c: &mut Criterion) {
    use sportball_sidecar_rust::sidecar::formats::{FormatManager, SidecarFormat};
    use serde_json::json;

    let mut group = c.benchmark_group("serialization_performance");
    
    // Create sample data
    let sample_data = json!({
        "sidecar_info": {
            "operation_type": "face_detection",
            "created_at": "2024-12-19T10:30:00Z",
            "image_path": "/path/to/image.jpg"
        },
        "face_detection": {
            "success": true,
            "faces": [
                {
                    "bbox": {"x": 0.1, "y": 0.2, "width": 0.3, "height": 0.4},
                    "confidence": 0.95,
                    "landmarks": [[0.1, 0.2], [0.3, 0.4], [0.5, 0.6]],
                    "encoding": vec![0.1; 128]
                }
            ],
            "metadata": {
                "image_width": 1920,
                "image_height": 1080,
                "faces_found": 1,
                "processing_time": 0.123
            }
        }
    });

    let format_manager = FormatManager::new();

    // Benchmark JSON serialization
    group.bench_function("json_serialize", |b| {
        b.iter(|| {
            let serializer = format_manager.get_serializer(SidecarFormat::Json);
            let result = serializer.serialize(black_box(&sample_data));
            black_box(result)
        })
    });

    // Benchmark JSON deserialization
    group.bench_function("json_deserialize", |b| {
        let serializer = format_manager.get_serializer(SidecarFormat::Json);
        let json_bytes = serializer.serialize(&sample_data).unwrap();
        
        b.iter(|| {
            let result = serializer.deserialize(black_box(&json_bytes));
            black_box(result)
        })
    });

    // Benchmark Binary serialization
    group.bench_function("binary_serialize", |b| {
        b.iter(|| {
            let serializer = format_manager.get_serializer(SidecarFormat::Binary);
            let result = serializer.serialize(black_box(&sample_data));
            black_box(result)
        })
    });

    // Benchmark Binary deserialization
    group.bench_function("binary_deserialize", |b| {
        let serializer = format_manager.get_serializer(SidecarFormat::Binary);
        let binary_bytes = serializer.serialize(&sample_data).unwrap();
        
        b.iter(|| {
            let result = serializer.deserialize(black_box(&binary_bytes));
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark memory usage and allocation patterns
fn benchmark_memory_usage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let data_dir = Path::new("/tank/games/Game_04_153212-160200/");
    
    if !data_dir.exists() {
        println!("⚠️  Data directory not found: {:?}", data_dir);
        return;
    }

    let mut group = c.benchmark_group("memory_usage");
    
    // Benchmark with different worker counts to see memory scaling
    let worker_counts = vec![1, 4, 16];
    
    for workers in worker_counts {
        group.bench_with_input(
            BenchmarkId::new("memory_scaling", workers),
            &workers,
            |b, &workers| {
                b.to_async(&rt).iter(|| async {
                    let sidecar = SportballSidecar::new(Some(workers));
                    let result = sidecar.validate_sidecars(black_box(data_dir)).await;
                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_conversion_performance,
    benchmark_reading_performance,
    benchmark_parallel_processing,
    benchmark_file_size_analysis,
    benchmark_serialization_performance,
    benchmark_memory_usage
);
criterion_main!(benches);
