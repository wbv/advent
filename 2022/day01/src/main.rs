#![doc = include_str!("../README.md")]

use std::io::BufRead;

use clap::Parser;
use log::*;

#[derive(Parser, Debug)]
#[command(about = None, long_about = None)]
struct Args {
    /// Path to a file containing the puzzle input
    #[arg()]
    input: clio::Input,
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut args = Args::parse();

    println!("{}", max_sum(args.input.lock()));

    Ok(())
}

fn max_sum<B: BufRead>(input: B) -> usize {
    let fold = input.lines()
        .map(|l| l.unwrap_or(String::from("")).parse::<usize>().unwrap_or(0))
        .fold([0, 0], |acc, x| {
            debug!("acc = {acc:?}");
            if x == 0 {
                debug!("zero found");
                [acc[0].max(acc[1]), 0]
            } else {
                debug!("nonzero acc:");
                [acc[0], acc[1] + x]
            }
        });

    debug!("acc = {fold:?}");

    fold[0]
}
