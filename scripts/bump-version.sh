#!/bin/bash

# Script to bump version and create a release
# Usage: ./scripts/bump-version.sh [patch|minor|major]

set -e

BUMP_TYPE=${1:-patch}

if [[ ! "$BUMP_TYPE" =~ ^(patch|minor|major)$ ]]; then
    echo "Error: Invalid bump type. Use 'patch', 'minor', or 'major'"
    echo "Usage: $0 [patch|minor|major]"
    exit 1
fi

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "Error: Must be on main branch to create a release"
    echo "Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "Error: Working directory is not clean. Please commit or stash changes."
    git status --short
    exit 1
fi

# Get current version
CURRENT_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
echo "Current version: $CURRENT_VERSION"

# Install cargo-edit if not available
if ! command -v cargo-set-version &> /dev/null; then
    echo "Installing cargo-edit..."
    cargo install cargo-edit
fi

# Bump version
echo "Bumping $BUMP_TYPE version..."
cargo set-version --bump $BUMP_TYPE

# Get new version
NEW_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
TAG_NAME="v$NEW_VERSION"

echo "New version: $NEW_VERSION"
echo "Tag name: $TAG_NAME"

# Run tests to make sure everything still works
echo "Running tests..."
cargo test

# Commit version bump
echo "Committing version bump..."
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $NEW_VERSION"

# Create and push tag
echo "Creating and pushing tag: $TAG_NAME"
git tag "$TAG_NAME"
git push origin main
git push origin "$TAG_NAME"

echo "✅ Version bumped to $NEW_VERSION and tag $TAG_NAME pushed!"
echo "🚀 GitHub Actions will now build and create the release automatically."
echo ""
echo "Release will be available at:"
echo "https://github.com/raykrueger/rollpoly/releases/tag/$TAG_NAME"
