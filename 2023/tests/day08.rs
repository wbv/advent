use aoc2023::day08::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example"))
        .expect("failed to get a solution");
    assert_eq!(answer, 2);
}

#[test]
fn example2() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example2"))
        .expect("failed to get a solution");
    assert_eq!(answer, 6);
}

#[test]
fn part1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 16343);
}

#[test]
fn example3() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example3"))
        .expect("failed to get a solution");
    assert_eq!(answer, 6);
}

#[test]
fn part2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 15299095336639);
}
