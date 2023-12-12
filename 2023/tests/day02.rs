use aoc2023::day02::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let input = get_input(DAY, "example");
    let answer = solve_part1(input, 12, 13, 14)
        .expect("failed to get a solution");
    assert_eq!(answer, 8);
}

#[test]
fn part1() {
    log_init();
    let input = get_input(DAY, "input");
    let answer = solve_part1(input, 12, 13, 14)
        .expect("failed to get a solution");
    assert_eq!(answer, 2541);
}

#[test]
fn example2() {
    log_init();
    let input = get_input(DAY, "example");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 2286);
}

#[test]
fn part2() {
    log_init();
    let input = get_input(DAY, "input");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 66016);
}
