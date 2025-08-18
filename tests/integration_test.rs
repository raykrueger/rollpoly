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

//! Integration tests for the Rollpoly library
//!
//! These tests verify the public API functionality and ensure that
//! the library works correctly when used as an external dependency.

use rollpoly::{roll, DiceError};

#[test]
fn test_public_api_basic_dice_rolling() {
    // Test that the public API works for basic dice rolling
    let result = roll("2d6").expect("Valid dice notation should work");
    assert_eq!(result.len(), 2);

    for &die_result in &result {
        assert!(die_result >= 1 && die_result <= 6);
    }
}

#[test]
fn test_public_api_arithmetic_operations() {
    // Test arithmetic operations through public API
    let test_cases = vec![
        ("1d6 + 5", 2),
        ("2d4 - 1", 3),
        ("1d8 * 2", 2),
        ("3d6 / 2", 4),
        ("2d10 // 3", 3),
    ];

    for (notation, expected_len) in test_cases {
        let result = roll(notation).expect("Valid notation should work");
        assert_eq!(
            result.len(),
            expected_len,
            "Failed for notation: {}",
            notation
        );
    }
}

#[test]
fn test_public_api_error_handling() {
    // Test that error handling works through public API
    let invalid_inputs = vec!["invalid", "d", "4d", "abc", ""];

    for invalid_input in invalid_inputs {
        let result = roll(invalid_input);
        assert!(
            result.is_err(),
            "Should return error for: {}",
            invalid_input
        );

        let error = result.unwrap_err();
        // Just check that the error contains meaningful information
        let error_str = error.to_string();
        assert!(
            error_str.contains(invalid_input) || error_str.contains("Empty"),
            "Error should contain meaningful information about the input: {}",
            error_str
        );
    }
}

#[test]
fn test_error_type_implements_required_traits() {
    // Test that DiceError implements the required traits
    let error = roll("invalid").unwrap_err();

    // Test Debug trait
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());

    // Test Display trait
    let display_str = format!("{}", error);
    assert!(display_str.contains("invalid"));
    assert!(display_str.contains("Invalid dice notation"));

    // Test Error trait (std::error::Error)
    let error_trait: &dyn std::error::Error = &error;
    assert!(error_trait.source().is_none()); // Our error doesn't have a source
}

#[test]
fn test_randomness_across_multiple_calls() {
    // Test that the library produces varied results across calls
    let mut results = std::collections::HashSet::new();

    for _ in 0..20 {
        let result = roll("1d20").expect("Valid notation should work");
        results.insert(result[0]);
    }

    // Should have at least some variety in 20 rolls of a d20
    assert!(results.len() > 3, "Expected some randomness in dice rolls");
}

#[test]
fn test_large_dice_counts() {
    // Test that the library properly enforces the dice count limit
    let result = roll("11d6");
    assert!(result.is_err(), "Should reject dice counts over the limit");

    match result.unwrap_err() {
        DiceError::TooManyDice { count, max } => {
            assert_eq!(count, 11);
            assert_eq!(max, 10);
        }
        _ => panic!("Expected TooManyDice error"),
    }

    // Test that exactly the maximum works
    let result = roll("10d6").expect("Should handle exactly the maximum dice count");
    assert_eq!(result.len(), 10);

    for &die_result in &result {
        assert!(die_result >= 1 && die_result <= 6);
    }
}

#[test]
fn test_edge_case_die_sizes() {
    // Test various die sizes work correctly
    let die_sizes = vec![2, 3, 4, 6, 8, 10, 12, 20, 100, 1000];

    for die_size in die_sizes {
        let notation = format!("1d{}", die_size);
        let result = roll(&notation).expect("Valid die size should work");
        assert_eq!(result.len(), 1);
        assert!(result[0] >= 1 && result[0] <= die_size);
    }
}
