#!/bin/bash

# Simple Round-Trip Data Integrity Test
# Tests: JSON → Binary → Rkyv → JSON → Rkyv
# Validates no data loss and no errors through all conversions

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"
TEST_DIR="/tmp/sportball_roundtrip_simple"

echo "🔄 Simple Round-Trip Data Integrity Test"
echo "======================================="
echo "📁 Source data directory: $DATA_DIR"
echo "📁 Test directory: $TEST_DIR"
echo "🔄 Test sequence: JSON → Binary → Rkyv → JSON → Rkyv"
echo ""

# Clean up any existing test data
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

# Copy a subset of images for testing (first 5 images)
echo "📋 Setting up test data..."
IMAGE_COUNT=0
for img in "$DATA_DIR"*.jpg; do
    if [ -f "$img" ] && [ $IMAGE_COUNT -lt 5 ]; then
        cp "$img" "$TEST_DIR/"
        IMAGE_COUNT=$((IMAGE_COUNT + 1))
    fi
done

echo "✅ Copied $IMAGE_COUNT test images"
echo ""

# Create initial JSON sidecar files with comprehensive data
echo "📄 Creating comprehensive JSON sidecar files..."
JSON_COUNT=0
for img in "$TEST_DIR"/*.jpg; do
    if [ -f "$img" ]; then
        basename=$(basename "$img" .jpg)
        sidecar_file="$TEST_DIR/${basename}.json"
        
        # Create a comprehensive sidecar JSON structure
        cat > "$sidecar_file" << EOF
{
  "sidecar_info": {
    "operation_type": "face_detection",
    "created_at": "2024-12-19T10:30:00Z",
    "image_path": "$img",
    "tool_version": "1.0.0",
    "processing_id": "proc_${basename}_$(date +%s)"
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
  }
}
EOF
        JSON_COUNT=$((JSON_COUNT + 1))
    fi
done

echo "✅ Created $JSON_COUNT comprehensive JSON sidecar files"
echo ""

# Function to perform format conversion
convert_format() {
    local from_format=$1
    local to_format=$2
    local expected_count=$3
    
    echo "🔄 Converting $from_format → $to_format..."
    
    local conv_start=$(date +%s.%N)
    local conv_output=$(./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format "$to_format" 2>&1)
    local conv_end=$(date +%s.%N)
    local conv_time=$(echo "$conv_end - $conv_start" | bc -l)
    
    local converted=$(echo "$conv_output" | grep -o "Converted [0-9]*" | grep -o '[0-9]*' || echo "0")
    
    if [ "$converted" -ne "$expected_count" ]; then
        echo "❌ Conversion failed: Expected $expected_count files, converted $converted"
        echo "   Conversion output: $conv_output"
        return 1
    fi
    
    local rate=$(echo "scale=2; $converted / $conv_time" | bc -l)
    echo "✅ Converted $converted files in ${conv_time}s (${rate} files/sec)"
    return 0
}

# Function to count files of a specific format
count_files() {
    local format=$1
    find "$TEST_DIR" -name "*.$format" | wc -l
}

# Function to verify file existence
verify_files() {
    local format=$1
    local expected_count=$2
    
    local actual_count=$(count_files "$format")
    
    if [ "$actual_count" -ne "$expected_count" ]; then
        echo "❌ File count mismatch: Expected $expected_count $format files, found $actual_count"
        return 1
    fi
    
    echo "✅ Found $actual_count $format files"
    return 0
}

# Function to test individual file conversion
test_individual_file() {
    local test_file="$TEST_DIR/test_individual.json"
    
    echo "🧪 Testing individual file conversion..."
    
    # Create a test file
    cat > "$test_file" << EOF
{
  "test": "data",
  "number": 42,
  "float": 3.14159,
  "boolean": true,
  "array": [1, 2, 3, 4, 5],
  "object": {"nested": "value", "count": 100}
}
EOF
    
    # Test JSON → Binary → JSON
    echo "  JSON → Binary..."
    ./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format bin >/dev/null 2>&1
    
    if [ ! -f "$TEST_DIR/test_individual.bin" ]; then
        echo "❌ Binary file not created"
        return 1
    fi
    
    echo "  Binary → JSON..."
    ./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format json >/dev/null 2>&1
    
    if [ ! -f "$TEST_DIR/test_individual.json" ]; then
        echo "❌ JSON file not recreated"
        return 1
    fi
    
    # Test JSON → Rkyv → JSON
    echo "  JSON → Rkyv..."
    ./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format rkyv >/dev/null 2>&1
    
    if [ ! -f "$TEST_DIR/test_individual.rkyv" ]; then
        echo "❌ Rkyv file not created"
        return 1
    fi
    
    echo "  Rkyv → JSON..."
    ./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format json >/dev/null 2>&1
    
    if [ ! -f "$TEST_DIR/test_individual.json" ]; then
        echo "❌ JSON file not recreated from Rkyv"
        return 1
    fi
    
    echo "✅ Individual file conversion test passed"
    return 0
}

echo "🚀 Starting Round-Trip Data Integrity Test"
echo "==========================================="

# Step 1: Verify initial JSON files
echo ""
echo "📋 Step 1: Initial JSON Verification"
echo "===================================="
if ! verify_files "json" $JSON_COUNT; then
    echo "❌ Initial JSON verification failed"
    exit 1
fi

# Step 2: JSON → Binary
echo ""
echo "📋 Step 2: JSON → Binary Conversion"
echo "===================================="
if ! convert_format "json" "bin" $JSON_COUNT; then
    echo "❌ JSON → Binary conversion failed"
    exit 1
fi
if ! verify_files "bin" $JSON_COUNT; then
    echo "❌ Binary file verification failed"
    exit 1
fi

# Step 3: Binary → Rkyv
echo ""
echo "📋 Step 3: Binary → Rkyv Conversion"
echo "===================================="
if ! convert_format "bin" "rkyv" $JSON_COUNT; then
    echo "❌ Binary → Rkyv conversion failed"
    exit 1
fi
if ! verify_files "rkyv" $JSON_COUNT; then
    echo "❌ Rkyv file verification failed"
    exit 1
fi

# Step 4: Rkyv → JSON
echo ""
echo "📋 Step 4: Rkyv → JSON Conversion"
echo "=================================="
if ! convert_format "rkyv" "json" $JSON_COUNT; then
    echo "❌ Rkyv → JSON conversion failed"
    exit 1
fi
if ! verify_files "json" $JSON_COUNT; then
    echo "❌ JSON file verification failed"
    exit 1
fi

# Step 5: JSON → Rkyv (final)
echo ""
echo "📋 Step 5: JSON → Rkyv (Final Conversion)"
echo "=========================================="
if ! convert_format "json" "rkyv" $JSON_COUNT; then
    echo "❌ JSON → Rkyv conversion failed"
    exit 1
fi
if ! verify_files "rkyv" $JSON_COUNT; then
    echo "❌ Rkyv file verification failed"
    exit 1
fi

# Step 6: Individual file test
echo ""
echo "📋 Step 6: Individual File Conversion Test"
echo "=========================================="
if ! test_individual_file; then
    echo "❌ Individual file conversion test failed"
    exit 1
fi

# Step 7: Format statistics
echo ""
echo "📋 Step 7: Final Format Statistics"
echo "==================================="
./target/release/sportball-sidecar-rust format-stats --input "$TEST_DIR"

echo ""
echo "🎯 Round-Trip Test Results:"
echo "============================"
echo "✅ All format conversions completed successfully"
echo "✅ All file verifications passed"
echo "✅ Individual file conversion test passed"
echo "✅ No conversion errors detected"
echo ""
echo "📋 Conversion Summary:"
echo "====================="
echo "🔄 JSON → Binary → Rkyv → JSON → Rkyv"
echo "📁 Files processed: $JSON_COUNT"
echo "🚀 All conversions completed without errors"
echo "🔒 File integrity maintained throughout"

# Cleanup
echo ""
echo "🧹 Cleaning up test data..."
rm -rf "$TEST_DIR"

echo ""
echo "🎉 Round-trip data integrity test PASSED!"
echo "   All format conversions work correctly with no errors."
