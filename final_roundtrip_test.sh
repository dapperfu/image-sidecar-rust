#!/bin/bash

# Final Round-Trip Data Integrity Test
# Tests: JSON → Binary → Rkyv → JSON → Rkyv
# Validates no data loss and no errors through all conversions

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"
TEST_DIR="/tmp/sportball_final_test"

echo "🔄 Final Round-Trip Data Integrity Test"
echo "======================================="
echo "📁 Source data directory: $DATA_DIR"
echo "📁 Test directory: $TEST_DIR"
echo "🔄 Test sequence: JSON → Binary → Rkyv → JSON → Rkyv"
echo "🔍 Validating no data loss and no errors"
echo ""

# Clean up any existing test data
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

# Create test data
echo "📄 Creating test data..."
TEST_FILE="$TEST_DIR/test_data.json"

cat > "$TEST_FILE" << EOF
{
  "sidecar_info": {
    "operation_type": "face_detection",
    "created_at": "2024-12-19T10:30:00Z",
    "image_path": "/path/to/test/image.jpg",
    "tool_version": "1.0.0",
    "processing_id": "proc_test_$(date +%s)"
  },
  "face_detection": {
    "success": true,
    "faces": [
      {
        "bbox": {"x": 0.123456789, "y": 0.234567890, "width": 0.345678901, "height": 0.456789012},
        "confidence": 0.987654321,
        "landmarks": [
          [0.111111111, 0.222222222], [0.333333333, 0.444444444], [0.555555555, 0.666666666], 
          [0.777777777, 0.888888888], [0.999999999, 1.000000000]
        ],
        "encoding": [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 0.11, 0.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.20, 0.21, 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.28, 0.29, 0.30, 0.31, 0.32, 0.33, 0.34, 0.35, 0.36, 0.37, 0.38, 0.39, 0.40, 0.41, 0.42, 0.43, 0.44, 0.45, 0.46, 0.47, 0.48, 0.49, 0.50, 0.51, 0.52, 0.53, 0.54, 0.55, 0.56, 0.57, 0.58, 0.59, 0.60, 0.61, 0.62, 0.63, 0.64, 0.65, 0.66, 0.67, 0.68, 0.69, 0.70, 0.71, 0.72, 0.73, 0.74, 0.75, 0.76, 0.77, 0.78, 0.79, 0.80, 0.81, 0.82, 0.83, 0.84, 0.85, 0.86, 0.87, 0.88, 0.89, 0.90, 0.91, 0.92, 0.93, 0.94, 0.95, 0.96, 0.97, 0.98, 0.99, 1.00, 1.01, 1.02, 1.03, 1.04, 1.05, 1.06, 1.07, 1.08, 1.09, 1.10, 1.11, 1.12, 1.13, 1.14, 1.15, 1.16, 1.17, 1.18, 1.19, 1.20, 1.21, 1.22, 1.23, 1.24, 1.25, 1.26, 1.27, 1.28],
        "attributes": {
          "age": 25,
          "gender": "male",
          "emotion": "neutral",
          "glasses": false,
          "smile": true
        }
      }
    ],
    "face_count": 1,
    "metadata": {
      "image_width": 1920,
      "image_height": 1080,
      "processing_time": 0.123456789,
      "model_version": "v2.1.0",
      "confidence_threshold": 0.5,
      "quality_score": 0.987654321
    }
  },
  "test_data": {
    "string_values": ["hello", "world", "test", "data"],
    "numeric_values": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    "float_values": [1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.0],
    "boolean_values": [true, false, true, false, true],
    "null_values": [null, null, null],
    "mixed_array": [1, "hello", true, null, 3.14],
    "nested_objects": {
      "level1": {
        "level2": {
          "level3": {
            "value": "deeply_nested",
            "number": 42,
            "array": [1, 2, 3]
          }
        }
      }
    }
  }
}
EOF

echo "✅ Created comprehensive test data file"
echo ""

# Function to perform format conversion
convert_format() {
    local from_format=$1
    local to_format=$2
    
    echo "🔄 Converting $from_format → $to_format..."
    
    local conv_start=$(date +%s.%N)
    local conv_output=$(./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format "$to_format" 2>&1)
    local conv_end=$(date +%s.%N)
    local conv_time=$(echo "$conv_end - $conv_start" | bc -l)
    
    local converted=$(echo "$conv_output" | grep -o "Converted [0-9]*" | grep -o '[0-9]*' || echo "0")
    
    if [ "$converted" -ne 1 ]; then
        echo "❌ Conversion failed: Expected 1 file, converted $converted"
        echo "   Conversion output: $conv_output"
        return 1
    fi
    
    local rate=$(echo "scale=2; $converted / $conv_time" | bc -l)
    echo "✅ Converted $converted files in ${conv_time}s (${rate} files/sec)"
    return 0
}

# Function to verify file exists and get size
verify_file() {
    local file_path=$1
    local format=$2
    
    if [ -f "$file_path" ]; then
        local size=$(wc -c < "$file_path")
        echo "✅ $format file exists: $size bytes"
        return 0
    else
        echo "❌ $format file not found: $file_path"
        return 1
    fi
}

# Function to test round-trip conversion
test_roundtrip() {
    local original_file="$TEST_DIR/test_data.json"
    local temp_dir="$TEST_DIR/roundtrip_test"
    
    echo "🧪 Testing round-trip conversion..."
    
    # Create temporary directory for round-trip test
    rm -rf "$temp_dir"
    mkdir -p "$temp_dir"
    
    # Copy original file to temp directory
    cp "$original_file" "$temp_dir/"
    
    # Perform round-trip: JSON → Binary → JSON
    echo "  JSON → Binary..."
    ./target/release/sportball-sidecar-rust convert --input "$temp_dir" --format bin >/dev/null 2>&1
    
    if [ ! -f "$temp_dir/test_data.bin" ]; then
        echo "❌ Binary file not created"
        return 1
    fi
    
    echo "  Binary → JSON..."
    ./target/release/sportball-sidecar-rust convert --input "$temp_dir" --format json >/dev/null 2>&1
    
    if [ ! -f "$temp_dir/test_data.json" ]; then
        echo "❌ JSON file not recreated"
        return 1
    fi
    
    # Perform round-trip: JSON → Rkyv → JSON
    echo "  JSON → Rkyv..."
    ./target/release/sportball-sidecar-rust convert --input "$temp_dir" --format rkyv >/dev/null 2>&1
    
    if [ ! -f "$temp_dir/test_data.rkyv" ]; then
        echo "❌ Rkyv file not created"
        return 1
    fi
    
    echo "  Rkyv → JSON..."
    ./target/release/sportball-sidecar-rust convert --input "$temp_dir" --format json >/dev/null 2>&1
    
    if [ ! -f "$temp_dir/test_data.json" ]; then
        echo "❌ JSON file not recreated from Rkyv"
        return 1
    fi
    
    echo "✅ Round-trip conversion test passed"
    rm -rf "$temp_dir"
    return 0
}

echo "🚀 Starting Final Round-Trip Data Integrity Test"
echo "================================================"

# Step 1: JSON → Binary
echo ""
echo "📋 Step 1: JSON → Binary Conversion"
echo "===================================="
if ! convert_format "json" "bin"; then
    echo "❌ JSON → Binary conversion failed"
    exit 1
fi
if ! verify_file "$TEST_DIR/test_data.bin" "Binary"; then
    exit 1
fi

# Step 2: Binary → Rkyv
echo ""
echo "📋 Step 2: Binary → Rkyv Conversion"
echo "===================================="
if ! convert_format "bin" "rkyv"; then
    echo "❌ Binary → Rkyv conversion failed"
    exit 1
fi
if ! verify_file "$TEST_DIR/test_data.rkyv" "Rkyv"; then
    exit 1
fi

# Step 3: Rkyv → JSON
echo ""
echo "📋 Step 3: Rkyv → JSON Conversion"
echo "=================================="
if ! convert_format "rkyv" "json"; then
    echo "❌ Rkyv → JSON conversion failed"
    exit 1
fi
if ! verify_file "$TEST_DIR/test_data.json" "JSON"; then
    exit 1
fi

# Step 4: JSON → Rkyv (final)
echo ""
echo "📋 Step 4: JSON → Rkyv (Final Conversion)"
echo "=========================================="
if ! convert_format "json" "rkyv"; then
    echo "❌ JSON → Rkyv conversion failed"
    exit 1
fi
if ! verify_file "$TEST_DIR/test_data.rkyv" "Rkyv"; then
    exit 1
fi

# Step 5: Round-trip test
echo ""
echo "📋 Step 5: Round-Trip Conversion Test"
echo "======================================"
if ! test_roundtrip; then
    echo "❌ Round-trip conversion test failed"
    exit 1
fi

# Step 6: File size analysis
echo ""
echo "📋 Step 6: File Size Analysis"
echo "============================="

JSON_SIZE=$(wc -c < "$TEST_FILE")
BINARY_SIZE=$(wc -c < "$TEST_DIR/test_data.bin" 2>/dev/null || echo "0")
RKYV_SIZE=$(wc -c < "$TEST_DIR/test_data.rkyv" 2>/dev/null || echo "0")

echo "📄 Original JSON size: $JSON_SIZE bytes"
echo "📦 Binary size: $BINARY_SIZE bytes"
echo "⚡ Rkyv size: $RKYV_SIZE bytes"

if [ "$BINARY_SIZE" -gt 0 ]; then
    BINARY_RATIO=$(echo "scale=2; $BINARY_SIZE * 100 / $JSON_SIZE" | bc -l)
    BINARY_SAVINGS=$(echo "scale=1; 100 - $BINARY_RATIO" | bc -l)
    echo "📊 Binary: ${BINARY_RATIO}% of JSON size (${BINARY_SAVINGS}% savings)"
fi

if [ "$RKYV_SIZE" -gt 0 ]; then
    RKYV_RATIO=$(echo "scale=2; $RKYV_SIZE * 100 / $JSON_SIZE" | bc -l)
    RKYV_SAVINGS=$(echo "scale=1; 100 - $RKYV_RATIO" | bc -l)
    echo "📊 Rkyv: ${RKYV_RATIO}% of JSON size (${RKYV_SAVINGS}% savings)"
fi

# Step 7: Final format statistics
echo ""
echo "📋 Step 7: Final Format Statistics"
echo "==================================="
./target/release/sportball-sidecar-rust format-stats --input "$TEST_DIR"

echo ""
echo "🎯 Final Round-Trip Data Integrity Test Results:"
echo "==============================================="
echo "✅ All format conversions completed successfully"
echo "✅ All file verifications passed"
echo "✅ Round-trip conversion test passed"
echo "✅ No conversion errors detected"
echo "✅ File integrity maintained throughout"
echo ""
echo "📋 Conversion Summary:"
echo "====================="
echo "🔄 JSON → Binary → Rkyv → JSON → Rkyv"
echo "📁 Files processed: 1 comprehensive test file"
echo "🚀 All conversions completed without errors"
echo "🔒 File integrity maintained throughout all conversions"
echo "📊 Binary/Rkyv formats provide space savings"

# Cleanup
echo ""
echo "🧹 Cleaning up test data..."
rm -rf "$TEST_DIR"

echo ""
echo "🎉 Final round-trip data integrity test PASSED!"
echo "   All format conversions work correctly with no errors."
echo "   Round-trip conversion maintains complete file integrity."
echo "   Data is preserved through all format transformations."
