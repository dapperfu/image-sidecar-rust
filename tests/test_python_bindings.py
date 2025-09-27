"""
Tests for Python bindings of image-sidecar-rust.

This module contains tests to verify that the Python bindings work correctly
and provide the expected functionality.
"""

import pytest
import tempfile
import os
from pathlib import Path
from image_sidecar_rust import ImageSidecar, SidecarFormat, OperationType
from image_sidecar_rust.exceptions import SidecarError


class TestImageSidecar:
    """Test cases for ImageSidecar class."""
    
    def test_initialization(self) -> None:
        """Test that ImageSidecar can be initialized."""
        sidecar = ImageSidecar()
        assert sidecar.rust_available is True
    
    def test_initialization_with_workers(self) -> None:
        """Test initialization with specific worker count."""
        sidecar = ImageSidecar(max_workers=8)
        assert sidecar.rust_available is True
    
    def test_validate_sidecars_empty_directory(self) -> None:
        """Test validation of empty directory."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            results = sidecar.validate_sidecars(temp_dir)
            assert isinstance(results, list)
            assert len(results) == 0
    
    def test_get_statistics_empty_directory(self) -> None:
        """Test statistics collection for empty directory."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            stats = sidecar.get_statistics(temp_dir)
            assert isinstance(stats, dict)
            assert stats['total_images'] == 0
            assert stats['total_sidecars'] == 0
            assert stats['coverage_percentage'] == 0.0
            assert isinstance(stats['operation_counts'], dict)
            assert isinstance(stats['avg_processing_times'], dict)
            assert isinstance(stats['success_rate_percentages'], dict)
            assert isinstance(stats['avg_data_sizes'], dict)
    
    def test_find_sidecars_empty_directory(self) -> None:
        """Test finding sidecars in empty directory."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            sidecars = sidecar.find_sidecars(temp_dir)
            assert isinstance(sidecars, list)
            assert len(sidecars) == 0
    
    def test_create_sidecar(self) -> None:
        """Test creating a sidecar file."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            # Create a fake image file
            image_path = Path(temp_dir) / "test.jpg"
            image_path.write_bytes(b"fake image data")
            
            # Create sidecar
            data = {"test": "data", "confidence": 0.95}
            result = sidecar.create_sidecar(
                image_path, 
                OperationType.FACE_DETECTION, 
                data
            )
            
            assert isinstance(result, dict)
            assert 'image_path' in result
            assert 'sidecar_path' in result
            assert 'operation' in result
            assert 'data_size' in result
            assert 'created_at' in result
            assert 'is_valid' in result
            
            # Verify the sidecar file was created
            sidecar_path = Path(result['sidecar_path'])
            assert sidecar_path.exists()
            assert sidecar_path.stat().st_size > 0
    
    def test_cleanup_orphaned_empty_directory(self) -> None:
        """Test cleanup in empty directory."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            count = sidecar.cleanup_orphaned(temp_dir)
            assert isinstance(count, int)
            assert count == 0
    
    def test_convert_directory_format_empty_directory(self) -> None:
        """Test format conversion in empty directory."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            count = sidecar.convert_directory_format(temp_dir, SidecarFormat.BINARY)
            assert isinstance(count, int)
            assert count == 0
    
    def test_get_format_statistics_empty_directory(self) -> None:
        """Test format statistics for empty directory."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            stats = sidecar.get_format_statistics(temp_dir)
            assert isinstance(stats, dict)
            assert len(stats) == 0
    
    def test_set_get_default_format(self) -> None:
        """Test setting and getting default format."""
        sidecar = ImageSidecar()
        
        # Test setting different formats
        format_mappings = {"json": "json", "binary": "bin", "rkyv": "rkyv"}
        for format_str, expected in format_mappings.items():
            sidecar.set_default_format(format_str)
            current_format = sidecar.get_default_format()
            assert current_format == expected
        
        # Test with SidecarFormat enum
        sidecar.set_default_format(SidecarFormat.BINARY)
        assert sidecar.get_default_format() == "bin"


class TestSidecarFormat:
    """Test cases for SidecarFormat class."""
    
    def test_valid_formats(self) -> None:
        """Test creating SidecarFormat with valid formats."""
        formats = ["json", "binary", "rkyv"]
        for fmt in formats:
            sidecar_format = SidecarFormat(fmt)
            assert str(sidecar_format) == fmt
            assert repr(sidecar_format) == f"SidecarFormat('{fmt}')"
    
    def test_case_insensitive(self) -> None:
        """Test that format strings are case insensitive."""
        formats = ["JSON", "Binary", "RKYV"]
        expected = ["json", "binary", "rkyv"]
        
        for fmt, expected_fmt in zip(formats, expected):
            sidecar_format = SidecarFormat(fmt)
            assert str(sidecar_format) == expected_fmt
    
    def test_invalid_format(self) -> None:
        """Test that invalid format raises ValueError."""
        with pytest.raises(ValueError, match="Unknown format"):
            SidecarFormat("invalid_format")


class TestOperationType:
    """Test cases for OperationType class."""
    
    def test_valid_operations(self) -> None:
        """Test creating OperationType with valid operations."""
        operations = [
            "face_detection",
            "object_detection", 
            "ball_detection",
            "quality_assessment",
            "game_detection",
            "yolov8"
        ]
        
        for op in operations:
            operation_type = OperationType(op)
            assert str(operation_type) == op
            assert repr(operation_type) == f"OperationType('{op}')"
    
    def test_case_insensitive(self) -> None:
        """Test that operation strings are case insensitive."""
        operations = ["FACE_DETECTION", "Object_Detection", "BALL_DETECTION"]
        expected = ["face_detection", "object_detection", "ball_detection"]
        
        for op, expected_op in zip(operations, expected):
            operation_type = OperationType(op)
            assert str(operation_type) == expected_op
    
    def test_invalid_operation(self) -> None:
        """Test that invalid operation raises ValueError."""
        with pytest.raises(ValueError, match="Unknown operation"):
            OperationType("invalid_operation")


class TestIntegration:
    """Integration tests for the complete workflow."""
    
    def test_complete_workflow(self) -> None:
        """Test a complete workflow: create, validate, get stats."""
        sidecar = ImageSidecar()
        
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            
            # Create multiple sidecar files
            for i in range(3):
                image_path = temp_path / f"test_{i}.jpg"
                image_path.write_bytes(b"fake image data")
                
                data = {"test": f"data_{i}", "confidence": 0.9 + i * 0.01}
                sidecar.create_sidecar(
                    image_path,
                    OperationType.FACE_DETECTION,
                    data
                )
            
            # Validate sidecars
            results = sidecar.validate_sidecars(temp_dir)
            assert len(results) == 3
            for result in results:
                assert result['is_valid'] is True
                assert 'file_path' in result
                assert 'processing_time' in result
            
            # Get statistics
            stats = sidecar.get_statistics(temp_dir)
            assert stats['total_images'] == 3
            assert stats['total_sidecars'] == 3
            assert stats['coverage_percentage'] > 0
            
            # Find sidecars
            found_sidecars = sidecar.find_sidecars(temp_dir)
            assert len(found_sidecars) == 3
            
            # Get format statistics
            format_stats = sidecar.get_format_statistics(temp_dir)
            assert len(format_stats) > 0
            assert sum(format_stats.values()) == 3


if __name__ == "__main__":
    pytest.main([__file__])
