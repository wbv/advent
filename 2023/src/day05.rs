#![cfg(not(doctest))]

use super::*;

pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<usize> {
    let mut lines = input.lines();

    // get seeds
    let seeds = lines.next().unwrap().unwrap()
        .split_once(':').unwrap().1
        .split_whitespace()
        .map(|word| usize::from_str_radix(word, 10).unwrap())
        .collect::<Vec<_>>();
    debug!("Seeds: {:?}", seeds);

    Ok(0)
}
