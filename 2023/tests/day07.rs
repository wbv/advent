use aoc2023::day07::*;

mod common;
use common::*;

const DAY: u8 = get_day!();

#[test]
fn example1() {
    log_init();
    let answer = solve_part1(get_input(DAY, Example))
        .expect("failed to get a solution");
    assert_eq!(answer, 6440);
}

#[test] #[ignore]
fn part1() {
    log_init();
    let answer = solve_part1(get_input(DAY, Full))
        .expect("failed to get a solution");
    assert_eq!(answer, 0);
}

#[test] #[ignore]
fn example2() {
    log_init();
    let answer = solve_part2(get_input(DAY, Example))
        .expect("failed to get a solution");
    assert_eq!(answer, 0);
}

#[test] #[ignore]
fn part2() {
    log_init();
    let answer = solve_part2(get_input(DAY, Full))
        .expect("failed to get a solution");
    assert_eq!(answer, 0);
}
