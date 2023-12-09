use aoc2023::day05::*;

use std::{fmt::Display, path::Path, io::BufReader, fs::File};

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

    let input = get_reader("inputs/day05/example.txt");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 35);
}

#[test]
fn part1() {
    log_init();

    let input = get_reader("inputs/day05/part1.txt");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 382895070);
}

#[test]
fn example2() {
    log_init();

    let input = get_reader("inputs/day05/example.txt");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 46);
}

#[test] #[ignore] // very slow, skip by default
fn part2() {
    log_init();

    let input = get_reader("inputs/day05/part1.txt");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 17729182);
}
