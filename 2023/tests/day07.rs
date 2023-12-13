use aoc2023::day07::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example"))
        .expect("failed to get a solution");
    assert_eq!(answer, 6440);
}

#[test]
fn part1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 249748283);
}

#[test]
fn example2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example"))
        .expect("failed to get a solution");
    assert_eq!(answer, 5905);
}

#[test]
fn part2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 248029057);
}
