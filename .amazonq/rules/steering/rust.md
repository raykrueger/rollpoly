# Rust Idioms and Style Guide

Concise guide for idiomatic Rust practices and conventions.

## Naming Conventions

- **Crates**: `snake_case` (prefer single words)
- **Files/Modules**: `snake_case` (`my_module.rs`, `mod my_module;`)
- **Functions/Variables**: `snake_case` (`calculate_total_price`, `user_count`)
- **Types**: `PascalCase` (`UserAccount`, `HttpStatus`, `Drawable`)
- **Constants**: `SCREAMING_SNAKE_CASE` (`MAX_RETRY_ATTEMPTS`)
- **Generics**: Single uppercase letters (`T`, `Input`, `Output`)

## Public API Guidelines

- **Minimize public surface**: Only expose what users need
- **Use `#[non_exhaustive]`** on enums/structs that may grow
- **Prefer `&str` over `String`** for parameters
- **Return `Result<T, E>`** for fallible operations
- **Use semantic versioning**: Major.Minor.Patch (breaking.feature.bugfix)

## Code Style

- **Always use `rustfmt`**: `cargo fmt`
- **Import grouping**: std, external crates, local modules
- **Line length**: 100 characters (rustfmt default)

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::config::Config;
```

## Error Handling

**Applications**: Use `anyhow` for flexible error handling
```rust
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config: {}", path))?;
    Ok(toml::from_str(&contents)?)
}
```

**Libraries**: Use `thiserror` for structured error types
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}
```

## Memory Management

- **Prefer borrowing**: Use `&str` instead of `String` for parameters
- **Avoid explicit lifetimes** when compiler can infer
- **Clone sparingly**: Only when necessary, prefer borrowing

```rust
// Good - compiler infers lifetimes
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

## Collections & Iterators

- **Prefer iterators** over index-based loops
- **Use `collect()` sparingly** - often unnecessary
- **Chain operations** for readability

```rust
// Good - iterator chains
let sum: i32 = data
    .iter()
    .filter(|&&x| x > 0)
    .map(|x| x * 2)
    .sum();
```

## Pattern Matching

- **Use `match` for exhaustive patterns**
- **Use `if let` for single pattern**
- **Prefer `matches!` macro for boolean checks**

```rust
// Exhaustive matching
match result {
    Ok(value) => process(value),
    Err(e) => handle_error(e),
}

// Single pattern
if let Some(value) = option {
    process(value);
}
```

## Option & Result

- **Use `?` operator** for early returns
- **Avoid `unwrap()`** in production code
- **Use `expect()`** with descriptive messages for development

```rust
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    let trimmed = s.trim();
    let number = trimmed.parse()?;
    Ok(number)
}
```

## Performance Best Practices

### Memory
- **Pre-allocate collections**: `Vec::with_capacity(n)` when size is known
- **Avoid `format!()` in hot paths**: Use static strings
- **Pass expensive resources by reference**: RNG, large structs

### Parsing
- **Single-pass parsing**: Avoid multiple `contains()` calls
- **Use byte matching** for ASCII operations
- **Cache compiled regexes** with `lazy_static`

### Common Anti-patterns
```rust
// Avoid - repeated allocations
for item in items {
    let mut temp = Vec::new(); // Allocates every iteration
}

// Better - reuse allocation
let mut temp = Vec::new();
for item in items {
    temp.clear(); // Reuse capacity
}
```

## Testing

- **Unit tests**: In same file with `#[cfg(test)]`
- **Integration tests**: In `tests/` directory
- **Use descriptive test names**: `test_should_return_error_when_input_invalid`

## Pre-commit Checks

Always run before committing:
```bash
cargo test    # Run tests
cargo clippy  # Check code quality
cargo fmt     # Format code
cargo check   # Check compilation
```

## Dependencies

- **Pin major versions**: `serde = "1.0"`
- **Minimal dependencies**: Only add what you need
- **Performance-conscious choices**: `thiserror` for libraries, `anyhow` for apps
- **Check with `cargo audit`** regularly

## Documentation

- **Document public APIs** with `///`
- **Include examples** in doc comments
- **Use `cargo doc --open`** to preview

```rust
/// Calculates the sum of all positive numbers in the slice.
/// 
/// # Examples
/// 
/// ```
/// let numbers = [1, -2, 3, 4];
/// assert_eq!(sum_positive(&numbers), 8);
/// ```
pub fn sum_positive(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&&x| x > 0).sum()
}
```
