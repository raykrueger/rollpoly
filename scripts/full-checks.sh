#!/bin/bash
# Full quality checks script for RKDice project
# Run this when you're ready to address all clippy warnings

echo "Running full quality checks (including clippy)..."

# Run tests
echo "🧪 Running tests..."
if ! cargo test; then
    echo "❌ Tests failed."
    exit 1
fi

# Run clippy with all warnings
echo "📎 Running clippy..."
if ! cargo clippy -- -D warnings; then
    echo "❌ Clippy warnings found."
    echo "💡 Fix clippy warnings or use #[allow(...)] if intentional"
    exit 1
fi

# Check formatting
echo "🎨 Checking code formatting..."
if ! cargo fmt -- --check; then
    echo "❌ Code is not formatted."
    exit 1
fi

# Check compilation
echo "🔧 Checking compilation..."
if ! cargo check; then
    echo "❌ Compilation failed."
    exit 1
fi

echo "✅ All quality checks passed!"
echo "🎉 Your code is ready for production!"
