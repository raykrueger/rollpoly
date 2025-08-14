// Copyright 2025 rkrueger
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `RKDice` - A dice rolling library for terminal applications
//!
//! This library provides functionality to parse and evaluate dice notation
//! strings and return the results as arrays of numbers.
//!
//! # Features
//!
//! - **Basic dice rolling**: Roll any number of dice with any number of sides (e.g., `4d10`, `d20`)
//! - **Arithmetic operations**: Add, subtract, multiply, and divide dice results (e.g., `3d6 + 5`)
//! - **Error handling**: Comprehensive error reporting for invalid input
//! - **Random number generation**: Uses cryptographically secure random number generation
//!
//! # Quick Start
//!
//! ```
//! use rkdice::roll;
//!
//! // Roll 4 ten-sided dice and add 17
//! let results = roll("4d10 + 17").unwrap();
//! println!("Rolled: {:?}", results);
//!
//! // Handle errors gracefully
//! match roll("invalid input") {
//!     Ok(results) => println!("Results: {:?}", results),
//!     Err(error) => println!("Error: {}", error),
//! }
//! ```
//!
//! # Supported Syntax
//!
//! The library supports the following dice notation syntax:
//!
//! - `4d10`: Roll a 10-sided die 4 times
//! - `d20`: Roll a 20-sided die once (implicit count)
//! - `2d6 + 3`: Roll 2d6 and add 3
//! - `3d8 - 2`: Roll 3d8 and subtract 2
//! - `1d4 * 3`: Roll 1d4 and multiply by 3
//! - `5d6 / 2`: Roll 5d6 and divide by 2
//! - `4d8 // 3`: Roll 4d8 and floor divide by 3
//!
//! # Error Handling
//!
//! All functions return `Result` types with descriptive error messages.
//! The [`DiceError`] type provides both the original input and a description
//! of what went wrong.

use thiserror::Error;

/// Error type for dice rolling operations
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DiceError {
    #[error("Empty dice notation provided")]
    EmptyInput,

    #[error("Invalid dice notation '{input}': {reason}")]
    InvalidNotation { input: String, reason: String },

    #[error("Invalid die size '{size}': must be a positive integer")]
    InvalidDieSize { size: String },

    #[error("Invalid dice count '{count}': must be a positive integer")]
    InvalidDiceCount { count: String },

    #[error("Invalid modifier '{modifier}': must be a valid integer")]
    InvalidModifier { modifier: String },

    #[error("Unsupported operator '{operator}' in dice notation '{input}'")]
    UnsupportedOperator { operator: String, input: String },
}

/// Rolls dice based on the provided dice notation string.
///
/// # Arguments
///
/// * `dice_notation` - A string slice containing the dice notation (e.g., "4d10 + 17")
///
/// # Returns
///
/// Returns a `Result<Vec<i32>, DiceError>` containing the individual dice roll results and any modifiers,
/// or an error if the input is invalid.
/// For basic rolls like "4d10 + 17", this would return the 4 individual die results
/// plus the modifier as separate elements.
///
/// # Errors
///
/// This function will return an error if:
/// * The dice notation is empty or contains only whitespace
/// * The dice notation contains invalid characters or syntax
/// * The dice notation is malformed (e.g., missing die size, invalid operators)
/// * Numeric values in the notation cannot be parsed
///
/// # Examples
///
/// ```
/// use rkdice::roll;
///
/// let results = roll("2d6 + 3").unwrap();
/// assert_eq!(results.len(), 3); // 2 dice + 1 modifier
///
/// // Invalid input returns an error
/// let error = roll("invalid nonsense").unwrap_err();
/// assert!(error.to_string().contains("invalid nonsense"));
/// ```
pub fn roll(dice_notation: &str) -> Result<Vec<i32>, DiceError> {
    // Trim whitespace and check for empty input
    let notation = dice_notation.trim();

    if notation.is_empty() {
        return Err(DiceError::EmptyInput);
    }

    // Handle basic dice notation parsing
    parse_and_roll_dice(notation).ok_or_else(|| DiceError::InvalidNotation {
        input: dice_notation.to_string(),
        reason: "Unable to parse dice notation".to_string(),
    })
}

/// Parses dice notation and returns the rolled results
fn parse_and_roll_dice(notation: &str) -> Option<Vec<i32>> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Handle different operators
    if notation.contains(" + ") {
        return parse_arithmetic_operation(notation, "+");
    } else if notation.contains(" - ") {
        return parse_arithmetic_operation(notation, "-");
    } else if notation.contains(" * ") {
        return parse_arithmetic_operation(notation, "*");
    } else if notation.contains(" // ") {
        return parse_arithmetic_operation(notation, "//");
    } else if notation.contains(" / ") {
        return parse_arithmetic_operation(notation, "/");
    }

    // Handle simple dice notation (e.g., "4d10", "d6")
    if let Some((count, sides)) = parse_simple_dice(notation) {
        let mut results = Vec::new();
        for _ in 0..count {
            results.push(rng.gen_range(1..=sides));
        }
        return Some(results);
    }

    None
}

/// Parses simple dice notation like "4d10" or "d6"
fn parse_simple_dice(notation: &str) -> Option<(i32, i32)> {
    if let Some(d_pos) = notation.find('d') {
        let count_str = &notation[..d_pos];
        let sides_str = &notation[d_pos + 1..];

        // Handle implicit count (e.g., "d6" means "1d6")
        let count = if count_str.is_empty() {
            1
        } else {
            count_str.parse().ok()?
        };

        let sides = sides_str.parse().ok()?;

        Some((count, sides))
    } else {
        None
    }
}

/// Parses arithmetic operations with dice
fn parse_arithmetic_operation(notation: &str, operator: &str) -> Option<Vec<i32>> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let parts: Vec<&str> = notation.split(&format!(" {operator} ")).collect();
    if parts.len() != 2 {
        return None;
    }

    let dice_part = parts[0].trim();
    let modifier_part = parts[1].trim();

    // Parse the dice part
    if let Some((count, sides)) = parse_simple_dice(dice_part) {
        let mut results = Vec::new();

        // Roll the dice
        for _ in 0..count {
            results.push(rng.gen_range(1..=sides));
        }

        // Parse and add the modifier
        if let Ok(modifier_value) = modifier_part.parse::<i32>() {
            match operator {
                "-" | "//" => results.push(-modifier_value), // Negative to distinguish from regular division
                "+" | "*" | "/" => results.push(modifier_value),
                _ => return None,
            }
        } else {
            return None;
        }

        Some(results)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test constants for reusable values
    const MIN_DIE_VALUE: i32 = 1;
    const D4_MAX: i32 = 4;
    const D6_MAX: i32 = 6;
    const D8_MAX: i32 = 8;
    const D10_MAX: i32 = 10;
    const D12_MAX: i32 = 12;
    const D20_MAX: i32 = 20;
    const D100_MAX: i32 = 100;

    // Helper function for creating test data
    fn assert_die_result_in_range(result: i32, min: i32, max: i32, die_type: &str) {
        assert!(
            result >= min && result <= max,
            "Die result {} for {} should be between {} and {} inclusive",
            result,
            die_type,
            min,
            max
        );
    }

    // Group tests by functionality
    mod basic_dice_rolling {
        use super::*;

        #[test]
        fn test_roll_single_d6_returns_one_result() {
            // Arrange
            let dice_notation = "1d6";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                1,
                "Single d6 should return exactly one result"
            );
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D6_MAX, "d6");
        }

        #[test]
        fn test_roll_multiple_dice_returns_correct_count() {
            // Arrange
            let dice_notation = "4d10";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 4, "4d10 should return exactly 4 results");
            for (index, &die_result) in result.iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D10_MAX,
                    &format!("d10 at index {}", index),
                );
            }
        }

        #[test]
        fn test_roll_implicit_single_die_defaults_to_one() {
            // Arrange
            let dice_notation = "d20";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                1,
                "Implicit single die should return exactly one result"
            );
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D20_MAX, "d20");
        }
    }

    mod arithmetic_operations {
        use super::*;

        #[test]
        fn test_roll_with_addition_includes_modifier() {
            // Arrange
            let dice_notation = "4d10 + 17";
            let expected_modifier = 17;

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                5,
                "4d10 + 17 should return 4 dice results + 1 modifier"
            );

            // Verify dice results are in valid range
            for (index, &die_result) in result[0..4].iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D10_MAX,
                    &format!("d10 at index {}", index),
                );
            }

            // Verify modifier is correct
            assert_eq!(
                result[4], expected_modifier,
                "Last element should be the addition modifier"
            );
        }

        #[test]
        fn test_roll_with_subtraction_includes_negative_modifier() {
            // Arrange
            let dice_notation = "2d20 - 3";
            let expected_modifier = -3;

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                3,
                "2d20 - 3 should return 2 dice results + 1 modifier"
            );

            // Verify dice results are in valid range
            for (index, &die_result) in result[0..2].iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D20_MAX,
                    &format!("d20 at index {}", index),
                );
            }

            // Verify negative modifier is correct
            assert_eq!(
                result[2], expected_modifier,
                "Last element should be the subtraction modifier"
            );
        }

        #[test]
        fn test_roll_with_multiplication_includes_multiplier() {
            // Arrange
            let dice_notation = "1d4 * 3";
            let expected_multiplier = 3;

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                2,
                "1d4 * 3 should return 1 dice result + 1 multiplier"
            );
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D4_MAX, "d4");
            assert_eq!(
                result[1], expected_multiplier,
                "Second element should be the multiplier"
            );
        }

        #[test]
        fn test_roll_with_division_includes_divisor() {
            // Arrange
            let dice_notation = "5d6 / 3";
            let expected_divisor = 3;

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                6,
                "5d6 / 3 should return 5 dice results + 1 divisor"
            );

            // Verify dice results are in valid range
            for (index, &die_result) in result[0..5].iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D6_MAX,
                    &format!("d6 at index {}", index),
                );
            }

            // Verify divisor is correct
            assert_eq!(
                result[5], expected_divisor,
                "Last element should be the divisor"
            );
        }

        #[test]
        fn test_roll_with_floor_division_includes_negative_divisor() {
            // Arrange
            let dice_notation = "5d6 // 3";
            let expected_floor_divisor = -3; // Negative to distinguish from regular division

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(
                result.len(),
                6,
                "5d6 // 3 should return 5 dice results + 1 floor divisor"
            );

            // Verify dice results are in valid range
            for (index, &die_result) in result[0..5].iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D6_MAX,
                    &format!("d6 at index {}", index),
                );
            }

            // Verify floor divisor is represented as negative
            assert_eq!(
                result[5], expected_floor_divisor,
                "Last element should be the floor divisor (negative)"
            );
        }
    }

    mod die_size_variations {
        use super::*;

        #[test]
        fn test_roll_d4_returns_valid_range() {
            // Arrange
            let dice_notation = "1d4";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D4_MAX, "d4");
        }

        #[test]
        fn test_roll_d6_returns_valid_range() {
            // Arrange
            let dice_notation = "1d6";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D6_MAX, "d6");
        }

        #[test]
        fn test_roll_d8_returns_valid_range() {
            // Arrange
            let dice_notation = "1d8";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D8_MAX, "d8");
        }

        #[test]
        fn test_roll_d10_returns_valid_range() {
            // Arrange
            let dice_notation = "1d10";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D10_MAX, "d10");
        }

        #[test]
        fn test_roll_d12_returns_valid_range() {
            // Arrange
            let dice_notation = "1d12";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D12_MAX, "d12");
        }

        #[test]
        fn test_roll_d20_returns_valid_range() {
            // Arrange
            let dice_notation = "1d20";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D20_MAX, "d20");
        }

        #[test]
        fn test_roll_d100_returns_valid_range() {
            // Arrange
            let dice_notation = "1d100";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 1);
            assert_die_result_in_range(result[0], MIN_DIE_VALUE, D100_MAX, "d100");
        }
    }

    mod dice_count_variations {
        use super::*;

        #[test]
        fn test_roll_two_dice_returns_two_results() {
            // Arrange
            let dice_notation = "2d6";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 2, "2d6 should return exactly 2 results");
            for (index, &die_result) in result.iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D6_MAX,
                    &format!("d6 at index {}", index),
                );
            }
        }

        #[test]
        fn test_roll_three_dice_returns_three_results() {
            // Arrange
            let dice_notation = "3d8";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 3, "3d8 should return exactly 3 results");
            for (index, &die_result) in result.iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D8_MAX,
                    &format!("d8 at index {}", index),
                );
            }
        }

        #[test]
        fn test_roll_ten_dice_returns_ten_results() {
            // Arrange
            let dice_notation = "10d10";

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 10, "10d10 should return exactly 10 results");
            for (index, &die_result) in result.iter().enumerate() {
                assert_die_result_in_range(
                    die_result,
                    MIN_DIE_VALUE,
                    D10_MAX,
                    &format!("d10 at index {}", index),
                );
            }
        }
    }

    mod randomness_and_edge_cases {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn test_roll_produces_varied_results_over_multiple_calls() {
            // Arrange
            let dice_notation = "1d20";
            let mut results = HashSet::new();
            let iterations = 50;

            // Act
            for _ in 0..iterations {
                let roll_result =
                    roll(dice_notation).expect("Valid dice notation should not error");
                results.insert(roll_result[0]);
            }

            // Assert
            assert!(
                results.len() > 5,
                "Expected varied random results over {} iterations, got only {} unique values",
                iterations,
                results.len()
            );
        }

        #[test]
        fn test_roll_with_whitespace_handles_spaces_correctly() {
            // Arrange
            let dice_notation = " 2d6 + 5 ";
            let expected_modifier = 5;

            // Act
            let result = roll(dice_notation).expect("Valid dice notation should not error");

            // Assert
            assert_eq!(result.len(), 3, "Whitespace should not affect parsing");
            assert_eq!(
                result[2], expected_modifier,
                "Modifier should be parsed correctly despite whitespace"
            );
        }

        #[test]
        fn test_roll_stays_within_bounds_over_many_iterations() {
            // Arrange
            let dice_notation = "1d6";
            let iterations = 1000;

            // Act & Assert
            for iteration in 0..iterations {
                let result = roll(dice_notation).expect("Valid dice notation should not error");
                assert_eq!(result.len(), 1, "Should always return exactly one result");
                assert_die_result_in_range(
                    result[0],
                    MIN_DIE_VALUE,
                    D6_MAX,
                    &format!("d6 at iteration {}", iteration),
                );
            }
        }
    }

    mod error_handling {
        use super::*;

        #[test]
        fn test_roll_with_invalid_input_returns_error_with_input() {
            // Arrange
            let invalid_input = "unparsable nonsense";

            // Act
            let result = roll(invalid_input);

            // Assert
            assert!(result.is_err(), "Invalid input should return an error");

            let error = result.unwrap_err();

            // Check the full error display includes the input
            let error_display = error.to_string();
            assert!(
                error_display.contains(invalid_input),
                "Error display should contain the invalid input: {}",
                error_display
            );
            assert!(
                error_display.contains("Invalid dice notation"),
                "Error display should contain error context: {}",
                error_display
            );

            // Now check the specific error type
            match error {
                DiceError::InvalidNotation { input, reason: _ } => {
                    assert_eq!(
                        input, invalid_input,
                        "Error should contain the original input"
                    );
                }
                _ => panic!("Expected InvalidNotation error, got: {:?}", error),
            }
        }

        #[test]
        fn test_roll_with_empty_string_returns_error() {
            // Arrange
            let empty_input = "";

            // Act
            let result = roll(empty_input);

            // Assert
            assert!(result.is_err(), "Empty input should return an error");

            let error = result.unwrap_err();
            assert!(
                matches!(error, DiceError::EmptyInput),
                "Expected EmptyInput error, got: {:?}",
                error
            );
        }

        #[test]
        fn test_roll_with_malformed_dice_notation_returns_error() {
            // Arrange
            let test_cases = vec![
                "d",         // Missing die size
                "4d",        // Missing die size
                "d + 5",     // Missing die size with modifier
                "4x6",       // Wrong separator
                "abc",       // Non-numeric
                "4d6 +",     // Incomplete modifier
                "4d6 + abc", // Invalid modifier
            ];

            for invalid_input in test_cases {
                // Act
                let result = roll(invalid_input);

                // Assert
                assert!(
                    result.is_err(),
                    "Invalid input '{}' should return an error",
                    invalid_input
                );

                let error = result.unwrap_err();
                match error {
                    DiceError::InvalidNotation { input, reason: _ } => {
                        assert_eq!(
                            input, invalid_input,
                            "Error should contain the original input for '{}'",
                            invalid_input
                        );
                    }
                    DiceError::EmptyInput => {
                        // Empty input is also acceptable for some test cases
                        assert_eq!(
                            invalid_input, "",
                            "EmptyInput error should only occur for empty input"
                        );
                    }
                    _ => panic!("Unexpected error type for '{}': {:?}", invalid_input, error),
                }
            }
        }
    }
}
