use aoc2023::day01::*;

use std::{fmt::Display, fs::File, io::BufReader, path::Path};

fn get_reader<P: AsRef<Path> + Copy + Display>(path: P) -> BufReader<File> {
    let input = File::open(path).expect(format!("failed to open test input file: {path}").as_str());
    BufReader::new(input)
}

#[test]
fn example1() {
    let input = get_reader("inputs/day01/example.txt");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 142);
}

#[test]
fn part1() {
    let input = get_reader("inputs/day01/part1.txt");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 53194);
}

#[test]
fn example2() {
    let input = get_reader("inputs/day01/example2.txt");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 281);
}

#[test]
fn part2() {
    let input = get_reader("inputs/day01/part2.txt");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 54249);
}
