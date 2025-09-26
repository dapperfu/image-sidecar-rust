#!/usr/bin/env python3
"""
Setup script for sportball-sidecar-rust.
This is a minimal setup.py required by versioneer for mixed Python/Rust projects.
The actual build is handled by maturin via pyproject.toml.
"""

import versioneer

# Minimal setup for versioneer compatibility
setup_args = {
    "version": versioneer.get_version(),
    "cmdclass": versioneer.get_cmdclass(),
}

if __name__ == "__main__":
    # This is just for versioneer - maturin handles the actual build
    pass
