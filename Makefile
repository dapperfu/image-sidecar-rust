# Makefile for sportball-sidecar-rust
# High-performance Rust implementation for sportball JSON sidecar operations

# Variables
CARGO = cargo
RUST_LOG = info
TARGET_DIR = target
RELEASE_BINARY = $(TARGET_DIR)/release/sportball-sidecar-rust
DEBUG_BINARY = $(TARGET_DIR)/debug/sportball-sidecar-rust
MATURIN = maturin
PYTHON = python3
PIP = pip3

# Default target
.PHONY: all
all: build

# Build targets
.PHONY: build
build: $(RELEASE_BINARY)

$(RELEASE_BINARY):
	$(CARGO) build --release

.PHONY: debug
debug: $(DEBUG_BINARY)

$(DEBUG_BINARY):
	$(CARGO) build

# Development targets
.PHONY: check
check:
	$(CARGO) check

.PHONY: test
test:
	$(CARGO) test

.PHONY: test-release
test-release:
	$(CARGO) test --release

.PHONY: clippy
clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: fmt
fmt:
	$(CARGO) fmt

.PHONY: clean
clean:
	$(CARGO) clean

# Benchmarking
.PHONY: bench
bench:
	$(CARGO) bench

.PHONY: bench-release
bench-release:
	$(CARGO) bench --release

# Installation
.PHONY: install
install: $(RELEASE_BINARY)
	cp $(RELEASE_BINARY) /usr/local/bin/sportball-sidecar-rust

.PHONY: uninstall
uninstall:
	rm -f /usr/local/bin/sportball-sidecar-rust

# Documentation
.PHONY: docs
docs:
	$(CARGO) doc --no-deps --open

.PHONY: docs-release
docs-release:
	$(CARGO) doc --release --no-deps --open

# Linting and formatting
.PHONY: lint
lint: clippy fmt

.PHONY: ci
ci: check test clippy fmt

# Performance testing
.PHONY: perf-test
perf-test: $(RELEASE_BINARY)
	@echo "Running performance tests..."
	@echo "Creating test data..."
	@mkdir -p test_data
	@for i in $$(seq 1 1000); do \
		echo '{"test": "data_'$$i'", "value": '$$i'}' > test_data/test_$$i.json; \
	done
	@echo "Running validation benchmark..."
	time $(RELEASE_BINARY) validate --input test_data --workers 16
	@echo "Running statistics benchmark..."
	time $(RELEASE_BINARY) stats --input test_data
	@echo "Cleaning up test data..."
	@rm -rf test_data

# Development workflow
.PHONY: dev
dev: check test

.PHONY: release
release: clean build test-release bench-release

# Python targets
.PHONY: python-build
python-build:
	$(MATURIN) develop --features python

.PHONY: python-build-release
python-build-release:
	$(MATURIN) develop --release --features python

.PHONY: python-wheel
python-wheel:
	$(MATURIN) build --features python

.PHONY: python-wheel-release
python-wheel-release:
	$(MATURIN) build --release --features python

.PHONY: python-test
python-test:
	$(PYTHON) -m pytest tests/ -v

.PHONY: python-install
python-install:
	$(PIP) install -e . --features python

.PHONY: python-uninstall
python-uninstall:
	$(PIP) uninstall sportball-sidecar-rust -y

.PHONY: python-clean
python-clean:
	$(CARGO) clean
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info/
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build        - Build release binary"
	@echo "  debug        - Build debug binary"
	@echo "  check        - Check code without building"
	@echo "  test         - Run tests"
	@echo "  clippy       - Run clippy linter"
	@echo "  fmt          - Format code"
	@echo "  clean        - Clean build artifacts"
	@echo "  bench        - Run benchmarks"
	@echo "  install      - Install binary to /usr/local/bin"
	@echo "  uninstall    - Remove binary from /usr/local/bin"
	@echo "  docs         - Generate and open documentation"
	@echo "  lint         - Run clippy and fmt"
	@echo "  ci           - Run CI checks (check, test, clippy, fmt)"
	@echo "  perf-test    - Run performance tests"
	@echo "  dev          - Development workflow (check, test)"
	@echo "  release      - Release workflow (clean, build, test, bench)"
	@echo ""
	@echo "Python targets:"
	@echo "  python-build        - Build Python extension in development mode"
	@echo "  python-build-release - Build Python extension in release mode"
	@echo "  python-wheel        - Build Python wheel"
	@echo "  python-wheel-release - Build Python wheel in release mode"
	@echo "  python-test         - Run Python tests"
	@echo "  python-install      - Install Python package"
	@echo "  python-uninstall    - Uninstall Python package"
	@echo "  python-clean        - Clean Python build artifacts"
	@echo ""
	@echo "  help         - Show this help message"
