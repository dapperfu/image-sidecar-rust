# Image Sidecar Rust - Implementation Guide

## Overview

This guide provides step-by-step instructions for implementing the high-performance image-sidecar-rust tool in existing image processing workflows that work with sidecar files. The tool supports multiple formats (JSON, Binary, Rkyv) with automatic detection and conversion capabilities.

Originally developed for sportball photography workflows, this tool has evolved into a general-purpose high-performance sidecar processing solution suitable for any image processing pipeline that uses sidecar files for metadata storage.

## Prerequisites

- Rust toolchain installed (`rustc`, `cargo`)
- Access to the image-sidecar-rust source code
- Understanding of your existing sidecar workflow

## Implementation Steps

### 1. Build the Rust Tool

```bash
# Clone or navigate to the image-sidecar-rust directory
cd /path/to/image-sidecar-rust

# Build the release binary
cargo build --release

# Verify the binary works
./target/release/sportball-sidecar-rust --help
```

### 2. Integration Approaches

#### Option A: Direct CLI Integration

Replace existing sidecar processing with Rust tool calls:

```bash
# Instead of Python-based validation
python validate_sidecars.py --input /path/to/sidecars

# Use Rust tool
./target/release/image-sidecar-rust validate --input /path/to/sidecars --workers 16
```

#### Option B: Python Wrapper Integration

Create a Python wrapper that automatically uses Rust when available:

```python
import subprocess
import json
import os
from pathlib import Path

class ImageSidecarManager:
    def __init__(self, rust_binary_path=None):
        if rust_binary_path is None:
            # Try to find the binary in common locations
            possible_paths = [
                "./target/release/image-sidecar-rust",
                "../image-sidecar-rust/target/release/image-sidecar-rust",
                "/usr/local/bin/image-sidecar-rust",
                # Legacy sportball paths for backward compatibility
                "./target/release/sportball-sidecar-rust",
                "../sportball-sidecar-rust/target/release/sportball-sidecar-rust",
                "/usr/local/bin/sportball-sidecar-rust"
            ]
            
            for path in possible_paths:
                if os.path.exists(path):
                    self.rust_binary = path
                    self.rust_available = True
                    break
            else:
                self.rust_binary = None
                self.rust_available = False
        else:
            self.rust_binary = rust_binary_path
            self.rust_available = os.path.exists(rust_binary_path)
    
    def validate_sidecars(self, directory, workers=16):
        """Validate sidecar files using Rust tool"""
        if not self.rust_available:
            raise RuntimeError("Rust binary not available")
        
        cmd = [
            self.rust_binary,
            "validate",
            "--input", str(directory),
            "--workers", str(workers),
            "--output", "-"
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"Rust validation failed: {result.stderr}")
        
        return json.loads(result.stdout)
    
    def get_statistics(self, directory):
        """Get sidecar statistics using Rust tool"""
        if not self.rust_available:
            raise RuntimeError("Rust binary not available")
        
        cmd = [
            self.rust_binary,
            "stats",
            "--input", str(directory),
            "--output", "-"
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"Rust stats failed: {result.stderr}")
        
        return json.loads(result.stdout)
    
    def convert_format(self, directory, target_format, dry_run=False):
        """Convert sidecar files to different format"""
        if not self.rust_available:
            raise RuntimeError("Rust binary not available")
        
        cmd = [
            self.rust_binary,
            "convert",
            "--input", str(directory),
            "--format", target_format
        ]
        
        if dry_run:
            cmd.append("--dry-run")
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"Rust conversion failed: {result.stderr}")
        
        return result.stdout
    
    def get_format_statistics(self, directory):
        """Get format distribution statistics"""
        if not self.rust_available:
            raise RuntimeError("Rust binary not available")
        
        cmd = [
            self.rust_binary,
            "format-stats",
            "--input", str(directory),
            "--output", "-"
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"Rust format stats failed: {result.stderr}")
        
        return json.loads(result.stdout)

# Usage example
def main():
    rust_manager = ImageSidecarManager()
    
    if rust_manager.rust_available:
        print("✅ Using Rust implementation")
        
        # Validate sidecars
        results = rust_manager.validate_sidecars("/path/to/sidecars")
        print(f"Validated {results['total_files']} files")
        
        # Get statistics
        stats = rust_manager.get_statistics("/path/to/sidecars")
        print(f"Total sidecars: {stats['total_sidecars']}")
        
        # Convert to binary format
        print("Converting to binary format...")
        rust_manager.convert_format("/path/to/sidecars", "bin")
        
        # Check format distribution
        format_stats = rust_manager.get_format_statistics("/path/to/sidecars")
        print(f"Format distribution: {format_stats['format_distribution']}")
        
    else:
        print("⚠️  Rust binary not available, falling back to Python")
        # Fallback to existing Python implementation
```

#### Option C: Makefile Integration

Add Rust tool targets to your existing Makefile:

```makefile
# Add to existing Makefile

# Rust tool paths
RUST_TOOL_DIR = ../sportball-sidecar-rust
RUST_BINARY = $(RUST_TOOL_DIR)/target/release/sportball-sidecar-rust

# Build Rust tool
.PHONY: build-rust-sidecar
build-rust-sidecar:
	cd $(RUST_TOOL_DIR) && cargo build --release

# Validate sidecars with Rust
.PHONY: validate-rust
validate-rust: build-rust-sidecar
	$(RUST_BINARY) validate --input $(SIDECAR_DIR) --workers 16

# Get statistics with Rust
.PHONY: stats-rust
stats-rust: build-rust-sidecar
	$(RUST_BINARY) stats --input $(SIDECAR_DIR)

# Convert to binary format
.PHONY: convert-binary
convert-binary: build-rust-sidecar
	$(RUST_BINARY) convert --input $(SIDECAR_DIR) --format bin

# Convert to Rkyv format
.PHONY: convert-rkyv
convert-rkyv: build-rust-sidecar
	$(RUST_BINARY) convert --input $(SIDECAR_DIR) --format rkyv

# Show format statistics
.PHONY: format-stats
format-stats: build-rust-sidecar
	$(RUST_BINARY) format-stats --input $(SIDECAR_DIR)

# Performance benchmark
.PHONY: benchmark-rust
benchmark-rust: build-rust-sidecar
	cd $(RUST_TOOL_DIR) && cargo bench

# Test integration
.PHONY: test-integration
test-integration: build-rust-sidecar
	$(RUST_BINARY) validate --input $(SIDECAR_DIR) --workers 8
	$(RUST_BINARY) stats --input $(SIDECAR_DIR)
	$(RUST_BINARY) format-stats --input $(SIDECAR_DIR)
```

### 3. Workflow Integration Examples

#### Example 1: Photography Pipeline Integration

```bash
#!/bin/bash
# Enhanced photography pipeline with Rust sidecar processing

SIDECAR_DIR="/path/to/photos"
RUST_TOOL="./target/release/sportball-sidecar-rust"

echo "🚀 Starting enhanced photography pipeline"

# Step 1: Validate existing sidecars
echo "📋 Validating sidecars..."
$RUST_TOOL validate --input "$SIDECAR_DIR" --workers 16

# Step 2: Get current statistics
echo "📊 Getting statistics..."
$RUST_TOOL stats --input "$SIDECAR_DIR" --output pipeline_stats.json

# Step 3: Check format distribution
echo "📈 Analyzing format distribution..."
$RUST_TOOL format-stats --input "$SIDECAR_DIR" --output format_analysis.json

# Step 4: Convert to optimal format (binary for speed)
echo "⚡ Converting to binary format for performance..."
$RUST_TOOL convert --input "$SIDECAR_DIR" --format bin

# Step 5: Validate converted files
echo "✅ Validating converted files..."
$RUST_TOOL validate --input "$SIDECAR_DIR" --workers 16

echo "🎯 Pipeline complete!"
```

#### Example 2: Batch Processing Script

```python
#!/usr/bin/env python3
"""
Batch sidecar processing with Rust integration
"""

import os
import json
import subprocess
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import argparse

class BatchSidecarProcessor:
    def __init__(self, rust_binary_path):
        self.rust_binary = rust_binary_path
        self.verify_rust_binary()
    
    def verify_rust_binary(self):
        """Verify Rust binary exists and works"""
        if not os.path.exists(self.rust_binary):
            raise FileNotFoundError(f"Rust binary not found: {self.rust_binary}")
        
        # Test the binary
        result = subprocess.run([self.rust_binary, "--help"], 
                              capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"Rust binary not working: {result.stderr}")
    
    def process_directory(self, directory, workers=16, target_format=None):
        """Process a single directory"""
        print(f"📁 Processing directory: {directory}")
        
        # Validate sidecars
        print("  📋 Validating sidecars...")
        validation_result = self.run_command([
            "validate", "--input", directory, "--workers", str(workers)
        ])
        
        # Get statistics
        print("  📊 Getting statistics...")
        stats_result = self.run_command([
            "stats", "--input", directory
        ])
        
        # Convert format if requested
        if target_format:
            print(f"  ⚡ Converting to {target_format} format...")
            conversion_result = self.run_command([
                "convert", "--input", directory, "--format", target_format
            ])
        
        # Get format statistics
        print("  📈 Getting format statistics...")
        format_stats = self.run_command([
            "format-stats", "--input", directory
        ])
        
        return {
            "directory": directory,
            "validation": validation_result,
            "stats": stats_result,
            "format_stats": format_stats
        }
    
    def run_command(self, args):
        """Run Rust command and return JSON result"""
        cmd = [self.rust_binary] + args + ["--output", "-"]
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise RuntimeError(f"Command failed: {result.stderr}")
        
        return json.loads(result.stdout)
    
    def batch_process(self, directories, workers=16, target_format=None, max_parallel=4):
        """Process multiple directories in parallel"""
        results = []
        
        with ThreadPoolExecutor(max_workers=max_parallel) as executor:
            futures = []
            
            for directory in directories:
                future = executor.submit(
                    self.process_directory, 
                    directory, 
                    workers, 
                    target_format
                )
                futures.append(future)
            
            for future in futures:
                try:
                    result = future.result()
                    results.append(result)
                    print(f"✅ Completed: {result['directory']}")
                except Exception as e:
                    print(f"❌ Failed: {e}")
        
        return results

def main():
    parser = argparse.ArgumentParser(description="Batch sidecar processing")
    parser.add_argument("--directories", nargs="+", required=True,
                       help="Directories to process")
    parser.add_argument("--rust-binary", required=True,
                       help="Path to Rust binary")
    parser.add_argument("--workers", type=int, default=16,
                       help="Number of workers for parallel processing")
    parser.add_argument("--target-format", choices=["json", "bin", "rkyv"],
                       help="Target format for conversion")
    parser.add_argument("--max-parallel", type=int, default=4,
                       help="Maximum parallel directory processing")
    parser.add_argument("--output", help="Output file for results")
    
    args = parser.parse_args()
    
    # Initialize processor
    processor = BatchSidecarProcessor(args.rust_binary)
    
    # Process directories
    print(f"🚀 Starting batch processing of {len(args.directories)} directories")
    results = processor.batch_process(
        args.directories,
        workers=args.workers,
        target_format=args.target_format,
        max_parallel=args.max_parallel
    )
    
    # Save results
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"📄 Results saved to: {args.output}")
    
    print(f"🎯 Batch processing complete! Processed {len(results)} directories")

if __name__ == "__main__":
    main()
```

#### Example 3: Docker Integration

```dockerfile
# Dockerfile for sidecar processing service
FROM rust:1.70 as builder

WORKDIR /app
COPY sportball-sidecar-rust/ .
RUN cargo build --release

FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/sportball-sidecar-rust /usr/local/bin/

# Create non-root user
RUN useradd -m -u 1000 sidecar
USER sidecar

WORKDIR /data

# Default command
CMD ["sportball-sidecar-rust", "--help"]
```

```yaml
# docker-compose.yml for sidecar processing
version: '3.8'

services:
  sidecar-processor:
    build: .
    volumes:
      - ./sidecars:/data/sidecars:ro
      - ./output:/data/output
    environment:
      - RUST_LOG=info
    command: >
      sportball-sidecar-rust validate 
      --input /data/sidecars 
      --workers 16
      --output /data/output/validation_results.json

  sidecar-converter:
    build: .
    volumes:
      - ./sidecars:/data/sidecars
      - ./output:/data/output
    environment:
      - RUST_LOG=info
    command: >
      sportball-sidecar-rust convert 
      --input /data/sidecars 
      --format bin
    depends_on:
      - sidecar-processor
```

### 4. Performance Optimization Guidelines

#### Format Selection Strategy

```bash
#!/bin/bash
# Format optimization script

SIDECAR_DIR="$1"
RUST_TOOL="./target/release/sportball-sidecar-rust"

echo "🔍 Analyzing sidecar directory for format optimization..."

# Get current format distribution
FORMAT_STATS=$($RUST_TOOL format-stats --input "$SIDECAR_DIR" --output -)

# Extract counts
JSON_COUNT=$(echo "$FORMAT_STATS" | jq '.format_distribution.Json // 0')
BINARY_COUNT=$(echo "$FORMAT_STATS" | jq '.format_distribution.Binary // 0')
RKYV_COUNT=$(echo "$FORMAT_STATS" | jq '.format_distribution.Rkyv // 0')

echo "📊 Current format distribution:"
echo "  JSON: $JSON_COUNT files"
echo "  Binary: $BINARY_COUNT files"
echo "  Rkyv: $RKYV_COUNT files"

# Performance benchmark
echo "⚡ Running performance benchmark..."

# Test JSON performance
if [ "$JSON_COUNT" -gt 0 ]; then
    echo "📄 Testing JSON performance..."
    JSON_TIME=$(time -p $RUST_TOOL validate --input "$SIDECAR_DIR" --workers 16 2>&1 | grep real | awk '{print $2}')
    echo "  JSON validation time: ${JSON_TIME}s"
fi

# Test Binary performance
if [ "$BINARY_COUNT" -gt 0 ]; then
    echo "📦 Testing Binary performance..."
    BINARY_TIME=$(time -p $RUST_TOOL validate --input "$SIDECAR_DIR" --workers 16 2>&1 | grep real | awk '{print $2}')
    echo "  Binary validation time: ${BINARY_TIME}s"
fi

# Recommendations
echo "💡 Optimization recommendations:"
if [ "$JSON_COUNT" -gt 0 ] && [ "$BINARY_COUNT" -eq 0 ]; then
    echo "  → Convert to Binary format for 2-3x performance improvement"
    echo "  → Run: $RUST_TOOL convert --input $SIDECAR_DIR --format bin"
elif [ "$BINARY_COUNT" -gt 0 ] && [ "$RKYV_COUNT" -eq 0 ]; then
    echo "  → Consider Rkyv format for maximum performance"
    echo "  → Run: $RUST_TOOL convert --input $SIDECAR_DIR --format rkyv"
else
    echo "  → Format distribution looks optimal"
fi
```

### 5. Monitoring and Maintenance

#### Health Check Script

```bash
#!/bin/bash
# Sidecar health check script

SIDECAR_DIR="$1"
RUST_TOOL="./target/release/sportball-sidecar-rust"
LOG_FILE="sidecar_health_$(date +%Y%m%d_%H%M%S).log"

echo "🏥 Starting sidecar health check..."
echo "📁 Directory: $SIDECAR_DIR"
echo "📝 Log file: $LOG_FILE"

# Check Rust binary
if [ ! -f "$RUST_TOOL" ]; then
    echo "❌ Rust binary not found: $RUST_TOOL" | tee -a "$LOG_FILE"
    exit 1
fi

# Validate sidecars
echo "📋 Validating sidecars..." | tee -a "$LOG_FILE"
VALIDATION_RESULT=$($RUST_TOOL validate --input "$SIDECAR_DIR" --workers 8 2>&1)
echo "$VALIDATION_RESULT" | tee -a "$LOG_FILE"

# Check for errors
INVALID_COUNT=$(echo "$VALIDATION_RESULT" | jq '.invalid_files // 0')
if [ "$INVALID_COUNT" -gt 0 ]; then
    echo "⚠️  Found $INVALID_COUNT invalid files" | tee -a "$LOG_FILE"
fi

# Get statistics
echo "📊 Getting statistics..." | tee -a "$LOG_FILE"
STATS_RESULT=$($RUST_TOOL stats --input "$SIDECAR_DIR" 2>&1)
echo "$STATS_RESULT" | tee -a "$LOG_FILE"

# Format analysis
echo "📈 Format analysis..." | tee -a "$LOG_FILE"
FORMAT_RESULT=$($RUST_TOOL format-stats --input "$SIDECAR_DIR" 2>&1)
echo "$FORMAT_RESULT" | tee -a "$LOG_FILE"

echo "✅ Health check complete!" | tee -a "$LOG_FILE"
```

### 6. Migration Strategies

#### Gradual Migration Approach

```bash
#!/bin/bash
# Gradual migration script

SOURCE_DIR="$1"
BACKUP_DIR="${SOURCE_DIR}_backup_$(date +%Y%m%d_%H%M%S)"
RUST_TOOL="./target/release/sportball-sidecar-rust"

echo "🔄 Starting gradual migration to Rust sidecar processing..."

# Step 1: Create backup
echo "📦 Creating backup..."
cp -r "$SOURCE_DIR" "$BACKUP_DIR"
echo "✅ Backup created: $BACKUP_DIR"

# Step 2: Test with dry run
echo "🧪 Testing conversion with dry run..."
$RUST_TOOL convert --input "$SOURCE_DIR" --format bin --dry-run

# Step 3: Convert a small subset first
echo "📝 Converting first 10 files..."
find "$SOURCE_DIR" -name "*.json" | head -10 | while read file; do
    echo "Converting: $file"
    $RUST_TOOL convert --input "$(dirname "$file")" --format bin
done

# Step 4: Validate converted files
echo "✅ Validating converted files..."
$RUST_TOOL validate --input "$SOURCE_DIR" --workers 8

# Step 5: Performance comparison
echo "⚡ Running performance comparison..."
echo "Before conversion:"
time $RUST_TOOL validate --input "$SOURCE_DIR" --workers 8

echo "After conversion:"
time $RUST_TOOL validate --input "$SOURCE_DIR" --workers 8

echo "🎯 Migration test complete!"
echo "💡 If satisfied, run full conversion:"
echo "   $RUST_TOOL convert --input $SOURCE_DIR --format bin"
```

### 7. Troubleshooting Guide

#### Common Issues and Solutions

```bash
#!/bin/bash
# Troubleshooting script

echo "🔧 Sportball Sidecar Rust Troubleshooting Guide"
echo "==============================================="

# Check Rust installation
echo "1. Checking Rust installation..."
if command -v rustc &> /dev/null; then
    echo "✅ Rust installed: $(rustc --version)"
else
    echo "❌ Rust not installed. Install from: https://rustup.rs/"
fi

# Check binary existence
echo "2. Checking binary existence..."
if [ -f "./target/release/sportball-sidecar-rust" ]; then
    echo "✅ Binary found"
else
    echo "❌ Binary not found. Run: cargo build --release"
fi

# Check binary permissions
echo "3. Checking binary permissions..."
if [ -x "./target/release/sportball-sidecar-rust" ]; then
    echo "✅ Binary is executable"
else
    echo "❌ Binary not executable. Run: chmod +x ./target/release/sportball-sidecar-rust"
fi

# Test binary functionality
echo "4. Testing binary functionality..."
if ./target/release/sportball-sidecar-rust --help &> /dev/null; then
    echo "✅ Binary works correctly"
else
    echo "❌ Binary not working. Check compilation errors."
fi

# Check directory permissions
echo "5. Checking directory permissions..."
TEST_DIR="/tmp/sidecar_test_$$"
mkdir -p "$TEST_DIR"
if [ -w "$TEST_DIR" ]; then
    echo "✅ Write permissions OK"
    rm -rf "$TEST_DIR"
else
    echo "❌ Write permissions issue"
fi

echo "🔍 Troubleshooting complete!"
```

### 8. Best Practices

#### Performance Optimization

1. **Format Selection**:
   - Use JSON for debugging and development
   - Use Binary for production (2-3x faster)
   - Use Rkyv for maximum performance (3-5x faster)

2. **Worker Configuration**:
   - Use 16-32 workers for optimal parallel processing
   - Monitor CPU usage and adjust accordingly
   - Consider I/O bottlenecks with very high worker counts

3. **Memory Management**:
   - Binary format uses less memory than JSON
   - Rkyv format provides zero-copy operations
   - Monitor memory usage with large datasets

#### Error Handling

1. **Graceful Degradation**:
   - Always provide Python fallback
   - Check Rust binary availability before use
   - Handle conversion errors gracefully

2. **Validation**:
   - Always validate after format conversion
   - Use dry-run mode for safe testing
   - Keep backups before major conversions

3. **Monitoring**:
   - Log all operations for debugging
   - Monitor performance metrics
   - Set up health checks for production

## Conclusion

The enhanced sportball-sidecar-rust tool provides significant performance improvements while maintaining full compatibility with existing workflows. By following this implementation guide, you can seamlessly integrate the Rust tool into your sidecar processing pipeline and achieve 2-5x performance improvements.

For questions or support, refer to the main README.md or create an issue in the project repository.
