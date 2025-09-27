"""
Core Python interface for image-sidecar-rust.

This module provides the main Python interface to the Rust implementation,
offering high-performance sidecar file operations.
"""

from typing import Optional, Dict, List, Union, Any
from pathlib import Path
import json

from .exceptions import SidecarError, ValidationError, StatisticsError, FormatError


class SidecarFormat:
    """Enumeration of supported sidecar formats."""
    
    JSON = "json"
    BINARY = "binary"
    RKYV = "rkyv"
    
    def __init__(self, format_str: str) -> None:
        """Initialize with format string.
        
        Args:
            format_str: Format string ('json', 'binary', or 'rkyv')
        """
        format_str = format_str.lower()
        if format_str not in [self.JSON, self.BINARY, self.RKYV]:
            raise ValueError(f"Unknown format: {format_str}")
        self._format = format_str
    
    def __str__(self) -> str:
        return self._format
    
    def __repr__(self) -> str:
        return f"SidecarFormat('{self._format}')"


class OperationType:
    """Enumeration of supported operation types."""
    
    FACE_DETECTION = "face_detection"
    OBJECT_DETECTION = "object_detection"
    BALL_DETECTION = "ball_detection"
    QUALITY_ASSESSMENT = "quality_assessment"
    GAME_DETECTION = "game_detection"
    YOLOV8 = "yolov8"
    
    def __init__(self, operation_str: str) -> None:
        """Initialize with operation string.
        
        Args:
            operation_str: Operation string
        """
        operation_str = operation_str.lower()
        if operation_str not in [
            self.FACE_DETECTION, self.OBJECT_DETECTION, self.BALL_DETECTION,
            self.QUALITY_ASSESSMENT, self.GAME_DETECTION, self.YOLOV8
        ]:
            raise ValueError(f"Unknown operation: {operation_str}")
        self._operation = operation_str
    
    def __str__(self) -> str:
        return self._operation
    
    def __repr__(self) -> str:
        return f"OperationType('{self._operation}')"


class ImageSidecar:
    """
    High-performance Rust implementation for image JSON sidecar operations.
    
    This class provides a Python interface to the Rust implementation, offering
    3-10x performance improvements over pure Python implementations.
    
    Example:
        >>> sidecar = ImageSidecar(max_workers=16)
        >>> results = sidecar.validate_sidecars("/path/to/directory")
        >>> stats = sidecar.get_statistics("/path/to/directory")
    """
    
    def __init__(self, max_workers: Optional[int] = None) -> None:
        """Initialize the ImageSidecar instance.
        
        Args:
            max_workers: Maximum number of worker threads. If None, uses all available CPU cores.
        """
        try:
            import image_sidecar_rust.image_sidecar_rust as rust_ext
            self._rust_impl = rust_ext.PyImageSidecar(max_workers)
            self._rust_available = True
        except ImportError:
            self._rust_impl = None
            self._rust_available = False
            raise ImportError(
                "Rust implementation not available. Please ensure the package is properly installed."
            )
    
    @property
    def rust_available(self) -> bool:
        """Check if the Rust implementation is available."""
        return self._rust_available
    
    def validate_sidecars(self, directory: Union[str, Path]) -> List[Dict[str, Any]]:
        """Validate JSON sidecar files in parallel.
        
        Args:
            directory: Directory path to validate sidecar files in
            
        Returns:
            List of validation results with 'sidecar_path', 'is_valid', 
            'error_message', and 'processing_time_ms' keys
            
        Raises:
            ValidationError: If validation fails
        """
        if not self._rust_available:
            raise ValidationError("Rust implementation not available")
        
        try:
            directory_str = str(directory)
            results = self._rust_impl.validate_sidecars(directory_str)
            return [
                {
                    'file_path': result.file_path,
                    'is_valid': result.is_valid,
                    'error': result.error,
                    'processing_time': result.processing_time,
                    'file_size': result.file_size,
                }
                for result in results
            ]
        except Exception as e:
            raise ValidationError(f"Validation failed: {e}")
    
    def get_statistics(self, directory: Union[str, Path]) -> Dict[str, Any]:
        """Get comprehensive statistics about sidecar files.
        
        Args:
            directory: Directory path to analyze
            
        Returns:
            Dictionary with statistics including 'total_files', 'valid_files',
            'invalid_files', 'total_size_bytes', 'average_size_bytes',
            'operation_counts', 'format_counts', and 'processing_time_ms'
            
        Raises:
            StatisticsError: If statistics collection fails
        """
        if not self._rust_available:
            raise StatisticsError("Rust implementation not available")
        
        try:
            directory_str = str(directory)
            stats = self._rust_impl.get_statistics(directory_str)
            return {
                'total_images': stats.total_images,
                'total_sidecars': stats.total_sidecars,
                'coverage_percentage': stats.coverage_percentage,
                'operation_counts': stats.operation_counts,
                'avg_processing_times': stats.avg_processing_times,
                'success_rate_percentages': stats.success_rate_percentages,
                'avg_data_sizes': stats.avg_data_sizes,
            }
        except Exception as e:
            raise StatisticsError(f"Statistics collection failed: {e}")
    
    def find_sidecars(self, directory: Union[str, Path]) -> List[Dict[str, Any]]:
        """Find all sidecar files in a directory.
        
        Args:
            directory: Directory path to search
            
        Returns:
            List of sidecar info dictionaries with 'image_path', 'sidecar_path',
            'operation', 'format', 'size_bytes', and 'created_at' keys
        """
        if not self._rust_available:
            raise SidecarError("Rust implementation not available")
        
        try:
            directory_str = str(directory)
            sidecars = self._rust_impl.find_sidecars(directory_str)
            return [
                {
                    'image_path': sidecar.image_path,
                    'sidecar_path': sidecar.sidecar_path,
                    'operation': str(sidecar.operation),
                    'data_size': sidecar.data_size,
                    'created_at': sidecar.created_at,
                    'is_valid': sidecar.is_valid,
                }
                for sidecar in sidecars
            ]
        except Exception as e:
            raise SidecarError(f"Sidecar search failed: {e}")
    
    def create_sidecar(
        self,
        image_path: Union[str, Path],
        operation: Union[str, OperationType],
        data: Dict[str, Any],
    ) -> Dict[str, Any]:
        """Create a new sidecar file.
        
        Args:
            image_path: Path to the image file
            operation: Operation type (string or OperationType enum)
            data: Data to store in the sidecar file
            
        Returns:
            Dictionary with sidecar info
            
        Raises:
            SidecarError: If sidecar creation fails
        """
        if not self._rust_available:
            raise SidecarError("Rust implementation not available")
        
        try:
            image_path_str = str(image_path)
            if isinstance(operation, str):
                op_type = OperationType(operation)
            else:
                op_type = operation
            
            # Convert Python OperationType to Rust PyOperationType
            import image_sidecar_rust.image_sidecar_rust as rust_ext
            rust_op_type = rust_ext.PyOperationType(str(op_type))
            
            sidecar_info = self._rust_impl.create_sidecar(
                image_path_str, rust_op_type, data
            )
            return {
                'image_path': sidecar_info.image_path,
                'sidecar_path': sidecar_info.sidecar_path,
                'operation': str(sidecar_info.operation),
                'data_size': sidecar_info.data_size,
                'created_at': sidecar_info.created_at,
                'is_valid': sidecar_info.is_valid,
            }
        except Exception as e:
            raise SidecarError(f"Sidecar creation failed: {e}")
    
    def cleanup_orphaned(self, directory: Union[str, Path]) -> int:
        """Clean up orphaned sidecar files.
        
        Args:
            directory: Directory path to clean up
            
        Returns:
            Number of orphaned files removed
        """
        if not self._rust_available:
            raise SidecarError("Rust implementation not available")
        
        try:
            directory_str = str(directory)
            return self._rust_impl.cleanup_orphaned(directory_str)
        except Exception as e:
            raise SidecarError(f"Cleanup failed: {e}")
    
    def convert_directory_format(
        self,
        directory: Union[str, Path],
        target_format: Union[str, SidecarFormat],
    ) -> int:
        """Convert sidecar files between formats.
        
        Args:
            directory: Directory path to convert
            target_format: Target format (string or SidecarFormat enum)
            
        Returns:
            Number of files converted
        """
        if not self._rust_available:
            raise FormatError("Rust implementation not available")
        
        try:
            directory_str = str(directory)
            if isinstance(target_format, str):
                fmt = SidecarFormat(target_format)
            else:
                fmt = target_format
            
            # Convert Python SidecarFormat to Rust PySidecarFormat
            import image_sidecar_rust.image_sidecar_rust as rust_ext
            rust_fmt = rust_ext.PySidecarFormat(str(fmt))
            
            return self._rust_impl.convert_directory_format(directory_str, rust_fmt)
        except Exception as e:
            raise FormatError(f"Format conversion failed: {e}")
    
    def get_format_statistics(self, directory: Union[str, Path]) -> Dict[str, int]:
        """Get format statistics for a directory.
        
        Args:
            directory: Directory path to analyze
            
        Returns:
            Dictionary mapping format names to file counts
        """
        if not self._rust_available:
            raise StatisticsError("Rust implementation not available")
        
        try:
            directory_str = str(directory)
            return self._rust_impl.get_format_statistics(directory_str)
        except Exception as e:
            raise StatisticsError(f"Format statistics failed: {e}")
    
    def set_default_format(self, format: Union[str, SidecarFormat]) -> None:
        """Set the default format for new sidecar files.
        
        Args:
            format: Default format (string or SidecarFormat enum)
        """
        if not self._rust_available:
            raise SidecarError("Rust implementation not available")
        
        if isinstance(format, str):
            fmt = SidecarFormat(format)
        else:
            fmt = format
        
        # Convert Python SidecarFormat to Rust PySidecarFormat
        import image_sidecar_rust.image_sidecar_rust as rust_ext
        rust_fmt = rust_ext.PySidecarFormat(str(fmt))
        
        self._rust_impl.set_default_format(rust_fmt)
    
    def get_default_format(self) -> str:
        """Get the current default format.
        
        Returns:
            Current default format as string
        """
        if not self._rust_available:
            raise SidecarError("Rust implementation not available")
        
        return str(self._rust_impl.get_default_format())
