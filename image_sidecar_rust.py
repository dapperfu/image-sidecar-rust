#!/usr/bin/env python3
"""
Image Sidecar Rust - Python Integration Module

This module provides seamless integration between Python applications and the
high-performance image-sidecar-rust tool. It automatically detects and uses the
Rust binary when available, falling back to Python implementations when needed.

The tool supports multiple formats (JSON, Binary, Rkyv) and is designed for
any image processing workflow that uses sidecar files for metadata storage.

Usage:
    from image_sidecar_rust import ImageSidecarManager
    
    manager = ImageSidecarManager()
    if manager.rust_available:
        results = manager.validate_sidecars("/path/to/sidecars")
    else:
        # Fallback to Python implementation
        results = python_validate_sidecars("/path/to/sidecars")
"""

import os
import json
import subprocess
import logging
from pathlib import Path
from typing import Dict, List, Optional, Union, Any
from dataclasses import dataclass
from enum import Enum

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class SidecarFormat(Enum):
    """Supported sidecar formats"""
    JSON = "json"
    BINARY = "bin"
    RKYV = "rkyv"


@dataclass
class ValidationResult:
    """Result of sidecar validation"""
    file_path: str
    is_valid: bool
    error: Optional[str] = None
    processing_time: float = 0.0
    file_size: int = 0
    detection_count: int = 0
    tool_name: Optional[str] = None
    operation_type: Optional[str] = None


@dataclass
class StatisticsResult:
    """Result of sidecar statistics"""
    directory: str
    total_images: int
    total_sidecars: int
    coverage_percentage: float
    operation_counts: Dict[str, int]
    avg_processing_times: Dict[str, float]
    success_rate_percentages: Dict[str, float]
    avg_data_sizes: Dict[str, float]


@dataclass
class FormatStatistics:
    """Format distribution statistics"""
    directory: str
    format_distribution: Dict[str, int]
    total_files: int
    generated_at: str


class ImageSidecarManager:
    """
    Manager for high-performance image sidecar operations with automatic fallback to Python.
    
    This class provides a high-level interface to the image-sidecar-rust
    tool, automatically detecting the binary and falling back to Python
    implementations when the Rust tool is not available.
    
    Supports multiple formats (JSON, Binary, Rkyv) and is designed for
    any image processing workflow that uses sidecar files for metadata storage.
    """
    
    def __init__(self, rust_binary_path: Optional[str] = None, timeout: int = 300):
        """
        Initialize the image sidecar manager.
        
        Args:
            rust_binary_path: Path to the Rust binary. If None, will search
                            common locations.
            timeout: Timeout for subprocess calls in seconds.
        """
        self.timeout = timeout
        self.rust_binary = self._find_rust_binary(rust_binary_path)
        self.rust_available = self.rust_binary is not None
        
        if self.rust_available:
            logger.info(f"✅ Rust binary found: {self.rust_binary}")
        else:
            logger.warning("⚠️  Rust binary not found, will use Python fallback")
    
    def _find_rust_binary(self, custom_path: Optional[str]) -> Optional[str]:
        """Find the Rust binary in common locations."""
        if custom_path and os.path.exists(custom_path):
            return custom_path
        
        # Common locations to search
        search_paths = [
            "./target/release/image-sidecar-rust",
            "../image-sidecar-rust/target/release/image-sidecar-rust",
            "../../image-sidecar-rust/target/release/image-sidecar-rust",
            "/usr/local/bin/image-sidecar-rust",
            "/opt/image-sidecar-rust/bin/image-sidecar-rust",
            # Legacy sportball paths for backward compatibility
            "./target/release/sportball-sidecar-rust",
            "../sportball-sidecar-rust/target/release/sportball-sidecar-rust",
            "/usr/local/bin/sportball-sidecar-rust",
        ]
        
        for path in search_paths:
            if os.path.exists(path) and os.access(path, os.X_OK):
                return path
        
        return None
    
    def _run_command(self, args: List[str]) -> Dict[str, Any]:
        """Run a Rust command and return JSON result."""
        if not self.rust_available:
            raise RuntimeError("Rust binary not available")
        
        cmd = [self.rust_binary] + args + ["--output", "-"]
        
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=self.timeout,
                check=True
            )
            return json.loads(result.stdout)
        except subprocess.TimeoutExpired:
            raise RuntimeError(f"Command timed out after {self.timeout} seconds")
        except subprocess.CalledProcessError as e:
            raise RuntimeError(f"Command failed: {e.stderr}")
        except json.JSONDecodeError as e:
            raise RuntimeError(f"Invalid JSON output: {e}")
    
    def validate_sidecars(
        self, 
        directory: Union[str, Path], 
        workers: int = 16,
        operation_type: Optional[str] = None
    ) -> List[ValidationResult]:
        """
        Validate sidecar files in a directory.
        
        Args:
            directory: Directory containing sidecar files
            workers: Number of parallel workers
            operation_type: Filter by operation type (optional)
            
        Returns:
            List of ValidationResult objects
        """
        args = [
            "validate",
            "--input", str(directory),
            "--workers", str(workers)
        ]
        
        if operation_type:
            args.extend(["--operation-type", operation_type])
        
        result = self._run_command(args)
        
        # Convert to ValidationResult objects
        validation_results = []
        for item in result.get("results", []):
            validation_results.append(ValidationResult(
                file_path=item["file_path"],
                is_valid=item["is_valid"],
                error=item.get("error"),
                processing_time=item.get("processing_time", 0.0),
                file_size=item.get("file_size", 0),
                detection_count=item.get("detection_count", 0),
                tool_name=item.get("tool_name"),
                operation_type=item.get("operation_type")
            ))
        
        return validation_results
    
    def get_statistics(
        self, 
        directory: Union[str, Path],
        operation_type: Optional[str] = None
    ) -> StatisticsResult:
        """
        Get comprehensive statistics about sidecar files.
        
        Args:
            directory: Directory containing sidecar files
            operation_type: Filter by operation type (optional)
            
        Returns:
            StatisticsResult object
        """
        args = ["stats", "--input", str(directory)]
        
        if operation_type:
            args.extend(["--operation-type", operation_type])
        
        result = self._run_command(args)
        
        return StatisticsResult(
            directory=result["directory"],
            total_images=result["total_images"],
            total_sidecars=result["total_sidecars"],
            coverage_percentage=result["coverage_percentage"],
            operation_counts=result["operation_counts"],
            avg_processing_times=result["avg_processing_times"],
            success_rate_percentages=result["success_rate_percentages"],
            avg_data_sizes=result["avg_data_sizes"]
        )
    
    def convert_format(
        self, 
        directory: Union[str, Path], 
        target_format: SidecarFormat,
        dry_run: bool = False
    ) -> str:
        """
        Convert sidecar files to a different format.
        
        Args:
            directory: Directory containing sidecar files
            target_format: Target format for conversion
            dry_run: If True, show what would be converted without converting
            
        Returns:
            Conversion result message
        """
        args = [
            "convert",
            "--input", str(directory),
            "--format", target_format.value
        ]
        
        if dry_run:
            args.append("--dry-run")
        
        result = subprocess.run(
            [self.rust_binary] + args,
            capture_output=True,
            text=True,
            timeout=self.timeout,
            check=True
        )
        
        return result.stdout
    
    def get_format_statistics(self, directory: Union[str, Path]) -> FormatStatistics:
        """
        Get format distribution statistics.
        
        Args:
            directory: Directory containing sidecar files
            
        Returns:
            FormatStatistics object
        """
        result = self._run_command([
            "format-stats",
            "--input", str(directory)
        ])
        
        return FormatStatistics(
            directory=result["directory"],
            format_distribution=result["format_distribution"],
            total_files=result["total_files"],
            generated_at=result["generated_at"]
        )
    
    def cleanup_orphaned(
        self, 
        directory: Union[str, Path], 
        dry_run: bool = False
    ) -> int:
        """
        Clean up orphaned sidecar files.
        
        Args:
            directory: Directory containing sidecar files
            dry_run: If True, show what would be cleaned without cleaning
            
        Returns:
            Number of files removed
        """
        args = ["cleanup", "--input", str(directory)]
        
        if dry_run:
            args.append("--dry-run")
        
        result = subprocess.run(
            [self.rust_binary] + args,
            capture_output=True,
            text=True,
            timeout=self.timeout,
            check=True
        )
        
        # Extract count from output
        output = result.stdout
        if "Removed" in output:
            # Extract number from "Removed X orphaned sidecar files"
            import re
            match = re.search(r'Removed (\d+)', output)
            if match:
                return int(match.group(1))
        
        return 0
    
    def export_data(
        self, 
        directory: Union[str, Path], 
        output_file: Union[str, Path],
        format_type: str = "json",
        operation_type: Optional[str] = None
    ) -> None:
        """
        Export sidecar data to a file.
        
        Args:
            directory: Directory containing sidecar files
            output_file: Output file path
            format_type: Export format (json, csv)
            operation_type: Filter by operation type (optional)
        """
        args = [
            "export",
            "--input", str(directory),
            "--output", str(output_file),
            "--format", format_type
        ]
        
        if operation_type:
            args.extend(["--operation-type", operation_type])
        
        subprocess.run(
            [self.rust_binary] + args,
            timeout=self.timeout,
            check=True
        )
    
    def get_performance_info(self) -> Dict[str, Any]:
        """Get performance information about the Rust tool."""
        if not self.rust_available:
            return {"rust_available": False}
        
        try:
            # Test with a small operation
            import tempfile
            with tempfile.TemporaryDirectory() as temp_dir:
                # Create a test file
                test_file = Path(temp_dir) / "test.json"
                test_file.write_text('{"test": "data"}')
                
                start_time = time.time()
                self.validate_sidecars(temp_dir, workers=1)
                end_time = time.time()
                
                return {
                    "rust_available": True,
                    "binary_path": self.rust_binary,
                    "test_processing_time": end_time - start_time,
                    "status": "working"
                }
        except Exception as e:
            return {
                "rust_available": True,
                "binary_path": self.rust_binary,
                "status": "error",
                "error": str(e)
            }


# Python fallback implementations
def python_validate_sidecars(directory: Union[str, Path]) -> List[ValidationResult]:
    """Python fallback for sidecar validation."""
    logger.warning("Using Python fallback for validation")
    
    results = []
    directory = Path(directory)
    
    for file_path in directory.glob("*.json"):
        try:
            with open(file_path, 'r') as f:
                json.load(f)  # Validate JSON
            results.append(ValidationResult(
                file_path=str(file_path),
                is_valid=True,
                file_size=file_path.stat().st_size
            ))
        except Exception as e:
            results.append(ValidationResult(
                file_path=str(file_path),
                is_valid=False,
                error=str(e)
            ))
    
    return results


def python_get_statistics(directory: Union[str, Path]) -> StatisticsResult:
    """Python fallback for statistics."""
    logger.warning("Using Python fallback for statistics")
    
    directory = Path(directory)
    sidecar_files = list(directory.glob("*.json"))
    image_files = list(directory.glob("*.jpg")) + list(directory.glob("*.png"))
    
    return StatisticsResult(
        directory=str(directory),
        total_images=len(image_files),
        total_sidecars=len(sidecar_files),
        coverage_percentage=(len(sidecar_files) / len(image_files) * 100) if image_files else 0,
        operation_counts={},
        avg_processing_times={},
        success_rate_percentages={},
        avg_data_sizes={}
    )


# Convenience functions
def create_manager(rust_binary_path: Optional[str] = None) -> ImageSidecarManager:
    """Create an image sidecar manager instance."""
    return ImageSidecarManager(rust_binary_path)


def validate_with_fallback(
    directory: Union[str, Path], 
    workers: int = 16,
    rust_binary_path: Optional[str] = None
) -> List[ValidationResult]:
    """
    Validate sidecars with automatic Rust/Python fallback.
    
    Args:
        directory: Directory containing sidecar files
        workers: Number of parallel workers
        rust_binary_path: Path to Rust binary (optional)
        
    Returns:
        List of ValidationResult objects
    """
    manager = create_manager(rust_binary_path)
    
    if manager.rust_available:
        return manager.validate_sidecars(directory, workers)
    else:
        return python_validate_sidecars(directory)


def get_stats_with_fallback(
    directory: Union[str, Path],
    rust_binary_path: Optional[str] = None
) -> StatisticsResult:
    """
    Get statistics with automatic Rust/Python fallback.
    
    Args:
        directory: Directory containing sidecar files
        rust_binary_path: Path to Rust binary (optional)
        
    Returns:
        StatisticsResult object
    """
    manager = create_manager(rust_binary_path)
    
    if manager.rust_available:
        return manager.get_statistics(directory)
    else:
        return python_get_statistics(directory)


# Example usage
if __name__ == "__main__":
    import time
    
    # Example usage
    manager = create_manager()
    
    if manager.rust_available:
        print("✅ Using Rust implementation")
        
        # Validate sidecars
        results = manager.validate_sidecars("/path/to/sidecars")
        print(f"Validated {len(results)} files")
        
        # Get statistics
        stats = manager.get_statistics("/path/to/sidecars")
        print(f"Total sidecars: {stats.total_sidecars}")
        
        # Convert to binary format
        print("Converting to binary format...")
        manager.convert_format("/path/to/sidecars", SidecarFormat.BINARY)
        
        # Check format distribution
        format_stats = manager.get_format_statistics("/path/to/sidecars")
        print(f"Format distribution: {format_stats.format_distribution}")
        
    else:
        print("⚠️  Rust binary not available, using Python fallback")
        results = python_validate_sidecars("/path/to/sidecars")
        print(f"Validated {len(results)} files with Python fallback")
