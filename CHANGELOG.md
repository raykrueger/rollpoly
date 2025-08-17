# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2025-08-17

### Fixed
- Release workflow now only triggers on version tag pushes, not all commits to main

## [0.2.1] - 2025-08-15

### Added
- Comprehensive dice-to-dice operations (e.g., `2d6 + 3d4`)
- Daggerheart Hope/Fear mechanics support
- Advanced dice mechanics: Keep Highest/Lowest (K/k), Drop Highest/Lowest (X/x)
- Success counting with failure conditions (>, <, f)
- Exploding dice mechanics (!)
- Rerolling dice (r/R) with single and continuous reroll options
- Interactive shell mode for continuous dice rolling
- Statistical analysis mode with verbose output
- Examples command showing syntax usage
- Cross-platform release automation (Linux, macOS, Windows)

### Changed
- Improved error handling with detailed error messages
- Enhanced CLI with multiple subcommands (roll, shell, stats, examples)
- Better dice notation parsing with comprehensive validation

### Technical
- Full test coverage with 65+ unit tests and integration tests
- Pre-commit hooks for code quality enforcement
- Automated dependency management with Dependabot
- Multi-platform CI/CD pipeline
- Comprehensive documentation with examples

## [0.2.0] - 2025-08-14

### Added
- Initial public release
- Basic dice rolling functionality (XdY notation)
- Arithmetic operations (+, -, *, /, //)
- Command-line interface
- Library API for programmatic use
- Comprehensive error handling
- Documentation and examples

## [0.1.0] - 2025-08-14

### Added
- Initial development version
- Core dice rolling engine
- Basic parsing functionality

[Unreleased]: https://github.com/raykrueger/rollpoly/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/raykrueger/rollpoly/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/raykrueger/rollpoly/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/raykrueger/rollpoly/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/raykrueger/rollpoly/releases/tag/v0.1.0
