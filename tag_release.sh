#!/bin/bash
# Date-based release tagging script for image-sidecar-rust

set -e

# Get current date in YYYY.MM.DD format
RELEASE_DATE=$(date +"%Y.%m.%d")
TAG_NAME="v${RELEASE_DATE}"

# Check if tag already exists
if git tag -l | grep -q "^${TAG_NAME}$"; then
    echo "❌ Tag ${TAG_NAME} already exists!"
    echo "Available tags:"
    git tag -l | grep "^v[0-9]" | sort -V
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    echo "❌ Working directory is not clean. Please commit or stash changes first."
    git status --short
    exit 1
fi

# Create and push the tag
echo "🏷️  Creating date-based tag: ${TAG_NAME}"
git tag -a "${TAG_NAME}" -m "Release ${RELEASE_DATE}"

echo "📤 Pushing tag to remote..."
git push origin "${TAG_NAME}"

echo "✅ Successfully created and pushed tag: ${TAG_NAME}"
echo ""
echo "📋 Next steps:"
echo "   1. Create a GitHub release from this tag"
echo "   2. Update CHANGELOG.md if needed"
echo "   3. Test the release: python -c \"import image_sidecar_rust; print(image_sidecar_rust.__version__)\""
