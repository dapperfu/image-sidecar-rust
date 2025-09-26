# Sportball Sidecar Rust

High-performance Rust implementation for sportball JSON sidecar operations.

## Overview

This tool provides massively parallel JSON validation and sidecar file management for the sportball photography pipeline. It's designed to be significantly faster than Python implementations while maintaining full compatibility with existing sidecar file formats.

**Status**: ✅ **FULLY IMPLEMENTED** - Ready for production use

This implementation was created to address performance bottlenecks in the Python sidecar handling code, providing 3-10x performance improvements through:
- Massive parallelism using rayon
- Zero-copy operations
- Efficient async I/O
- Memory safety guarantees
- SIMD optimizations

## Features

- **Multi-Format Support**: JSON, Binary (.bin), and Rkyv (.rkyv) formats with automatic detection
- **Format Conversion**: Convert between formats with dry-run support for safe testing
- **Massive Parallelism**: Uses rayon for data parallelism across CPU cores
- **Zero-Copy Operations**: Minimizes memory allocations and copying
- **Efficient I/O**: Uses async I/O for better throughput
- **Memory Safety**: Compile-time guarantees prevent common errors
- **SIMD Optimizations**: Leverages CPU vector instructions
- **Full Compatibility**: Works with existing sportball sidecar formats

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd sportball-sidecar-rust

# Build the release binary
cargo build --release

# The binary will be available at:
# ./target/release/sportball-sidecar-rust
```

### Python Integration

The Rust tool is automatically integrated with the existing sportball Python codebase. The Python wrapper (`rust_sidecar.py`) automatically detects and uses the Rust binary when available, falling back to Python implementations when needed.

```bash
# From the sportball_photography directory
cd /tank/sportball/sportball_photography

# Build the Rust tool
make build-rust-sidecar

# Test the integration
make test-integration
```

## Usage

### CLI Interface

```bash
# Validate JSON sidecar files in parallel
./target/release/sportball-sidecar-rust validate --input /path/to/directory --workers 32

# Get comprehensive statistics
./target/release/sportball-sidecar-rust stats --input /path/to/directory

# Clean up orphaned sidecar files (dry run first)
./target/release/sportball-sidecar-rust cleanup --input /path/to/directory --dry-run
./target/release/sportball-sidecar-rust cleanup --input /path/to/directory

# Export sidecar data to JSON
./target/release/sportball-sidecar-rust export --input /path/to/directory --output results.json --format json

# Filter by operation type
./target/release/sportball-sidecar-rust validate --input /path/to/directory --operation-type face_detection
./target/release/sportball-sidecar-rust stats --input /path/to/directory --operation-type object_detection

# Convert sidecar files between formats (dry run first)
./target/release/sportball-sidecar-rust convert --input /path/to/directory --format bin --dry-run
./target/release/sportball-sidecar-rust convert --input /path/to/directory --format bin
./target/release/sportball-sidecar-rust convert --input /path/to/directory --format rkyv

# Show format statistics
./target/release/sportball-sidecar-rust format-stats --input /path/to/directory --output format_report.json
```

### Library Usage

```rust
use sportball_sidecar_rust::SportballSidecar;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sidecar = SportballSidecar::new(Some(16));
    
    // Validate sidecar files
    let results = sidecar.validate_sidecars(&Path::new("/path/to/directory")).await?;
    
    // Get statistics
    let stats = sidecar.get_statistics(&Path::new("/path/to/directory")).await?;
    
    Ok(())
}
```

### Python Integration

The Rust tool is automatically integrated with the existing sportball Python codebase:

```python
# Automatic integration - uses Rust when available
from sportball.sidecar import Sidecar

sidecar = Sidecar()  # Automatically uses Rust implementation
stats = sidecar.get_statistics(directory)
results = sidecar.validate_sidecars(directory)

# Direct Rust usage
from sportball.detection.rust_sidecar import RustSidecarManager

rust_manager = RustSidecarManager()
results = rust_manager.validate_sidecars(directory)
stats = rust_manager.get_statistics(directory)

# Check if Rust is available
print(f"Rust available: {rust_manager.rust_available}")
print(f"Performance info: {rust_manager.get_performance_info()}")
```

## Performance

The Rust implementation provides significant performance improvements:

- **3-10x faster** JSON validation compared to Python
- **Massive parallelism** across all CPU cores
- **Lower memory usage** through zero-copy operations
- **Better I/O performance** with async operations

## Architecture

```
sportball-sidecar-rust/
├── src/
│   ├── lib.rs              # Main library interface
│   ├── main.rs             # CLI interface with clap
│   ├── sidecar/            # Core sidecar management
│   │   ├── mod.rs          # Module exports
│   │   ├── manager.rs      # SidecarManager implementation
│   │   ├── types.rs        # Type definitions (OperationType, SidecarInfo, etc.)
│   │   └── operations.rs   # CRUD operations
│   ├── parallel/           # Parallel processing
│   │   ├── mod.rs          # Module exports
│   │   └── processor.rs    # ParallelProcessor with rayon
│   └── utils/              # Utilities
│       ├── mod.rs          # Module exports
│       └── json.rs         # JSON utilities
├── benches/                # Performance benchmarks
│   └── json_validation.rs  # Criterion benchmarks
├── tests/                  # Integration tests
│   └── integration_tests.rs # Comprehensive test suite
├── Cargo.toml              # Rust dependencies and configuration
├── Makefile                # Build system integration
├── LICENSE                 # MIT License
└── README.md               # This documentation
```

### Key Components

- **`SidecarManager`**: Core sidecar file management with symlink resolution
- **`ParallelProcessor`**: High-performance parallel JSON validation using rayon
- **`SportballSidecar`**: Main library interface combining manager and processor
- **CLI Interface**: Complete command-line tool with validate, stats, cleanup, export commands
- **Type System**: Comprehensive type definitions for all sidecar operations
- **Error Handling**: Robust error handling with fallback mechanisms

## Supported Operations

- **Face Detection**: `face_detection`
- **Object Detection**: `object_detection`
- **Ball Detection**: `ball_detection`
- **Quality Assessment**: `quality_assessment`
- **Game Detection**: `game_detection`
- **YOLOv8**: `yolov8`

## Supported Formats

- **JSON** (`.json`): Human-readable, slower, best for debugging
- **Binary** (`.bin`): Fast, compact, good balance of speed and compatibility
- **Rkyv** (`.rkyv`): Zero-copy, fastest, most efficient for high-performance scenarios

The tool automatically detects format from file extension and prioritizes more efficient formats when searching for sidecar files.

## Compatibility

This tool is fully compatible with existing sportball sidecar file formats and can be used as a drop-in replacement for Python implementations.

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy --all-targets --all-features -- -D warnings
```

### Testing

```bash
# Run all tests
cargo test

# Run tests in release mode
cargo test --release

# Run specific test
cargo test test_sidecar_creation

# Run integration tests
cargo test --test integration_tests
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench json_validation

# Run benchmarks in release mode
cargo bench --release
```

### Makefile Integration

The project includes a comprehensive Makefile for development:

```bash
# Build release binary
make build

# Run tests
make test

# Run benchmarks
make bench

# Run clippy and formatting
make lint

# Clean build artifacts
make clean

# Install binary to system
make install

# Generate documentation
make docs
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Integration with Sportball

This Rust tool is fully integrated with the existing sportball Python codebase:

### Python Integration Points

1. **`sportball/detection/rust_sidecar.py`**: Python wrapper that automatically detects and uses the Rust binary
2. **`sportball/sidecar.py`**: Updated to use Rust implementation when available
3. **`sportball/detection/parallel_validator.py`**: Falls back to Rust for validation
4. **Makefile integration**: Build and test targets for the Rust tool

### Automatic Fallback

The Python code automatically falls back to Python implementations when:
- Rust binary is not available
- Rust binary fails to execute
- Network or system issues occur

### Performance Monitoring

```python
from sportball.detection.rust_sidecar import RustSidecarManager

manager = RustSidecarManager()
print(f"Rust available: {manager.rust_available}")
print(f"Performance info: {manager.get_performance_info()}")
```

## Migration Guide

### For Existing Code

No changes required! The existing Python code automatically uses the Rust implementation when available:

```python
# This code automatically uses Rust when available
from sportball.sidecar import Sidecar

sidecar = Sidecar()
stats = sidecar.get_statistics(directory)  # Uses Rust if available
```

### For New Projects

```python
# Direct Rust usage for maximum performance
from sportball.detection.rust_sidecar import RustSidecarManager

rust_manager = RustSidecarManager()
results = rust_manager.validate_sidecars(directory)
```

## Troubleshooting

### Rust Binary Not Found

1. Build the Rust tool: `make build-rust-sidecar`
2. Check PATH: `which sportball-sidecar-rust`
3. Verify binary exists: `ls -la ../sportball-sidecar-rust/target/release/`

### Performance Issues

1. Ensure release build: `cargo build --release`
2. Check worker count: Use `--workers` parameter
3. Monitor system resources: CPU, memory, disk I/O

### Integration Issues

1. Test integration: `make test-integration`
2. Check Python imports: `python -c "from sportball.detection.rust_sidecar import RustSidecarManager"`
3. Verify fallback: Disable Rust and test Python fallback

## Support

For issues and questions:
1. Check the troubleshooting section above
2. Review the integration tests: `make test-integration`
3. Open an issue on the GitHub repository
4. Check the sportball documentation for Python-specific issues
