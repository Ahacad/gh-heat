#!/bin/bash
set -e

# Make sure we have cargo-release installed
if ! command -v cargo-release &> /dev/null; then
    echo "Installing cargo-release..."
    cargo install cargo-release
fi

# Check if version is provided
if [ -z "$1" ]; then
    echo "Usage: ./release.sh <version>"
    echo "Example: ./release.sh 0.1.1"
    exit 1
fi

VERSION=$1

# Update version
echo "Releasing version $VERSION..."

# Dry run first to check if everything is in order
echo "Running dry run..."
cargo release --no-publish --no-push --no-tag --dry-run $VERSION

# Ask for confirmation before publishing
read -p "Ready to create a release v$VERSION? This will push to GitHub. (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Perform the actual release
    cargo release $VERSION --no-publish --execute
    
    echo "Release v$VERSION created and pushed to GitHub!"
    echo "GitHub Actions will automatically build and publish release assets."
else
    echo "Release canceled."
fi
