//! You try to ask why they can't just use a weather machine ("not powerful enough") and where
//! they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot
//! of questions") and hang on did you just say the sky ("of course, where do you think snow comes
//! from") when you realize that the Elves are already loading you into a trebuchet ("please hold
//! still, we need to strap you in").
//!
//!As they're making the final adjustments, they discover that their calibration document (your
//!puzzle input) has been amended by a very young Elf who was apparently just excited to show off
//!her art skills. Consequently, the Elves are having trouble reading the values on the document.

use std::io::BufRead;

use aoc2023::*;

///
/// The newly-improved calibration document consists of lines of text; each line originally
/// contained a specific calibration value that the Elves now need to recover. On each line, the
/// calibration value can be found by combining the first digit and the last digit (in that order)
/// to form a single two-digit number.
///
/// For example:
///
/// ```
/// #[input_str("../../inputs/day01/example.txt")
/// ```
///
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding
/// these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
fn solve_part1<B: BufRead>(input: B) -> std::io::Result<()> {
    info!("Solving (Part 1)...");
    let mut sum = 0usize;

    let mut lines = input.lines();
    while let Some(Ok(line)) = lines.next() {
        let line = line.as_bytes();
        let first = line.iter().find(|c| c.is_ascii_digit());
        let last = line.iter().rfind(|c| c.is_ascii_digit());
        match (first, last) {
            (Some(first), Some(last)) => {
                let value = (first - b'0') * 10 + (last - b'0');
                debug!("Line value: {value}");
                sum += value as usize;
                debug!("Running sum: {sum}");
            },
            _ => {
                error!("Failed to find digits in {:?}", String::from_utf8(line.into()));
            }
        }
    }

    println!("{sum}");
    Ok(())
}

fn solve_part2<B: BufRead>(input: B) -> std::io::Result<()> {
    info!("Solving (Part 2)...");
    for _ in input.lines() {
        //
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    match Args::parse().mode {
        RunMode::Part1 { mut input } => {
            solve_part1(input.lock())?;
        },
        RunMode::Part2 { mut input } => {
            solve_part2(input.lock())?;
        },
    }

    Ok(())
}
