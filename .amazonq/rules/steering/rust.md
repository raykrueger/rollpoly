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

**Breaking vs Non-Breaking Changes:**
- **Non-breaking**: Adding enum variants, struct fields, trait methods with defaults
- **Breaking**: Changing function signatures, removing public items, changing return types
- **Use semantic versioning**: Major.Minor.Patch (breaking.feature.bugfix)

**API Design:**
- **Minimize public surface**: Only expose what users need
- **Use `#[non_exhaustive]`** on enums/structs that may grow
- **Prefer `&str` over `String`** for parameters
- **Return `Result<T, E>`** for fallible operations
- **Use builder patterns** for complex configuration

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

## Pre-commit Checks

Always run these checks before committing:

```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Check for compilation errors
cargo check
```

## Performance Best Practices

### Memory Management

- **Pre-allocate collections**: Use `Vec::with_capacity(n)` when size is known
- **Avoid unnecessary allocations**: Prefer `&str` over `String` for parameters
- **Reuse allocations**: Clear and reuse `Vec`/`HashMap` instead of creating new ones
- **Use `Box<str>` for immutable strings** that don't need to grow
- **Consider `SmallVec`** for collections that are usually small (< 24 bytes)

```rust
// Good - pre-allocate when size is known
let mut results = Vec::with_capacity(dice_count);
for _ in 0..dice_count {
    results.push(roll_die());
}

// Avoid - causes multiple reallocations
let mut results = Vec::new();
for _ in 0..dice_count {
    results.push(roll_die()); // May reallocate multiple times
}
```

### String Operations

- **Avoid `format!()` in hot paths**: Use static strings or pre-computed values
- **Use `&'static str` for constants**: Avoid runtime string creation
- **Prefer single-pass parsing**: Avoid multiple `contains()` or `find()` calls
- **Use `split_once()` over `split().collect()`** for simple cases

```rust
// Good - avoid format! in loops
const OPERATORS: &[&str] = &[" + ", " - ", " * ", " / "];
for op in OPERATORS {
    if input.contains(op) {
        return parse_operation(input, op);
    }
}

// Avoid - creates temporary strings
for op in &["+", "-", "*", "/"] {
    if input.contains(&format!(" {op} ")) { // Allocates each time
        return parse_operation(input, op);
    }
}
```

### Function Design

- **Pass expensive resources by reference**: RNG, large structs, etc.
- **Use `impl Trait` for zero-cost abstractions**: Especially for iterators
- **Prefer iterators over index-based loops**: Often faster and more idiomatic
- **Avoid cloning in hot paths**: Use borrowing when possible

```rust
// Good - pass RNG by reference to avoid initialization overhead
fn roll_dice(count: usize, sides: i32, rng: &mut impl Rng) -> Vec<i32> {
    (0..count).map(|_| rng.gen_range(1..=sides)).collect()
}

// Avoid - creates new RNG each call
fn roll_dice(count: usize, sides: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng(); // Initialization overhead
    (0..count).map(|_| rng.gen_range(1..=sides)).collect()
}
```

### Parsing and Text Processing

- **Use byte string matching** for ASCII-only operations
- **Implement single-pass parsers** when possible
- **Cache compiled regexes** using `lazy_static` or `once_cell`
- **Prefer `chars().nth()` over string slicing** for Unicode safety

```rust
// Good - single pass parsing
fn find_operator(input: &str) -> Option<(&str, usize)> {
    let bytes = input.as_bytes();
    for (i, window) in bytes.windows(3).enumerate() {
        match window {
            b" + " => return Some(("+", i)),
            b" - " => return Some(("-", i)),
            b" * " => return Some(("*", i)),
            _ => continue,
        }
    }
    None
}

// Avoid - multiple string scans
fn find_operator(input: &str) -> Option<&str> {
    if input.contains(" + ") { Some("+") }
    else if input.contains(" - ") { Some("-") }
    else if input.contains(" * ") { Some("*") }
    else { None }
}
```

### Error Handling Performance

- **Use `?` operator**: More efficient than manual match statements
- **Avoid string formatting in error paths**: Use static messages when possible
- **Consider `anyhow` for applications**: Lower overhead than custom error types
- **Use `thiserror` for libraries**: Zero-cost error conversion

```rust
// Good - static error messages
#[derive(Error, Debug)]
pub enum DiceError {
    #[error("Invalid die size: must be positive")]
    InvalidDieSize,
    #[error("Too many dice: maximum {max}, got {count}")]
    TooManyDice { count: usize, max: usize },
}

// Avoid - dynamic formatting in hot paths
fn validate_die_size(size: i32) -> Result<(), String> {
    if size <= 0 {
        Err(format!("Invalid die size: {}", size)) // Allocates
    } else {
        Ok(())
    }
}
```

### Benchmarking Guidelines

- **Use `criterion` for micro-benchmarks**: More accurate than `std::time`
- **Profile with `perf` or `valgrind`**: Identify actual bottlenecks
- **Test with realistic data**: Don't optimize for artificial cases
- **Measure allocations**: Use `dhat` or similar tools

```rust
// Example criterion benchmark
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_dice_rolling(c: &mut Criterion) {
    c.bench_function("roll 4d6K3", |b| {
        b.iter(|| roll(black_box("4d6K3")))
    });
}

criterion_group!(benches, bench_dice_rolling);
criterion_main!(benches);
```

### When NOT to Optimize

- **Don't optimize prematurely**: Profile first, optimize bottlenecks
- **Readability over micro-optimizations**: Unless in proven hot paths
- **Avoid unsafe unless necessary**: Safety is more important than speed
- **Don't sacrifice API ergonomics**: For minor performance gains

### Hot Path Optimization Patterns

**Identifying Hot Paths:**
- Functions called in loops or recursively
- String parsing and validation logic
- Collection operations with unknown sizes
- Resource initialization (RNG, connections, etc.)

**Common Hot Path Anti-patterns:**
```rust
// Anti-pattern: Repeated allocations in loops
for item in items {
    let mut temp = Vec::new(); // Allocates every iteration
    process_item(item, &mut temp);
}

// Better: Reuse allocation
let mut temp = Vec::new();
for item in items {
    temp.clear(); // Reuse existing capacity
    process_item(item, &mut temp);
}

// Anti-pattern: String formatting in validation
fn validate_input(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Err(format!("Input cannot be empty: '{}'", input)) // Allocates
    } else {
        Ok(())
    }
}

// Better: Static error messages
#[derive(Error, Debug)]
enum ValidationError {
    #[error("Input cannot be empty")]
    EmptyInput,
}

// Anti-pattern: Multiple string scans
if input.contains("pattern1") || input.contains("pattern2") {
    // Scans string multiple times
}

// Better: Single pass with state machine or regex
```

**Performance Measurement:**
```rust
// Add to Cargo.toml for benchmarking
[dev-dependencies]
criterion = "0.5"

// Simple timing for development
let start = std::time::Instant::now();
expensive_operation();
println!("Operation took: {:?}", start.elapsed());
```

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

### Performance-Oriented Dependency Choices

- **Prefer `thiserror` over `anyhow`** for libraries (zero-cost error conversion)
- **Use `anyhow` for applications** (lower overhead than custom error types)
- **Consider `smallvec`** for collections usually < 24 bytes
- **Use `once_cell` or `std::sync::LazyLock`** for expensive static initialization
- **Profile before adding dependencies**: Each crate adds compile time and binary size

```toml
# Performance-conscious dependency selection
[dependencies]
# Zero-cost error handling for libraries
thiserror = "2.0"
# Flexible error handling for applications  
anyhow = "1.0"
# Small vector optimization
smallvec = "1.0"
# Lazy static initialization
once_cell = "1.0"

[dev-dependencies]
# Accurate benchmarking
criterion = "0.5"
```

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
