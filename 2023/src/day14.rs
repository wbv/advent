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

use std::{ops::{Index, IndexMut}, collections::HashMap};
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
    let input: Vec<String> = input.into_iter().collect();
    let mut platform: Platform = input.as_slice().into();
    debug!("Platform:\n{platform:?}");
    platform.tilt();
    debug!("Platform (tilted):\n{platform:?}");
    platform.weigh()
}

/// # Spin Cycles
///
/// The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that,
/// you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side
/// of the control panel labeled "spin cycle" attempts to do just that!
///
/// Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then
/// south, then east. After each tilt, the rounded rocks roll as far as they can before the
/// platform tilts in the next direction. After one cycle, the platform will have finished rolling
/// the rounded rocks in those four directions in that order.
///
/// Here's what happens in the example above after each of the first few cycles:
///
/// After 1 cycle:
/// ```
/// .....#....
/// ....#...O#
/// ...OO##...
/// .OO#......
/// .....OOO#.
/// .O#...O#.#
/// ....O#....
/// ......OOOO
/// #...O###..
/// #..OO#....
/// ```
///
/// After 2 cycles:
/// ```
/// .....#....
/// ....#...O#
/// .....##...
/// ..O#......
/// .....OOO#.
/// .O#...O#.#
/// ....O#...O
/// .......OOO
/// #..OO###..
/// #.OOO#...O
/// ```
///
/// After 3 cycles:
/// ```
/// .....#....
/// ....#...O#
/// .....##...
/// ..O#......
/// .....OOO#.
/// .O#...O#.#
/// ....O#...O
/// .......OOO
/// #...O###.O
/// #.OOO#...O
/// ```
///
/// This process should work if you leave it running long enough, but you're still worried about
/// the north support beams. To make sure they'll survive for a while, you need to calculate the
/// total load on the north support beams after 1000000000 cycles.
///
/// In the above example, after 1000000000 cycles, the total load on the north support beams is 64.
///
/// Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north
/// support beams?
pub fn solve_part2<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    let input: Vec<String> = input.into_iter().collect();
    let mut platform: Platform = input.as_slice().into();
    let mut seen_platforms = HashMap::<Vec<u8>, u32>::new();
    debug!("Platform:\n{platform:?}");
    seen_platforms.insert(platform.map.clone(), 0);

    const CYCLES: u32 = 1_000_000_000;
    for iteration in 1..=CYCLES {
        platform.spin_cycle();
        debug!("Platform (cycle: {iteration}):\n{platform:?}\nWeight: {}", platform.weigh());
        // check if we hit a loop and "fast-forward"
        if let Some(prev_iteration) = seen_platforms.insert(platform.map.clone(), iteration) {
            let repeating = iteration - prev_iteration;
            let remaining = (CYCLES - iteration) % repeating;
            info!("Same platform on cycle {iteration} as on prior cycle {prev_iteration}.");
            info!("Fast-forwarding {} cycles, then cycling {remaining} more times...", CYCLES - iteration);
            for _ in 1..=remaining {
                platform.spin_cycle();
            }
            break;
        }
    }

    platform.weigh()
}

#[derive(Clone)]
struct Platform {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl From<&[String]> for Platform {
    fn from(lines: &[String]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let map: Vec<u8> = lines.iter()
            .flat_map(|line| {
                height += 1;
                width = width.max(line.len());
                line.as_bytes().to_owned()
            })
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

impl Platform {
    fn tilt(&mut self) {
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
                        //debug!("moved stone at {x}, {y} to {x}, {newrow}");
                    } else {
                        // note that we can't move the stone at all (found an obstacle)
                        //debug!("couldn't move stone at {x}, {y}");
                    }
                }
            }
        }
    }

    /// rotates the map clockwise by 90 degrees
    fn rotate(&mut self) {
        let old = self.clone();
        for j in 0..old.height {
            for i in 0..old.width {
                self[(old.height - 1 - j, i)] = old[(i, j)];
            }
        }

        (self.width, self.height) = (self.height, self.width);
    }

    /// (tilt then rotate) x4
    fn spin_cycle(&mut self) {
        for _ in 1..=4 {
            self.tilt();
            self.rotate();
        }
    }

    fn weigh(&mut self) -> usize {
        // on each level (top to bottom)
        (0..self.height).map(|row| {
            // count the number of round stones
            let found = self.map[(self.width*row)..(self.width*row + self.width)]
                .iter()
                .filter(|&&c| c == b'O')
                .count();
            // compute (and return) the "weighted" count
            let level = self.height - row;
            //debug!("on level: {level} found {found} stones");
            found * level
        }).sum()
    }
}

impl Index<(usize, usize)> for Platform {
    type Output = u8;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!("invalid index into platform map");
        } else {
            &self.map[index.0 + index.1 * self.width]
        }
    }
}

impl IndexMut<(usize, usize)> for Platform {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!("invalid index into platform map");
        } else {
            &mut self.map[index.0 + index.1 * self.width]
        }
    }
}

impl std::fmt::Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                write!(f, "{}", self.map[i + self.width * j] as char)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

testcase!(ex1, solve_part1, "example", 136);
testcase!(part1, solve_part1, "input", 109665);
testcase!(ex2, solve_part2, "example", 64);
testcase!(part2, solve_part2, "input", 96061);
