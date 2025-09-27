"""
Exception classes for sportball-sidecar-rust.

This module defines custom exception classes that wrap Rust errors
and provide Python-friendly error handling.
"""

from typing import Optional


class SidecarError(Exception):
    """Base exception for all sidecar-related errors."""
    
    def __init__(self, message: str, error_code: Optional[str] = None) -> None:
        """Initialize the exception with a message and optional error code.
        
        Args:
            message: Human-readable error message
            error_code: Optional error code for programmatic handling
        """
        super().__init__(message)
        self.message = message
        self.error_code = error_code


class ValidationError(SidecarError):
    """Exception raised when sidecar validation fails."""
    pass


class StatisticsError(SidecarError):
    """Exception raised when statistics collection fails."""
    pass


class FormatError(SidecarError):
    """Exception raised when format conversion fails."""
    pass


class IOWarning(SidecarError):
    """Exception raised for I/O related warnings."""
    pass
