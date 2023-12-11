mod common;
use common::*;
use aoc2023::day01::*;

#[test]
fn example1() {
    log_init();
    let input = get_reader("inputs/day01/example");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 142);
}

#[test]
fn part1() {
    log_init();
    let input = get_reader("inputs/day01/input");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 53194);
}

#[test]
fn example2() {
    log_init();
    let input = get_reader("inputs/day01/example2");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 281);
}

#[test]
fn part2() {
    log_init();
    let input = get_reader("inputs/day01/input");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 54249);
}
