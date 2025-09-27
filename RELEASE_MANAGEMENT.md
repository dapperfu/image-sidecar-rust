# Release Management

This project uses **date-based versioning** with automatic version management via versioneer.

## Version Format

Versions follow the format: `YYYY.MM.DD` (e.g., `2025.09.26`)

### Release Types

- **Daily**: `YYYY.MM.DD` (e.g., `2025.09.26`)
- **Weekly**: `YYYY.WW` (e.g., `2025.39`) 
- **Monthly**: `YYYY.MM` (e.g., `2025.09`)
- **Quarterly**: `YYYY.Q` (e.g., `2025.Q3`)

## Quick Start

### Using Makefile (Recommended)

```bash
# Create a daily release
make release-daily

# Create a weekly release  
make release-weekly

# Create a monthly release
make release-monthly

# Create a quarterly release
make release-quarterly

# Test releases (dry run)
make release-daily-dry
make release-weekly-dry
make release-monthly-dry
make release-quarterly-dry
```

### Using Python Script Directly

```bash
# Create releases
python release.py --type daily
python release.py --type weekly
python release.py --type monthly
python release.py --type quarterly

# Dry run (test without creating tags)
python release.py --type daily --dry-run

# Create tag locally without pushing
python release.py --type daily --no-push
```

### Using Simple Shell Script

```bash
# Simple daily release
./tag_release.sh
```

## How It Works

1. **Versioneer Integration**: The project uses versioneer to automatically manage versions based on git tags
2. **Automatic Versioning**: When you create a tag, versioneer automatically updates the version in the Python package
3. **Git Integration**: All releases are git tags that can be used for GitHub releases

## Version Examples

- `v2025.09.26` - Daily release for September 26, 2025
- `v2025.39` - Weekly release for week 39 of 2025
- `v2025.09` - Monthly release for September 2025
- `v2025.Q3` - Quarterly release for Q3 2025

## Development Versions

When working on development branches or with uncommitted changes, versioneer automatically generates development versions:

- `0.1.0+2.gca42e0b` - Development version (2 commits ahead of last tag)
- `0.1.0+0.gca42e0b.dirty` - Dirty working directory

## Best Practices

1. **Clean Working Directory**: Always commit or stash changes before creating releases
2. **Test First**: Use `--dry-run` to test release creation
3. **Consistent Naming**: Stick to one release type per project (e.g., daily for CI/CD, monthly for stable releases)
4. **Documentation**: Update CHANGELOG.md when creating releases

## Integration with CI/CD

You can integrate this with your CI/CD pipeline:

```yaml
# GitHub Actions example
- name: Create Daily Release
  run: |
    git config --local user.email "action@github.com"
    git config --local user.name "GitHub Action"
    make release-daily
```

## Troubleshooting

### "Working directory is not clean"
- Commit or stash your changes before creating a release
- Use `git status` to see what files are modified

### "Tag already exists"
- The tag for today's date already exists
- Use a different release type or wait for the next day/week/month

### Version not updating
- Make sure versioneer is properly installed: `pip install versioneer`
- Rebuild the package: `pip install -e .`

