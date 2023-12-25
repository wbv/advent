#![doc = include_str!("../README.md")]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

use log::*;

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path
};

/// Attempts to init the logging subsystem.
pub fn log_init() {
    let _ = env_logger::builder().format_timestamp(None).try_init();
}

/// Takes a path to a file and returns a [BufReader](std::io::BufReader) over its contents.
fn get_reader<P: AsRef<Path> + Copy + Display>(path: P) -> std::io::Result<BufReader<File>> {
    let input = std::fs::File::open(path.as_ref())?;
    Ok(BufReader::new(input))
}

fn get_filename<S: AsRef<str>>(day: u8, file: S) -> String {
    format!("inputs/day{day:02}/{}", file.as_ref())
}

/// Loads a test file into a `Vec<String>` (one String per line), given a day and filename
pub fn get_input<S: AsRef<str>>(day: u8, kind: S) -> Vec<String> {
    let filename = get_filename(day, kind.as_ref());
    get_reader(filename.as_str())
        .expect(format!("failed to open test input file: day{day:02} - {}", kind.as_ref()).as_str())
        .lines()
        .map(|l| l.expect("i/o error when reading"))
        .collect()
}

/// Auto-expands to the day (as a [`u8`]) corresponding based on the current filename.
#[macro_export]
macro_rules! get_day {
    () => {
        {
            let modpath = module_path!().as_bytes();
            let tens = modpath[modpath.len() - 2];
            let ones = modpath[modpath.len() - 1];

            // if there is no tens place
            if tens < 0x30 || tens > 0x39 {
                ones - 0x30
            } else {
                (tens - 0x30) * 10 + (ones - 0x30)
            }
        }
    }
}

/// Generates a test for a day of Advent of Code.
///
/// Requires the name of the test, the function to test, its input file, and the expected answer.
/// Any extra arguments required by the test function can be optionally added as extra args at the
/// end. It will automatically look for test files under the appropriate /inputs/dayXX/ folder.
#[macro_export]
macro_rules! testcase {
    ($name:ident, $partfn:ident, $inputfile:expr, $expected:expr $(,$partfnarg:expr)* ) => {
        #[test]
        fn $name() {
            log_init();
            let input = get_input(get_day!(), $inputfile);
            let answer = $partfn(input$(, $partfnarg)*);
            assert_eq!(answer, $expected);
        }
    };
}
