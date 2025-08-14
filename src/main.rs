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

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rkdice")]
#[command(about = "A terminal dice rolling application", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Dice notation to roll (if no subcommand is provided)
    #[arg(help = "Dice notation like '4d10 + 17', '2d20 - 3', etc.")]
    dice: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Number of times to repeat the roll
    #[arg(short = 'n', long, default_value = "1")]
    repeat: usize,
}

#[derive(Subcommand)]
enum Commands {
    /// Roll dice using the specified notation
    Roll {
        /// Dice notation to roll
        #[arg(help = "Dice notation like '4d10 + 17', '2d20 - 3', etc.")]
        notation: String,

        /// Number of times to repeat the roll
        #[arg(short = 'n', long, default_value = "1")]
        repeat: usize,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show examples of dice notation
    Examples,
    /// Run statistical analysis on dice rolls
    Stats {
        /// Dice notation to analyze
        #[arg(help = "Dice notation like '3d6', '2d20', etc.")]
        notation: String,

        /// Number of rolls for statistical analysis
        #[arg(short = 'n', long, default_value = "1000")]
        rolls: usize,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Roll {
            notation,
            repeat,
            verbose,
        }) => {
            roll_dice(&notation, repeat, verbose)
                .with_context(|| format!("Failed to roll dice with notation '{notation}'"))?;
        }
        Some(Commands::Examples) => {
            show_examples();
        }
        Some(Commands::Stats {
            notation,
            rolls,
            verbose,
        }) => {
            run_statistics(&notation, rolls, verbose)
                .with_context(|| format!("Failed to run statistics for notation '{notation}'"))?;
        }
        None => {
            // Handle direct dice notation or show help
            if let Some(dice_notation) = cli.dice {
                roll_dice(&dice_notation, cli.repeat, cli.verbose).with_context(|| {
                    format!("Failed to roll dice with notation '{dice_notation}'")
                })?;
            } else {
                // Show interactive mode or help
                show_interactive_mode();
            }
        }
    }

    Ok(())
}

fn roll_dice(notation: &str, repeat: usize, verbose: bool) -> Result<()> {
    if verbose {
        println!("Rolling '{notation}' {repeat} time(s)");
    }

    for i in 1..=repeat {
        let results = rkdice::roll(notation)
            .with_context(|| format!("Invalid dice notation: '{notation}'"))?;

        let sum = results.iter().sum::<i32>();

        if repeat > 1 {
            if verbose {
                println!(
                    "Roll {i}: You rolled a {sum} with {notation} using dice {results:?}"
                );
            } else {
                println!("Roll {i}: You rolled a {sum} with dice {results:?}");
            }
        } else if verbose {
            println!("You rolled a {sum} with {notation} using dice {results:?}");
        } else {
            println!("You rolled a {sum} with dice {results:?}");
        }
    }

    Ok(())
}

fn show_examples() {
    println!("RKDice - Dice Notation Examples");
    println!("===============================");
    println!();
    println!("Basic dice rolls:");
    println!("  rkdice 1d6        # Roll one 6-sided die");
    println!("  rkdice 4d10       # Roll four 10-sided dice");
    println!("  rkdice d20        # Roll one 20-sided die (implicit count)");
    println!();
    println!("Arithmetic operations:");
    println!("  rkdice '3d6 + 5'  # Roll 3d6 and add 5");
    println!("  rkdice '2d20 - 3' # Roll 2d20 and subtract 3");
    println!("  rkdice '1d4 * 2'  # Roll 1d4 and multiply by 2");
    println!("  rkdice '5d6 / 3'  # Roll 5d6 and divide by 3");
    println!("  rkdice '4d8 // 2' # Roll 4d8 and floor divide by 2");
    println!();
    println!("Using subcommands:");
    println!("  rkdice roll '2d6 + 3' -n 5    # Roll 5 times");
    println!("  rkdice stats 3d6 -n 10000     # Statistical analysis");
    println!();
    println!("Options:");
    println!("  -v, --verbose     # Show detailed output");
    println!("  -n, --repeat N    # Repeat the roll N times");
}

fn run_statistics(notation: &str, rolls: usize, verbose: bool) -> Result<()> {
    if verbose {
        println!("Running statistical analysis for '{notation}' over {rolls} rolls");
    }

    let mut sums = Vec::new();

    for _ in 0..rolls {
        let roll_result = rkdice::roll(notation)
            .with_context(|| format!("Invalid dice notation for statistics: '{notation}'"))?;
        let sum: i32 = roll_result.iter().sum();
        sums.push(sum);
    }

    // Calculate statistics
    let min_sum = *sums.iter().min().unwrap();
    let max_sum = *sums.iter().max().unwrap();
    #[allow(clippy::cast_precision_loss)]
    let average = f64::from(sums.iter().sum::<i32>()) / sums.len() as f64;

    // Calculate median
    let mut sorted_sums = sums.clone();
    sorted_sums.sort_unstable();
    let median = if sorted_sums.len() % 2 == 0 {
        let mid = sorted_sums.len() / 2;
        f64::from(sorted_sums[mid - 1] + sorted_sums[mid]) / 2.0
    } else {
        f64::from(sorted_sums[sorted_sums.len() / 2])
    };

    println!("Statistical Analysis for '{notation}' ({rolls} rolls)");
    println!("==========================================");
    println!("Minimum sum: {min_sum}");
    println!("Maximum sum: {max_sum}");
    println!("Average sum: {average:.2}");
    println!("Median sum:  {median:.1}");

    if verbose {
        // Show distribution
        let mut distribution = std::collections::HashMap::new();
        for sum in &sums {
            *distribution.entry(*sum).or_insert(0) += 1;
        }

        println!("\nDistribution:");
        let mut dist_vec: Vec<_> = distribution.iter().collect();
        dist_vec.sort_by_key(|&(sum, _)| sum);

        for (sum, count) in dist_vec.iter().take(10) {
            #[allow(clippy::cast_precision_loss)]
            let percentage = (f64::from(**count) / rolls as f64) * 100.0;
            println!("  Sum {sum}: {count} times ({percentage:.1}%)");
        }

        if dist_vec.len() > 10 {
            println!("  ... and {} more unique sums", dist_vec.len() - 10);
        }
    }

    Ok(())
}

fn show_interactive_mode() {
    println!("RKDice - Terminal Dice Rolling");
    println!("==============================");
    println!();
    println!("Usage:");
    println!("  rkdice <DICE_NOTATION>     # Roll dice directly");
    println!("  rkdice roll <NOTATION>     # Roll dice using subcommand");
    println!("  rkdice examples            # Show notation examples");
    println!("  rkdice stats <NOTATION>    # Run statistical analysis");
    println!("  rkdice --help              # Show detailed help");
    println!();
    println!("Examples:");
    println!("  rkdice 2d6");
    println!("  rkdice '3d6 + 5'");
    println!("  rkdice roll 4d10 -n 5");
}
