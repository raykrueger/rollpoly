# Rust Idioms and Style Guide

Concise guide for idiomatic Rust practices and conventions.

## Naming Conventions

- **Crates**: `snake_case` (prefer single words)
- **Files/Modules**: `snake_case` (`my_module.rs`, `mod my_module;`)
- **Functions/Variables**: `snake_case` (`calculate_total_price`, `user_count`)
- **Types**: `PascalCase` (`UserAccount`, `HttpStatus`, `Drawable`)
- **Constants**: `SCREAMING_SNAKE_CASE` (`MAX_RETRY_ATTEMPTS`)
- **Generics**: Single uppercase letters (`T`, `Input`, `Output`)

## Project Structure

```
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Library root
│   ├── main.rs         # Binary root
│   └── modules/        # Additional modules
├── tests/              # Integration tests
├── examples/           # Example code
└── README.md
```

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
- **Use descriptive lifetime names** when needed

```rust
// Good - compiler infers lifetimes
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// When explicit lifetimes needed
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Ownership & Borrowing

- **Move by default**: `let y = x;` moves ownership
- **Borrow when needed**: `&x` for immutable, `&mut x` for mutable
- **Clone sparingly**: Only when necessary, prefer borrowing
- **Use `Cow<str>`** for flexible string handling

```rust
// Good - borrowing
fn process_data(data: &[i32]) -> i32 {
    data.iter().sum()
}

// When you need ownership
fn take_ownership(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}
```

## Collections & Iterators

- **Prefer iterators** over index-based loops
- **Use `collect()` sparingly** - often unnecessary
- **Chain operations** for readability

```rust
// Good - iterator chains
let result: Vec<_> = data
    .iter()
    .filter(|&&x| x > 0)
    .map(|x| x * 2)
    .collect();

// Better - avoid collect when possible
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

// Boolean check
if matches!(status, Status::Active | Status::Pending) {
    // handle active or pending
}
```

## Option & Result

- **Use `?` operator** for early returns
- **Avoid `unwrap()`** in production code
- **Use `expect()`** with descriptive messages for development

```rust
// Good - using ? operator
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    let trimmed = s.trim();
    let number = trimmed.parse()?;
    Ok(number)
}

// Good - expect with context
let config = std::fs::read_to_string("config.toml")
    .expect("Config file should exist");
```

## Testing

- **Unit tests**: In same file with `#[cfg(test)]`
- **Integration tests**: In `tests/` directory
- **Use descriptive test names**: `test_should_return_error_when_input_invalid`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_calculate_correct_sum() {
        let result = calculate_sum(&[1, 2, 3]);
        assert_eq!(result, 6);
    }
}
```

## Performance Tips

- **Use `&str` over `String`** for parameters
- **Prefer `Vec::with_capacity()`** when size is known
- **Use `Box<str>` for immutable strings**
- **Consider `SmallVec`** for small collections

## Common Patterns

```rust
// Builder pattern
pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }
    
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }
}

// Newtype pattern
#[derive(Debug, Clone)]
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
```

## Cargo & Dependencies

- **Pin major versions**: `serde = "1.0"`
- **Use workspace** for multi-crate projects
- **Minimal dependencies**: Only add what you need
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
