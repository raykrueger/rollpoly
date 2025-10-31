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

#![allow(clippy::multiple_crate_versions)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rollpoly")]
#[command(about = "A comprehensive dice rolling application for tabletop gaming", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Dice notation to roll (if no subcommand is provided)
    #[arg(help = "Dice notation like '4d10 + 17', '2d20 - 3', etc.")]
    dice: Option<String>,

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
    /// Start interactive shell for continuous dice rolling
    Shell,
    /// Roll Daggerheart Duality dice (2d12 with Hope/Fear mechanics)
    #[command(name = "dh")]
    Dh,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Roll { notation, repeat }) => {
            roll_dice(&notation, repeat)
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
        Some(Commands::Shell) => {
            run_interactive_shell();
        }
        Some(Commands::Dh) => {
            roll_daggerheart_duality()
                .with_context(|| "Failed to roll Daggerheart duality dice")?;
        }
        None => {
            // Handle direct dice notation or show help
            if let Some(dice_notation) = cli.dice {
                roll_dice(&dice_notation, cli.repeat).with_context(|| {
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

fn roll_dice(notation: &str, repeat: usize) -> Result<()> {
    if repeat > 1 {
        println!("Rolling '{notation}' {repeat} time(s)");
    }

    for i in 1..=repeat {
        let results = rollpoly::roll(notation)
            .with_context(|| format!("Invalid dice notation: '{notation}'"))?;

        let sum = results.iter().sum::<i32>();

        if repeat > 1 {
            println!("Roll {i}: You rolled a {sum} with {notation} using dice {results:?}");
        } else {
            println!("You rolled a {sum} with {notation} using dice {results:?}");
        }
    }

    Ok(())
}

fn roll_daggerheart_duality() -> Result<()> {
    let results = rollpoly::roll("2d12")
        .with_context(|| "Failed to roll 2d12 for Daggerheart duality dice")?;

    let hope_die = results[0]; // First die represents Hope
    let fear_die = results[1]; // Second die represents Fear
    let total = hope_die + fear_die;

    let result_type = match hope_die.cmp(&fear_die) {
        std::cmp::Ordering::Equal => format!("🎯 Rolled {total} CRITICAL!"),
        std::cmp::Ordering::Greater => format!("✨ Rolled {total} with Hope"),
        std::cmp::Ordering::Less => format!("😰 Rolled {total} with Fear"),
    };

    println!("{result_type} [Hope: {hope_die}, Fear: {fear_die}]");

    Ok(())
}

fn show_examples() {
    println!("Rollpoly - Dice Notation Examples");
    println!("=================================");
    println!();
    println!("Basic dice rolls:");
    println!("  rollpoly 1d6        # Roll one 6-sided die");
    println!("  rollpoly 4d10       # Roll four 10-sided dice");
    println!("  rollpoly d20        # Roll one 20-sided die (implicit count)");
    println!();
    println!("Arithmetic operations:");
    println!("  rollpoly '3d6 + 5'  # Roll 3d6 and add 5");
    println!("  rollpoly '2d20 - 3' # Roll 2d20 and subtract 3");
    println!("  rollpoly '1d4 * 2'  # Roll 1d4 and multiply by 2");
    println!("  rollpoly '5d6 / 3'  # Roll 5d6 and divide by 3");
    println!("  rollpoly '4d8 // 2' # Roll 4d8 and floor divide by 2");
    println!();
    println!("Dice-to-dice operations:");
    println!("  rollpoly '2d12 + 1d6' # Daggerheart with Advantage");
    println!("  rollpoly '2d12 - 1d6' # Daggerheart with Disadvantage");
    println!("  rollpoly '3d6 + 2d4'  # Multiple dice pools combined");
    println!("  rollpoly '4d6K3 + 1d4' # Keep highest 3 of 4d6, add 1d4");
    println!();
    println!("Game-specific commands:");
    println!("  rollpoly dh             # Daggerheart Duality dice (2d12 Hope/Fear)");
    println!();
    println!("Keep highest (K) and keep lowest (k):");
    println!("  rollpoly 4d10K      # Roll 4d10 and keep only the highest");
    println!("  rollpoly 7d12K3     # Roll 7d12 and keep the highest 3");
    println!("  rollpoly 3d6k       # Roll 3d6 and keep only the lowest");
    println!("  rollpoly 5d6k3      # Roll 5d6 and keep the lowest 3");
    println!("  rollpoly 2d20K      # Advantage roll (D&D 5e)");
    println!("  rollpoly 2d20k      # Disadvantage roll (D&D 5e)");
    println!("  rollpoly '4d6K3 + 2' # Keep highest 3 of 4d6, then add 2");
    println!();
    println!("Drop highest (X) and drop lowest (x):");
    println!("  rollpoly 6d8X       # Roll 6d8 and drop the highest");
    println!("  rollpoly 5d10X3     # Roll 5d10 and drop the highest 3");
    println!("  rollpoly 6d8x       # Roll 6d8 and drop the lowest");
    println!("  rollpoly 5d10x3     # Roll 5d10 and drop the lowest 3");
    println!("  rollpoly 4d6x       # Character generation (drop lowest)");
    println!("  rollpoly '6d6X2 + 5' # Drop highest 2 of 6d6, then add 5");
    println!();
    println!("Count successes (> or <):");
    println!("  rollpoly '5d10>7'   # Count rolls above 7 (World of Darkness)");
    println!("  rollpoly '12d6>4'   # Count rolls above 4 (Shadowrun)");
    println!("  rollpoly '8d6<3'    # Count rolls below 3");
    println!("  rollpoly 'd20>15'   # Single die success check");
    println!();
    println!("Count successes with failures (f):");
    println!("  rollpoly '10d10>6f<3' # Successes >6, failures <3");
    println!("  rollpoly '4d20<5f>19' # Successes <5, failures >19");
    println!("  rollpoly '6d6>4f<2'   # Advanced dice pool mechanics");
    println!();
    println!("Exploding dice (!):");
    println!("  rollpoly '2d6!'       # Explode on max (6s)");
    println!("  rollpoly '4d6!6'      # Explode on 6s (Shadowrun)");
    println!("  rollpoly '3d10!10'    # Explode on 10s");
    println!("  rollpoly 'd20!>15'    # Explode on 16+ (Rule of 6 variant)");
    println!("  rollpoly '2d12!<3'    # Explode on 1s and 2s");
    println!();
    println!("Rerolling Dice (r/R):");
    println!("  rollpoly '4d6r1'      # Reroll any 1s once (Great Weapon Fighting)");
    println!("  rollpoly '2d6r<3'     # Reroll anything under 3 once");
    println!("  rollpoly '3d8R1'      # Keep rerolling 1s until no 1s remain");
    println!("  rollpoly '4d10R<3'    # Keep rerolling anything under 3");
    println!();
    println!("Repeat Rolls (x):");
    println!("  rollpoly '3d6x6'      # Roll 3d6 six times, return 6 roll results");
    println!("  rollpoly '2d20x3'     # Roll 2d20 three times, return 3 roll results");
    println!("  rollpoly '1d4x10'     # Roll 1d4 ten times, return 10 roll results");
    println!("  rollpoly '4d6K3x4'    # Roll 4d6K3 four times, return 4 roll results");
    println!();
    println!("Using subcommands:");
    println!("  rollpoly roll '2d6 + 3' -n 5    # Roll 5 times");
    println!("  rollpoly roll '4d6K3' -n 3      # Roll multiple times");
    println!("  rollpoly stats 3d6 -n 10000     # Statistical analysis");
    println!("  rollpoly stats 2d6 -n 100 -v    # Stats with verbose distribution");
    println!();
    println!("Options:");
    println!("  -n, --repeat N    # Repeat the roll N times");
}

fn run_statistics(notation: &str, rolls: usize, verbose: bool) -> Result<()> {
    if verbose {
        println!("Running statistical analysis for '{notation}' over {rolls} rolls");
    }

    let mut sums = Vec::with_capacity(rolls);

    for _ in 0..rolls {
        let roll_result = rollpoly::roll(notation)
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
    println!("Rollpoly - Advanced Dice Rolling");
    println!("================================");
    println!();
    println!("Usage:");
    println!("  rollpoly <DICE_NOTATION>     # Roll dice directly");
    println!("  rollpoly roll <NOTATION>     # Roll dice using subcommand");
    println!("  rollpoly dh                  # Roll Daggerheart Duality dice (2d12)");
    println!("  rollpoly shell               # Start interactive shell");
    println!("  rollpoly examples            # Show notation examples");
    println!("  rollpoly stats <NOTATION>    # Run statistical analysis");
    println!("  rollpoly --help              # Show detailed help");
    println!();
    println!("Examples:");
    println!("  rollpoly 2d6");
    println!("  rollpoly '3d6 + 5'");
    println!("  rollpoly dh                  # Hope/Fear mechanics with criticals");
    println!("  rollpoly shell               # Interactive mode with history");
    println!("  rollpoly roll 4d10 -n 5");
}

fn run_interactive_shell() {
    use rustyline::error::ReadlineError;
    use rustyline::{DefaultEditor, Result as RustylineResult};

    println!("Rollpoly Interactive Shell");
    println!("==========================");
    println!("Enter dice notation to roll, or type 'help' for commands.");
    println!("Type 'exit' or 'quit' to leave the shell.");
    println!("Use up/down arrows to navigate command history.");
    println!();

    // Create readline editor with history
    let rl: RustylineResult<DefaultEditor> = DefaultEditor::new();
    let mut editor = match rl {
        Ok(editor) => editor,
        Err(e) => {
            println!("❌ Failed to initialize readline: {e}");
            println!("💡 Falling back to basic input mode...");
            run_basic_shell();
            return;
        }
    };

    // Try to load history from file
    let history_file = dirs::home_dir().map(|mut path| {
        path.push(".rollpoly_history");
        path
    });

    if let Some(ref history_path) = history_file {
        let _ = editor.load_history(history_path);
    }

    loop {
        // Read input with readline support
        let readline = editor.readline("rollpoly> ");
        match readline {
            Ok(line) => {
                let input = line.trim();

                // Handle empty input
                if input.is_empty() {
                    continue;
                }

                // Add to history (rustyline handles duplicates automatically)
                let _ = editor.add_history_entry(input);

                // Handle shell commands
                match input.to_lowercase().as_str() {
                    "exit" | "quit" | "q" => {
                        println!("Thanks for rolling! Goodbye!");
                        break;
                    }
                    "help" | "h" => {
                        show_shell_help();
                        continue;
                    }
                    "examples" => {
                        show_examples();
                        continue;
                    }
                    "clear" | "cls" => {
                        // Clear screen (works on most terminals)
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    "history" => {
                        show_command_history(&editor);
                        continue;
                    }
                    "dh" | "daggerheart" => {
                        match roll_daggerheart_duality() {
                            Ok(()) => {}
                            Err(e) => println!("❌ Error rolling Daggerheart duality dice: {e}"),
                        }
                        continue;
                    }
                    _ => {}
                }

                // Try to parse and roll dice
                match rollpoly::roll(input) {
                    Ok(results) => {
                        let sum = results.iter().sum::<i32>();
                        let response = generate_roll_response(sum, &results);
                        println!("{response}");
                    }
                    Err(e) => {
                        println!("❌ Error: {e}");
                        println!("Type 'help' for available commands or 'examples' for dice notation examples.");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("Thanks for rolling! Goodbye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!();
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("❌ Error reading input: {err}");
                break;
            }
        }
    }

    // Save history to file
    if let Some(ref history_path) = history_file {
        let _ = editor.save_history(history_path);
    }
}

// Fallback function for basic shell without readline
fn run_basic_shell() {
    use std::io::{self, Write};

    loop {
        // Print prompt
        print!("rollpoly> ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!();
                println!("Goodbye!");
                break;
            }
            Ok(_) => {
                let input = input.trim();

                // Handle empty input
                if input.is_empty() {
                    continue;
                }

                // Handle shell commands
                match input.to_lowercase().as_str() {
                    "exit" | "quit" | "q" => {
                        println!("Thanks for rolling! Goodbye!");
                        break;
                    }
                    "help" | "h" => {
                        show_shell_help();
                        continue;
                    }
                    "examples" => {
                        show_examples();
                        continue;
                    }
                    "clear" | "cls" => {
                        // Clear screen (works on most terminals)
                        print!("\x1B[2J\x1B[1;1H");
                        io::stdout().flush().unwrap();
                        println!("Screen cleared!");
                        continue;
                    }
                    _ => {}
                }

                // Try to parse and roll dice
                match rollpoly::roll(input) {
                    Ok(results) => {
                        let sum = results.iter().sum::<i32>();
                        let response = generate_roll_response(sum, &results);
                        println!("{response}");
                    }
                    Err(e) => {
                        println!("❌ Error: {e}");
                        println!("Type 'help' for available commands or 'examples' for dice notation examples.");
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {e}");
                break;
            }
        }
    }
}

fn show_command_history(editor: &rustyline::DefaultEditor) {
    use rustyline::history::History;

    println!("Command History:");
    println!("================");

    let history = editor.history();
    if history.is_empty() {
        println!("No commands in history yet.");
        return;
    }

    // Show last 10 commands
    let start = if history.len() > 10 {
        history.len() - 10
    } else {
        0
    };

    for (i, entry) in history.iter().enumerate().skip(start) {
        println!("  {}: {}", i + 1, entry);
    }

    if history.len() > 10 {
        println!("  ... and {} more commands", history.len() - 10);
    }

    println!();
    println!("Use Up/Down arrows to navigate history");
}

fn generate_roll_response(sum: i32, results: &[i32]) -> String {
    // Format the dice results
    let dice_display = format_dice_results(results);

    // Simple, clean format with colon - no grammar issues
    format!("🎲 You rolled: {sum}! {dice_display}")
}

fn format_dice_results(results: &[i32]) -> String {
    if results.len() == 1 {
        format!("{}", results[0])
    } else if results.len() <= 6 {
        // Show individual dice for small rolls - clean and readable
        format!("{results:?}")
    } else {
        // For large rolls, show summary
        let min = results.iter().min().unwrap();
        let max = results.iter().max().unwrap();
        format!("{} dice (range: {min}-{max})", results.len())
    }
}

fn show_shell_help() {
    println!("Interactive Shell Commands");
    println!("==========================");
    println!();
    println!("Dice Rolling:");
    println!("  <dice_notation>   Roll dice using any supported notation");
    println!("  2d6               Roll two 6-sided dice");
    println!("  3d6 + 5           Roll 3d6 and add 5");
    println!("  4d10K3            Roll 4d10 and keep highest 3");
    println!();
    println!("Shell Commands:");
    println!("  help, h           Show this help message");
    println!("  examples          Show dice notation examples");
    println!("  dh                Roll Daggerheart Duality dice (2d12)");
    println!("  history           Show command history");
    println!("  clear, cls        Clear the screen");
    println!("  exit, quit, q     Exit the shell");
    println!();
    println!("Navigation:");
    println!("  Up/Down arrows    Navigate command history");
    println!("  Ctrl+C            Exit the shell");
    println!("  Ctrl+D            Exit the shell");
    println!();
    println!("Tips:");
    println!("  - Use quotes around complex expressions if needed");
    println!("  - Command history is saved between sessions");
    println!("  - All dice notation from the main CLI is supported");
    println!("  - Enjoy the randomized response formats!");
}
