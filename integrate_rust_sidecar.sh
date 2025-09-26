#!/bin/bash

# Sportball Sidecar Rust Integration Script
# This script helps integrate the Rust sidecar tool into existing workflows

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
RUST_TOOL_DIR="../sportball-sidecar-rust"
RUST_BINARY="$RUST_TOOL_DIR/target/release/sportball-sidecar-rust"
INSTALL_DIR="/usr/local/bin"
PYTHON_WRAPPER="image_sidecar_rust.py"

echo -e "${BLUE}🚀 Image Sidecar Rust Integration Script${NC}"
echo "================================================"

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "success")
            echo -e "${GREEN}✅ $message${NC}"
            ;;
        "warning")
            echo -e "${YELLOW}⚠️  $message${NC}"
            ;;
        "error")
            echo -e "${RED}❌ $message${NC}"
            ;;
        "info")
            echo -e "${BLUE}ℹ️  $message${NC}"
            ;;
    esac
}

# Check if Rust is installed
check_rust() {
    print_status "info" "Checking Rust installation..."
    
    if command -v rustc &> /dev/null; then
        print_status "success" "Rust installed: $(rustc --version)"
        return 0
    else
        print_status "error" "Rust not installed"
        echo "Install Rust from: https://rustup.rs/"
        return 1
    fi
}

# Build the Rust tool
build_rust_tool() {
    print_status "info" "Building Rust sidecar tool..."
    
    if [ ! -d "$RUST_TOOL_DIR" ]; then
        print_status "error" "Rust tool directory not found: $RUST_TOOL_DIR"
        echo "Please ensure the sportball-sidecar-rust directory exists"
        return 1
    fi
    
    cd "$RUST_TOOL_DIR"
    
    if cargo build --release; then
        print_status "success" "Rust tool built successfully"
        cd - > /dev/null
        return 0
    else
        print_status "error" "Failed to build Rust tool"
        cd - > /dev/null
        return 1
    fi
}

# Test the Rust binary
test_rust_binary() {
    print_status "info" "Testing Rust binary..."
    
    if [ ! -f "$RUST_BINARY" ]; then
        print_status "error" "Rust binary not found: $RUST_BINARY"
        return 1
    fi
    
    if [ ! -x "$RUST_BINARY" ]; then
        print_status "warning" "Making binary executable..."
        chmod +x "$RUST_BINARY"
    fi
    
    if "$RUST_BINARY" --help &> /dev/null; then
        print_status "success" "Rust binary working correctly"
        return 0
    else
        print_status "error" "Rust binary not working"
        return 1
    fi
}

# Install the binary system-wide
install_binary() {
    print_status "info" "Installing binary system-wide..."
    
    if [ ! -f "$RUST_BINARY" ]; then
        print_status "error" "Rust binary not found: $RUST_BINARY"
        return 1
    fi
    
    if sudo cp "$RUST_BINARY" "$INSTALL_DIR/"; then
        print_status "success" "Binary installed to $INSTALL_DIR"
        return 0
    else
        print_status "error" "Failed to install binary"
        return 1
    fi
}

# Create Python wrapper
create_python_wrapper() {
    print_status "info" "Creating Python integration wrapper..."
    
    if [ -f "$PYTHON_WRAPPER" ]; then
        print_status "success" "Python wrapper already exists: $PYTHON_WRAPPER"
        return 0
    else
        print_status "error" "Python wrapper not found: $PYTHON_WRAPPER"
        echo "Please ensure image_sidecar_rust.py exists in the current directory"
        return 1
    fi
}

# Test Python integration
test_python_integration() {
    print_status "info" "Testing Python integration..."
    
    if [ ! -f "$PYTHON_WRAPPER" ]; then
        print_status "error" "Python wrapper not found: $PYTHON_WRAPPER"
        return 1
    fi
    
    # Create a test directory with sample files
    TEST_DIR="/tmp/sidecar_test_$$"
    mkdir -p "$TEST_DIR"
    
    # Create test sidecar files
    echo '{"sidecar_info": {"operation_type": "face_detection"}, "faces": []}' > "$TEST_DIR/test1.json"
    echo '{"sidecar_info": {"operation_type": "object_detection"}, "objects": []}' > "$TEST_DIR/test2.json"
    
    # Test Python integration
    if python3 -c "
import sys
sys.path.insert(0, '.')
from image_sidecar_rust import create_manager
manager = create_manager('$RUST_BINARY')
if manager.rust_available:
    print('✅ Python integration working')
    results = manager.validate_sidecars('$TEST_DIR')
    print(f'Validated {len(results)} files')
else:
    print('⚠️  Rust binary not available in Python')
"; then
        print_status "success" "Python integration working"
        rm -rf "$TEST_DIR"
        return 0
    else
        print_status "error" "Python integration failed"
        rm -rf "$TEST_DIR"
        return 1
    fi
}

# Create Makefile integration
create_makefile_integration() {
    print_status "info" "Creating Makefile integration..."
    
    MAKEFILE_INTEGRATION="Makefile.rust-sidecar"
    
    cat > "$MAKEFILE_INTEGRATION" << EOF
# Rust Sidecar Integration Makefile
# Add these targets to your existing Makefile

# Rust tool paths
RUST_TOOL_DIR = $RUST_TOOL_DIR
RUST_BINARY = \$(RUST_TOOL_DIR)/target/release/sportball-sidecar-rust

# Build Rust tool
.PHONY: build-rust-sidecar
build-rust-sidecar:
	cd \$(RUST_TOOL_DIR) && cargo build --release

# Validate sidecars with Rust
.PHONY: validate-rust
validate-rust: build-rust-sidecar
	\$(RUST_BINARY) validate --input \$(SIDECAR_DIR) --workers 16

# Get statistics with Rust
.PHONY: stats-rust
stats-rust: build-rust-sidecar
	\$(RUST_BINARY) stats --input \$(SIDECAR_DIR)

# Convert to binary format
.PHONY: convert-binary
convert-binary: build-rust-sidecar
	\$(RUST_BINARY) convert --input \$(SIDECAR_DIR) --format bin

# Convert to Rkyv format
.PHONY: convert-rkyv
convert-rkyv: build-rust-sidecar
	\$(RUST_BINARY) convert --input \$(SIDECAR_DIR) --format rkyv

# Show format statistics
.PHONY: format-stats
format-stats: build-rust-sidecar
	\$(RUST_BINARY) format-stats --input \$(SIDECAR_DIR)

# Performance benchmark
.PHONY: benchmark-rust
benchmark-rust: build-rust-sidecar
	cd \$(RUST_TOOL_DIR) && cargo bench

# Test integration
.PHONY: test-integration
test-integration: build-rust-sidecar
	\$(RUST_BINARY) validate --input \$(SIDECAR_DIR) --workers 8
	\$(RUST_BINARY) stats --input \$(SIDECAR_DIR)
	\$(RUST_BINARY) format-stats --input \$(SIDECAR_DIR)

# Clean up orphaned sidecars
.PHONY: cleanup-rust
cleanup-rust: build-rust-sidecar
	\$(RUST_BINARY) cleanup --input \$(SIDECAR_DIR) --dry-run
	\$(RUST_BINARY) cleanup --input \$(SIDECAR_DIR)

# Export sidecar data
.PHONY: export-rust
export-rust: build-rust-sidecar
	\$(RUST_BINARY) export --input \$(SIDECAR_DIR) --output export.json --format json
EOF

    print_status "success" "Makefile integration created: $MAKEFILE_INTEGRATION"
    echo "Add the contents of $MAKEFILE_INTEGRATION to your existing Makefile"
}

# Create shell script integration
create_shell_integration() {
    print_status "info" "Creating shell script integration..."
    
    SHELL_INTEGRATION="sidecar_rust.sh"
    
    cat > "$SHELL_INTEGRATION" << EOF
#!/bin/bash
# Rust Sidecar Integration Script
# Usage: ./sidecar_rust.sh <command> <directory> [options]

RUST_BINARY="$RUST_BINARY"

# Check if Rust binary exists
if [ ! -f "\$RUST_BINARY" ]; then
    echo "❌ Rust binary not found: \$RUST_BINARY"
    echo "Run the integration script first: ./integrate_rust_sidecar.sh"
    exit 1
fi

# Function to show usage
show_usage() {
    echo "Usage: \$0 <command> <directory> [options]"
    echo ""
    echo "Commands:"
    echo "  validate [workers]     - Validate sidecar files"
    echo "  stats                 - Get statistics"
    echo "  convert <format>      - Convert format (json|bin|rkyv)"
    echo "  format-stats          - Show format distribution"
    echo "  cleanup [dry-run]     - Clean up orphaned files"
    echo "  export <file>         - Export data to file"
    echo ""
    echo "Examples:"
    echo "  \$0 validate /path/to/sidecars 16"
    echo "  \$0 convert /path/to/sidecars bin"
    echo "  \$0 stats /path/to/sidecars"
}

# Main script logic
case "\$1" in
    "validate")
        WORKERS=\${3:-16}
        "\$RUST_BINARY" validate --input "\$2" --workers "\$WORKERS"
        ;;
    "stats")
        "\$RUST_BINARY" stats --input "\$2"
        ;;
    "convert")
        if [ -z "\$3" ]; then
            echo "❌ Format required for convert command"
            echo "Supported formats: json, bin, rkyv"
            exit 1
        fi
        "\$RUST_BINARY" convert --input "\$2" --format "\$3"
        ;;
    "format-stats")
        "\$RUST_BINARY" format-stats --input "\$2"
        ;;
    "cleanup")
        if [ "\$3" = "dry-run" ]; then
            "\$RUST_BINARY" cleanup --input "\$2" --dry-run
        else
            "\$RUST_BINARY" cleanup --input "\$2"
        fi
        ;;
    "export")
        if [ -z "\$3" ]; then
            echo "❌ Output file required for export command"
            exit 1
        fi
        "\$RUST_BINARY" export --input "\$2" --output "\$3" --format json
        ;;
    *)
        show_usage
        exit 1
        ;;
esac
EOF

    chmod +x "$SHELL_INTEGRATION"
    print_status "success" "Shell integration created: $SHELL_INTEGRATION"
}

# Main integration function
main() {
    echo "Starting integration process..."
    echo ""
    
    # Check prerequisites
    if ! check_rust; then
        exit 1
    fi
    
    # Build the tool
    if ! build_rust_tool; then
        exit 1
    fi
    
    # Test the binary
    if ! test_rust_binary; then
        exit 1
    fi
    
    # Ask about system installation
    read -p "Install binary system-wide? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        install_binary
    fi
    
    # Create Python wrapper
    create_python_wrapper
    
    # Test Python integration
    test_python_integration
    
    # Create integration files
    create_makefile_integration
    create_shell_integration
    
    echo ""
    print_status "success" "Integration complete!"
    echo ""
    echo "Next steps:"
    echo "1. Add Makefile targets to your existing Makefile"
    echo "2. Use the Python wrapper: python_integration.py"
    echo "3. Use the shell script: ./sidecar_rust.sh"
    echo "4. Test with your data: ./sidecar_rust.sh validate /path/to/sidecars"
    echo ""
    echo "For more information, see:"
    echo "- IMPLEMENTATION_GUIDE.md"
    echo "- QUICK_REFERENCE.md"
}

# Run main function
main "$@"
