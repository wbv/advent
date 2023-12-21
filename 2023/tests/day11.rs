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

#[ignore]
#[test]
fn example2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example"))
        .expect("failed to get a solution");
    assert_eq!(answer, 0);
}

#[ignore]
#[test]
fn part2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 0);
}
