use aoc2023::day01::*;
use aoc2023::*;

fn main() -> std::io::Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let answer = match Args::parse().mode {
        RunMode::Part1 { mut input } => solve_part1(input.lock())?,
        RunMode::Part2 { mut input } => solve_part2(input.lock())?,
    };

    println!("{answer}");
    Ok(())
}
