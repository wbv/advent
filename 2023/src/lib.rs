#![doc = include_str!("../README.md")]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub use clap::{Parser, Subcommand};
pub use log::*;

#[derive(Parser, Debug)]
#[command(about = None, long_about = None)]
pub struct Args {
    /// What to do with the input (solve for part1 or part2)
    #[command(subcommand)]
    pub mode: RunMode,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RunMode {
    /// Compute solution according to Part 1
    Part1 {
        /// Path to a file containing the puzzle input
        #[arg(value_parser)]
        input: clio::Input,
    },
    /// Compute solution according to Part 1
    Part2 {
        /// Path to a file containing the puzzle input
        #[arg(value_parser)]
        input: clio::Input,
    },
}
