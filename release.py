#!/usr/bin/env python3
"""
Date-based release management for image-sidecar-rust.

This script provides different release strategies:
- daily: YYYY.MM.DD (e.g., 2025.09.26)
- weekly: YYYY.WW (e.g., 2025.39) 
- monthly: YYYY.MM (e.g., 2025.09)
- quarterly: YYYY.Q (e.g., 2025.Q3)
"""

import subprocess
import sys
import argparse
from datetime import datetime, timedelta
from pathlib import Path


def run_command(cmd, check=True):
    """Run a shell command and return the output."""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, check=check)
        return result.stdout.strip(), result.stderr.strip(), result.returncode
    except subprocess.CalledProcessError as e:
        if check:
            raise
        return e.stdout.strip(), e.stderr.strip(), e.returncode


def get_current_date():
    """Get current date components."""
    now = datetime.now()
    return {
        'year': now.year,
        'month': now.month,
        'day': now.day,
        'week': now.isocalendar()[1],
        'quarter': (now.month - 1) // 3 + 1
    }


def generate_version(release_type='daily'):
    """Generate version string based on release type."""
    date = get_current_date()
    
    if release_type == 'daily':
        return f"{date['year']}.{date['month']:02d}.{date['day']:02d}"
    elif release_type == 'weekly':
        return f"{date['year']}.{date['week']:02d}"
    elif release_type == 'monthly':
        return f"{date['year']}.{date['month']:02d}"
    elif release_type == 'quarterly':
        return f"{date['year']}.Q{date['quarter']}"
    else:
        raise ValueError(f"Unknown release type: {release_type}")


def check_working_directory():
    """Check if working directory is clean."""
    stdout, stderr, returncode = run_command("git diff-index --quiet HEAD --", check=False)
    if returncode != 0:
        print("❌ Working directory is not clean. Please commit or stash changes first.")
        run_command("git status --short")
        return False
    return True


def check_tag_exists(tag_name):
    """Check if tag already exists."""
    stdout, stderr, returncode = run_command(f"git tag -l | grep '^{tag_name}$'", check=False)
    return returncode == 0


def create_release(release_type='daily', push=True, dry_run=False):
    """Create a date-based release."""
    version = generate_version(release_type)
    tag_name = f"v{version}"
    
    print(f"🏷️  Creating {release_type} release: {tag_name}")
    
    if not check_working_directory():
        return False
    
    if check_tag_exists(tag_name):
        print(f"❌ Tag {tag_name} already exists!")
        print("Available tags:")
        stdout, _, _ = run_command("git tag -l | grep '^v[0-9]' | sort -V")
        print(stdout)
        return False
    
    if dry_run:
        print(f"🔍 DRY RUN: Would create tag {tag_name}")
        return True
    
    # Create the tag
    run_command(f"git tag -a {tag_name} -m 'Release {version} ({release_type})'")
    
    if push:
        print("📤 Pushing tag to remote...")
        run_command(f"git push origin {tag_name}")
        print(f"✅ Successfully created and pushed tag: {tag_name}")
    else:
        print(f"✅ Successfully created tag: {tag_name}")
    
    print("\n📋 Next steps:")
    print("   1. Create a GitHub release from this tag")
    print("   2. Update CHANGELOG.md if needed")
    print("   3. Test the release:")
    print(f"      python -c \"import image_sidecar_rust; print(image_sidecar_rust.__version__)\"")
    
    return True


def main():
    parser = argparse.ArgumentParser(description="Date-based release management")
    parser.add_argument('--type', choices=['daily', 'weekly', 'monthly', 'quarterly'], 
                       default='daily', help='Release type (default: daily)')
    parser.add_argument('--no-push', action='store_true', 
                       help='Create tag locally without pushing to remote')
    parser.add_argument('--dry-run', action='store_true', 
                       help='Show what would be done without actually doing it')
    
    args = parser.parse_args()
    
    try:
        success = create_release(
            release_type=args.type,
            push=not args.no_push,
            dry_run=args.dry_run
        )
        sys.exit(0 if success else 1)
    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
