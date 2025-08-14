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
- **Use `anyhow` for applications** - provides flexible error handling with context
- **Use `thiserror` for libraries** - creates custom error types that other crates can handle
- Create custom error types for libraries when you need structured error information

#### Application Error Handling with `anyhow`
```rust
use anyhow::{Context, Result};

fn read_config_file(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    
    let config: Config = toml::from_str(&contents)
        .context("Failed to parse config file as TOML")?;
    
    Ok(config)
}

fn main() -> Result<()> {
    let config = read_config_file("config.toml")?;
    println!("Loaded config: {:?}", config);
    Ok(())
}
```

#### Library Error Handling with `thiserror`
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),
    
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}

pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    
    if config.port == 0 {
        return Err(ConfigError::Invalid {
            message: "Port cannot be zero".to_string(),
        });
    }
    
    Ok(config)
}
```

#### Legacy Custom Error Implementation (when not using thiserror)
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

### Unit Test Structure and Organization

#### Test Module Placement
- Place unit tests in the same file as the code being tested
- Use `#[cfg(test)]` module to ensure tests are only compiled during testing
- Import the parent module with `use super::*;`
- Group related tests in sub-modules when appropriate

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Group related tests in sub-modules
    mod validation_tests {
        use super::*;
        
        #[test]
        fn test_valid_input() {
            // Test implementation
        }
    }

    mod calculation_tests {
        use super::*;
        
        #[test]
        fn test_basic_calculation() {
            // Test implementation
        }
    }
}
```

#### Test Function Naming
- Use descriptive names that clearly indicate what is being tested
- Follow the pattern: `test_[function_name]_[scenario]_[expected_outcome]`
- Use `snake_case` for test function names
- Be specific about the test scenario

```rust
#[test]
fn test_calculate_total_with_positive_tax_returns_correct_sum() {
    let result = calculate_total(100.0, 0.08);
    assert_eq!(result, 108.0);
}

#[test]
fn test_parse_config_with_empty_string_returns_error() {
    let result = parse_config("");
    assert!(result.is_err());
}

#[test]
fn test_divide_by_zero_panics_with_expected_message() {
    // Test implementation
}
```

#### Test Structure (Arrange-Act-Assert)
- Follow the AAA pattern: Arrange, Act, Assert
- Use clear variable names and comments when the setup is complex
- Separate the three phases with blank lines for readability

```rust
#[test]
fn test_user_authentication_with_valid_credentials_succeeds() {
    // Arrange
    let username = "test_user";
    let password = "secure_password";
    let mut auth_service = AuthService::new();
    
    // Act
    let result = auth_service.authenticate(username, password);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap().username, username);
}
```

#### Assertion Best Practices
- Use the most specific assertion possible
- Provide custom error messages for complex assertions
- Use `assert_eq!` and `assert_ne!` for equality comparisons
- Use `assert!` for boolean conditions
- Use `matches!` macro for pattern matching assertions

```rust
#[test]
fn test_dice_roll_returns_valid_range() {
    let result = roll_die(6);
    
    // Specific assertion with custom message
    assert!(
        result >= 1 && result <= 6,
        "Die roll {} should be between 1 and 6 inclusive",
        result
    );
}

#[test]
fn test_parse_result_matches_expected_pattern() {
    let result = parse_expression("2d6 + 3");
    
    // Pattern matching assertion
    assert!(matches!(result, Ok(Expression::DiceRoll { .. })));
}
```

#### Testing Error Conditions
- Test both success and failure cases
- Use `assert!(result.is_err())` for error conditions
- Test specific error types and messages when relevant
- Use `#[should_panic]` sparingly and with expected messages

```rust
#[test]
fn test_invalid_input_returns_specific_error() {
    let result = parse_dice_notation("invalid");
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::InvalidSyntax);
    assert!(error.to_string().contains("invalid dice notation"));
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_divide_by_zero_panics_with_expected_message() {
    divide(10, 0);
}
```

#### Testing with Mock Data and Fixtures
- Create helper functions for common test data
- Use constants for test values that are reused
- Consider using the `lazy_static` crate for complex fixtures

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DICE_NOTATION: &str = "3d6 + 2";
    const INVALID_DICE_NOTATION: &str = "invalid";

    fn create_test_user() -> User {
        User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        }
    }

    #[test]
    fn test_user_creation_with_valid_data() {
        let user = create_test_user();
        assert_eq!(user.name, "Test User");
    }
}
```

#### Parameterized Tests
- Use loops or macros for testing multiple similar cases
- Consider using the `rstest` crate for more advanced parameterized testing

```rust
#[test]
fn test_various_die_sizes() {
    let die_sizes = vec![4, 6, 8, 10, 12, 20, 100];
    
    for &size in &die_sizes {
        let result = roll_die(size);
        assert!(
            result >= 1 && result <= size,
            "Die d{} rolled {}, expected 1-{}",
            size, result, size
        );
    }
}

// Using rstest crate (add to dev-dependencies)
#[cfg(test)]
mod rstest_examples {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(4, 1, 4)]
    #[case(6, 1, 6)]
    #[case(20, 1, 20)]
    fn test_die_roll_ranges(#[case] die_size: i32, #[case] min: i32, #[case] max: i32) {
        let result = roll_die(die_size);
        assert!(result >= min && result <= max);
    }
}
```

#### Testing Randomness and Non-Deterministic Behavior
- Test properties rather than exact values
- Use statistical approaches for random behavior
- Test ranges and distributions when appropriate

```rust
#[test]
fn test_random_generation_produces_varied_results() {
    let mut results = std::collections::HashSet::new();
    
    // Generate multiple random values
    for _ in 0..100 {
        results.insert(generate_random_number(1, 10));
    }
    
    // Should have generated multiple different values
    assert!(
        results.len() > 5,
        "Expected varied random results, got only {} unique values",
        results.len()
    );
}

#[test]
fn test_dice_roll_stays_within_bounds() {
    for _ in 0..1000 {
        let result = roll_die(6);
        assert!(result >= 1 && result <= 6);
    }
}
```

#### Performance and Benchmark Tests
- Use `#[ignore]` for slow tests that shouldn't run by default
- Consider using the `criterion` crate for benchmarks
- Test performance-critical code paths

```rust
#[test]
#[ignore] // Run with: cargo test -- --ignored
fn test_large_dataset_performance() {
    let large_input = vec![1; 1_000_000];
    let start = std::time::Instant::now();
    
    let result = process_large_dataset(&large_input);
    
    let duration = start.elapsed();
    assert!(duration < std::time::Duration::from_secs(1));
    assert_eq!(result.len(), 1_000_000);
}
```

### Integration Tests
- Place integration tests in `tests/` directory
- Each file in `tests/` is a separate crate
- Test the public API only
- Use integration tests to test feature interactions

```rust
// tests/integration_test.rs
use my_crate::{Config, Database, process_request};

#[test]
fn test_end_to_end_request_processing() {
    // Arrange
    let config = Config::from_file("test_config.toml").unwrap();
    let db = Database::connect(&config.database_url).unwrap();
    
    // Act
    let result = process_request(&db, "test_request");
    
    // Assert
    assert!(result.is_ok());
}

#[test]
fn test_config_loading_from_various_sources() {
    // Test loading from file
    let config1 = Config::from_file("config.toml");
    assert!(config1.is_ok());
    
    // Test loading from environment
    std::env::set_var("DATABASE_URL", "sqlite://test.db");
    let config2 = Config::from_env();
    assert!(config2.is_ok());
}
```

### Documentation Tests
- Include examples in documentation comments
- Use `cargo test` to run doc tests
- Ensure examples are complete and runnable
- Use `no_run` attribute for examples that shouldn't execute

```rust
/// Calculates the area of a rectangle.
///
/// # Arguments
///
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
///
/// # Returns
///
/// The area as a floating-point number
///
/// # Examples
///
/// ```
/// use my_crate::calculate_area;
///
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// ```
///
/// # Panics
///
/// This function will panic if either width or height is negative:
///
/// ```should_panic
/// use my_crate::calculate_area;
///
/// calculate_area(-1.0, 5.0); // This will panic
/// ```
pub fn calculate_area(width: f64, height: f64) -> f64 {
    assert!(width >= 0.0 && height >= 0.0, "Dimensions must be non-negative");
    width * height
}
```

### Test-Driven Development (TDD)
- Write tests before implementation (Red-Green-Refactor cycle)
- Start with the simplest failing test
- Write minimal code to make tests pass
- Refactor while keeping tests green

```rust
// Step 1: Write failing test (RED)
#[test]
fn test_dice_parser_handles_simple_notation() {
    let result = parse_dice("1d6");
    assert!(result.is_ok());
    
    let dice = result.unwrap();
    assert_eq!(dice.count, 1);
    assert_eq!(dice.sides, 6);
}

// Step 2: Write minimal implementation (GREEN)
pub fn parse_dice(notation: &str) -> Result<Dice, ParseError> {
    // Minimal implementation to pass the test
    if notation == "1d6" {
        Ok(Dice { count: 1, sides: 6 })
    } else {
        Err(ParseError::InvalidNotation)
    }
}

// Step 3: Refactor and add more tests
```

### Test Organization and Maintenance
- Keep tests simple and focused on one thing
- Avoid testing implementation details
- Update tests when requirements change
- Remove or update obsolete tests
- Use descriptive test names that serve as documentation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Group tests by functionality
    mod dice_parsing {
        use super::*;
        
        #[test]
        fn parses_simple_dice_notation() { /* ... */ }
        
        #[test]
        fn handles_multiple_dice() { /* ... */ }
        
        #[test]
        fn rejects_invalid_notation() { /* ... */ }
    }

    mod dice_rolling {
        use super::*;
        
        #[test]
        fn rolls_within_expected_range() { /* ... */ }
        
        #[test]
        fn produces_different_results_over_time() { /* ... */ }
    }
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
- Use recommended crates for common functionality
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"], optional = true }
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"

[dev-dependencies]
criterion = "0.5"
tempfile = "3.0"

[features]
default = ["async"]
async = ["tokio"]
```

### Recommended Crates
Use these well-established crates for common functionality:

#### CLI Applications
- **`clap`**: Command-line argument parsing with derive macros
- **`anyhow`**: Flexible error handling for applications
- **`env_logger`** or **`tracing`**: Logging functionality
- **`dirs`**: Platform-specific directories (config, cache, etc.)

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
env_logger = "0.10"
dirs = "5.0"
```

#### Libraries
- **`thiserror`**: Custom error types for libraries (instead of anyhow)
- **`serde`**: Serialization and deserialization
- **`tokio`**: Async runtime
- **`tracing`**: Structured logging

```toml
[dependencies]
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
```

#### Testing and Development
- **`criterion`**: Benchmarking
- **`proptest`**: Property-based testing
- **`tempfile`**: Temporary files and directories for tests
- **`assert_cmd`**: Testing CLI applications

```toml
[dev-dependencies]
criterion = "0.5"
proptest = "1.0"
tempfile = "3.0"
assert_cmd = "2.0"
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

## CLI Development

### Command-Line Argument Parsing with `clap`
- Use `clap` with derive macros for type-safe CLI parsing
- Leverage `clap`'s built-in help generation and validation
- Use subcommands for complex CLI applications
- Implement proper error handling with `anyhow`

#### Basic CLI Application
```rust
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "my-tool")]
#[command(about = "A simple CLI tool", long_about = None)]
struct Cli {
    /// Input file to process
    #[arg(short, long)]
    input: String,
    
    /// Output file (optional)
    #[arg(short, long)]
    output: Option<String>,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Number of threads to use
    #[arg(short = 'j', long, default_value = "1")]
    threads: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Processing file: {}", cli.input);
    }
    
    // Process the file
    process_file(&cli.input, cli.output.as_deref(), cli.threads)?;
    
    Ok(())
}

fn process_file(input: &str, output: Option<&str>, threads: usize) -> Result<()> {
    // Implementation with proper error handling
    Ok(())
}
```

#### CLI with Subcommands
```rust
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "my-tool")]
#[command(about = "A CLI tool with subcommands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new item
    Add {
        /// Name of the item to add
        name: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Remove an item
    Remove {
        /// Name of the item to remove
        name: String,
    },
    /// List all items
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { name, description } => {
            add_item(&name, description.as_deref())?;
        }
        Commands::Remove { name } => {
            remove_item(&name)?;
        }
        Commands::List { detailed } => {
            list_items(detailed)?;
        }
    }
    
    Ok(())
}

fn add_item(name: &str, description: Option<&str>) -> Result<()> {
    // Implementation
    Ok(())
}

fn remove_item(name: &str) -> Result<()> {
    // Implementation
    Ok(())
}

fn list_items(detailed: bool) -> Result<()> {
    // Implementation
    Ok(())
}
```

#### CLI with Configuration and Environment Variables
```rust
use clap::Parser;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "my-tool")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Override the server URL
    #[arg(long, env = "SERVER_URL")]
    server_url: Option<String>,
    
    /// API key for authentication
    #[arg(long, env = "API_KEY")]
    api_key: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Config {
    server_url: String,
    api_key: String,
    timeout: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load configuration from file
    let mut config: Config = load_config(&cli.config)
        .with_context(|| format!("Failed to load config from {}", cli.config))?;
    
    // Override with CLI arguments if provided
    if let Some(url) = cli.server_url {
        config.server_url = url;
    }
    if let Some(key) = cli.api_key {
        config.api_key = key;
    }
    
    // Use the configuration
    run_application(config)?;
    
    Ok(())
}

fn load_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read config file {}", path))?;
    
    let config: Config = toml::from_str(&contents)
        .context("Could not parse config file as TOML")?;
    
    Ok(config)
}

fn run_application(config: Config) -> Result<()> {
    // Main application logic
    Ok(())
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
