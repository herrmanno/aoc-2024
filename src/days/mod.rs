//! Collection of all puzzles and utilities to run a day's puzzle

pub mod day01;

use aoc_runner::{Analyzer, Day};
use derive_aoc_runner::{Analyzer, AoC};

#[derive(Analyzer, AoC)]
pub struct Days(day01::Day01);
