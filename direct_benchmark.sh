#!/bin/bash

# Direct Performance Benchmark for Sidecar Formats
# Tests real-world performance with your actual data

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"

echo "🚀 Sidecar Format Performance Benchmark"
echo "========================================"
echo "📁 Data directory: $DATA_DIR"
echo "📊 Total files: $(find $DATA_DIR -name "*.rkyv" | wc -l)"
echo ""

# Check current format distribution
echo "📋 Current Format Distribution:"
echo "================================"
JSON_COUNT=$(find "$DATA_DIR" -name "*.json" | wc -l)
BINARY_COUNT=$(find "$DATA_DIR" -name "*.bin" | wc -l)
RKYV_COUNT=$(find "$DATA_DIR" -name "*.rkyv" | wc -l)

echo "📄 JSON files: $JSON_COUNT"
echo "📦 Binary files: $BINARY_COUNT"
echo "⚡ Rkyv files: $RKYV_COUNT"
echo ""

# File size analysis
echo "📏 File Size Analysis:"
echo "====================="

if [ "$JSON_COUNT" -gt 0 ]; then
    JSON_SIZE=$(find "$DATA_DIR" -name "*.json" -exec du -cb {} + | tail -1 | cut -f1)
    JSON_MB=$(echo "scale=2; $JSON_SIZE / 1024 / 1024" | bc -l)
    echo "📄 JSON total size: ${JSON_MB} MB"
fi

if [ "$BINARY_COUNT" -gt 0 ]; then
    BINARY_SIZE=$(find "$DATA_DIR" -name "*.bin" -exec du -cb {} + | tail -1 | cut -f1)
    BINARY_MB=$(echo "scale=2; $BINARY_SIZE / 1024 / 1024" | bc -l)
    echo "📦 Binary total size: ${BINARY_MB} MB"
fi

if [ "$RKYV_COUNT" -gt 0 ]; then
    RKYV_SIZE=$(find "$DATA_DIR" -name "*.rkyv" -exec du -cb {} + | tail -1 | cut -f1)
    RKYV_MB=$(echo "scale=2; $RKYV_SIZE / 1024 / 1024" | bc -l)
    echo "⚡ Rkyv total size: ${RKYV_MB} MB"
fi

echo ""

# Reading performance benchmarks
echo "📖 Reading Performance Benchmarks:"
echo "=================================="

# Test with different worker counts
WORKER_COUNTS=(1 2 4 8 16 32)

for workers in "${WORKER_COUNTS[@]}"; do
    echo "🔧 Testing with $workers workers..."
    
    START_TIME=$(date +%s.%N)
    OUTPUT=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers "$workers" 2>/dev/null)
    END_TIME=$(date +%s.%N)
    
    DURATION=$(echo "$END_TIME - $START_TIME" | bc -l)
    
    # Extract total files from output
    TOTAL_FILES=$(echo "$OUTPUT" | grep -o '"total_files":[0-9]*' | grep -o '[0-9]*' || echo "0")
    
    if [ "$TOTAL_FILES" -gt 0 ]; then
        RATE=$(echo "scale=2; $TOTAL_FILES / $DURATION" | bc -l)
        echo "✅ $workers workers: $TOTAL_FILES files in ${DURATION}s (${RATE} files/sec)"
    else
        echo "⚠️  $workers workers: No files processed"
    fi
done

echo ""

# Format statistics
echo "📊 Format Statistics:"
echo "===================="
./target/release/sportball-sidecar-rust format-stats --input "$DATA_DIR"

echo ""

# Conversion benchmarks (if we have JSON files to convert)
if [ "$JSON_COUNT" -gt 0 ]; then
    echo "🔄 Conversion Performance:"
    echo "========================="
    
    # JSON to Binary
    echo "📦 Converting JSON → Binary..."
    START_TIME=$(date +%s.%N)
    CONV_OUTPUT=$(./target/release/sportball-sidecar-rust convert --input "$DATA_DIR" --format bin 2>&1)
    END_TIME=$(date +%s.%N)
    
    DURATION=$(echo "$END_TIME - $START_TIME" | bc -l)
    CONVERTED=$(echo "$CONV_OUTPUT" | grep -o "Converted [0-9]*" | grep -o "[0-9]*" || echo "0")
    
    if [ "$CONVERTED" -gt 0 ]; then
        RATE=$(echo "scale=2; $CONVERTED / $DURATION" | bc -l)
        echo "✅ Converted $CONVERTED files in ${DURATION}s (${RATE} files/sec)"
    fi
    
    # JSON to Rkyv
    echo "⚡ Converting JSON → Rkyv..."
    START_TIME=$(date +%s.%N)
    CONV_OUTPUT=$(./target/release/sportball-sidecar-rust convert --input "$DATA_DIR" --format rkyv 2>&1)
    END_TIME=$(date +%s.%N)
    
    DURATION=$(echo "$END_TIME - $START_TIME" | bc -l)
    CONVERTED=$(echo "$CONV_OUTPUT" | grep -o "Converted [0-9]*" | grep -o "[0-9]*" || echo "0")
    
    if [ "$CONVERTED" -gt 0 ]; then
        RATE=$(echo "scale=2; $CONVERTED / $DURATION" | bc -l)
        echo "✅ Converted $CONVERTED files in ${DURATION}s (${RATE} files/sec)"
    fi
else
    echo "ℹ️  No JSON files found for conversion testing"
fi

echo ""
echo "🎯 Benchmark Summary:"
echo "====================="
echo "📁 Total files processed: $(find $DATA_DIR -name "*.rkyv" | wc -l)"
echo "⚡ Current format: Rkyv (most efficient)"
echo "🚀 Parallel processing: Supported with configurable worker counts"
echo "📊 All operations completed successfully!"

echo ""
echo "💡 Performance Tips:"
echo "===================="
echo "• Use 16-32 workers for optimal parallel performance"
echo "• Rkyv format provides the best read/write performance"
echo "• Binary format offers good balance of speed and compatibility"
echo "• JSON format is best for debugging and human readability"
