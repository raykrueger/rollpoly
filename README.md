# Rollpoly - Advanced Dice Rolling

A comprehensive Rust 2024 application for rolling polyhedral dice in the terminal. Also supports being imported as a library with advanced dice mechanics for tabletop gaming.

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
