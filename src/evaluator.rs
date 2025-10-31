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

//! Evaluator for dice expressions
//!
//! This module takes the parsed AST and evaluates it to produce actual dice roll results.

use crate::parser::{
    BinaryOp, Comparison, DiceExpression, ExplodeCondition, RerollCondition, RerollType,
};
use crate::DiceError;
use rand::Rng;

/// Evaluates a dice expression and returns the results
pub fn evaluate(expr: &DiceExpression) -> Result<Vec<i32>, DiceError> {
    let mut rng = rand::rng();
    evaluate_with_rng(expr, &mut rng)
}

/// Evaluates a dice expression with a provided RNG
#[allow(clippy::too_many_lines)] // Complex but well-structured function
fn evaluate_with_rng<R: Rng>(expr: &DiceExpression, rng: &mut R) -> Result<Vec<i32>, DiceError> {
    match expr {
        DiceExpression::Simple { count, sides } => {
            let mut results = Vec::with_capacity(*count);
            for _ in 0..*count {
                results.push(rng.random_range(1..=*sides));
            }
            Ok(results)
        }

        DiceExpression::KeepHighest { count, sides, keep } => {
            let mut results = Vec::with_capacity(*count);
            for _ in 0..*count {
                results.push(rng.random_range(1..=*sides));
            }
            results.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending (highest first)
            results.truncate(*keep);
            Ok(results)
        }

        DiceExpression::KeepLowest { count, sides, keep } => {
            let mut results = Vec::with_capacity(*count);
            for _ in 0..*count {
                results.push(rng.random_range(1..=*sides));
            }
            results.sort_unstable(); // Sort ascending (lowest first)
            results.truncate(*keep);
            Ok(results)
        }

        DiceExpression::DropHighest { count, sides, drop } => {
            let mut results = Vec::with_capacity(*count);
            for _ in 0..*count {
                results.push(rng.random_range(1..=*sides));
            }
            results.sort_unstable(); // Sort ascending (lowest first)
            results.truncate(count - drop); // Keep all but the highest
            Ok(results)
        }

        DiceExpression::DropLowest { count, sides, drop } => {
            let mut results = Vec::with_capacity(*count);
            for _ in 0..*count {
                results.push(rng.random_range(1..=*sides));
            }
            results.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending (highest first)
            results.truncate(count - drop); // Keep all but the lowest
            Ok(results)
        }

        DiceExpression::Exploding {
            count,
            sides,
            condition,
        } => {
            let mut all_results = Vec::new();

            for _ in 0..*count {
                const MAX_EXPLOSIONS: usize = 100;
                let mut die_results = Vec::new();
                let mut current_roll = rng.random_range(1..=*sides);
                die_results.push(current_roll);

                let mut explosion_count = 0;
                loop {
                    let should_explode = match condition {
                        ExplodeCondition::Max => current_roll == *sides,
                        ExplodeCondition::Value(target) => current_roll == *target,
                        ExplodeCondition::Comparison(Comparison::GreaterThan, target) => {
                            current_roll > *target
                        }
                        ExplodeCondition::Comparison(Comparison::LessThan, target) => {
                            current_roll < *target
                        }
                    };

                    if should_explode && explosion_count < MAX_EXPLOSIONS {
                        current_roll = rng.random_range(1..=*sides);
                        die_results.push(current_roll);
                        explosion_count += 1;
                    } else {
                        break;
                    }
                }

                all_results.extend(die_results);
            }

            Ok(all_results)
        }

        DiceExpression::SuccessCounting {
            count,
            sides,
            target,
            comparison,
        } => {
            let mut success_count = 0;

            for _ in 0..*count {
                let roll = rng.random_range(1..=*sides);
                let is_success = match comparison {
                    Comparison::GreaterThan => roll > *target,
                    Comparison::LessThan => roll < *target,
                };

                if is_success {
                    success_count += 1;
                }
            }

            Ok(vec![success_count])
        }

        DiceExpression::SuccessFailure {
            count,
            sides,
            success_target,
            success_comparison,
            failure_target,
            failure_comparison,
        } => {
            let mut net_successes = 0;

            for _ in 0..*count {
                let roll = rng.random_range(1..=*sides);

                let is_success = match success_comparison {
                    Comparison::GreaterThan => roll > *success_target,
                    Comparison::LessThan => roll < *success_target,
                };

                let is_failure = match failure_comparison {
                    Comparison::GreaterThan => roll > *failure_target,
                    Comparison::LessThan => roll < *failure_target,
                };

                if is_success {
                    net_successes += 1;
                }
                if is_failure {
                    net_successes -= 1;
                }
            }

            Ok(vec![net_successes])
        }

        DiceExpression::Rerolling {
            count,
            sides,
            condition,
            reroll_type,
        } => {
            let mut results = Vec::with_capacity(*count);

            for _ in 0..*count {
                let mut current_roll = rng.random_range(1..=*sides);

                if *reroll_type == RerollType::Once {
                    let should_reroll = match condition {
                        RerollCondition::Value(val) => current_roll == *val,
                        RerollCondition::Comparison(Comparison::GreaterThan, val) => {
                            current_roll > *val
                        }
                        RerollCondition::Comparison(Comparison::LessThan, val) => {
                            current_roll < *val
                        }
                    };
                    if should_reroll {
                        current_roll = rng.random_range(1..=*sides);
                    }
                } else {
                    // RerollType::Continuous
                    const MAX_REROLLS: usize = 100;
                    let mut reroll_count = 0;

                    loop {
                        let should_reroll = match condition {
                            RerollCondition::Value(val) => current_roll == *val,
                            RerollCondition::Comparison(Comparison::GreaterThan, val) => {
                                current_roll > *val
                            }
                            RerollCondition::Comparison(Comparison::LessThan, val) => {
                                current_roll < *val
                            }
                        };

                        if should_reroll && reroll_count < MAX_REROLLS {
                            current_roll = rng.random_range(1..=*sides);
                            reroll_count += 1;
                        } else {
                            break;
                        }
                    }
                }

                results.push(current_roll);
            }

            Ok(results)
        }

        DiceExpression::Binary { left, op, right } => {
            let left_results = evaluate_with_rng(left, rng)?;
            let right_results = evaluate_with_rng(right, rng)?;

            match op {
                BinaryOp::Add => {
                    let mut results = left_results;
                    results.extend(right_results);
                    Ok(results)
                }
                BinaryOp::Subtract => {
                    let mut results = left_results;
                    // For subtraction, negate the right side values
                    results.extend(right_results.iter().map(|&x| -x));
                    Ok(results)
                }
                BinaryOp::Multiply => {
                    let left_sum: i32 = left_results.iter().sum();
                    let right_sum: i32 = right_results.iter().sum();
                    Ok(vec![left_sum * right_sum])
                }
                BinaryOp::Divide => {
                    let left_sum: i32 = left_results.iter().sum();
                    let right_sum: i32 = right_results.iter().sum();
                    if right_sum == 0 {
                        return Err(DiceError::InvalidNotation {
                            input: "division by zero".to_string(),
                            reason: "Cannot divide by zero".to_string(),
                        });
                    }
                    Ok(vec![left_sum / right_sum])
                }
                BinaryOp::FloorDivide => {
                    let left_sum: i32 = left_results.iter().sum();
                    let right_sum: i32 = right_results.iter().sum();
                    if right_sum == 0 {
                        return Err(DiceError::InvalidNotation {
                            input: "division by zero".to_string(),
                            reason: "Cannot divide by zero".to_string(),
                        });
                    }
                    Ok(vec![left_sum.div_euclid(right_sum)])
                }
            }
        }

        DiceExpression::Constant(value) => Ok(vec![*value]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::DiceParser;

    #[test]
    fn test_evaluate_simple_dice() {
        let mut parser = DiceParser::new("2d6");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 2);
        for &result in &results {
            assert!(result >= 1 && result <= 6);
        }
    }

    #[test]
    fn test_evaluate_keep_highest() {
        let mut parser = DiceParser::new("4d6K3");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 3);
        // Results should be in descending order
        for i in 1..results.len() {
            assert!(results[i - 1] >= results[i]);
        }
        for &result in &results {
            assert!(result >= 1 && result <= 6);
        }
    }

    #[test]
    fn test_evaluate_arithmetic() {
        let mut parser = DiceParser::new("2d6 + 3");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 3); // 2 dice + 1 constant
                                      // First two should be dice results
        for &result in &results[0..2] {
            assert!(result >= 1 && result <= 6);
        }
        // Last should be the constant
        assert_eq!(results[2], 3);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let mut parser = DiceParser::new("2d6 * 3");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 1); // Single result (the product)
                                      // Result should be between (1+1)*3=6 and (6+6)*3=36
        assert!(results[0] >= 6 && results[0] <= 36);
        // Should be divisible by 3
        assert_eq!(results[0] % 3, 0);
    }

    #[test]
    fn test_evaluate_division() {
        let mut parser = DiceParser::new("6d6 / 2");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 1); // Single result (the quotient)
                                      // Result should be between (6*1)/2=3 and (6*6)/2=18
        assert!(results[0] >= 3 && results[0] <= 18);
    }

    #[test]
    fn test_evaluate_success_counting() {
        let mut parser = DiceParser::new("5d10>6");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 1);
        let success_count = results[0];
        assert!(success_count >= 0 && success_count <= 5);
    }

    #[test]
    fn test_evaluate_exploding_dice() {
        let mut parser = DiceParser::new("2d6!");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        // Should have at least 2 dice (the original rolls)
        assert!(results.len() >= 2);
        for &result in &results {
            assert!(result >= 1 && result <= 6);
        }
    }

    #[test]
    fn test_evaluate_constant() {
        let mut parser = DiceParser::new("42");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results, vec![42]);
    }

    #[test]
    fn test_evaluate_complex_expression() {
        let mut parser = DiceParser::new("4d6K3 + 2d8 - 1");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        // Should have: 3 kept dice + 2 dice + 1 constant (negated) = 6 elements
        assert_eq!(results.len(), 6);

        // Last element should be -1 (the negated constant)
        assert_eq!(results[5], -1);
    }

    #[test]
    fn test_evaluate_multiplication_with_dice() {
        let mut parser = DiceParser::new("1d6 * 2d4");
        let expr = parser.parse().unwrap();
        let results = evaluate(&expr).unwrap();

        assert_eq!(results.len(), 1); // Single result (the product)
                                      // Result should be between 1*2=2 and 6*8=48
        assert!(results[0] >= 2 && results[0] <= 48);
    }
}
