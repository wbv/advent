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
            }
            _ => {
                error!(
                    "Failed to find digits in {:?}",
                    String::from_utf8(line.into())
                );
            }
        }
    }

    println!("{sum}");
    Ok(())
}

/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out
/// with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid
/// "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each
/// line. For example:
///
/// ```
/// #[include_str("../../inputs/day01/example2.txt")
/// ```
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these
/// together produces 281.
///
/// What is the sum of all of the calibration values?
fn solve_part2<B: BufRead>(input: B) -> std::io::Result<()> {
    info!("Solving (Part 2)...");
    let mut sum = 0usize;

    let mut lines = input.lines();
    while let Some(Ok(line)) = lines.next() {
        let first = find_first_numeral(line.as_str());
        let last = find_last_numeral(line.as_str());
        match (first, last) {
            (Some(first), Some(last)) => {
                let value = first.val * 10 + last.val;
                debug!("Line value: {value}");
                sum += value;
                debug!("Running sum: {sum}");
            }
            _ => {
                error!(
                    "Failed to find digits in {:?}",
                    String::from_utf8(line.into())
                );
            }
        }
    }

    println!("{sum}");
    Ok(())
}

const PAIRS: [(usize, &str); 20] = [
    (0, "zero"),
    (0, "0"),
    (1, "one"),
    (1, "1"),
    (2, "two"),
    (2, "2"),
    (3, "three"),
    (3, "3"),
    (4, "four"),
    (4, "4"),
    (5, "five"),
    (5, "5"),
    (6, "six"),
    (6, "6"),
    (7, "seven"),
    (7, "7"),
    (8, "eight"),
    (8, "8"),
    (9, "nine"),
    (9, "9"),
];

struct Finding {
    /// The index where a digit (ascii or numeral) is found in some string
    idx: usize,
    /// The actual numeric value of that digit
    val: usize,
}

fn find_first_numeral(line: &str) -> Option<Finding> {
    let mut numeral: Option<Finding> = None;

    debug!("FORWARD SEARCH of '{line}'");
    for (textvalue, text) in PAIRS {
        if let Some(position) = line.find(text) {
            match numeral {
                None => {
                    debug!("Found text '{text:?}' at {position} (First found)");
                    numeral = Some(Finding {
                        idx: position,
                        val: textvalue,
                    });
                }
                Some(p) if p.idx < position => {
                    debug!("Found text '{text:?}' at {position}");
                    numeral = Some(Finding {
                        idx: position,
                        val: textvalue,
                    });
                }
                _ => {}
            }
        }
    }

    numeral
}

fn find_last_numeral(line: &str) -> Option<Finding> {
    let mut numeral: Option<Finding> = None;

    debug!("REVERSE SEARCH of '{line}'");
    for (textvalue, text) in PAIRS {
        if let Some(position) = line.rfind(text) {
            match numeral {
                None => {
                    debug!("Found text '{text:?}' at {position} (First found)");
                    numeral = Some(Finding {
                        idx: position,
                        val: textvalue,
                    });
                }
                Some(p) if p.idx > position => {
                    debug!("Found text '{text:?}' at {position}");
                    numeral = Some(Finding {
                        idx: position,
                        val: textvalue,
                    });
                }
                _ => {}
            }
        }
    }

    numeral
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    match Args::parse().mode {
        RunMode::Part1 { mut input } => {
            solve_part1(input.lock())?;
        }
        RunMode::Part2 { mut input } => {
            solve_part2(input.lock())?;
        }
    }

    Ok(())
}
