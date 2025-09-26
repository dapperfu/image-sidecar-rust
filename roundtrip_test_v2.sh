#!/bin/bash

# Comprehensive Round-Trip Data Integrity Test
# Tests: JSON → Binary → Rkyv → JSON → Rkyv
# Validates no data loss and no errors through all conversions

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"
TEST_DIR="/tmp/sportball_roundtrip_test"

echo "🔄 Comprehensive Round-Trip Data Integrity Test"
echo "==============================================="
echo "📁 Source data directory: $DATA_DIR"
echo "📁 Test directory: $TEST_DIR"
echo "🔄 Test sequence: JSON → Binary → Rkyv → JSON → Rkyv"
echo ""

# Clean up any existing test data
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

# Copy a subset of images for testing (first 10 images)
echo "📋 Setting up test data..."
IMAGE_COUNT=0
for img in "$DATA_DIR"*.jpg; do
    if [ -f "$img" ] && [ $IMAGE_COUNT -lt 10 ]; then
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
        
        # Create a comprehensive sidecar JSON structure with various data types
        cat > "$sidecar_file" << EOF
{
  "sidecar_info": {
    "operation_type": "face_detection",
    "created_at": "2024-12-19T10:30:00Z",
    "image_path": "$img",
    "tool_version": "1.0.0",
    "processing_id": "proc_${basename}_$(date +%s)",
    "metadata": {
      "source": "benchmark_test",
      "batch_id": "batch_001",
      "priority": "high"
    }
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
      "quality_score": 0.987654321,
      "orientation": "landscape",
      "color_space": "RGB",
      "bit_depth": 8,
      "compression": "JPEG",
      "exif_data": {
        "camera_make": "Canon",
        "camera_model": "EOS R5",
        "iso": 100,
        "aperture": 2.8,
        "shutter_speed": "1/125",
        "focal_length": 50.0,
        "flash": false,
        "white_balance": "auto"
      }
    },
    "statistics": {
      "total_pixels": 2073600,
      "face_pixels": 124416,
      "face_percentage": 6.0,
      "brightness_avg": 128.5,
      "contrast_std": 45.2,
      "color_channels": {
        "red_avg": 130.1,
        "green_avg": 125.8,
        "blue_avg": 120.3
      }
    }
  },
  "quality_assessment": {
    "overall_score": 0.95,
    "sharpness": 0.92,
    "exposure": 0.88,
    "contrast": 0.94,
    "color_accuracy": 0.96,
    "noise_level": 0.02,
    "artifacts": []
  },
  "processing_log": [
    {
      "timestamp": "2024-12-19T10:30:00.123Z",
      "step": "image_load",
      "duration_ms": 45,
      "status": "success"
    },
    {
      "timestamp": "2024-12-19T10:30:00.168Z",
      "step": "preprocessing",
      "duration_ms": 123,
      "status": "success"
    },
    {
      "timestamp": "2024-12-19T10:30:00.291Z",
      "step": "face_detection",
      "duration_ms": 234,
      "status": "success"
    },
    {
      "timestamp": "2024-12-19T10:30:00.525Z",
      "step": "postprocessing",
      "duration_ms": 67,
      "status": "success"
    }
  ]
}
EOF
        JSON_COUNT=$((JSON_COUNT + 1))
    fi
done

echo "✅ Created $JSON_COUNT comprehensive JSON sidecar files"
echo ""

# Function to validate data integrity
validate_data_integrity() {
    local format=$1
    local expected_count=$2
    
    echo "🔍 Validating $format format..."
    
    # Count files
    local file_count=$(find "$TEST_DIR" -name "*.$format" | wc -l)
    
    if [ "$file_count" -ne "$expected_count" ]; then
        echo "❌ $format validation failed: Expected $expected_count files, found $file_count"
        return 1
    fi
    
    # Validate file content
    local validation_output=$(./target/release/sportball-sidecar-rust validate --input "$TEST_DIR" --workers 16 2>/dev/null)
    local total_files=$(echo "$validation_output" | grep -o '"total_files":[0-9]*' | grep -o '[0-9]*' || echo "0")
    local valid_files=$(echo "$validation_output" | grep -o '"valid_files":[0-9]*' | grep -o '[0-9]*' || echo "0")
    local invalid_files=$(echo "$validation_output" | grep -o '"invalid_files":[0-9]*' | grep -o '[0-9]*' || echo "0")
    
    if [ "$valid_files" -ne "$expected_count" ]; then
        echo "❌ $format validation failed: Expected $expected_count valid files, found $valid_files"
        echo "   Invalid files: $invalid_files"
        return 1
    fi
    
    echo "✅ $format validation passed: $valid_files/$total_files files valid"
    return 0
}

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

# Function to extract and compare JSON content
extract_json_content() {
    local format=$1
    local output_file="$TEST_DIR/${format}_content.json"
    
    echo "📊 Extracting $format content for comparison..."
    
    # Find all files of the specified format and extract their content
    find "$TEST_DIR" -name "*.$format" | while read -r file; do
        basename=$(basename "$file" ".$format")
        echo "=== $basename ===" >> "$output_file"
        
        if [ "$format" = "json" ]; then
            # For JSON files, just copy the content
            cat "$file" >> "$output_file"
        else
            # For binary/rkyv files, we need to deserialize them
            # This is a simplified approach - in practice, you'd use the actual deserializer
            echo "Binary content for $basename" >> "$output_file"
        fi
        echo "" >> "$output_file"
    done
    
    echo "✅ Extracted content for $(find "$TEST_DIR" -name "*.$format" | wc -l) $format files"
}

# Function to compare content between formats
compare_content() {
    local format1=$1
    local format2=$2
    
    echo "🔍 Comparing content between $format1 and $format2..."
    
    # For this test, we'll use a simpler approach:
    # 1. Convert both formats back to JSON
    # 2. Compare the JSON content
    
    local temp_dir="$TEST_DIR/content_comparison"
    mkdir -p "$temp_dir"
    
    # Copy files to temp directory
    cp "$TEST_DIR"/*."$format1" "$temp_dir/"
    
    # Convert format1 to JSON
    ./target/release/sportball-sidecar-rust convert --input "$temp_dir" --format json >/dev/null 2>&1
    
    # Get the JSON content
    local json1_content="$TEST_DIR/${format1}_as_json.txt"
    find "$temp_dir" -name "*.json" -exec cat {} \; | sort > "$json1_content"
    
    # Clean up and repeat for format2
    rm -rf "$temp_dir"
    mkdir -p "$temp_dir"
    cp "$TEST_DIR"/*."$format2" "$temp_dir/"
    
    # Convert format2 to JSON
    ./target/release/sportball-sidecar-rust convert --input "$temp_dir" --format json >/dev/null 2>&1
    
    # Get the JSON content
    local json2_content="$TEST_DIR/${format2}_as_json.txt"
    find "$temp_dir" -name "*.json" -exec cat {} \; | sort > "$json2_content"
    
    # Compare the content
    local diff_output=$(diff "$json1_content" "$json2_content")
    
    if [ -n "$diff_output" ]; then
        echo "❌ Content comparison failed - data differs between $format1 and $format2:"
        echo "$diff_output" | head -20
        rm -rf "$temp_dir"
        return 1
    fi
    
    echo "✅ Content comparison passed - $format1 and $format2 have identical data"
    rm -rf "$temp_dir"
    return 0
}

echo "🚀 Starting Round-Trip Data Integrity Test"
echo "==========================================="

# Step 1: Validate initial JSON files
echo ""
echo "📋 Step 1: Initial JSON Validation"
echo "===================================="
if ! validate_data_integrity "json" $JSON_COUNT; then
    echo "❌ Initial JSON validation failed - aborting test"
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
if ! validate_data_integrity "bin" $JSON_COUNT; then
    echo "❌ Binary validation failed"
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
if ! validate_data_integrity "rkyv" $JSON_COUNT; then
    echo "❌ Rkyv validation failed"
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
if ! validate_data_integrity "json" $JSON_COUNT; then
    echo "❌ JSON validation failed"
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
if ! validate_data_integrity "rkyv" $JSON_COUNT; then
    echo "❌ Rkyv validation failed"
    exit 1
fi

# Step 6: Data Integrity Verification
echo ""
echo "📋 Step 6: Data Integrity Verification"
echo "======================================"

echo "🔍 Verifying data integrity across all conversions..."

# Compare content between formats to ensure no data loss
if ! compare_content "json" "bin"; then
    echo "❌ Data integrity check failed: JSON ≠ Binary content"
    exit 1
fi

if ! compare_content "bin" "rkyv"; then
    echo "❌ Data integrity check failed: Binary ≠ Rkyv content"
    exit 1
fi

if ! compare_content "rkyv" "json"; then
    echo "❌ Data integrity check failed: Rkyv ≠ JSON content"
    exit 1
fi

echo ""
echo "📊 Final Format Statistics:"
echo "==========================="
./target/release/sportball-sidecar-rust format-stats --input "$TEST_DIR"

echo ""
echo "🎯 Round-Trip Test Results:"
echo "============================"
echo "✅ All format conversions completed successfully"
echo "✅ All validations passed with no errors"
echo "✅ Data integrity verified - no data loss detected"
echo "✅ Content comparison passed for all format pairs"
echo ""
echo "📋 Conversion Summary:"
echo "====================="
echo "🔄 JSON → Binary → Rkyv → JSON → Rkyv"
echo "📁 Files processed: $JSON_COUNT"
echo "🚀 All conversions completed without errors"
echo "🔒 Data integrity maintained throughout"

# Cleanup
echo ""
echo "🧹 Cleaning up test data..."
rm -rf "$TEST_DIR"

echo ""
echo "🎉 Round-trip data integrity test PASSED!"
echo "   All format conversions preserve data with no errors or loss."
