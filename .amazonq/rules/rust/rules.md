# Rust Idioms and Style Guide

This document outlines idiomatic Rust practices and conventions as defined by the Rust community and official style guide.

## Naming Conventions

### Crate Names
- Use `snake_case` for crate names
- Prefer single words when possible
- Use hyphens in Cargo.toml, which automatically converts to underscores
- Examples: `serde`, `tokio`, `clap`, `my_awesome_crate`

### File and Module Names
- Use `snake_case` for file names: `my_module.rs`
- Module names follow the same convention: `mod my_module;`
- Use `lib.rs` for library crates, `main.rs` for binary crates
- Use `mod.rs` for module directories (though `mod_name.rs` is preferred in newer editions)

### Function Names
- Use `snake_case` for function names
- Be descriptive and clear about what the function does
- Use verbs for functions that perform actions
```rust
fn calculate_total_price() -> f64 { }
fn is_valid_email(email: &str) -> bool { }
fn parse_config_file(path: &Path) -> Result<Config, Error> { }
```

### Variable Names
- Use `snake_case` for variable names
- Prefer descriptive names over abbreviations
- Use single letters only for short-lived iterators or mathematical contexts
```rust
let user_count = 42;
let config_file_path = "/etc/myapp/config.toml";
let is_authenticated = check_credentials(&user);
```

### Type Names
- Use `PascalCase` (UpperCamelCase) for types
- This includes structs, enums, traits, and type aliases
```rust
struct UserAccount { }
enum HttpStatus { }
trait Drawable { }
type UserId = u64;
```

### Constant Names
- Use `SCREAMING_SNAKE_CASE` for constants and static variables
```rust
const MAX_RETRY_ATTEMPTS: u32 = 3;
static GLOBAL_CONFIG: Mutex<Config> = Mutex::new(Config::default());
```

### Generic Type Parameters
- Use single uppercase letters, starting with `T`
- Use descriptive names for complex generics
```rust
fn process<T>(item: T) -> T { }
fn convert<Input, Output>(input: Input) -> Output { }
```

## Project Structure

### Standard Directory Layout
```
my_project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs          # Library root (for libraries)
│   ├── main.rs         # Binary root (for executables)
│   ├── bin/            # Additional binaries
│   │   └── helper.rs
│   └── modules/        # Additional modules
├── tests/              # Integration tests
│   └── integration_test.rs
├── benches/            # Benchmarks
│   └── benchmark.rs
├── examples/           # Example code
│   └── basic_usage.rs
├── docs/               # Additional documentation
├── README.md
├── LICENSE
└── .gitignore
```

### Module Organization
- Keep modules focused and cohesive
- Use `pub` judiciously - prefer private by default
- Group related functionality together
- Use `mod.rs` or module files appropriately

```rust
// lib.rs
pub mod config;
pub mod database;
pub mod api;

mod internal_utils; // Private module
```

## Code Style and Formatting

### Use rustfmt
- Always format code with `cargo fmt`
- Configure rustfmt in `rustfmt.toml` if needed
- Standard formatting is 100 characters per line

### Imports (use statements)
- Group imports: std, external crates, local modules
- Use `self` and `super` appropriately
- Prefer glob imports sparingly
```rust
use std::collections::HashMap;
use std::fs::File;

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::config::Config;
use super::utils::helper_function;
```

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for nullable values
- Prefer `?` operator over explicit match for error propagation
- Create custom error types for libraries
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

fn process_data(input: &str) -> Result<String, CustomError> {
    if input.is_empty() {
        return Err(CustomError {
            message: "Input cannot be empty".to_string(),
        });
    }
    Ok(input.to_uppercase())
}
```

## Memory Management and Ownership

### Borrowing Guidelines
- Prefer borrowing over owned values when possible
- Use `&str` instead of `String` for function parameters when you don't need ownership
- Use `&[T]` instead of `Vec<T>` for function parameters when you don't need ownership
```rust
// Good
fn process_text(text: &str) -> usize {
    text.len()
}

// Less ideal for most cases
fn process_text_owned(text: String) -> usize {
    text.len()
}
```

### Lifetime Guidelines
- Avoid explicit lifetimes when possible (let the compiler infer)
- Use descriptive lifetime names when explicit lifetimes are needed
- Prefer `'static` for global constants
```rust
// Compiler can infer lifetimes
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Explicit lifetimes when needed
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Testing

### Unit Tests
- Place unit tests in the same file as the code being tested
- Use `#[cfg(test)]` module
- Test function names should be descriptive
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_with_tax() {
        let result = calculate_total(100.0, 0.08);
        assert_eq!(result, 108.0);
    }

    #[test]
    fn test_empty_input_returns_error() {
        let result = process_data("");
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero_panics() {
        divide(10, 0);
    }
}
```

### Integration Tests
- Place integration tests in `tests/` directory
- Each file in `tests/` is a separate crate
- Test the public API only
```rust
// tests/integration_test.rs
use my_crate::Config;

#[test]
fn test_config_loading() {
    let config = Config::from_file("test_config.toml").unwrap();
    assert_eq!(config.database_url, "sqlite://test.db");
}
```

### Documentation Tests
- Include examples in documentation comments
- Use `cargo test` to run doc tests
```rust
/// Calculates the area of a rectangle.
///
/// # Examples
///
/// ```
/// use my_crate::calculate_area;
///
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// ```
pub fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

## Documentation

### Documentation Comments
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include examples, panics, errors, and safety sections when relevant
```rust
//! This module provides utilities for file processing.

/// Reads a configuration file and parses it.
///
/// # Arguments
///
/// * `path` - The path to the configuration file
///
/// # Returns
///
/// Returns a `Result` containing the parsed `Config` or an error.
///
/// # Errors
///
/// This function will return an error if:
/// * The file cannot be read
/// * The file contains invalid TOML syntax
///
/// # Examples
///
/// ```
/// use my_crate::read_config;
///
/// let config = read_config("config.toml")?;
/// ```
pub fn read_config(path: &str) -> Result<Config, ConfigError> {
    // Implementation
}
```

## Performance and Idioms

### Iterator Usage
- Prefer iterators over index-based loops
- Use iterator adaptors and combinators
- Collect only when necessary
```rust
// Good
let even_squares: Vec<i32> = (0..10)
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .collect();

// Less idiomatic
let mut even_squares = Vec::new();
for i in 0..10 {
    if i % 2 == 0 {
        even_squares.push(i * i);
    }
}
```

### Pattern Matching
- Use pattern matching extensively
- Prefer `match` over multiple `if let` statements
- Use `if let` for simple cases
```rust
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}

// For simple cases
if let Some(value) = optional_value {
    println!("Got: {}", value);
}
```

### String Handling
- Use `&str` for string slices
- Use `String` for owned strings
- Use `format!` macro for string formatting
- Consider `Cow<str>` for conditional ownership
```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Use Cow for conditional ownership
use std::borrow::Cow;

fn process_text(input: &str, uppercase: bool) -> Cow<str> {
    if uppercase {
        Cow::Owned(input.to_uppercase())
    } else {
        Cow::Borrowed(input)
    }
}
```

## Cargo.toml Best Practices

### Metadata
- Include comprehensive metadata
- Use semantic versioning
- Specify minimum supported Rust version (MSRV)
```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A brief description of what this crate does"
homepage = "https://github.com/username/my_crate"
repository = "https://github.com/username/my_crate"
documentation = "https://docs.rs/my_crate"
readme = "README.md"
keywords = ["cli", "tool", "utility"]
categories = ["command-line-utilities"]
```

### Dependencies
- Specify version requirements carefully
- Use features to make dependencies optional
- Group dependencies logically
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"], optional = true }
clap = { version = "4.0", features = ["derive"] }

[dev-dependencies]
criterion = "0.5"
tempfile = "3.0"

[features]
default = ["async"]
async = ["tokio"]
```

## Clippy and Linting

### Essential Clippy Lints
- Run `cargo clippy` regularly
- Address clippy warnings
- Configure clippy in `Cargo.toml` or `.clippy.toml`
```toml
# In Cargo.toml
[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
cargo = "warn"
```

### Common Clippy Rules to Follow
- Avoid `unwrap()` in production code
- Use `expect()` with descriptive messages
- Prefer `matches!` macro for simple boolean matches
- Use `#[must_use]` for important return values

## Async Programming

### Async Function Guidelines
- Use `async`/`await` syntax
- Prefer `tokio` for async runtime
- Use `Send + Sync` bounds when necessary
```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn read_file_async(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
```

## Security Considerations

### Input Validation
- Validate all external input
- Use type-safe parsing
- Avoid `unsafe` code unless absolutely necessary
```rust
use std::str::FromStr;

fn parse_port(input: &str) -> Result<u16, String> {
    u16::from_str(input)
        .map_err(|_| format!("Invalid port number: {}", input))
        .and_then(|port| {
            if port == 0 {
                Err("Port cannot be zero".to_string())
            } else {
                Ok(port)
            }
        })
}
```

## Additional Best Practices

### Use Type-Driven Development
- Leverage Rust's type system
- Use newtypes for domain-specific values
- Make invalid states unrepresentable
```rust
#[derive(Debug, Clone, Copy)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
struct ProductId(u64);

// This prevents accidentally mixing user IDs and product IDs
fn get_user_orders(user_id: UserId) -> Vec<Order> {
    // Implementation
}
```

### Prefer Composition Over Inheritance
- Use traits for shared behavior
- Use composition and delegation
- Implement traits for external types using newtype pattern

### Use Builder Pattern for Complex Construction
```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: Duration,
    retries: u32,
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<Duration>,
    retries: Option<u32>,
}

impl ConfigBuilder {
    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            retries: self.retries.unwrap_or(3),
        })
    }
}
```

This guide represents the current best practices and idioms accepted by the Rust community. Following these conventions will make your code more readable, maintainable, and idiomatic to other Rust developers.
