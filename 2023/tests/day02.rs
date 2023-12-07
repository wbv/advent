use aoc2023::day02::*;

use std::{fmt::Display, fs::File, io::BufReader, path::Path};

fn log_init() {
    let _ = env_logger::builder().format_timestamp(None).try_init();
}

fn get_reader<P: AsRef<Path> + Copy + Display>(path: P) -> BufReader<File> {
    let input = File::open(path).expect(format!("failed to open test input file: {path}").as_str());
    BufReader::new(input)
}

#[test]
fn example1() {
    log_init();

    let input = get_reader("inputs/day02/example.txt");
    let answer = solve_part1(input, 12, 13, 14)
        .expect("failed to get a solution");
    assert_eq!(answer, 8);
}

#[test]
fn part1() {
    log_init();

    let input = get_reader("inputs/day02/part1.txt");
    let answer = solve_part1(input, 12, 13, 14)
        .expect("failed to get a solution");
    assert_eq!(answer, 2541);
}

#[test]
fn example2() {
    let input = get_reader("inputs/day02/example.txt");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 2286);
}

#[test]
fn part2() {
    let input = get_reader("inputs/day02/part2.txt");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 66016);
}
