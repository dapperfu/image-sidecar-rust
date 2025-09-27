"""
Image Sidecar Rust - High-performance Rust implementation for image JSON sidecar operations.

This package provides a high-performance Rust implementation for image JSON sidecar operations,
offering 3-10x performance improvements over Python implementations through:
- Massive parallelism using rayon
- Zero-copy operations
- Efficient async I/O
- Memory safety guarantees
- SIMD optimizations

The package supports multiple formats (JSON, Binary, Rkyv) and provides comprehensive
sidecar file management capabilities.
"""

from .core import ImageSidecar, SidecarFormat, OperationType
from .exceptions import SidecarError, ValidationError, StatisticsError

__version__ = "0.1.0"
__author__ = "Image Sidecar Team"
__email__ = "team@imagesidecar.com"

__all__ = [
    "ImageSidecar",
    "SidecarFormat", 
    "OperationType",
    "SidecarError",
    "ValidationError", 
    "StatisticsError",
]

from . import _version
__version__ = _version.get_versions()['version']
