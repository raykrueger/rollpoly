#!/bin/bash
# Script to enable full pre-commit hook with clippy checks
# Run this after addressing all clippy warnings

echo "Enabling full pre-commit hook with clippy checks..."

cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
# Pre-commit hook for RKDice project
# Ensures code quality before commits per steering document requirements

echo "Running pre-commit checks..."

# Run tests
echo "🧪 Running tests..."
if ! cargo test; then
    echo "❌ Tests failed. Commit aborted."
    exit 1
fi

# Run clippy
echo "📎 Running clippy..."
if ! cargo clippy -- -D warnings; then
    echo "❌ Clippy warnings found. Commit aborted."
    echo "💡 Fix clippy warnings or use #[allow(...)] if intentional"
    exit 1
fi

# Check formatting
echo "🎨 Checking code formatting..."
if ! cargo fmt -- --check; then
    echo "❌ Code is not formatted. Running cargo fmt..."
    cargo fmt
    echo "✅ Code formatted. Please stage the changes and commit again."
    exit 1
fi

# Check compilation
echo "🔧 Checking compilation..."
if ! cargo check; then
    echo "❌ Compilation failed. Commit aborted."
    exit 1
fi

echo "✅ All pre-commit checks passed!"
exit 0
EOF

chmod +x .git/hooks/pre-commit

echo "✅ Full pre-commit hook enabled!"
echo "🎯 All commits will now be validated with clippy checks."
