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

//! Recursive descent parser for dice notation
//!
//! This module implements a proper parser with clear grammar rules and precedence,
//! replacing the brittle string-manipulation approach.

#![allow(clippy::cast_sign_loss)] // All casts are validated to be positive

use crate::DiceError;

/// Abstract Syntax Tree for dice expressions
#[derive(Debug, Clone, PartialEq)]
pub enum DiceExpression {
    /// Simple dice roll (e.g., "3d6")
    Simple { count: usize, sides: i32 },

    /// Keep highest dice (e.g., "4d6K3")
    KeepHighest {
        count: usize,
        sides: i32,
        keep: usize,
    },

    /// Keep lowest dice (e.g., "4d6k2")
    KeepLowest {
        count: usize,
        sides: i32,
        keep: usize,
    },

    /// Drop highest dice (e.g., "5d6X2")
    DropHighest {
        count: usize,
        sides: i32,
        drop: usize,
    },

    /// Exploding dice (e.g., "3d6!", "2d10!>8")
    Exploding {
        count: usize,
        sides: i32,
        condition: ExplodeCondition,
    },

    /// Success counting (e.g., "5d10>6")
    SuccessCounting {
        count: usize,
        sides: i32,
        target: i32,
        comparison: Comparison,
    },

    /// Success/failure counting (e.g., "10d10>6f<3")
    SuccessFailure {
        count: usize,
        sides: i32,
        success_target: i32,
        success_comparison: Comparison,
        failure_target: i32,
        failure_comparison: Comparison,
    },

    /// Rerolling dice (e.g., "4d6r1", "3d8R<3")
    Rerolling {
        count: usize,
        sides: i32,
        condition: RerollCondition,
        reroll_type: RerollType,
    },

    /// Repeat rolls (e.g., "3d6x6", "2d20x3")
    Repeat {
        expression: Box<DiceExpression>,
        times: usize,
    },

    /// Binary arithmetic operation (e.g., "2d6 + 3", "4d8 * 2d4")
    Binary {
        left: Box<DiceExpression>,
        op: BinaryOp,
        right: Box<DiceExpression>,
    },

    /// Constant value (e.g., "5" in "2d6 + 5")
    Constant(i32),
}

/// Binary arithmetic operators
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDivide,
}

/// Exploding dice conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplodeCondition {
    /// Explode on maximum value (e.g., "3d6!")
    Max,
    /// Explode on specific value (e.g., "3d6!5")
    Value(i32),
    /// Explode on comparison (e.g., "3d6!>4", "2d10!<3")
    Comparison(Comparison, i32),
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    GreaterThan,
    LessThan,
}

/// Reroll conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RerollCondition {
    /// Reroll specific value (e.g., "4d6r1")
    Value(i32),
    /// Reroll on comparison (e.g., "4d6r>4", "3d8r<3")
    Comparison(Comparison, i32),
}

/// Reroll types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RerollType {
    /// Reroll once (e.g., "4d6r1")
    Once,
    /// Reroll continuously until condition not met (e.g., "4d6R1")
    Continuous,
}

/// Recursive descent parser for dice notation
pub struct DiceParser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> DiceParser<'a> {
    /// Create a new parser for the given input
    pub const fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    /// Parse the input into a dice expression
    pub fn parse(&mut self) -> Result<DiceExpression, DiceError> {
        let expr = self.parse_expression()?;
        self.skip_whitespace();

        if !self.is_at_end() {
            return Err(DiceError::InvalidNotation {
                input: self.input.to_string(),
                reason: format!("Unexpected characters after position {}", self.position),
            });
        }

        Ok(expr)
    }

    /// Grammar: expression = term (('+' | '-') term)*
    fn parse_expression(&mut self) -> Result<DiceExpression, DiceError> {
        let mut left = self.parse_term()?;

        while let Some(op) = self.peek_additive_op() {
            self.consume_additive_op();
            let right = self.parse_term()?;
            left = DiceExpression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Grammar: term = factor (('*' | '/' | '//') factor)*
    fn parse_term(&mut self) -> Result<DiceExpression, DiceError> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.peek_multiplicative_op() {
            self.consume_multiplicative_op();
            let right = self.parse_factor()?;
            left = DiceExpression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Grammar: factor = dice | constant | '(' expression ')'
    fn parse_factor(&mut self) -> Result<DiceExpression, DiceError> {
        self.skip_whitespace();

        if self.peek_char() == Some('(') {
            self.advance(); // consume '('
            let expr = self.parse_expression()?;
            self.skip_whitespace();
            if self.peek_char() != Some(')') {
                return Err(DiceError::InvalidNotation {
                    input: self.input.to_string(),
                    reason: "Expected closing parenthesis ')'".to_string(),
                });
            }
            self.advance(); // consume ')'
            Ok(expr)
        } else if self.is_dice_notation() {
            self.parse_dice()
        } else {
            self.parse_constant()
        }
    }

    /// Parse dice notation with optional modifiers
    fn parse_dice(&mut self) -> Result<DiceExpression, DiceError> {
        self.skip_whitespace();

        // Parse optional count (defaults to 1)
        let count = if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            self.parse_number()? as usize
        } else {
            1
        };

        // Validate dice count
        if count == 0 {
            return Err(DiceError::InvalidDiceCount {
                count: count.to_string(),
            });
        }
        if count > 25 {
            return Err(DiceError::TooManyDice { count, max: 25 });
        }

        self.skip_whitespace();

        // Expect 'd'
        if self.peek_char() != Some('d') {
            return Err(DiceError::InvalidNotation {
                input: self.input.to_string(),
                reason: "Expected 'd' in dice notation".to_string(),
            });
        }
        self.advance(); // consume 'd'

        self.skip_whitespace();

        // Parse sides
        let sides = self.parse_number()?;
        if sides <= 0 {
            return Err(DiceError::InvalidDieSize {
                size: sides.to_string(),
            });
        }

        // Check for modifiers
        let mut expr = self.parse_dice_modifiers(count, sides)?;

        // Check for repeat modifier (x followed by number)
        self.skip_whitespace();
        if self.peek_char() == Some('x') && self.position + 1 < self.input.len() {
            // Look ahead to see if there's a digit after potential whitespace
            let mut lookahead_pos = self.position + 1;
            while lookahead_pos < self.input.len()
                && self
                    .input
                    .chars()
                    .nth(lookahead_pos)
                    .unwrap()
                    .is_whitespace()
            {
                lookahead_pos += 1;
            }
            let next_char = if lookahead_pos < self.input.len() {
                self.input.chars().nth(lookahead_pos)
            } else {
                None
            };

            if next_char.is_some_and(|c| c.is_ascii_digit()) {
                self.advance(); // consume 'x'
                self.skip_whitespace();
                let times = self.parse_number()? as usize;
                if times == 0 {
                    return Err(DiceError::InvalidNotation {
                        input: self.input.to_string(),
                        reason: "Repeat count must be positive".to_string(),
                    });
                }
                expr = DiceExpression::Repeat {
                    expression: Box::new(expr),
                    times,
                };
            }
        }

        Ok(expr)
    }

    /// Parse dice modifiers (keep, drop, exploding, success counting, rerolling)
    #[allow(clippy::too_many_lines)] // Complex but well-structured function
    fn parse_dice_modifiers(
        &mut self,
        count: usize,
        sides: i32,
    ) -> Result<DiceExpression, DiceError> {
        self.skip_whitespace();

        match self.peek_char() {
            Some('K') => {
                self.advance(); // consume 'K'
                self.skip_whitespace();
                let keep = if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.parse_number()? as usize
                } else {
                    1
                };
                if keep > count {
                    return Err(DiceError::InvalidNotation {
                        input: self.input.to_string(),
                        reason: "Cannot keep more dice than rolled".to_string(),
                    });
                }
                Ok(DiceExpression::KeepHighest { count, sides, keep })
            }
            Some('k') => {
                self.advance(); // consume 'k'
                self.skip_whitespace();
                let keep = if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.parse_number()? as usize
                } else {
                    1
                };
                if keep > count {
                    return Err(DiceError::InvalidNotation {
                        input: self.input.to_string(),
                        reason: "Cannot keep more dice than rolled".to_string(),
                    });
                }
                Ok(DiceExpression::KeepLowest { count, sides, keep })
            }
            Some('X') => {
                self.advance(); // consume 'X'
                self.skip_whitespace();
                let drop = if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.parse_number()? as usize
                } else {
                    1
                };
                if drop >= count {
                    return Err(DiceError::InvalidNotation {
                        input: self.input.to_string(),
                        reason: "Cannot drop all dice".to_string(),
                    });
                }
                Ok(DiceExpression::DropHighest { count, sides, drop })
            }
            Some('!') => {
                self.advance(); // consume '!'
                self.skip_whitespace();
                let condition = self.parse_explode_condition()?;
                Ok(DiceExpression::Exploding {
                    count,
                    sides,
                    condition,
                })
            }
            Some('>' | '<') => {
                let comparison = if self.peek_char() == Some('>') {
                    self.advance();
                    Comparison::GreaterThan
                } else {
                    self.advance();
                    Comparison::LessThan
                };
                self.skip_whitespace();
                let target = self.parse_number()?;

                self.skip_whitespace();
                // Check for failure condition
                if self.peek_char() == Some('f') {
                    self.advance(); // consume 'f'
                    self.skip_whitespace();
                    let failure_comparison = if self.peek_char() == Some('>') {
                        self.advance();
                        Comparison::GreaterThan
                    } else if self.peek_char() == Some('<') {
                        self.advance();
                        Comparison::LessThan
                    } else {
                        return Err(DiceError::InvalidNotation {
                            input: self.input.to_string(),
                            reason: "Expected '>' or '<' after 'f'".to_string(),
                        });
                    };
                    self.skip_whitespace();
                    let failure_target = self.parse_number()?;

                    // Validate that success and failure conditions don't conflict
                    match (comparison, failure_comparison) {
                        (Comparison::GreaterThan, Comparison::GreaterThan)
                        | (Comparison::LessThan, Comparison::LessThan) => {
                            return Err(DiceError::InvalidNotation {
                                input: self.input.to_string(),
                                reason: "Success and failure conditions cannot both be greater than or both less than".to_string(),
                            });
                        }
                        _ => {}
                    }

                    Ok(DiceExpression::SuccessFailure {
                        count,
                        sides,
                        success_target: target,
                        success_comparison: comparison,
                        failure_target,
                        failure_comparison,
                    })
                } else {
                    Ok(DiceExpression::SuccessCounting {
                        count,
                        sides,
                        target,
                        comparison,
                    })
                }
            }
            Some('r' | 'R') => {
                let reroll_type = if self.peek_char() == Some('r') {
                    self.advance();
                    RerollType::Once
                } else {
                    self.advance();
                    RerollType::Continuous
                };
                self.skip_whitespace();
                let condition = self.parse_reroll_condition()?;
                Ok(DiceExpression::Rerolling {
                    count,
                    sides,
                    condition,
                    reroll_type,
                })
            }
            _ => Ok(DiceExpression::Simple { count, sides }),
        }
    }

    /// Parse exploding dice condition
    fn parse_explode_condition(&mut self) -> Result<ExplodeCondition, DiceError> {
        match self.peek_char() {
            None | Some(' ' | '+' | '-' | '*' | '/' | ')') => {
                // Simple exploding on max value
                Ok(ExplodeCondition::Max)
            }
            Some('>') => {
                self.advance(); // consume '>'
                let target = self.parse_number()?;
                Ok(ExplodeCondition::Comparison(
                    Comparison::GreaterThan,
                    target,
                ))
            }
            Some('<') => {
                self.advance(); // consume '<'
                let target = self.parse_number()?;
                Ok(ExplodeCondition::Comparison(Comparison::LessThan, target))
            }
            Some(c) if c.is_ascii_digit() => {
                let target = self.parse_number()?;
                Ok(ExplodeCondition::Value(target))
            }
            Some(c) => Err(DiceError::InvalidNotation {
                input: self.input.to_string(),
                reason: format!("Unexpected character '{c}' in exploding condition"),
            }),
        }
    }

    /// Parse reroll condition
    fn parse_reroll_condition(&mut self) -> Result<RerollCondition, DiceError> {
        match self.peek_char() {
            Some('>') => {
                self.advance(); // consume '>'
                let target = self.parse_number()?;
                Ok(RerollCondition::Comparison(Comparison::GreaterThan, target))
            }
            Some('<') => {
                self.advance(); // consume '<'
                let target = self.parse_number()?;
                Ok(RerollCondition::Comparison(Comparison::LessThan, target))
            }
            Some(c) if c.is_ascii_digit() => {
                let target = self.parse_number()?;
                Ok(RerollCondition::Value(target))
            }
            _ => Err(DiceError::InvalidNotation {
                input: self.input.to_string(),
                reason: "Expected reroll condition after 'r' or 'R'".to_string(),
            }),
        }
    }

    /// Parse a constant number
    fn parse_constant(&mut self) -> Result<DiceExpression, DiceError> {
        let number = self.parse_number()?;
        Ok(DiceExpression::Constant(number))
    }

    /// Parse a number from the current position
    fn parse_number(&mut self) -> Result<i32, DiceError> {
        self.skip_whitespace();
        let start = self.position;

        // Handle negative numbers
        if self.peek_char() == Some('-') {
            self.advance();
        }

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if start == self.position
            || (self.position == start + 1 && self.input.chars().nth(start) == Some('-'))
        {
            return Err(DiceError::InvalidNotation {
                input: self.input.to_string(),
                reason: "Expected number".to_string(),
            });
        }

        let number_str = &self.input[start..self.position];
        number_str.parse().map_err(|_| DiceError::InvalidNotation {
            input: self.input.to_string(),
            reason: format!("Invalid number: '{number_str}'"),
        })
    }

    /// Check if current position looks like dice notation
    fn is_dice_notation(&self) -> bool {
        let mut pos = self.position;

        // Skip whitespace
        while pos < self.input.len() && self.input.chars().nth(pos).unwrap().is_whitespace() {
            pos += 1;
        }

        // Check for optional number followed by 'd'
        while pos < self.input.len() && self.input.chars().nth(pos).unwrap().is_ascii_digit() {
            pos += 1;
        }

        // Skip whitespace after number
        while pos < self.input.len() && self.input.chars().nth(pos).unwrap().is_whitespace() {
            pos += 1;
        }

        pos < self.input.len() && self.input.chars().nth(pos) == Some('d')
    }

    /// Peek at additive operators
    fn peek_additive_op(&mut self) -> Option<BinaryOp> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('+') => Some(BinaryOp::Add),
            Some('-') => {
                // Make sure it's not a negative number
                if self.position + 1 < self.input.len() {
                    let next_char = self.input.chars().nth(self.position + 1);
                    if next_char.is_some_and(|c| c.is_ascii_digit()) {
                        // Check if there's whitespace or alphanumeric before the minus (indicating subtraction)
                        if self.position > 0 {
                            let prev_char = self.input.chars().nth(self.position - 1);
                            if prev_char
                                .is_some_and(|c| c.is_whitespace() || c.is_ascii_alphanumeric())
                            {
                                return Some(BinaryOp::Subtract);
                            }
                        }
                    } else {
                        return Some(BinaryOp::Subtract);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Peek at multiplicative operators
    fn peek_multiplicative_op(&mut self) -> Option<BinaryOp> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('*') => Some(BinaryOp::Multiply),
            Some('/') => {
                if self.position + 1 < self.input.len()
                    && self.input.chars().nth(self.position + 1) == Some('/')
                {
                    Some(BinaryOp::FloorDivide)
                } else {
                    Some(BinaryOp::Divide)
                }
            }
            _ => None,
        }
    }

    /// Consume additive operator
    fn consume_additive_op(&mut self) {
        self.skip_whitespace();
        if matches!(self.peek_char(), Some('+' | '-')) {
            self.advance();
        }
    }

    /// Consume multiplicative operator
    fn consume_multiplicative_op(&mut self) {
        self.skip_whitespace();
        match self.peek_char() {
            Some('*') => {
                self.advance();
            }
            Some('/') => {
                self.advance();
                if self.peek_char() == Some('/') {
                    self.advance(); // consume second '/'
                }
            }
            _ => {}
        }
    }

    /// Peek at the current character without advancing
    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// Advance to the next character
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek_char();
        if ch.is_some() {
            self.position += 1;
        }
        ch
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Check if we're at the end of input
    const fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_dice() {
        let mut parser = DiceParser::new("2d6");
        let expr = parser.parse().unwrap();
        assert_eq!(expr, DiceExpression::Simple { count: 2, sides: 6 });
    }

    #[test]
    fn test_parse_implicit_single_die() {
        let mut parser = DiceParser::new("d20");
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            DiceExpression::Simple {
                count: 1,
                sides: 20
            }
        );
    }

    #[test]
    fn test_parse_keep_highest() {
        let mut parser = DiceParser::new("4d6K3");
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            DiceExpression::KeepHighest {
                count: 4,
                sides: 6,
                keep: 3
            }
        );
    }

    #[test]
    fn test_parse_arithmetic() {
        let mut parser = DiceParser::new("2d6 + 3");
        let expr = parser.parse().unwrap();
        match expr {
            DiceExpression::Binary { left, op, right } => {
                assert_eq!(*left, DiceExpression::Simple { count: 2, sides: 6 });
                assert_eq!(op, BinaryOp::Add);
                assert_eq!(*right, DiceExpression::Constant(3));
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_arithmetic_no_spaces() {
        let mut parser = DiceParser::new("2d6+3");
        let expr = parser.parse().unwrap();
        match expr {
            DiceExpression::Binary { left, op, right } => {
                assert_eq!(*left, DiceExpression::Simple { count: 2, sides: 6 });
                assert_eq!(op, BinaryOp::Add);
                assert_eq!(*right, DiceExpression::Constant(3));
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_dice_to_dice() {
        let mut parser = DiceParser::new("2d12 + 1d6");
        let expr = parser.parse().unwrap();
        match expr {
            DiceExpression::Binary { left, op, right } => {
                assert_eq!(
                    *left,
                    DiceExpression::Simple {
                        count: 2,
                        sides: 12
                    }
                );
                assert_eq!(op, BinaryOp::Add);
                assert_eq!(*right, DiceExpression::Simple { count: 1, sides: 6 });
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut parser = DiceParser::new("4d6K3 + 2d8 - 1");
        let expr = parser.parse().unwrap();
        // This should parse as: (4d6K3 + 2d8) - 1
        match expr {
            DiceExpression::Binary {
                left,
                op: BinaryOp::Subtract,
                right,
            } => {
                assert_eq!(*right, DiceExpression::Constant(1));
                match *left {
                    DiceExpression::Binary {
                        left: inner_left,
                        op: BinaryOp::Add,
                        right: inner_right,
                    } => {
                        assert_eq!(
                            *inner_left,
                            DiceExpression::KeepHighest {
                                count: 4,
                                sides: 6,
                                keep: 3
                            }
                        );
                        assert_eq!(*inner_right, DiceExpression::Simple { count: 2, sides: 8 });
                    }
                    _ => panic!("Expected nested binary expression"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_exploding_dice() {
        let mut parser = DiceParser::new("3d6!");
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            DiceExpression::Exploding {
                count: 3,
                sides: 6,
                condition: ExplodeCondition::Max
            }
        );
    }

    #[test]
    fn test_parse_success_counting() {
        let mut parser = DiceParser::new("5d10>6");
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            DiceExpression::SuccessCounting {
                count: 5,
                sides: 10,
                target: 6,
                comparison: Comparison::GreaterThan
            }
        );
    }

    #[test]
    fn test_parse_repeat_rolls() {
        let mut parser = DiceParser::new("3d6x4");
        let expr = parser.parse().unwrap();
        match expr {
            DiceExpression::Repeat { expression, times } => {
                assert_eq!(times, 4);
                match *expression {
                    DiceExpression::Simple { count, sides } => {
                        assert_eq!(count, 3);
                        assert_eq!(sides, 6);
                    }
                    _ => panic!("Expected simple dice expression inside repeat"),
                }
            }
            _ => panic!("Expected repeat expression"),
        }
    }

    #[test]
    fn test_parse_error_too_many_dice() {
        let mut parser = DiceParser::new("30d6");
        let result = parser.parse();
        assert!(matches!(
            result,
            Err(DiceError::TooManyDice { count: 30, max: 25 })
        ));
    }

    #[test]
    fn test_parse_error_invalid_notation() {
        let mut parser = DiceParser::new("invalid");
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_spaces_in_dice_notation() {
        // Test spaces around 'd'
        let mut parser = DiceParser::new("2 d 6");
        let expr = parser.parse().unwrap();
        assert_eq!(expr, DiceExpression::Simple { count: 2, sides: 6 });

        // Test spaces in keep notation
        let mut parser = DiceParser::new("4 d 6 K 3");
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            DiceExpression::KeepHighest {
                count: 4,
                sides: 6,
                keep: 3
            }
        );

        // Test spaces in arithmetic
        let mut parser = DiceParser::new("2 d 6 + 3");
        let expr = parser.parse().unwrap();
        match expr {
            DiceExpression::Binary { left, op, right } => {
                assert_eq!(*left, DiceExpression::Simple { count: 2, sides: 6 });
                assert_eq!(op, BinaryOp::Add);
                assert_eq!(*right, DiceExpression::Constant(3));
            }
            _ => panic!("Expected binary expression"),
        }

        // Test spaces in implicit single die
        let mut parser = DiceParser::new("d 20");
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            DiceExpression::Simple {
                count: 1,
                sides: 20
            }
        );
    }
}
