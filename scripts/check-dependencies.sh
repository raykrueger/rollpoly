#!/bin/bash

# Script to manually check for outdated dependencies and security vulnerabilities
# Usage: ./scripts/check-dependencies.sh
#
# Note: This project uses Dependabot for automated dependency updates.
# This script is useful for manual checks and immediate updates.

set -e

echo "🔍 Manual dependency check for rollpoly..."
echo "💡 Note: Dependabot automatically creates PRs for dependency updates daily"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if required tools are installed
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${YELLOW}Installing $1...${NC}"
        cargo install "$1"
    fi
}

echo "📦 Ensuring required tools are installed..."
check_tool cargo-outdated
check_tool cargo-audit
echo ""

# Check for outdated dependencies
echo -e "${BLUE}📊 Checking for outdated dependencies...${NC}"
echo ""

OUTDATED_OUTPUT=$(cargo outdated 2>/dev/null || true)
if [ -n "$OUTDATED_OUTPUT" ]; then
    echo "$OUTDATED_OUTPUT"
    echo ""
    
    # Count outdated dependencies
    OUTDATED_COUNT=$(echo "$OUTDATED_OUTPUT" | grep -c "→" || echo "0")
    if [ "$OUTDATED_COUNT" -gt 0 ]; then
        echo -e "${YELLOW}Found $OUTDATED_COUNT outdated dependencies${NC}"
        echo -e "${BLUE}💡 Dependabot will create PRs for these automatically${NC}"
    else
        echo -e "${GREEN}✅ All dependencies are up to date!${NC}"
    fi
else
    echo -e "${GREEN}✅ All dependencies are up to date!${NC}"
fi

echo ""
echo "----------------------------------------"
echo ""

# Security audit
echo -e "${BLUE}🔒 Running security audit...${NC}"
echo ""

AUDIT_OUTPUT=$(cargo audit 2>/dev/null || true)
if echo "$AUDIT_OUTPUT" | grep -q "Vulnerabilities found!"; then
    echo -e "${RED}🚨 Security vulnerabilities found:${NC}"
    echo "$AUDIT_OUTPUT"
    echo ""
    echo -e "${RED}⚠️  URGENT: Address these security vulnerabilities immediately!${NC}"
    echo -e "${BLUE}💡 Dependabot will also create PRs for security updates${NC}"
else
    echo -e "${GREEN}✅ No security vulnerabilities found!${NC}"
fi

echo ""
echo "----------------------------------------"
echo ""

# Summary and recommendations
echo -e "${BLUE}📋 Summary and Recommendations:${NC}"
echo ""

if echo "$OUTDATED_OUTPUT" | grep -q "→" || echo "$AUDIT_OUTPUT" | grep -q "Vulnerabilities found!"; then
    echo -e "${YELLOW}🔧 Actions available:${NC}"
    echo ""
    
    if echo "$AUDIT_OUTPUT" | grep -q "Vulnerabilities found!"; then
        echo -e "${RED}1. URGENT - Address security vulnerabilities immediately:${NC}"
        echo "   cargo update"
        echo "   cargo audit"
        echo ""
    fi
    
    echo -e "${YELLOW}2. Manual update (or wait for Dependabot PR):${NC}"
    echo "   # Check what will be updated:"
    echo "   cargo outdated"
    echo ""
    echo "   # Update all to latest compatible versions:"
    echo "   cargo update"
    echo ""
    echo "   # Or update specific packages:"
    echo "   cargo update -p <package_name>"
    echo ""
    
    echo -e "${YELLOW}3. Test after updates:${NC}"
    echo "   cargo test"
    echo "   ./scripts/full-checks.sh"
    echo ""
    
    echo -e "${BLUE}🤖 Automated option:${NC}"
    echo "   • Dependabot runs daily and creates PRs automatically"
    echo "   • PRs include dependency changes and are ready to review"
    echo "   • Security updates get higher priority"
    echo ""
    
else
    echo -e "${GREEN}🎉 Everything looks good!${NC}"
    echo "   • All dependencies are up to date"
    echo "   • No security vulnerabilities found"
    echo "   • No action needed"
fi

echo ""
echo -e "${BLUE}💡 About Dependabot:${NC}"
echo "   • Runs daily at 2 AM UTC to check for updates"
echo "   • Creates grouped PRs for patch and minor updates"
echo "   • Handles GitHub Actions updates weekly"
echo "   • Automatically rebases PRs when conflicts occur"
echo "   • Includes security vulnerability fixes"
echo ""

echo -e "${BLUE}🔧 Pro tips:${NC}"
echo "   • Review and merge Dependabot PRs regularly"
echo "   • Use this script for immediate manual updates"
echo "   • Security updates should be prioritized"
echo "   • Always test thoroughly after dependency updates"
echo ""

# Check if we're in a git repository and show status
if git rev-parse --git-dir > /dev/null 2>&1; then
    if [ -n "$(git status --porcelain)" ]; then
        echo -e "${YELLOW}📝 Note: You have uncommitted changes${NC}"
        echo "   Consider committing current work before updating dependencies"
        echo ""
    fi
fi

echo "✨ Manual dependency check complete!"
echo "🤖 Remember: Dependabot is handling automated updates for you!"
