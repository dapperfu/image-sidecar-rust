#!/bin/bash

# Simple Sidecar Format Performance Benchmark
# Tests conversion and reading performance with parallel processing

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"

echo "🚀 Starting Sidecar Format Performance Benchmark"
echo "📁 Data directory: $DATA_DIR"
echo "📊 Total files: $(find $DATA_DIR -name "*.json" | wc -l)"
echo ""

echo "🔄 Phase 1: Conversion Performance Benchmarks"
echo "=============================================="

# JSON to Binary conversion benchmark
echo "📦 Converting JSON → Binary..."
CONV_START=$(date +%s.%N)
CONVERTED_OUTPUT=$(./target/release/sportball-sidecar-rust convert --input "$DATA_DIR" --format bin 2>&1)
CONV_END=$(date +%s.%N)
CONV_TIME=$(echo "$CONV_END - $CONV_START" | bc -l)

# Extract converted count from output
CONVERTED_COUNT=$(echo "$CONVERTED_OUTPUT" | grep -o "Converted [0-9]*" | grep -o "[0-9]*" || echo "0")

echo "✅ Converted $CONVERTED_COUNT files in ${CONV_TIME}s"
if [ "$CONVERTED_COUNT" -gt 0 ]; then
    RATE=$(echo "scale=2; $CONVERTED_COUNT / $CONV_TIME" | bc -l)
    echo "📊 Rate: ${RATE} files/second"
fi

# JSON to Rkyv conversion benchmark
echo "⚡ Converting JSON → Rkyv (placeholder)..."
CONV_START=$(date +%s.%N)
CONVERTED_OUTPUT=$(./target/release/sportball-sidecar-rust convert --input "$DATA_DIR" --format rkyv 2>&1)
CONV_END=$(date +%s.%N)
CONV_TIME=$(echo "$CONV_END - $CONV_START" | bc -l)

CONVERTED_COUNT=$(echo "$CONVERTED_OUTPUT" | grep -o "Converted [0-9]*" | grep -o "[0-9]*" || echo "0")

echo "✅ Converted $CONVERTED_COUNT files in ${CONV_TIME}s"
if [ "$CONVERTED_COUNT" -gt 0 ]; then
    RATE=$(echo "scale=2; $CONVERTED_COUNT / $CONV_TIME" | bc -l)
    echo "📊 Rate: ${RATE} files/second"
fi

echo ""
echo "📖 Phase 2: Reading Performance Benchmarks"
echo "=========================================="

# JSON reading benchmark
echo "📄 Benchmarking JSON reading..."
READ_START=$(date +%s.%N)
JSON_OUTPUT=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers 16 2>/dev/null)
READ_END=$(date +%s.%N)
READ_TIME=$(echo "$READ_END - $READ_START" | bc -l)

# Extract total files from JSON output
JSON_COUNT=$(echo "$JSON_OUTPUT" | grep -o '"total_files":[0-9]*' | grep -o '[0-9]*' || echo "0")

echo "✅ Read $JSON_COUNT JSON files in ${READ_TIME}s"
if [ "$JSON_COUNT" -gt 0 ]; then
    RATE=$(echo "scale=2; $JSON_COUNT / $READ_TIME" | bc -l)
    echo "📊 Rate: ${RATE} files/second"
fi

# Binary reading benchmark (now that we have binary files)
echo "📦 Benchmarking Binary reading..."
READ_START=$(date +%s.%N)
BINARY_OUTPUT=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers 16 2>/dev/null)
READ_END=$(date +%s.%N)
READ_TIME=$(echo "$READ_END - $READ_START" | bc -l)

BINARY_COUNT=$(echo "$BINARY_OUTPUT" | grep -o '"total_files":[0-9]*' | grep -o '[0-9]*' || echo "0")

echo "✅ Read $BINARY_COUNT Binary files in ${READ_TIME}s"
if [ "$BINARY_COUNT" -gt 0 ]; then
    RATE=$(echo "scale=2; $BINARY_COUNT / $READ_TIME" | bc -l)
    echo "📊 Rate: ${RATE} files/second"
fi

echo ""
echo "⚡ Phase 3: Parallel Processing Benchmarks"
echo "=========================================="

# Test different worker counts for parallel processing
WORKER_COUNTS=(1 2 4 8 16 32)

for workers in "${WORKER_COUNTS[@]}"; do
    echo "🔧 Testing with $workers workers..."
    
    READ_START=$(date +%s.%N)
    PARALLEL_OUTPUT=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers "$workers" 2>/dev/null)
    READ_END=$(date +%s.%N)
    READ_TIME=$(echo "$READ_END - $READ_START" | bc -l)
    
    PARALLEL_COUNT=$(echo "$PARALLEL_OUTPUT" | grep -o '"total_files":[0-9]*' | grep -o '[0-9]*' || echo "0")
    
    if [ "$PARALLEL_COUNT" -gt 0 ]; then
        RATE=$(echo "scale=2; $PARALLEL_COUNT / $READ_TIME" | bc -l)
        echo "✅ $workers workers: $PARALLEL_COUNT files in ${READ_TIME}s (${RATE} files/sec)"
    else
        echo "⚠️  $workers workers: No files processed"
    fi
done

echo ""
echo "📊 Phase 4: File Size Analysis"
echo "=============================="

# Analyze file sizes
echo "📏 Analyzing file sizes..."

JSON_SIZE=$(find "$DATA_DIR" -name "*.json" -exec du -cb {} + 2>/dev/null | tail -1 | cut -f1 || echo "0")
BINARY_SIZE=$(find "$DATA_DIR" -name "*.bin" -exec du -cb {} + 2>/dev/null | tail -1 | cut -f1 || echo "0")

if [ "$JSON_SIZE" -gt 0 ]; then
    JSON_MB=$(echo "scale=2; $JSON_SIZE / 1024 / 1024" | bc -l)
    echo "📄 Total JSON size: ${JSON_MB} MB"
fi

if [ "$BINARY_SIZE" -gt 0 ]; then
    BINARY_MB=$(echo "scale=2; $BINARY_SIZE / 1024 / 1024" | bc -l)
    echo "📦 Total Binary size: ${BINARY_MB} MB"
    
    if [ "$JSON_SIZE" -gt 0 ]; then
        SIZE_RATIO=$(echo "scale=2; $BINARY_SIZE * 100 / $JSON_SIZE" | bc -l)
        SPACE_SAVINGS=$(echo "scale=1; 100 - $SIZE_RATIO" | bc -l)
        echo "📊 Binary is ${SIZE_RATIO}% of JSON size (space savings: ${SPACE_SAVINGS}%)"
    fi
fi

echo ""
echo "📈 Phase 5: Performance Summary"
echo "================================"

echo ""
echo "✅ Benchmark Complete!"
echo ""
echo "📋 Quick Summary:"
echo "=================="
echo "📁 Files processed: $(find $DATA_DIR -name "*.json" | wc -l)"
echo "📄 JSON files found: $(find $DATA_DIR -name "*.json" | wc -l)"
echo "📦 Binary files found: $(find $DATA_DIR -name "*.bin" | wc -l)"
echo "⚡ Rkyv files found: $(find $DATA_DIR -name "*.rkyv" | wc -l)"

echo ""
echo "🎯 Benchmark complete! Check the output above for performance metrics."
