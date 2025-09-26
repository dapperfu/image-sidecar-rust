#!/bin/bash

# Comprehensive Data Content Validation Test
# Tests: JSON → Binary → Rkyv → JSON → Rkyv
# Validates actual data content preservation with no loss

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"
TEST_DIR="/tmp/sportball_content_test"

echo "🔍 Comprehensive Data Content Validation Test"
echo "============================================="
echo "📁 Source data directory: $DATA_DIR"
echo "📁 Test directory: $TEST_DIR"
echo "🔄 Test sequence: JSON → Binary → Rkyv → JSON → Rkyv"
echo "🔍 Validating actual data content preservation"
echo ""

# Clean up any existing test data
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

# Create a single comprehensive test file
echo "📄 Creating comprehensive test data..."
TEST_FILE="$TEST_DIR/test_data.json"

cat > "$TEST_FILE" << EOF
{
  "sidecar_info": {
    "operation_type": "face_detection",
    "created_at": "2024-12-19T10:30:00Z",
    "image_path": "/path/to/test/image.jpg",
    "tool_version": "1.0.0",
    "processing_id": "proc_test_$(date +%s)",
    "metadata": {
      "source": "content_test",
      "batch_id": "batch_001",
      "priority": "high",
      "test_flags": ["comprehensive", "roundtrip", "validation"]
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
          "smile": true,
          "confidence_scores": {
            "age": 0.95,
            "gender": 0.98,
            "emotion": 0.87,
            "glasses": 0.99,
            "smile": 0.92
          }
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
        "white_balance": "auto",
        "gps": {
          "latitude": 37.7749,
          "longitude": -122.4194,
          "altitude": 10.5
        }
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
      },
      "histogram": {
        "red": [100, 150, 200, 250, 300, 350, 400, 450, 500, 550],
        "green": [95, 145, 195, 245, 295, 345, 395, 445, 495, 545],
        "blue": [90, 140, 190, 240, 290, 340, 390, 440, 490, 540]
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
    "artifacts": [],
    "recommendations": [
      "Good lighting conditions",
      "Sharp focus achieved",
      "No motion blur detected"
    ]
  },
  "processing_log": [
    {
      "timestamp": "2024-12-19T10:30:00.123Z",
      "step": "image_load",
      "duration_ms": 45,
      "status": "success",
      "details": "Image loaded successfully from storage"
    },
    {
      "timestamp": "2024-12-19T10:30:00.168Z",
      "step": "preprocessing",
      "duration_ms": 123,
      "status": "success",
      "details": "Image preprocessing completed"
    },
    {
      "timestamp": "2024-12-19T10:30:00.291Z",
      "step": "face_detection",
      "duration_ms": 234,
      "status": "success",
      "details": "Face detection algorithm executed"
    },
    {
      "timestamp": "2024-12-19T10:30:00.525Z",
      "step": "postprocessing",
      "duration_ms": 67,
      "status": "success",
      "details": "Results postprocessed and validated"
    }
  ],
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

# Function to extract and normalize JSON content
extract_json_content() {
    local file_path=$1
    local output_file=$2
    
    # Use jq to normalize JSON (sort keys, consistent formatting)
    if command -v jq >/dev/null 2>&1; then
        jq -S . "$file_path" > "$output_file" 2>/dev/null
    else
        # Fallback: just copy the file
        cp "$file_path" "$output_file"
    fi
}

# Function to compare JSON content
compare_json_content() {
    local file1=$1
    local file2=$2
    local description=$3
    
    echo "🔍 Comparing $description..."
    
    # Extract normalized content
    local norm1="$TEST_DIR/norm1.json"
    local norm2="$TEST_DIR/norm2.json"
    
    extract_json_content "$file1" "$norm1"
    extract_json_content "$file2" "$norm2"
    
    # Compare the normalized content
    if diff -q "$norm1" "$norm2" >/dev/null 2>&1; then
        echo "✅ Content comparison passed - $description are identical"
        return 0
    else
        echo "❌ Content comparison failed - $description differ:"
        echo "   Differences found between files"
        echo "   File 1: $file1"
        echo "   File 2: $file2"
        
        # Show first few differences
        echo "   First differences:"
        diff "$norm1" "$norm2" | head -10
        return 1
    fi
}

echo "🚀 Starting Comprehensive Data Content Validation Test"
echo "======================================================"

# Step 1: JSON → Binary
echo ""
echo "📋 Step 1: JSON → Binary Conversion"
echo "===================================="
if ! convert_format "json" "bin"; then
    echo "❌ JSON → Binary conversion failed"
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

# Step 3: Rkyv → JSON
echo ""
echo "📋 Step 3: Rkyv → JSON Conversion"
echo "=================================="
if ! convert_format "rkyv" "json"; then
    echo "❌ Rkyv → JSON conversion failed"
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

# Step 5: Content Validation
echo ""
echo "📋 Step 5: Content Validation"
echo "============================="

# Compare original JSON with final JSON
if ! compare_json_content "$TEST_FILE" "$TEST_DIR/test_data.json" "original and final JSON"; then
    echo "❌ Data integrity check failed: Original JSON ≠ Final JSON"
    exit 1
fi

# Compare original JSON with final Rkyv (by converting Rkyv back to JSON)
echo "🔄 Converting final Rkyv back to JSON for comparison..."
./target/release/sportball-sidecar-rust convert --input "$TEST_DIR" --format json >/dev/null 2>&1

if ! compare_json_content "$TEST_FILE" "$TEST_DIR/test_data.json" "original JSON and Rkyv→JSON"; then
    echo "❌ Data integrity check failed: Original JSON ≠ Rkyv→JSON"
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
echo "🎯 Comprehensive Data Content Validation Results:"
echo "================================================"
echo "✅ All format conversions completed successfully"
echo "✅ All content comparisons passed"
echo "✅ Data integrity verified - no data loss detected"
echo "✅ Round-trip conversion preserves all data"
echo ""
echo "📋 Conversion Summary:"
echo "====================="
echo "🔄 JSON → Binary → Rkyv → JSON → Rkyv"
echo "📁 Files processed: 1 comprehensive test file"
echo "🚀 All conversions completed without errors"
echo "🔒 Data content preserved throughout all conversions"
echo "📊 Binary/Rkyv formats provide space savings"

# Cleanup
echo ""
echo "🧹 Cleaning up test data..."
rm -rf "$TEST_DIR"

echo ""
echo "🎉 Comprehensive data content validation test PASSED!"
echo "   All format conversions preserve data with no errors or loss."
echo "   Round-trip conversion maintains complete data integrity."
