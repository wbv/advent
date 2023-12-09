use aoc2023::day03::*;

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

    let input = get_reader("inputs/day03/example.txt");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 4361);
}

#[test]
fn part1() {
    log_init();

    let input = get_reader("inputs/day03/part1.txt");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 556057);
}

//#[test]
//fn example2() {
//    log_init();
//
//    let input = get_reader("inputs/day03/example.txt");
//    let answer = solve_part2(input)
//        .expect("failed to get a solution");
//    assert_eq!(answer, 467835);
//}

