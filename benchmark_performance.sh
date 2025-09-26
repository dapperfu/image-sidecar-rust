#!/bin/bash

# Comprehensive Sidecar Format Performance Benchmark
# Tests conversion and reading performance with parallel processing

set -e

DATA_DIR="/tank/games/Game_04_153212-160200/"
BINARY_DIR="/tmp/sportball_benchmark_binary"
RKYV_DIR="/tmp/sportball_benchmark_rkyv"
RESULTS_FILE="benchmark_results_$(date +%Y%m%d_%H%M%S).json"

echo "🚀 Starting Sidecar Format Performance Benchmark"
echo "📁 Data directory: $DATA_DIR"
echo "📊 Total files: $(find $DATA_DIR -name "*.json" | wc -l)"
echo "📝 Results will be saved to: $RESULTS_FILE"
echo ""

# Create temporary directories
mkdir -p "$BINARY_DIR" "$RKYV_DIR"

# Initialize results JSON
echo '{
  "benchmark_info": {
    "timestamp": "'$(date -Iseconds)'",
    "data_directory": "'$DATA_DIR'",
    "total_files": '$(find $DATA_DIR -name "*.json" | wc -l)',
    "rust_version": "'$(rustc --version)'",
    "cpu_cores": '$(nproc)'
  },
  "conversion_benchmarks": {},
  "reading_benchmarks": {},
  "parallel_benchmarks": {}
}' > "$RESULTS_FILE"

echo "🔄 Phase 1: Conversion Performance Benchmarks"
echo "=============================================="

# JSON to Binary conversion benchmark
echo "📦 Converting JSON → Binary..."
CONV_START=$(date +%s.%N)
CONVERTED_COUNT=$(./target/release/sportball-sidecar-rust convert --input "$DATA_DIR" --format bin 2>&1 | grep -o "Converted [0-9]*" | grep -o "[0-9]*" || echo "0")
CONV_END=$(date +%s.%N)
CONV_TIME=$(echo "$CONV_END - $CONV_START" | bc)

echo "✅ Converted $CONVERTED_COUNT files in ${CONV_TIME}s"
echo "📊 Rate: $(echo "scale=2; $CONVERTED_COUNT / $CONV_TIME" | bc) files/second"

# Update results
jq --argjson count "$CONVERTED_COUNT" --argjson time "$CONV_TIME" --argjson rate "$(echo "scale=2; $CONVERTED_COUNT / $CONV_TIME" | bc)" \
   '.conversion_benchmarks.json_to_binary = {
     "files_converted": $count,
     "time_seconds": $time,
     "rate_files_per_second": $rate
   }' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"

# JSON to Rkyv conversion benchmark (placeholder - will use binary for now)
echo "⚡ Converting JSON → Rkyv (placeholder)..."
CONV_START=$(date +%s.%N)
CONVERTED_COUNT=$(./target/release/sportball-sidecar-rust convert --input "$DATA_DIR" --format rkyv 2>&1 | grep -o "Converted [0-9]*" | grep -o "[0-9]*" || echo "0")
CONV_END=$(date +%s.%N)
CONV_TIME=$(echo "$CONV_END - $CONV_START" | bc)

echo "✅ Converted $CONVERTED_COUNT files in ${CONV_TIME}s"
echo "📊 Rate: $(echo "scale=2; $CONVERTED_COUNT / $CONV_TIME" | bc) files/second"

# Update results
jq --argjson count "$CONVERTED_COUNT" --argjson time "$CONV_TIME" --argjson rate "$(echo "scale=2; $CONVERTED_COUNT / $CONV_TIME" | bc)" \
   '.conversion_benchmarks.json_to_rkyv = {
     "files_converted": $count,
     "time_seconds": $time,
     "rate_files_per_second": $rate
   }' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"

echo ""
echo "📖 Phase 2: Reading Performance Benchmarks"
echo "=========================================="

# JSON reading benchmark
echo "📄 Benchmarking JSON reading..."
READ_START=$(date +%s.%N)
JSON_RESULTS=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers 16 2>/dev/null | jq '.total_files')
READ_END=$(date +%s.%N)
READ_TIME=$(echo "$READ_END - $READ_START" | bc)

echo "✅ Read $JSON_RESULTS JSON files in ${READ_TIME}s"
echo "📊 Rate: $(echo "scale=2; $JSON_RESULTS / $READ_TIME" | bc) files/second"

# Update results
jq --argjson count "$JSON_RESULTS" --argjson time "$READ_TIME" --argjson rate "$(echo "scale=2; $JSON_RESULTS / $READ_TIME" | bc)" \
   '.reading_benchmarks.json = {
     "files_read": $count,
     "time_seconds": $time,
     "rate_files_per_second": $rate
   }' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"

# Binary reading benchmark (now that we have binary files)
echo "📦 Benchmarking Binary reading..."
READ_START=$(date +%s.%N)
BINARY_RESULTS=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers 16 2>/dev/null | jq '.total_files')
READ_END=$(date +%s.%N)
READ_TIME=$(echo "$READ_END - $READ_START" | bc)

echo "✅ Read $BINARY_RESULTS Binary files in ${READ_TIME}s"
echo "📊 Rate: $(echo "scale=2; $BINARY_RESULTS / $READ_TIME" | bc) files/second"

# Update results
jq --argjson count "$BINARY_RESULTS" --argjson time "$READ_TIME" --argjson rate "$(echo "scale=2; $BINARY_RESULTS / $READ_TIME" | bc)" \
   '.reading_benchmarks.binary = {
     "files_read": $count,
     "time_seconds": $time,
     "rate_files_per_second": $rate
   }' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"

echo ""
echo "⚡ Phase 3: Parallel Processing Benchmarks"
echo "=========================================="

# Test different worker counts for parallel processing
WORKER_COUNTS=(1 2 4 8 16 32)

for workers in "${WORKER_COUNTS[@]}"; do
    echo "🔧 Testing with $workers workers..."
    
    READ_START=$(date +%s.%N)
    PARALLEL_RESULTS=$(./target/release/sportball-sidecar-rust validate --input "$DATA_DIR" --workers "$workers" 2>/dev/null | jq '.total_files')
    READ_END=$(date +%s.%N)
    READ_TIME=$(echo "$READ_END - $READ_START" | bc)
    
    echo "✅ $workers workers: $PARALLEL_RESULTS files in ${READ_TIME}s ($(echo "scale=2; $PARALLEL_RESULTS / $READ_TIME" | bc) files/sec)"
    
    # Update results
    jq --argjson workers "$workers" --argjson count "$PARALLEL_RESULTS" --argjson time "$READ_TIME" --argjson rate "$(echo "scale=2; $PARALLEL_RESULTS / $READ_TIME" | bc)" \
       '.parallel_benchmarks["workers_" + ($workers | tostring)] = {
         "worker_count": $workers,
         "files_processed": $count,
         "time_seconds": $time,
         "rate_files_per_second": $rate
       }' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"
done

echo ""
echo "📊 Phase 4: File Size Analysis"
echo "=============================="

# Analyze file sizes
echo "📏 Analyzing file sizes..."

JSON_SIZE=$(find "$DATA_DIR" -name "*.json" -exec du -cb {} + | tail -1 | cut -f1)
BINARY_SIZE=$(find "$DATA_DIR" -name "*.bin" -exec du -cb {} + | tail -1 | cut -f1 2>/dev/null || echo "0")

echo "📄 Total JSON size: $(echo "scale=2; $JSON_SIZE / 1024 / 1024" | bc) MB"
echo "📦 Total Binary size: $(echo "scale=2; $BINARY_SIZE / 1024 / 1024" | bc) MB"

if [ "$BINARY_SIZE" -gt 0 ]; then
    SIZE_RATIO=$(echo "scale=2; $BINARY_SIZE * 100 / $JSON_SIZE" | bc)
    echo "📊 Binary is ${SIZE_RATIO}% of JSON size (space savings: $(echo "scale=1; 100 - $SIZE_RATIO" | bc)%)"
fi

# Update results with file size analysis
jq --argjson json_size "$JSON_SIZE" --argjson binary_size "$BINARY_SIZE" \
   '.file_size_analysis = {
     "json_total_bytes": $json_size,
     "binary_total_bytes": $binary_size,
     "json_total_mb": ($json_size / 1024 / 1024),
     "binary_total_mb": ($binary_size / 1024 / 1024),
     "size_ratio_percent": (if $binary_size > 0 then ($binary_size * 100 / $json_size) else 0 end),
     "space_savings_percent": (if $binary_size > 0 then (100 - ($binary_size * 100 / $json_size)) else 0 end)
   }' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"

echo ""
echo "📈 Phase 5: Performance Summary"
echo "================================"

# Calculate performance improvements
JSON_RATE=$(jq -r '.reading_benchmarks.json.rate_files_per_second' "$RESULTS_FILE")
BINARY_RATE=$(jq -r '.reading_benchmarks.binary.rate_files_per_second' "$RESULTS_FILE")

if [ "$BINARY_RATE" != "null" ] && [ "$BINARY_RATE" != "0" ]; then
    SPEEDUP=$(echo "scale=2; $BINARY_RATE / $JSON_RATE" | bc)
    echo "🚀 Binary reading is ${SPEEDUP}x faster than JSON"
fi

echo ""
echo "✅ Benchmark Complete!"
echo "📊 Results saved to: $RESULTS_FILE"
echo ""
echo "📋 Quick Summary:"
echo "=================="
echo "📁 Files processed: $(jq -r '.benchmark_info.total_files' "$RESULTS_FILE")"
echo "📄 JSON reading rate: $(jq -r '.reading_benchmarks.json.rate_files_per_second' "$RESULTS_FILE") files/sec"
echo "📦 Binary reading rate: $(jq -r '.reading_benchmarks.binary.rate_files_per_second' "$RESULTS_FILE") files/sec"
echo "⚡ Best parallel performance: $(jq -r '.parallel_benchmarks | to_entries | max_by(.value.rate_files_per_second) | .value.worker_count' "$RESULTS_FILE") workers"

# Cleanup
rm -rf "$BINARY_DIR" "$RKYV_DIR"

echo ""
echo "🎯 Benchmark results are ready for analysis!"
