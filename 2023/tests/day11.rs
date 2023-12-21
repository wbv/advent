use aoc2023::day11::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example"))
        .expect("failed to get a solution");
    assert_eq!(answer, 374);
}

#[test]
fn part1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 9795148);
}

#[test]
fn example2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example"), 10)
        .expect("failed to get a solution");
    assert_eq!(answer, 1030);
}

#[test]
fn example3() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example"), 100)
        .expect("failed to get a solution");
    assert_eq!(answer, 8410);
}

#[test]
fn part2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "input"), 1_000_000)
        .expect("failed to get a solution");
    assert_eq!(answer, 650672493820);
}
