#!/bin/bash
# Full quality checks script for RKDice project
# Used by pre-commit hook and can be run manually

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
