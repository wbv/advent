use aoc2023::day10::{solve_part1, solve_part2};

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example1"))
        .expect("failed to get a solution");
    assert_eq!(answer, 4);
}

#[test]
fn example2() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example2"))
        .expect("failed to get a solution");
    assert_eq!(answer, 4);
}

#[test]
fn example3() {
    log_init();
    let answer = solve_part1(get_input(DAY, "example3"))
        .expect("failed to get a solution");
    assert_eq!(answer, 8);
}

#[test]
fn part1() {
    log_init();
    let answer = solve_part1(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 6842);
}

#[test]
#[ignore]
fn example4() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example4"))
        .expect("failed to get a solution");
    assert_eq!(answer, 4);
}

#[test]
#[ignore]
fn example5() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example5"))
        .expect("failed to get a solution");
    assert_eq!(answer, 4);
}


#[test]
#[ignore]
fn example6() {
    log_init();
    let answer = solve_part2(get_input(DAY, "example6"))
        .expect("failed to get a solution");
    assert_eq!(answer, 8);
}

#[test]
#[ignore]
fn part2() {
    log_init();
    let answer = solve_part2(get_input(DAY, "input"))
        .expect("failed to get a solution");
    assert_eq!(answer, 957);
}
