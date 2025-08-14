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

//! Benchmarks for the RKDice library
//!
//! These benchmarks measure the performance of various dice rolling operations
//! to ensure the library performs well under different usage patterns.
//!
//! Run benchmarks with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rkdice::roll;

fn benchmark_simple_dice_rolls(c: &mut Criterion) {
    c.bench_function("roll_1d6", |b| b.iter(|| roll(black_box("1d6"))));

    c.bench_function("roll_4d6", |b| b.iter(|| roll(black_box("4d6"))));

    c.bench_function("roll_10d10", |b| b.iter(|| roll(black_box("10d10"))));

    c.bench_function("roll_100d6", |b| b.iter(|| roll(black_box("100d6"))));
}

fn benchmark_arithmetic_operations(c: &mut Criterion) {
    c.bench_function("roll_with_addition", |b| {
        b.iter(|| roll(black_box("3d6 + 5")))
    });

    c.bench_function("roll_with_subtraction", |b| {
        b.iter(|| roll(black_box("2d20 - 3")))
    });

    c.bench_function("roll_with_multiplication", |b| {
        b.iter(|| roll(black_box("1d8 * 2")))
    });

    c.bench_function("roll_with_division", |b| {
        b.iter(|| roll(black_box("5d6 / 3")))
    });
}

fn benchmark_error_cases(c: &mut Criterion) {
    c.bench_function("invalid_input", |b| b.iter(|| roll(black_box("invalid"))));

    c.bench_function("empty_input", |b| b.iter(|| roll(black_box(""))));

    c.bench_function("malformed_dice", |b| b.iter(|| roll(black_box("d"))));
}

fn benchmark_parsing_variations(c: &mut Criterion) {
    c.bench_function("implicit_single_die", |b| b.iter(|| roll(black_box("d20"))));

    c.bench_function("with_whitespace", |b| {
        b.iter(|| roll(black_box(" 2d6 + 3 ")))
    });

    c.bench_function("large_numbers", |b| b.iter(|| roll(black_box("1d1000"))));
}

criterion_group!(
    benches,
    benchmark_simple_dice_rolls,
    benchmark_arithmetic_operations,
    benchmark_error_cases,
    benchmark_parsing_variations
);
criterion_main!(benches);
