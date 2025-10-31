# Rollpoly - Advanced Dice Rolling

A comprehensive Rust 2024 application for rolling polyhedral dice in the terminal. Also supports being imported as a library with advanced dice mechanics for tabletop gaming.

## CLI Usage

Simple usage of command line arguments to roll dice.

```bash
# Roll dice directly
rollpoly 2d6
rollpoly '3d6 + 5'
rollpoly 4d10K3

# Roll multiple times
rollpoly '2d20' -n 5
rollpoly roll '4d6K3' -n 6

# Statistical analysis
rollpoly stats 3d6 -n 1000
rollpoly stats 2d6 -n 10000 -v

# Game-specific commands
rollpoly dh                 # Daggerheart Duality dice (Hope/Fear)

# Show examples and help
rollpoly examples
rollpoly --help
```

Interactive shell usage for continuous interaction.

```bash
# Start interactive shell
rollpoly shell
# Roll dice in the shell
rollpoly> 2d6
🎲 You rolled: 8! [4, 4]
rollpoly> 4d6K3
🎲 You rolled: 15! [6, 5, 4]
rollpoly> exit
Thanks for rolling! Goodbye!
```

## Library API

Rollpoly can be used as a Rust library in your own projects for dice rolling functionality.

### Installation

Add rollpoly to your `Cargo.toml`:

```toml
[dependencies]
rollpoly = "0.6"
```

### Basic Usage

The primary function is `roll()` which takes a dice notation string and returns a vector of results:

```rust
use rollpoly::roll;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic dice rolling
    let results = roll("2d6")?;
    println!("Rolled 2d6: {:?}", results);
    // Example output: [3, 5]
    
    // With arithmetic
    let results = roll("4d6 + 10")?;
    println!("Rolled 4d6+10: {:?}", results);
    // Example output: [2, 4, 6, 1, 10] (dice results + modifier)
    
    // Advanced mechanics
    let results = roll("4d6K3")?;  // Keep highest 3 of 4d6
    println!("Rolled 4d6K3: {:?}", results);
    // Example output: [6, 5, 4] (lowest die dropped)
    
    Ok(())
}
```

### Error Handling

The library uses comprehensive error types for robust error handling:

```rust
use rollpoly::{roll, DiceError};

fn safe_roll(notation: &str) -> Result<Vec<i32>, DiceError> {
    match roll(notation) {
        Ok(results) => {
            println!("Successfully rolled {}: {:?}", notation, results);
            Ok(results)
        }
        Err(DiceError::EmptyInput) => {
            eprintln!("Error: No dice notation provided");
            Err(DiceError::EmptyInput)
        }
        Err(DiceError::InvalidNotation { input, reason }) => {
            eprintln!("Error: Invalid notation '{}' - {}", input, reason);
            Err(DiceError::InvalidNotation { input, reason })
        }
        Err(DiceError::InvalidDieSize { size }) => {
            eprintln!("Error: Invalid die size '{}'", size);
            Err(DiceError::InvalidDieSize { size })
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
```

### Integration Examples

#### Game Character Creation

```rust
use rollpoly::roll;

struct Character {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl Character {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Character {
            strength: roll("4d6K3")?.iter().sum(),
            dexterity: roll("4d6K3")?.iter().sum(),
            constitution: roll("4d6K3")?.iter().sum(),
            intelligence: roll("4d6K3")?.iter().sum(),
            wisdom: roll("4d6K3")?.iter().sum(),
            charisma: roll("4d6K3")?.iter().sum(),
        })
    }
}
```

#### Damage Calculator

```rust
use rollpoly::roll;

fn calculate_damage(weapon: &str, critical: bool) -> Result<i32, Box<dyn std::error::Error>> {
    let base_damage = if critical {
        // Double dice on critical hit
        match weapon {
            "longsword" => roll("2d8 + 3")?,  // Doubled 1d8
            "greataxe" => roll("2d12 + 5")?,  // Doubled 1d12
            "dagger" => roll("2d4 + 2")?,     // Doubled 1d4
            _ => roll("2d6")?,
        }
    } else {
        match weapon {
            "longsword" => roll("1d8 + 3")?,
            "greataxe" => roll("1d12 + 5")?,
            "dagger" => roll("1d4 + 2")?,
            _ => roll("1d6")?,
        }
    };
    
    Ok(base_damage.iter().sum())
}
```

#### Statistical Analysis

```rust
use rollpoly::roll;
use std::collections::HashMap;

fn analyze_dice_distribution(notation: &str, iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut results = HashMap::new();
    
    for _ in 0..iterations {
        let roll_result: i32 = roll(notation)?.iter().sum();
        *results.entry(roll_result).or_insert(0) += 1;
    }
    
    println!("Distribution for {} over {} rolls:", notation, iterations);
    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by_key(|&(value, _)| value);
    
    for (value, count) in sorted_results {
        let percentage = (*count as f64 / iterations as f64) * 100.0;
        println!("  {}: {} ({:.1}%)", value, count, percentage);
    }
    
    Ok(())
}

// Usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    analyze_dice_distribution("3d6", 10000)?;
    Ok(())
}
```

#### Custom Game Mechanics

```rust
use rollpoly::roll;

// Daggerheart Duality dice mechanics
fn daggerheart_duality_roll() -> Result<(i32, String), Box<dyn std::error::Error>> {
    let results = roll("2d12")?;
    let hope_die = results[0];    // First die represents Hope
    let fear_die = results[1];    // Second die represents Fear
    let total = hope_die + fear_die;
    
    let result_type = if hope_die == fear_die {
        format!("Rolled {} CRITICAL!", total)
    } else if hope_die > fear_die {
        format!("Rolled {} with Hope", total)
    } else {
        format!("Rolled {} with Fear", total)
    };
    
    Ok((total, result_type))
}

// Shadowrun-style success counting
fn shadowrun_test(dice_pool: usize, threshold: i32) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let notation = format!("{}d6", dice_pool);
    let results = roll(&notation)?;
    
    let successes = results.iter().filter(|&&x| x >= threshold).count() as i32;
    let glitches = results.iter().filter(|&&x| x == 1).count() as i32;
    
    Ok((successes, glitches))
}
```

### API Reference

#### Functions

- **`roll(dice_notation: &str) -> Result<Vec<i32>, DiceError>`**
  - Primary function for rolling dice
  - Takes a dice notation string (e.g., "2d6", "4d10K3", "3d6 + 5")
  - Returns a vector containing individual dice results and any modifiers
  - For "4d6 + 5", returns `[die1, die2, die3, die4, modifier]`

#### Error Types

- **`DiceError::EmptyInput`** - Empty or whitespace-only input
- **`DiceError::InvalidNotation { input, reason }`** - Malformed dice notation
- **`DiceError::InvalidDieSize { size }`** - Invalid number of sides (must be positive)
- **`DiceError::InvalidDiceCount { count }`** - Invalid number of dice (must be positive)
- **`DiceError::InvalidModifier { modifier }`** - Invalid arithmetic modifier
- **`DiceError::UnsupportedOperator { operator, input }`** - Unsupported mathematical operator

#### Return Values

The `roll()` function returns a `Vec<i32>` containing:
- Individual dice roll results
- Any arithmetic modifiers as separate elements
- For advanced mechanics (Keep/Drop), only the kept dice are returned

Examples:
- `roll("2d6")` → `[3, 5]` (two dice results)
- `roll("2d6 + 3")` → `[3, 5, 3]` (two dice + modifier)
- `roll("4d6K3")` → `[6, 5, 4]` (highest 3 of 4 dice)
- `roll("3d6>4")` → `[2]` (count of successes above 4)

### Thread Safety

The library is thread-safe and can be used in concurrent applications:

```rust
use rollpoly::roll;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let results = roll("3d6").unwrap();
                println!("Thread {}: {:?}", i, results);
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    Ok(())
}
```

## Syntax

### Basic Syntax

```
4d10 + 17: Roll a 10-sided die 4 times and add 17 to the result
2d20 - 3: Roll a 20 sided die 2 times and subtract 3
1d4 * 3: Roll a 4 sided die once and multiply by 3
d4 * 3: Same as above, leaving out the number of dice will default to 1
5d6 / 3: Roll 5d6 and divide by 3
5d6 // 3: Same as above but floor division
```

### Advanced Syntax

#### Keep Highest (K):

Used to (K)eep the highest roll. Can be followed by a number to keep that number
of dice or by nothing to indicate keeping only one.

```
4d10K: Roll 4d10 and keep only the highest roll
7d12K3: Roll 7d12 and keep the highest three rolls
7d12K3 + 4: Roll as above then add 4
```

#### Keep Lowest (k):

Same as above but keeping the lowest.

```
3d3k: Roll 3d3 and keep the lowest roll
100d6k99: Roll 100d6 and keep all but the highest.
2d20k: Roll 2d20 and keep the lowest. This is a disadvantage roll in 5e
```

#### Drop Highest (X):

Used to drop the highest roll. Can be followed by a number to drop that number
of dice or by nothing to indicate dropping just one.

```
6d8X: Roll 6d8 and drop the highest
5d10X3 Roll 5d10 and drop the highest 3
```

#### Drop Lowest (x):

```
6d8x: Roll 6d8 and drop the lowest
5d10x3 Roll 5d10 and drop the lowest 3
```

#### Count Successes (> or <):

Counts the number of rolls above or below a certain value.

```
4d20>19: Rolls 4d20 and counts the number of rolls above 19
10d12<3: Rolls 10d12 and counts the number of rolls below 3
```

#### Count failures (f):

Addition to counting successes to specify an additional 'failure' condition.
Each failure will decrease the score by 1 while each success will still increase
by 1.

```
10d10>6f<3: Roll 10d10 and count successes over 6 and failures under 3
4d20<5f>19: Roll 4d20 and count successes under 5 and failures over 19
5d100<5f>3: Invalid, you cannot have your failure and success comparison both be more than or less than.
```

#### Exploding Dice (!):

Exploding dice is usually known as 'Rule of 6' or 'Rule of 10,' as it is in
Shadowrun. As long as the roll passes the specified comparison, another dice is
rolled and added to the total. This process repeats until a number that does not
match the comparison is rolled.

```
2d20!: Roll 2d20 and explode every time a 20 is rolled
7d20!3: Roll 7d20 and explode every time a 3 is rolled
4d6! Roll 4d6 and explode every time a 6 is rolled
d20!>10: Roll a d20 and explode every time a number higher than 10 is rolled
3d12!<2: Roll 3d12 and explode every time a 1 is rolled.
```

#### Rerolling Dice (r/R):

Rerolling allows you to roll certain dice again based on specific conditions,
replacing the original result with the new roll. Use lowercase 'r' for rerolling
once, or uppercase 'R' to keep rerolling until the condition is no longer met.

```
4d6r1: Roll 4d6 and reroll any 1s once (D&D Great Weapon Fighting)
2d6r<3: Roll 2d6 and reroll anything under 3 once
3d8R1: Roll 3d8 and keep rerolling 1s until no 1s remain
4d10R<3: Roll 4d10 and keep rerolling anything under 3
1d20r1r2: Roll 1d20 and reroll 1s and 2s once
2d6r>4: Roll 2d6 and reroll anything over 4 once
```

## Developer Guide

### Prerequisites

- **Rust 2024 Edition** (latest stable)
- **Git** with hooks enabled
- **Cargo** (comes with Rust)

### Development Workflow

#### Initial Setup

```bash
# Clone the repository
git clone https://github.com/raykrueger/rollpoly.git
cd rollpoly
```

#### Install pre-commit hooks

```bash
# Enable pre-commit hooks for automatic quality checks
./scripts/install-pre-commit-hook.sh
```

#### Running Tests

```bash
# Run all tests (unit + integration + doc tests)
cargo test

# Run specific test
cargo test test_roll_simple_dice
```

#### Code Quality Checks

```bash
# Run all quality checks manually
./scripts/full-checks.sh

# Individual checks
cargo test          # All tests
cargo clippy        # Linting (zero warnings enforced)
cargo fmt           # Code formatting
cargo check         # Compilation check
```

#### Making Changes

1. **Write code** following Rust idioms (see steering docs)
2. **Add tests** for new functionality
3. **Run checks**: `./scripts/full-checks.sh`
4. **Commit**: Pre-commit hook runs automatically

#### Dependency Management

```bash
# Check for outdated dependencies and security issues
./scripts/check-dependencies.sh

# Update dependencies (after reviewing)
cargo update

# Verify everything still works
cargo test && ./scripts/full-checks.sh
```

**Automated monitoring:**
- **Dependabot integration** for automated dependency updates and security patches
- **Native auto-merge** using Dependabot's built-in functionality when all tests pass
- **Daily dependency checks** with grouped pull requests for patch/minor updates
- **Weekly GitHub Actions updates** to keep workflows current

### Architecture

#### Core Components

- **`roll()`**: Main public API function
- **Parser functions**: Convert dice notation to operations
- **Helper functions**: Execute specific dice mechanics
- **Error handling**: Comprehensive error types

#### Key Design Decisions

- **`usize` for dice counts**: Semantically correct, eliminates casting
- **Helper function extraction**: Reduces complexity, improves maintainability
- **Comprehensive validation**: Reject invalid input at parse time
- **Zero-copy where possible**: Efficient string handling

### Quality Standards

#### Code Quality

- **Zero clippy warnings** (enforced by pre-commit)
- **100% test coverage** for public APIs
- **Idiomatic Rust** following steering guidelines
- **Comprehensive documentation** with examples

#### Dependency Management

- **Dependabot integration** for automated dependency updates and security patches
- **Native auto-merge** using Dependabot's built-in functionality when all tests pass
- **Daily dependency checks** with grouped pull requests for patch/minor updates
- **Weekly GitHub Actions updates** to keep workflows current
- **Manual checking** available via `./scripts/check-dependencies.sh`

#### Testing Strategy

- **Unit tests**: Test individual functions in isolation
- **Integration tests**: Test public API behavior
- **Doc tests**: Ensure examples in documentation work
- **Property-based testing**: Validate dice roll ranges

#### Git Workflow

- **Conventional commits**: `type(scope): description`
- **Pre-commit validation**: Automatic quality checks
- **Clean history**: Meaningful commit messages
- **Steering compliance**: AI tooling follows established rules

### Common Tasks

#### Adding New Dice Syntax

1. **Add parser function** in `lib.rs`
2. **Add helper function** for dice mechanics
3. **Update main parser** to recognize new syntax
4. **Add comprehensive tests**
5. **Update README** with syntax examples

### Release Process

#### Automated Releases

The project uses GitHub Actions for automated releases with multi-platform builds:

**Platforms supported:**
- **Linux**: AMD64, ARM64 (`.tar.gz`)
- **macOS**: Intel (AMD64), Apple Silicon (ARM64) (`.tar.gz`)
- **Windows**: AMD64, ARM64 (`.zip`)

**Two ways to create releases:**

1. **Manual workflow trigger** (recommended):
   ```bash
   # Use the GitHub web interface:
   # Actions → Release → Run workflow → Select version bump type
   ```

2. **Local script**:
   ```bash
   # Bump version and create tag locally
   ./scripts/bump-version.sh patch   # or minor, major
   ```

#### Version Bumping

- **patch**: Bug fixes (0.1.0 → 0.1.1)
- **minor**: New features (0.1.0 → 0.2.0)  
- **major**: Breaking changes (0.1.0 → 1.0.0)

The workflow automatically:
- Runs full test suite on multiple platforms
- Builds optimized binaries for all targets
- Creates GitHub release with changelog
- Uploads release artifacts

### Contributing

1. **Follow steering guidelines** in `.amazonq/rules/steering/`
2. **Write tests** for all new functionality
3. **Ensure zero clippy warnings**
4. **Use conventional commit messages**
5. **Update documentation** as needed

The pre-commit hook enforces all quality standards automatically.

## AI Tooling

Almost all of this code was written by the Amazon Q CLI.

### Steering

All AI tooling must follow the rules established in steering documents found in .amazonq/rules/steering/\*.md

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
