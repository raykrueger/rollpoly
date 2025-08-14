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

//! Basic usage example for the RKDice library
//!
//! This example demonstrates how to use the RKDice library to roll dice
//! with various notations and handle both successful rolls and errors.
//!
//! Run this example with: `cargo run --example basic_usage`

use rkdice::roll;

fn main() {
    println!("RKDice Library - Basic Usage Example");
    println!("====================================");

    // Basic dice rolling examples
    let examples = vec![
        // Simple dice rolls
        "1d6",
        "2d8",
        "4d10",
        // Dice with arithmetic operations
        "3d6 + 5",
        "2d20 - 3",
        "1d4 * 2",
        "5d6 / 2",
        "3d8 // 2",
        // Implicit single die
        "d20",
        "d100",
        // Edge cases and errors
        "invalid input",
        "",
        "d",
    ];

    for example in examples {
        println!("\nRolling: \"{}\"", example);

        match roll(example) {
            Ok(results) => {
                println!("  Results: {:?}", results);
                println!("  Sum: {}", results.iter().sum::<i32>());

                // Show interpretation of results
                if results.len() > 1 {
                    let dice_results = &results[..results.len() - 1];
                    let modifier = results[results.len() - 1];

                    if modifier >= 0 {
                        println!(
                            "  Interpretation: dice {:?} + modifier {}",
                            dice_results, modifier
                        );
                    } else {
                        println!(
                            "  Interpretation: dice {:?} + modifier {}",
                            dice_results, modifier
                        );
                    }
                } else {
                    println!("  Interpretation: single die result");
                }
            }
            Err(error) => {
                println!("  Error: {}", error);
            }
        }
    }

    // Demonstrate statistical analysis
    println!("\n\nStatistical Analysis Example");
    println!("============================");

    let notation = "3d6";
    let num_rolls = 1000;
    let mut results = Vec::new();

    for _ in 0..num_rolls {
        if let Ok(roll_result) = roll(notation) {
            let sum: i32 = roll_result.iter().sum();
            results.push(sum);
        }
    }

    if !results.is_empty() {
        let min = *results.iter().min().unwrap();
        let max = *results.iter().max().unwrap();
        let average = results.iter().sum::<i32>() as f64 / results.len() as f64;

        println!("Rolled {} {} times:", notation, num_rolls);
        println!("  Minimum: {}", min);
        println!("  Maximum: {}", max);
        println!("  Average: {:.2}", average);
        println!("  Expected average for 3d6: 10.5");
    }
}
