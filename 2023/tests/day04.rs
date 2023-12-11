mod common;
use common::*;
use aoc2023::day04::*;

#[test]
fn example1() {
    log_init();
    let input = get_reader("inputs/day04/example");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 13);
}

#[test]
fn part1() {
    log_init();
    let input = get_reader("inputs/day04/input");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 20667);
}

#[test]
fn example2() {
    log_init();
    let input = get_reader("inputs/day04/example");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 30);
}

#[test]
fn part2() {
    log_init();
    let input = get_reader("inputs/day04/input");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 5833065);
}
