#![cfg(not(doctest))]

//! # Parabolic Reflector Dish
//!
//! You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish
//! attached to the side of another large mountain.
//!
//! The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the
//! shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly
//! the wrong direction. If the dish is meant to focus light, all it's doing right now is sending
//! it in a vague direction.
//!
//! This system must be what provides the energy for the lava! If you focus the reflector dish,
//! maybe you can go where it's pointing and use the light to fix the lava production.
//!
//! Upon closer inspection, the individual mirrors each appear to be connected via an elaborate
//! system of ropes and pulleys to a large metal platform below the dish. The platform is covered
//! in large rocks of various shapes. Depending on their position, the weight of the rocks deforms
//! the platform, and the shape of the platform controls which ropes move and ultimately the focus
//! of the dish.

use std::ops::{Index, IndexMut};
use super::*;
type AdvInt = usize;

/// In short: if you move the rocks, you can focus the dish. The platform even has a control panel
/// on the side that lets you tilt it in one of four directions! The rounded rocks (`O`) will roll
/// when the platform is tilted, while the cube-shaped rocks (`#`) will stay in place. You note the
/// positions of all of the empty spaces (`.`) and rocks (your puzzle input). For example:
///
/// ```
/// O....#....
/// O.OO#....#
/// .....##...
/// OO.#O....O
/// .O.....O#.
/// O.#..O.#.#
/// ..O..#O..O
/// .......O..
/// #....###..
/// #OO..#....
/// ```
///
/// Start by tilting the lever so all of the rocks will slide north as far as they will go:
///
/// ```
/// OOOO.#.O..
/// OO..#....#
/// OO..O##..O
/// O..#.OO...
/// ........#.
/// ..#....#.#
/// ..O..#.O.O
/// ..O.......
/// #....###..
/// #....#....
/// ```
///
/// You notice that the support beams along the north side of the platform are damaged; to ensure
/// the platform doesn't collapse, you should calculate the total load on the north support beams.
///
/// The amount of load caused by a single rounded rock (`O`) is equal to the number of rows from
/// the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped
/// rocks (`#`) don't contribute to load.) So, the amount of load caused by each rock in each row
/// is as follows:
///
/// ```
/// OOOO.#.O.. 10
/// OO..#....#  9
/// OO..O##..O  8
/// O..#.OO...  7
/// ........#.  6
/// ..#....#.#  5
/// ..O..#.O.O  4
/// ..O.......  3
/// #....###..  2
/// #....#....  1
/// ```
///
/// The total load is the sum of the load caused by all of the rounded rocks. In this example, the
/// total load is 136.
///
/// Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load
/// on the north support beams?
pub fn solve_part1<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    input.into_iter()
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|lines| {
            let mut platform: Platform = lines.into();
            debug!("Platform:\n{platform:?}");
            platform.tilt(North);
            debug!("Platform (tilted):\n{platform:?}");
            platform.weigh(North)
        })
        .sum()
}

pub fn solve_part2<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    todo!()
}

struct Platform {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl From<&[String]> for Platform {
    fn from(lines: &[String]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let map: Vec<u8> = lines.into_iter()
            .map(|line| {
                height += 1;
                width = width.max(line.len());
                line.as_bytes().to_owned()
            })
            .flatten()
            .collect();

        // verify flattening assumptions
        debug_assert_eq!(width * height, map.len());

        Self {
            map,
            width,
            height
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North
}
use Direction::*;

impl Platform {
    fn tilt(&mut self, dir: Direction) {
        debug_assert_eq!(dir, North);
        // for each column
        for x in 0..self.width {
            // for each row in that column, try to move each found stone up (until they cant)
            for y in 0..self.height {
                if self[(x, y)] == b'O' {
                    // found a stone? move it up to highest available empty space
                    let mut newrow = y;
                    for row in (0..y).rev() {
                        match self[(x, row)] {
                           b'.' => newrow = row,
                           _ => break,
                        }
                    }
                    if newrow != y {
                        // swap and resume stone search
                        self[(x, newrow)] = b'O';
                        self[(x, y)] = b'.';
                        debug!("moved stone at {x}, {y} to {x}, {newrow}");
                    } else {
                        // note that we can't move the stone at all (found an obstacle)
                        debug!("couldn't move stone at {x}, {y}");
                    }
                }
            }
        }
    }

    fn weigh(&mut self, dir: Direction) -> usize {
        debug_assert_eq!(dir, North);

        // on each level (top to bottom)
        (0..self.height).map(|row| {
            // count the number of round stones
            let found = self.map[(self.width*row)..(self.width*row + self.width)]
                .iter()
                .filter(|&&c| c == b'O')
                .count();
            // compute (and return) the "weighted" count
            let level = self.height - row;
            debug!("on level: {level} found {found} stones");
            found * level
        }).sum()
    }
}

impl Index<(usize, usize)> for Platform {
    type Output = u8;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.map[index.0 + index.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for Platform {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.map
            .get_mut(index.0 + index.1 * self.width)
            .expect("invalid index into platform map")
    }
}

impl std::fmt::Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                write!(f, "{}", self.map[i + self.width * j] as char)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

testcase!(ex1, solve_part1, "example", 136);
testcase!(part1, solve_part1, "input", 109665);
//testcase!(ex2, solve_part2, "example", 0);
//testcase!(part2, solve_part2, "input", 0);
