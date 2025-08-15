#!/bin/bash
# Script to enable full pre-commit hook with clippy checks
# Note: Pre-commit hook now always runs full checks via full-checks.sh

echo "Enabling full pre-commit hook with clippy checks..."

cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
# Pre-commit hook for RKDice project
# Ensures code quality before commits per steering document requirements

# Run the full checks script
./scripts/full-checks.sh
EOF

chmod +x .git/hooks/pre-commit

echo "✅ Full pre-commit hook enabled!"
echo "🎯 All commits will now be validated via scripts/full-checks.sh"
echo "💡 You can also run './scripts/full-checks.sh' manually anytime"
