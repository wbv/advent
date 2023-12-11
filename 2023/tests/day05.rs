mod common;
use common::*;
use aoc2023::day05::*;

#[test]
fn example1() {
    log_init();
    let input = get_reader("inputs/day05/example");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 35);
}

#[test]
fn part1() {
    log_init();
    let input = get_reader("inputs/day05/input");
    let answer = solve_part1(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 382895070);
}

#[test]
fn example2() {
    log_init();
    let input = get_reader("inputs/day05/example");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 46);
}

#[test]
fn part2() {
    log_init();
    let input = get_reader("inputs/day05/input");
    let answer = solve_part2(input)
        .expect("failed to get a solution");
    assert_eq!(answer, 17729182);
}
