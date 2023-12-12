use aoc2023::day01::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let input = get_input(DAY, "example");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 142);
}

#[test]
fn part1() {
    log_init();
    let input = get_input(DAY, "input");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 53194);
}

#[test]
fn example2() {
    log_init();
    let input = get_input(DAY, "example2");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 281);
}

#[test]
fn part2() {
    log_init();
    let input = get_input(DAY, "input");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 54249);
}
