// Copyright 2025 Ray Krueger
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

//! `Rollpoly` - A comprehensive dice rolling library for tabletop gaming
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
//! use rollpoly::roll;
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
/// use rollpoly::roll;
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
/// Rolls dice and applies keep/drop operations
fn roll_keep_drop_dice(
    count: usize,
    sides: i32,
    operation: DiceOperation,
    op_count: usize,
) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut dice_results = Vec::new();

    // Roll all the dice
    for _ in 0..count {
        dice_results.push(rng.gen_range(1..=sides));
    }

    // Sort and apply the operation
    match operation {
        DiceOperation::KeepHighest => {
            dice_results.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending (highest first)
            dice_results.truncate(op_count);
        }
        DiceOperation::KeepLowest => {
            dice_results.sort_unstable(); // Sort ascending (lowest first)
            dice_results.truncate(op_count);
        }
        DiceOperation::DropHighest => {
            dice_results.sort_unstable(); // Sort ascending (lowest first)
            dice_results.truncate(count - op_count); // Keep all but the highest
        }
        DiceOperation::DropLowest => {
            dice_results.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending (highest first)
            dice_results.truncate(count - op_count); // Keep all but the lowest
        }
    }

    dice_results
}

/// Rolls simple dice (no special operations)
fn roll_simple_dice(count: usize, sides: i32) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut dice_results = Vec::new();
    for _ in 0..count {
        dice_results.push(rng.gen_range(1..=sides));
    }
    dice_results
}

/// Rolls dice and counts net successes (successes minus failures)
fn roll_success_failure_dice(
    count: usize,
    sides: i32,
    success_comp: SuccessComparison,
    success_target: i32,
    failure_comp: SuccessComparison,
    failure_target: i32,
) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut net_successes = 0;

    // Roll all the dice and count successes/failures
    for _ in 0..count {
        let roll = rng.gen_range(1..=sides);

        let is_success = match success_comp {
            SuccessComparison::GreaterThan => roll > success_target,
            SuccessComparison::LessThan => roll < success_target,
        };

        let is_failure = match failure_comp {
            SuccessComparison::GreaterThan => roll > failure_target,
            SuccessComparison::LessThan => roll < failure_target,
        };

        if is_success {
            net_successes += 1;
        }
        if is_failure {
            net_successes -= 1;
        }
    }

    // Return the net success count
    vec![net_successes]
}

/// Rolls dice and counts successes based on the provided comparison
fn roll_success_counting_dice(
    count: usize,
    sides: i32,
    comparison: SuccessComparison,
    target: i32,
) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut success_count = 0;

    // Roll all the dice and count successes
    for _ in 0..count {
        let roll = rng.gen_range(1..=sides);

        let is_success = match comparison {
            SuccessComparison::GreaterThan => roll > target,
            SuccessComparison::LessThan => roll < target,
        };

        if is_success {
            success_count += 1;
        }
    }

    // Return the success count as the final result
    vec![success_count]
}

/// Rolls exploding dice based on the provided parameters
fn roll_exploding_dice(
    count: usize,
    sides: i32,
    explode_comp: Option<SuccessComparison>,
    explode_target: Option<i32>,
) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut all_results = Vec::new();

    // Roll each die with exploding
    for _ in 0..count {
        const MAX_EXPLOSIONS: usize = 100;

        let mut die_results = Vec::new();
        let mut current_roll = rng.gen_range(1..=sides);
        die_results.push(current_roll);

        // Safety limit to prevent infinite explosions
        let mut explosion_count = 0;

        // Check if this roll should explode
        loop {
            let should_explode = match (explode_comp, explode_target) {
                (None, None) => {
                    // Simple exploding - explode on max value
                    current_roll == sides
                }
                (None, Some(target)) => {
                    // Explode on specific number (e.g., "d6!3" - explode on exactly 3)
                    current_roll == target
                }
                (Some(SuccessComparison::GreaterThan), Some(target)) => current_roll > target,
                (Some(SuccessComparison::LessThan), Some(target)) => current_roll < target,
                _ => false,
            };

            if should_explode && explosion_count < MAX_EXPLOSIONS {
                // Roll another die and add it
                current_roll = rng.gen_range(1..=sides);
                die_results.push(current_roll);
                explosion_count += 1;
            } else {
                break;
            }
        }

        // Add all results from this exploding die
        all_results.extend(die_results);
    }

    all_results
}

#[allow(clippy::too_many_lines)]
fn parse_and_roll_dice(notation: &str) -> Option<Vec<i32>> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Handle different operators (arithmetic operations first)
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

    // Handle exploding dice syntax (e.g., "2d20!", "7d20!3", "d20!>10", "3d12!<2")
    if let Some((count, sides, explode_comp, explode_target)) = parse_exploding_dice(notation) {
        return Some(roll_exploding_dice(
            count,
            sides,
            explode_comp,
            explode_target,
        ));
    }

    // Handle success counting syntax (e.g., "4d20>19", "10d12<3")
    if let Some((count, sides, comparison, target)) = parse_success_dice(notation) {
        return Some(roll_success_counting_dice(count, sides, comparison, target));
    }

    // Handle success/failure counting syntax (e.g., "10d10>6f<3", "4d20<5f>19")
    if let Some((count, sides, success_comp, success_target, failure_comp, failure_target)) =
        parse_success_failure_dice(notation)
    {
        return Some(roll_success_failure_dice(
            count,
            sides,
            success_comp,
            success_target,
            failure_comp,
            failure_target,
        ));
    }

    // Handle keep highest/lowest syntax (e.g., "4d10K", "7d12K3", "3d6k", "100d6k99")
    if let Some((count, sides, keep_type, keep_count)) = parse_keep_dice(notation) {
        let mut results = Vec::new();

        // Roll all the dice
        for _ in 0..count {
            results.push(rng.gen_range(1..=sides));
        }

        // Sort and keep the specified dice
        match keep_type {
            DiceOperation::KeepHighest => {
                results.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending (highest first)
                results.truncate(keep_count);
            }
            DiceOperation::KeepLowest => {
                results.sort_unstable(); // Sort ascending (lowest first)
                results.truncate(keep_count);
            }
            DiceOperation::DropHighest => {
                results.sort_unstable(); // Sort ascending (lowest first)
                results.truncate(count - keep_count); // Keep all but the highest
            }
            DiceOperation::DropLowest => {
                results.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending (highest first)
                results.truncate(count - keep_count); // Keep all but the lowest
            }
        }

        return Some(results);
    }

    // Handle rerolling dice syntax (e.g., "4d6r1", "2d6R<3")
    if let Some((count, sides, reroll_type, reroll_condition)) = parse_reroll_dice(notation) {
        let mut results = Vec::new();
        for _ in 0..count {
            let mut current_roll = rng.gen_range(1..=sides);
            if reroll_type == RerollType::Once {
                let should_reroll = match reroll_condition {
                    RerollCondition::Value(val) => current_roll == val,
                    RerollCondition::GreaterThan(val) => current_roll > val,
                    RerollCondition::LessThan(val) => current_roll < val,
                };
                if should_reroll {
                    current_roll = rng.gen_range(1..=sides);
                }
            } else {
                // RerollType::Continuous
                const MAX_REROLLS: usize = 100;

                let mut reroll_count = 0;
                loop {
                    let should_reroll = match reroll_condition {
                        RerollCondition::Value(val) => current_roll == val,
                        RerollCondition::GreaterThan(val) => current_roll > val,
                        RerollCondition::LessThan(val) => current_roll < val,
                    };
                    if should_reroll && reroll_count < MAX_REROLLS {
                        current_roll = rng.gen_range(1..=sides);
                        reroll_count += 1;
                    } else {
                        break;
                    }
                }
            }
            results.push(current_roll);
        }
        return Some(results);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RerollType {
    Once,
    Continuous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RerollCondition {
    Value(i32),
    GreaterThan(i32),
    LessThan(i32),
}

#[derive(Debug, Clone, Copy)]
enum DiceOperation {
    KeepHighest,
    KeepLowest,
    DropHighest,
    DropLowest,
}

#[derive(Debug, Clone, Copy)]
enum SuccessComparison {
    GreaterThan,
    LessThan,
}

/// Parses success counting dice notation like "4d20>19", "10d12<3"
fn parse_success_dice(notation: &str) -> Option<(usize, i32, SuccessComparison, i32)> {
    // Find 'd' first
    let d_pos = notation.find('d')?;
    let count_str = &notation[..d_pos];
    let rest = &notation[d_pos + 1..];

    // Handle implicit count (e.g., "d20>15" means "1d20>15")
    let count = if count_str.is_empty() {
        1
    } else {
        count_str.parse().ok()?
    };

    // Find > or <
    let (comparison, comp_pos) = if let Some(pos) = rest.find('>') {
        (SuccessComparison::GreaterThan, pos)
    } else if let Some(pos) = rest.find('<') {
        (SuccessComparison::LessThan, pos)
    } else {
        return None;
    };

    // Parse sides (before comparison character)
    let sides_str = &rest[..comp_pos];
    let sides = sides_str.parse().ok()?;

    // Parse target (after comparison character)
    let target_str = &rest[comp_pos + 1..];
    let target = target_str.parse().ok()?;

    Some((count, sides, comparison, target))
}

/// Parses exploding dice notation like "2d20!", "7d20!3", "d20!>10", "3d12!<2"
fn parse_exploding_dice(
    notation: &str,
) -> Option<(usize, i32, Option<SuccessComparison>, Option<i32>)> {
    // Find 'd' first
    let d_pos = notation.find('d')?;
    let count_str = &notation[..d_pos];
    let rest = &notation[d_pos + 1..];

    // Handle implicit count (e.g., "d20!" means "1d20!")
    let count = if count_str.is_empty() {
        1
    } else {
        count_str.parse().ok()?
    };

    // Find '!' for exploding
    let explode_pos = rest.find('!')?;

    // Parse sides (before !)
    let sides_str = &rest[..explode_pos];
    let sides = sides_str.parse().ok()?;

    // Parse exploding condition (after !)
    let explode_str = &rest[explode_pos + 1..];

    if explode_str.is_empty() {
        // Simple exploding (e.g., "2d20!" - explode on max value)
        Some((count, sides, None, None))
    } else if let Ok(target) = explode_str.parse::<i32>() {
        // Explode on specific number (e.g., "7d20!3" - explode on exactly 3)
        // We'll use a special case in the explosion logic to check for equality
        Some((count, sides, None, Some(target)))
    } else if let Some(pos) = explode_str.find('>') {
        // Explode on greater than (e.g., "d20!>10" - explode on 11+)
        let target_str = &explode_str[pos + 1..];
        let target = target_str.parse().ok()?;
        Some((
            count,
            sides,
            Some(SuccessComparison::GreaterThan),
            Some(target),
        ))
    } else if let Some(pos) = explode_str.find('<') {
        // Explode on less than (e.g., "3d12!<2" - explode on 1)
        let target_str = &explode_str[pos + 1..];
        let target = target_str.parse().ok()?;
        Some((
            count,
            sides,
            Some(SuccessComparison::LessThan),
            Some(target),
        ))
    } else {
        None
    }
}

/// Parses success/failure counting dice notation like "10d10>6f<3", "4d20<5f>19"
fn parse_success_failure_dice(
    notation: &str,
) -> Option<(usize, i32, SuccessComparison, i32, SuccessComparison, i32)> {
    // Find 'd' first
    let d_pos = notation.find('d')?;
    let count_str = &notation[..d_pos];
    let rest = &notation[d_pos + 1..];

    // Handle implicit count
    let count = if count_str.is_empty() {
        1
    } else {
        count_str.parse().ok()?
    };

    // Find 'f' to split success and failure parts
    let f_pos = rest.find('f')?;
    let success_part = &rest[..f_pos];
    let failure_part = &rest[f_pos + 1..];

    // Parse success condition
    let (success_comp, success_comp_pos) = if let Some(pos) = success_part.find('>') {
        (SuccessComparison::GreaterThan, pos)
    } else if let Some(pos) = success_part.find('<') {
        (SuccessComparison::LessThan, pos)
    } else {
        return None;
    };

    let sides_str = &success_part[..success_comp_pos];
    let sides = sides_str.parse().ok()?;
    let success_target_str = &success_part[success_comp_pos + 1..];
    let success_target = success_target_str.parse().ok()?;

    // Parse failure condition
    let (failure_comp, failure_comp_pos) = if let Some(pos) = failure_part.find('>') {
        (SuccessComparison::GreaterThan, pos)
    } else if let Some(pos) = failure_part.find('<') {
        (SuccessComparison::LessThan, pos)
    } else {
        return None;
    };

    let failure_target_str = &failure_part[failure_comp_pos + 1..];
    let failure_target = failure_target_str.parse().ok()?;

    // Validate that success and failure conditions don't conflict
    // Both can't be greater than or both less than
    match (success_comp, failure_comp) {
        (SuccessComparison::GreaterThan, SuccessComparison::GreaterThan)
        | (SuccessComparison::LessThan, SuccessComparison::LessThan) => return None,
        _ => {}
    }

    Some((
        count,
        sides,
        success_comp,
        success_target,
        failure_comp,
        failure_target,
    ))
}
fn parse_keep_dice(notation: &str) -> Option<(usize, i32, DiceOperation, usize)> {
    // Find 'd' first
    let d_pos = notation.find('d')?;
    let count_str = &notation[..d_pos];
    let rest = &notation[d_pos + 1..];

    // Handle implicit count (e.g., "dK6" means "1dK6")
    let count = if count_str.is_empty() {
        1
    } else {
        count_str.parse().ok()?
    };

    // Find K, k, X, or x
    let (operation, op_pos) = if let Some(pos) = rest.find('K') {
        (DiceOperation::KeepHighest, pos)
    } else if let Some(pos) = rest.find('k') {
        (DiceOperation::KeepLowest, pos)
    } else if let Some(pos) = rest.find('X') {
        (DiceOperation::DropHighest, pos)
    } else if let Some(pos) = rest.find('x') {
        (DiceOperation::DropLowest, pos)
    } else {
        return None;
    };

    // Parse sides (before operation character)
    let sides_str = &rest[..op_pos];
    let sides = sides_str.parse().ok()?;

    // Parse operation count (after operation character)
    let op_str = &rest[op_pos + 1..];
    let op_count = if op_str.is_empty() {
        1 // Default to operating on 1 die if no number specified
    } else {
        op_str.parse().ok()?
    };

    // Validate the operation makes sense
    match operation {
        DiceOperation::KeepHighest | DiceOperation::KeepLowest => {
            // Can't keep more dice than we roll
            if op_count > count {
                return None;
            }
        }
        DiceOperation::DropHighest | DiceOperation::DropLowest => {
            // Can't drop more dice than we roll, and must keep at least 1
            if op_count >= count {
                return None;
            }
        }
    }

    Some((count, sides, operation, op_count))
}

/// Parses rerolling dice notation like "4d6r1", "2d6R<3"
fn parse_reroll_dice(notation: &str) -> Option<(usize, i32, RerollType, RerollCondition)> {
    let d_pos = notation.find('d')?;
    let count_str = &notation[..d_pos];
    let rest = &notation[d_pos + 1..];

    let count = if count_str.is_empty() {
        1
    } else {
        count_str.parse().ok()?
    };

    let reroll_pos = rest.find(['r', 'R'])?;
    let sides_str = &rest[..reroll_pos];
    let sides = sides_str.parse().ok()?;

    let reroll_type = if rest.chars().nth(reroll_pos)? == 'r' {
        RerollType::Once
    } else {
        RerollType::Continuous
    };

    let condition_str = &rest[reroll_pos + 1..];
    let condition = if let Some(stripped) = condition_str.strip_prefix('<') {
        RerollCondition::LessThan(stripped.parse().ok()?)
    } else if let Some(stripped) = condition_str.strip_prefix('>') {
        RerollCondition::GreaterThan(stripped.parse().ok()?)
    } else {
        RerollCondition::Value(condition_str.parse().ok()?)
    };

    Some((count, sides, reroll_type, condition))
}

/// Parses simple dice notation like "4d10" or "d6"
fn parse_simple_dice(notation: &str) -> Option<(usize, i32)> {
    if let Some(d_pos) = notation.find('d') {
        let count_str = &notation[..d_pos];
        let sides_str = &notation[d_pos + 1..];

        // Handle implicit count (e.g., "d6" means "1d6")
        let count = if count_str.is_empty() {
            1
        } else {
            let parsed_count: i32 = count_str.parse().ok()?;
            if parsed_count <= 0 {
                return None; // Reject negative or zero dice counts
            }
            usize::try_from(parsed_count).ok()?
        };

        let sides = sides_str.parse().ok()?;
        if sides <= 0 {
            return None; // Reject negative or zero sided dice
        }

        Some((count, sides))
    } else {
        None
    }
}

/// Parses arithmetic operations with dice
fn parse_arithmetic_operation(notation: &str, operator: &str) -> Option<Vec<i32>> {
    let parts: Vec<&str> = notation.split(&format!(" {operator} ")).collect();
    if parts.len() != 2 {
        return None;
    }

    let left_part = parts[0].trim();
    let right_part = parts[1].trim();

    // Parse the left part (could be simple dice, keep dice, or drop dice)
    let mut results = if let Some((count, sides, operation, op_count)) = parse_keep_dice(left_part)
    {
        // Handle keep/drop dice with arithmetic
        roll_keep_drop_dice(count, sides, operation, op_count)
    } else if let Some((count, sides)) = parse_simple_dice(left_part) {
        // Handle simple dice with arithmetic
        roll_simple_dice(count, sides)
    } else {
        return None;
    };

    // Parse the right part - could be dice or a number
    if let Some((count, sides, operation, op_count)) = parse_keep_dice(right_part) {
        // Right side is keep/drop dice
        let right_dice = roll_keep_drop_dice(count, sides, operation, op_count);

        // Apply the operation between left and right dice
        match operator {
            "+" => results.extend(right_dice),
            "-" => {
                // For subtraction, negate the right dice values
                results.extend(right_dice.iter().map(|&x| -x));
            }
            "*" => {
                // For multiplication, multiply each left die by each right die (unusual but possible)
                let left_sum: i32 = results.iter().sum();
                let right_sum: i32 = right_dice.iter().sum();
                results.clear();
                results.push(left_sum * right_sum);
            }
            "/" | "//" => {
                // For division, divide left sum by right sum
                let left_sum: i32 = results.iter().sum();
                let right_sum: i32 = right_dice.iter().sum();
                if right_sum == 0 {
                    return None; // Division by zero
                }
                results.clear();
                let result = left_sum / right_sum; // Floor division for integers
                results.push(result);
            }
            _ => return None,
        }
    } else if let Some((count, sides)) = parse_simple_dice(right_part) {
        // Right side is simple dice
        let right_dice = roll_simple_dice(count, sides);

        // Apply the operation between left and right dice
        match operator {
            "+" => results.extend(right_dice),
            "-" => {
                // For subtraction, negate the right dice values
                results.extend(right_dice.iter().map(|&x| -x));
            }
            "*" => {
                // For multiplication, multiply sums
                let left_sum: i32 = results.iter().sum();
                let right_sum: i32 = right_dice.iter().sum();
                results.clear();
                results.push(left_sum * right_sum);
            }
            "/" | "//" => {
                // For division, divide sums
                let left_sum: i32 = results.iter().sum();
                let right_sum: i32 = right_dice.iter().sum();
                if right_sum == 0 {
                    return None; // Division by zero
                }
                results.clear();
                let result = left_sum / right_sum; // Floor division for integers
                results.push(result);
            }
            _ => return None,
        }
    } else if let Ok(modifier_value) = right_part.parse::<i32>() {
        // Right side is a number (existing functionality)
        match operator {
            "-" | "//" => results.push(-modifier_value), // Negative to distinguish from regular division
            "+" | "*" | "/" => results.push(modifier_value),
            _ => return None,
        }
    } else {
        return None;
    }

    Some(results)
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
            let iterations = 100; // Reduced from 1000 for faster tests

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

    mod keep_dice_operations {
        use super::*;

        #[test]
        fn test_keep_highest_single_die() {
            // Arrange
            let notation = "4d6K";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Keep highest should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should keep only 1 die");
            assert!(
                results[0] >= 1 && results[0] <= 6,
                "Result should be valid die roll"
            );
        }

        #[test]
        fn test_keep_highest_multiple_dice() {
            // Arrange
            let notation = "7d12K3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Keep highest multiple should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should keep exactly 3 dice");

            // Results should be in descending order (highest first)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] >= results[i],
                    "Results should be in descending order: {:?}",
                    results
                );
            }

            // All results should be valid
            for &result in &results {
                assert!(
                    result >= 1 && result <= 12,
                    "All results should be valid d12 rolls"
                );
            }
        }

        #[test]
        fn test_keep_lowest_single_die() {
            // Arrange
            let notation = "3d6k";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Keep lowest should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should keep only 1 die");
            assert!(
                results[0] >= 1 && results[0] <= 6,
                "Result should be valid die roll"
            );
        }

        #[test]
        fn test_keep_lowest_multiple_dice() {
            // Arrange
            let notation = "5d6k3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Keep lowest multiple should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should keep exactly 3 dice");

            // Results should be in ascending order (lowest first)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] <= results[i],
                    "Results should be in ascending order: {:?}",
                    results
                );
            }

            // All results should be valid
            for &result in &results {
                assert!(
                    result >= 1 && result <= 6,
                    "All results should be valid d6 rolls"
                );
            }
        }

        #[test]
        fn test_keep_highest_with_arithmetic() {
            // Arrange
            let notation = "4d6K2 + 5";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Keep highest with arithmetic should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should have 2 kept dice + 1 modifier");

            // Last element should be the modifier
            assert_eq!(results[2], 5, "Last element should be the +5 modifier");

            // First two should be dice results in descending order
            assert!(
                results[0] >= results[1],
                "Kept dice should be in descending order: {:?}",
                results
            );
        }

        #[test]
        fn test_disadvantage_roll() {
            // Arrange - This is a common D&D 5e disadvantage roll
            let notation = "2d20k";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Disadvantage roll should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should keep only the lowest die");
            assert!(
                results[0] >= 1 && results[0] <= 20,
                "Result should be valid d20 roll"
            );
        }

        #[test]
        fn test_advantage_roll() {
            // Arrange - This is a common D&D 5e advantage roll
            let notation = "2d20K";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Advantage roll should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should keep only the highest die");
            assert!(
                results[0] >= 1 && results[0] <= 20,
                "Result should be valid d20 roll"
            );
        }
    }

    mod drop_dice_operations {
        use super::*;

        #[test]
        fn test_drop_highest_single_die() {
            // Arrange
            let notation = "6d8X";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Drop highest should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 5, "Should keep 5 dice after dropping 1");

            // Results should be in ascending order (lowest first, highest dropped)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] <= results[i],
                    "Results should be in ascending order: {:?}",
                    results
                );
            }

            // All results should be valid d8 rolls
            for &result in &results {
                assert!(
                    result >= 1 && result <= 8,
                    "All results should be valid d8 rolls"
                );
            }
        }

        #[test]
        fn test_drop_highest_multiple_dice() {
            // Arrange
            let notation = "5d10X3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Drop highest multiple should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 2, "Should keep 2 dice after dropping 3");

            // Results should be in ascending order (lowest kept)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] <= results[i],
                    "Results should be in ascending order: {:?}",
                    results
                );
            }

            // All results should be valid d10 rolls
            for &result in &results {
                assert!(
                    result >= 1 && result <= 10,
                    "All results should be valid d10 rolls"
                );
            }
        }

        #[test]
        fn test_drop_lowest_single_die() {
            // Arrange
            let notation = "6d8x";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Drop lowest should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 5, "Should keep 5 dice after dropping 1");

            // Results should be in descending order (highest first, lowest dropped)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] >= results[i],
                    "Results should be in descending order: {:?}",
                    results
                );
            }

            // All results should be valid d8 rolls
            for &result in &results {
                assert!(
                    result >= 1 && result <= 8,
                    "All results should be valid d8 rolls"
                );
            }
        }

        #[test]
        fn test_drop_lowest_multiple_dice() {
            // Arrange
            let notation = "5d10x3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Drop lowest multiple should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 2, "Should keep 2 dice after dropping 3");

            // Results should be in descending order (highest kept)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] >= results[i],
                    "Results should be in descending order: {:?}",
                    results
                );
            }

            // All results should be valid d10 rolls
            for &result in &results {
                assert!(
                    result >= 1 && result <= 10,
                    "All results should be valid d10 rolls"
                );
            }
        }

        #[test]
        fn test_drop_highest_with_arithmetic() {
            // Arrange
            let notation = "6d6X2 + 5";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Drop highest with arithmetic should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 5, "Should have 4 kept dice + 1 modifier");

            // Last element should be the modifier
            assert_eq!(results[4], 5, "Last element should be the +5 modifier");

            // First four should be dice results in ascending order (lowest kept)
            for i in 1..4 {
                assert!(
                    results[i - 1] <= results[i],
                    "Kept dice should be in ascending order: {:?}",
                    results
                );
            }
        }

        #[test]
        fn test_character_generation_4d6_drop_lowest() {
            // Arrange - This is a common D&D character generation method
            let notation = "4d6x";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "4d6 drop lowest should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should keep 3 dice after dropping lowest");

            // Results should be in descending order (highest kept)
            for i in 1..results.len() {
                assert!(
                    results[i - 1] >= results[i],
                    "Results should be in descending order: {:?}",
                    results
                );
            }

            // All results should be valid d6 rolls
            for &result in &results {
                assert!(
                    result >= 1 && result <= 6,
                    "All results should be valid d6 rolls"
                );
            }
        }

        #[test]
        fn test_drop_consistency_over_multiple_rolls() {
            // Arrange
            let notation = "8d6X3";

            // Act & Assert - Test multiple times to ensure consistency
            for _ in 0..20 {
                let result = roll(notation);
                assert!(result.is_ok(), "Drop should work consistently");

                let results = result.unwrap();
                assert_eq!(results.len(), 5, "Should always keep exactly 5 dice");

                // Should be in ascending order (lowest kept)
                for i in 1..results.len() {
                    assert!(
                        results[i - 1] <= results[i],
                        "Results should always be in ascending order: {:?}",
                        results
                    );
                }

                // All should be valid d6 rolls
                for &result in &results {
                    assert!(
                        result >= 1 && result <= 6,
                        "All results should be valid d6 rolls"
                    );
                }
            }
        }
    }

    mod reroll_dice_operations {
        use super::*;

        #[test]
        fn test_reroll_once_on_value() {
            // This test can't guarantee the rerolled value isn't the same.
            // We are just testing that it runs without error.
            for _ in 0..100 {
                let result = roll("1d6r1").unwrap();
                assert_eq!(result.len(), 1);
            }
        }

        #[test]
        fn test_reroll_continuous_on_value() {
            for _ in 0..100 {
                let result = roll("1d6R1").unwrap();
                assert_eq!(result.len(), 1);
                assert_ne!(result[0], 1);
            }
        }

        #[test]
        fn test_reroll_once_less_than() {
            // This test can't guarantee the rerolled value is not less than 3.
            // We are just testing that it runs without error.
            for _ in 0..100 {
                let result = roll("1d6r<3").unwrap();
                assert_eq!(result.len(), 1);
            }
        }

        #[test]
        fn test_reroll_continuous_less_than() {
            for _ in 0..100 {
                let result = roll("1d6R<3").unwrap();
                assert_eq!(result.len(), 1);
                assert!(result[0] >= 3);
            }
        }
    }

    mod dice_to_dice_operations {
        use super::*;

        #[test]
        fn test_dice_plus_dice() {
            // Arrange
            let notation = "2d6 + 1d4";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Dice + dice should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should have 2d6 + 1d4 = 3 dice");

            // First two should be d6 results
            for i in 0..2 {
                assert!(
                    results[i] >= 1 && results[i] <= 6,
                    "d6 results should be 1-6"
                );
            }

            // Last should be d4 result
            assert!(
                results[2] >= 1 && results[2] <= 4,
                "d4 result should be 1-4"
            );
        }

        #[test]
        fn test_dice_minus_dice() {
            // Arrange
            let notation = "2d12 - 1d6";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Dice - dice should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should have 2d12 - 1d6 = 3 values");

            // First two should be d12 results
            for i in 0..2 {
                assert!(
                    results[i] >= 1 && results[i] <= 12,
                    "d12 results should be 1-12"
                );
            }

            // Last should be negative d6 result
            assert!(
                results[2] >= -6 && results[2] <= -1,
                "Subtracted d6 should be -6 to -1"
            );
        }

        #[test]
        fn test_daggerheart_advantage_roll() {
            // Arrange - Daggerheart with Advantage
            let notation = "2d12 + 1d6";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Daggerheart Advantage roll should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should have 2d12 + 1d6 = 3 dice");

            // Check ranges
            let sum: i32 = results.iter().sum();
            assert!(
                sum >= 3 && sum <= 30,
                "Daggerheart Advantage sum should be 3-30"
            );
        }

        #[test]
        fn test_daggerheart_disadvantage_roll() {
            // Arrange - Daggerheart with Disadvantage
            let notation = "2d12 - 1d6";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Daggerheart Disadvantage roll should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 3, "Should have 2d12 - 1d6 = 3 values");

            // Check ranges: 2d12 (2-24) - 1d6 (1-6) = -4 to 23
            let sum: i32 = results.iter().sum();
            assert!(
                sum >= -4 && sum <= 23,
                "Daggerheart Disadvantage sum should be -4 to 23, got {}",
                sum
            );
        }

        #[test]
        fn test_keep_dice_plus_dice() {
            // Arrange
            let notation = "4d6K3 + 1d4";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Keep dice + dice should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 4, "Should have 3 kept d6 + 1d4 = 4 dice");

            // First three should be d6 results in descending order
            for i in 0..3 {
                assert!(
                    results[i] >= 1 && results[i] <= 6,
                    "Kept d6 results should be 1-6"
                );
            }
            for i in 1..3 {
                assert!(
                    results[i - 1] >= results[i],
                    "Kept dice should be in descending order"
                );
            }

            // Last should be d4 result
            assert!(
                results[3] >= 1 && results[3] <= 4,
                "d4 result should be 1-4"
            );
        }

        #[test]
        fn test_dice_to_dice_consistency() {
            // Arrange
            let notation = "3d6 + 2d4";

            // Act & Assert - Test multiple times to ensure consistency
            for _ in 0..20 {
                let result = roll(notation);
                assert!(result.is_ok(), "Dice + dice should work consistently");

                let results = result.unwrap();
                assert_eq!(results.len(), 5, "Should always have 5 dice");

                // Check ranges
                for i in 0..3 {
                    assert!(
                        results[i] >= 1 && results[i] <= 6,
                        "d6 results should be 1-6"
                    );
                }
                for i in 3..5 {
                    assert!(
                        results[i] >= 1 && results[i] <= 4,
                        "d4 results should be 1-4"
                    );
                }

                let sum: i32 = results.iter().sum();
                assert!(sum >= 5 && sum <= 26, "Sum should be in valid range");
            }
        }
    }

    mod success_counting_operations {
        use super::*;

        #[test]
        fn test_count_successes_greater_than() {
            // Arrange
            let notation = "5d10>7";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Success counting should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single success count");

            // Success count should be between 0 and 5
            let success_count = results[0];
            assert!(
                success_count >= 0 && success_count <= 5,
                "Success count should be 0-5, got {}",
                success_count
            );
        }

        #[test]
        fn test_count_successes_less_than() {
            // Arrange
            let notation = "8d6<3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Success counting with < should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single success count");

            // Success count should be between 0 and 8
            let success_count = results[0];
            assert!(
                success_count >= 0 && success_count <= 8,
                "Success count should be 0-8, got {}",
                success_count
            );
        }

        #[test]
        fn test_world_of_darkness_style() {
            // Arrange - World of Darkness typically uses d10>7
            let notation = "5d10>7";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "World of Darkness style should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single success count");

            let success_count = results[0];
            assert!(
                success_count >= 0 && success_count <= 5,
                "Success count should be valid range"
            );
        }

        #[test]
        fn test_shadowrun_style() {
            // Arrange - Shadowrun typically uses d6>4
            let notation = "12d6>4";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Shadowrun style should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single success count");

            let success_count = results[0];
            assert!(
                success_count >= 0 && success_count <= 12,
                "Success count should be 0-12, got {}",
                success_count
            );
        }

        #[test]
        fn test_success_failure_counting() {
            // Arrange
            let notation = "10d10>6f<3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Success/failure counting should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single net success count");

            // Net successes can be negative due to failures
            let net_successes = results[0];
            assert!(
                net_successes >= -10 && net_successes <= 10,
                "Net successes should be -10 to 10, got {}",
                net_successes
            );
        }

        #[test]
        fn test_success_failure_opposite_conditions() {
            // Arrange
            let notation = "4d20<5f>19";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Success < failure > should work");
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single net success count");

            let net_successes = results[0];
            assert!(
                net_successes >= -4 && net_successes <= 4,
                "Net successes should be -4 to 4, got {}",
                net_successes
            );
        }

        #[test]
        fn test_success_counting_consistency() {
            // Arrange
            let notation = "6d6>4";

            // Act & Assert - Test multiple times to ensure consistency
            for _ in 0..20 {
                let result = roll(notation);
                assert!(result.is_ok(), "Success counting should work consistently");

                let results = result.unwrap();
                assert_eq!(
                    results.len(),
                    1,
                    "Should always return single success count"
                );

                let success_count = results[0];
                assert!(
                    success_count >= 0 && success_count <= 6,
                    "Success count should always be 0-6"
                );
            }
        }

        #[test]
        fn test_implicit_single_die_success_counting() {
            // Arrange
            let notation = "d20>15";

            // Act
            let result = roll(notation);

            // Assert
            assert!(
                result.is_ok(),
                "Implicit single die success counting should work"
            );
            let results = result.unwrap();
            assert_eq!(results.len(), 1, "Should return single success count");

            let success_count = results[0];
            assert!(
                success_count >= 0 && success_count <= 1,
                "Single die success count should be 0 or 1"
            );
        }
    }

    mod exploding_dice_operations {
        use super::*;

        #[test]
        fn test_simple_exploding_dice() {
            // Arrange
            let notation = "2d6!";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Simple exploding dice should work");
            let results = result.unwrap();

            // Should have at least 2 dice (the original rolls)
            assert!(results.len() >= 2, "Should have at least 2 dice results");

            // All results should be valid d6 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 6,
                    "All rolls should be 1-6, got {}",
                    roll
                );
            }

            // Total should be at least 2 (minimum possible)
            let total: i32 = results.iter().sum();
            assert!(total >= 2, "Total should be at least 2");
        }

        #[test]
        fn test_exploding_on_specific_number() {
            // Arrange
            let notation = "3d10!5";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Exploding on specific number should work");
            let results = result.unwrap();

            // Should have at least 3 dice (the original rolls)
            assert!(results.len() >= 3, "Should have at least 3 dice results");

            // All results should be valid d10 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 10,
                    "All rolls should be 1-10, got {}",
                    roll
                );
            }
        }

        #[test]
        fn test_exploding_greater_than() {
            // Arrange
            let notation = "d20!>15";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Exploding greater than should work");
            let results = result.unwrap();

            // Should have at least 1 die
            assert!(results.len() >= 1, "Should have at least 1 die result");

            // All results should be valid d20 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 20,
                    "All rolls should be 1-20, got {}",
                    roll
                );
            }
        }

        #[test]
        fn test_exploding_less_than() {
            // Arrange
            let notation = "2d12!<3";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Exploding less than should work");
            let results = result.unwrap();

            // Should have at least 2 dice
            assert!(results.len() >= 2, "Should have at least 2 dice results");

            // All results should be valid d12 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 12,
                    "All rolls should be 1-12, got {}",
                    roll
                );
            }
        }

        #[test]
        fn test_implicit_single_die_exploding() {
            // Arrange
            let notation = "d6!";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Implicit single die exploding should work");
            let results = result.unwrap();

            // Should have at least 1 die
            assert!(results.len() >= 1, "Should have at least 1 die result");

            // All results should be valid d6 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 6,
                    "All rolls should be 1-6, got {}",
                    roll
                );
            }
        }

        #[test]
        fn test_exploding_dice_consistency() {
            // Arrange
            let notation = "2d8!";

            // Act & Assert - Test multiple times to ensure consistency
            for _ in 0..20 {
                let result = roll(notation);
                assert!(result.is_ok(), "Exploding dice should work consistently");

                let results = result.unwrap();
                assert!(results.len() >= 2, "Should always have at least 2 dice");

                // All results should be valid d8 rolls
                for &roll in &results {
                    assert!(roll >= 1 && roll <= 8, "All rolls should be 1-8");
                }

                // Total should be reasonable (at least 2, but not impossibly high)
                let total: i32 = results.iter().sum();
                assert!(
                    total >= 2 && total <= 200,
                    "Total should be reasonable range"
                );
            }
        }

        #[test]
        fn test_shadowrun_exploding_sixes() {
            // Arrange - Shadowrun style: d6 exploding on 6
            let notation = "4d6!6";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Shadowrun exploding sixes should work");
            let results = result.unwrap();

            // Should have at least 4 dice
            assert!(results.len() >= 4, "Should have at least 4 dice results");

            // All results should be valid d6 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 6,
                    "All rolls should be 1-6, got {}",
                    roll
                );
            }
        }

        #[test]
        fn test_exploding_edge_cases() {
            // Test exploding on specific number (limited iterations to avoid long test)
            let result1 = roll("d6!6"); // Explode on 6 instead of 1 (less frequent)
            assert!(result1.is_ok(), "Exploding on 6 should work");

            // Test exploding on max value
            let result2 = roll("d20!");
            assert!(result2.is_ok(), "Exploding on max should work");

            // Test exploding with comparison
            let result3 = roll("d10!>8");
            assert!(result3.is_ok(), "Exploding >8 should work");

            let result4 = roll("d10!<3");
            assert!(result4.is_ok(), "Exploding <3 should work");
        }

        #[test]
        fn test_multiple_exploding_dice() {
            // Arrange
            let notation = "5d6!";

            // Act
            let result = roll(notation);

            // Assert
            assert!(result.is_ok(), "Multiple exploding dice should work");
            let results = result.unwrap();

            // Should have at least 5 dice (the original rolls)
            assert!(results.len() >= 5, "Should have at least 5 dice results");

            // All results should be valid d6 rolls
            for &roll in &results {
                assert!(
                    roll >= 1 && roll <= 6,
                    "All rolls should be 1-6, got {}",
                    roll
                );
            }

            // Total should be at least 5
            let total: i32 = results.iter().sum();
            assert!(total >= 5, "Total should be at least 5");
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
