#!/bin/bash

# Script to publish rollpoly to crates.io with safety checks
# Usage: ./scripts/publish-crate.sh [--dry-run]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if this is a dry run
DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    echo -e "${YELLOW}🧪 Running in DRY RUN mode${NC}"
fi

echo -e "${BLUE}📦 Preparing to publish rollpoly to crates.io${NC}"

# Ensure we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    echo -e "${RED}❌ Must be on main branch to publish. Currently on: $CURRENT_BRANCH${NC}"
    exit 1
fi

# Ensure working directory is clean
if [[ -n $(git status --porcelain) ]]; then
    echo -e "${RED}❌ Working directory is not clean. Commit or stash changes first.${NC}"
    git status --short
    exit 1
fi

# Ensure we're up to date with remote
echo -e "${BLUE}🔄 Checking if local branch is up to date...${NC}"
git fetch origin main
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse origin/main)
if [[ "$LOCAL" != "$REMOTE" ]]; then
    echo -e "${RED}❌ Local branch is not up to date with origin/main${NC}"
    echo "Run: git pull origin main"
    exit 1
fi

# Get current version
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}📋 Current version: $CURRENT_VERSION${NC}"

# Check if this version already exists on crates.io
echo -e "${BLUE}🔍 Checking if version $CURRENT_VERSION already exists on crates.io...${NC}"
if cargo search rollpoly | grep -q "rollpoly = \"$CURRENT_VERSION\""; then
    echo -e "${RED}❌ Version $CURRENT_VERSION already exists on crates.io${NC}"
    echo "Bump the version first using: ./scripts/bump-version.sh [patch|minor|major]"
    exit 1
fi

# Run full test suite
echo -e "${BLUE}🧪 Running full test suite...${NC}"
./scripts/full-checks.sh

# Check package contents
echo -e "${BLUE}📋 Checking package contents...${NC}"
cargo package --list

# Verify package builds correctly
echo -e "${BLUE}🔨 Verifying package builds correctly...${NC}"
cargo package

# Show what would be published
echo -e "${BLUE}📦 Package contents that will be published:${NC}"
tar -tzf "target/package/rollpoly-$CURRENT_VERSION.crate" | head -20
echo "..."
echo "Total files: $(tar -tzf "target/package/rollpoly-$CURRENT_VERSION.crate" | wc -l)"

# Final confirmation
if [[ "$DRY_RUN" == "false" ]]; then
    echo -e "${YELLOW}⚠️  This will publish rollpoly v$CURRENT_VERSION to crates.io${NC}"
    echo -e "${YELLOW}⚠️  This action cannot be undone!${NC}"
    echo ""
    read -p "Are you sure you want to publish? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        echo -e "${YELLOW}❌ Publication cancelled${NC}"
        exit 1
    fi

    # Publish to crates.io
    echo -e "${BLUE}🚀 Publishing to crates.io...${NC}"
    cargo publish

    echo -e "${GREEN}✅ Successfully published rollpoly v$CURRENT_VERSION to crates.io!${NC}"
    echo -e "${GREEN}📦 View at: https://crates.io/crates/rollpoly${NC}"
    echo -e "${GREEN}📚 Docs will be available at: https://docs.rs/rollpoly/$CURRENT_VERSION${NC}"
else
    echo -e "${GREEN}✅ Dry run completed successfully!${NC}"
    echo -e "${BLUE}📦 Package is ready for publication${NC}"
    echo -e "${BLUE}🚀 Run without --dry-run to publish: ./scripts/publish-crate.sh${NC}"
fi

# Cleanup
rm -f "target/package/rollpoly-$CURRENT_VERSION.crate"

echo -e "${GREEN}🎉 Done!${NC}"
