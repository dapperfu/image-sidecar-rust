# Image Sidecar Rust - Quick Reference

## Installation & Setup

```bash
# Build the tool
cargo build --release

# Verify installation
./target/release/image-sidecar-rust --help
```

## Common Operations

### Validation
```bash
# Basic validation
./target/release/image-sidecar-rust validate --input /path/to/sidecars

# With parallel workers
./target/release/image-sidecar-rust validate --input /path/to/sidecars --workers 16

# Save results to file
./target/release/image-sidecar-rust validate --input /path/to/sidecars --output results.json
```

### Statistics
```bash
# Get comprehensive statistics
./target/release/image-sidecar-rust stats --input /path/to/sidecars

# Save statistics to file
./target/release/image-sidecar-rust stats --input /path/to/sidecars --output stats.json
```

### Format Conversion
```bash
# Dry run (safe testing)
./target/release/image-sidecar-rust convert --input /path/to/sidecars --format bin --dry-run

# Convert to binary format
./target/release/image-sidecar-rust convert --input /path/to/sidecars --format bin

# Convert to Rkyv format
./target/release/image-sidecar-rust convert --input /path/to/sidecars --format rkyv

# Convert back to JSON
./target/release/image-sidecar-rust convert --input /path/to/sidecars --format json
```

### Format Analysis
```bash
# Show format distribution
./target/release/image-sidecar-rust format-stats --input /path/to/sidecars

# Save format statistics
./target/release/image-sidecar-rust format-stats --input /path/to/sidecars --output format_report.json
```

### Cleanup
```bash
# Dry run cleanup
./target/release/image-sidecar-rust cleanup --input /path/to/sidecars --dry-run

# Remove orphaned sidecars
./target/release/image-sidecar-rust cleanup --input /path/to/sidecars
```

### Export
```bash
# Export to JSON
./target/release/image-sidecar-rust export --input /path/to/sidecars --output export.json --format json
```

## Supported Formats

| Format | Extension | Use Case | Performance |
|--------|-----------|----------|-------------|
| JSON   | `.json`   | Debugging, human-readable | Baseline |
| Binary | `.bin`    | Production, balanced | 2-3x faster |
| Rkyv   | `.rkyv`   | Maximum performance | 3-5x faster |

## Performance Tips

### Worker Configuration
- **1-4 workers**: Small datasets, limited CPU
- **8-16 workers**: Medium datasets, balanced performance
- **16-32 workers**: Large datasets, high-performance systems
- **32+ workers**: Very large datasets, high-end systems

### Format Selection
- **Development**: Use JSON for easy debugging
- **Production**: Use Binary for optimal performance
- **High-throughput**: Use Rkyv for maximum speed

### Memory Optimization
- Binary format uses ~30-50% less memory than JSON
- Rkyv format provides zero-copy operations
- Monitor memory usage with large datasets

## Integration Examples

### Python Integration
```python
import subprocess
import json

def validate_sidecars(directory, workers=16):
    cmd = [
        "./target/release/image-sidecar-rust",
        "validate", "--input", directory,
        "--workers", str(workers), "--output", "-"
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(result.stdout)

# Usage
results = validate_sidecars("/path/to/sidecars")
print(f"Validated {results['total_files']} files")
```

### Bash Script Integration
```bash
#!/bin/bash
SIDECAR_DIR="/path/to/sidecars"
RUST_TOOL="./target/release/image-sidecar-rust"

# Validate sidecars
echo "Validating sidecars..."
$RUST_TOOL validate --input "$SIDECAR_DIR" --workers 16

# Get statistics
echo "Getting statistics..."
$RUST_TOOL stats --input "$SIDECAR_DIR"

# Convert to binary for performance
echo "Converting to binary format..."
$RUST_TOOL convert --input "$SIDECAR_DIR" --format bin
```

### Makefile Integration
```makefile
# Add to your Makefile
RUST_TOOL = ./target/release/image-sidecar-rust
SIDECAR_DIR = /path/to/sidecars

validate-sidecars:
	$(RUST_TOOL) validate --input $(SIDECAR_DIR) --workers 16

convert-binary:
	$(RUST_TOOL) convert --input $(SIDECAR_DIR) --format bin

stats-sidecars:
	$(RUST_TOOL) stats --input $(SIDECAR_DIR)
```

## Troubleshooting

### Common Issues

1. **Binary not found**
   ```bash
   # Build the tool
   cargo build --release
   ```

2. **Permission denied**
   ```bash
   # Make binary executable
   chmod +x ./target/release/image-sidecar-rust
   ```

3. **No files found**
   ```bash
   # Check directory path
   ls -la /path/to/sidecars
   
   # Check supported file extensions
   find /path/to/sidecars -name "*.json" -o -name "*.bin" -o -name "*.rkyv"
   ```

4. **Conversion errors**
   ```bash
   # Use dry run first
   ./target/release/image-sidecar-rust convert --input /path/to/sidecars --format bin --dry-run
   
   # Check file permissions
   ls -la /path/to/sidecars
   ```

### Performance Issues

1. **Slow processing**
   - Increase worker count: `--workers 32`
   - Convert to binary format: `--format bin`
   - Check disk I/O performance

2. **High memory usage**
   - Use binary format instead of JSON
   - Reduce worker count
   - Process in smaller batches

3. **Conversion failures**
   - Check file permissions
   - Ensure sufficient disk space
   - Validate source files first

## Output Examples

### Validation Output
```json
{
  "total_files": 100,
  "valid_files": 98,
  "invalid_files": 2,
  "results": [
    {
      "file_path": "sidecar1.json",
      "is_valid": true,
      "processing_time": 0.001234,
      "file_size": 1024,
      "operation_type": "FaceDetection"
    }
  ]
}
```

### Statistics Output
```json
{
  "directory": "/path/to/sidecars",
  "total_images": 100,
  "total_sidecars": 98,
  "coverage_percentage": 98.0,
  "operation_counts": {
    "face_detection": 45,
    "object_detection": 30,
    "quality_assessment": 23
  }
}
```

### Format Statistics Output
```json
{
  "directory": "/path/to/sidecars",
  "format_distribution": {
    "Json": 50,
    "Binary": 30,
    "Rkyv": 18
  },
  "total_files": 98,
  "generated_at": "2024-12-19T10:30:00Z"
}
```

## Command Line Options

### Global Options
- `--help`: Show help information
- `--version`: Show version information

### Common Options
- `--input, -i`: Input directory path
- `--output, -o`: Output file path (use `-` for stdout)
- `--workers, -w`: Number of parallel workers
- `--dry-run`: Show what would be done without making changes

### Format Options
- `--format`: Target format (json, bin, rkyv)
- `--operation-type`: Filter by operation type

## Performance Benchmarks

Typical performance improvements over Python:

| Operation | JSON | Binary | Rkyv |
|-----------|------|--------|------|
| Validation | 1x | 2-3x | 3-5x |
| Statistics | 1x | 2-3x | 3-5x |
| File Size | 100% | 30-50% | 20-40% |
| Memory Usage | 100% | 50-70% | 30-50% |

*Benchmarks may vary based on system configuration and data characteristics.*
