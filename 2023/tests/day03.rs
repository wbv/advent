use aoc2023::day03::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let input = get_input(DAY, "example");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 4361);
}

#[test]
fn part1() {
    log_init();
    let input = get_input(DAY, "input");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 556057);
}

#[test]
fn example2() {
    log_init();
    let input = get_input(DAY, "example");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 467835);
}

#[test]
fn part2() {
    log_init();
    let input = get_input(DAY, "input");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 82824352);
}
