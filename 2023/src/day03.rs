#![cfg(not(doctest))]

//! # Gear Ratios
//!
//! You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you
//! up to the water source, but this is as far as he can bring you. You go inside.
//!
//! It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//!
//! "Aaah!"
//!
//! You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I
//! wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while
//! before I can fix it." You offer to help.

use regex::bytes::{Regex, Match};

use super::*;

use std::io::BufRead;

/// # Symbol Locality
///
/// The engineer explains that an engine part seems to be missing from the engine, but nobody can
/// figure out which one. If you can add up all the part numbers in the engine schematic, it should
/// be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the engine.
/// There are lots of numbers and symbols you don't really understand, but apparently any number
/// adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
/// (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
///
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
///
/// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol:
/// 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a
/// part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of all of the part
/// numbers in the engine schematic?
pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<usize> {
    let mut sum = 0;

    let sch = Schematic::from_bufread(input);
    let re = Regex::new("[0-9]+").unwrap();
    for num in re.find_iter(&sch.flat_map) {
        debug!("match: {:?}", String::from_utf8(num.as_bytes().to_owned()));
        let neighbors = sch.neighbors(num);
        debug!("neigbors: {:?}", String::from_utf8(neighbors.clone()));

        if neighbors.iter().any(|&c| c != b'.') {
            sum += String::from_utf8(num.as_bytes().to_owned()).unwrap()
                .parse::<usize>().unwrap();
        }
    }

    Ok(sum)
}

#[derive(Default, Debug)]
struct Schematic {
    flat_map: Vec<u8>,
    width: usize,
    height: usize,
}

impl Schematic {
    pub fn from_bufread<B: BufRead>(buf: B) -> Self {
        let mut lines = buf.lines();
        let mut schematic = Schematic::default();
        while let Some(Ok(line)) = lines.next() {
            let line = line.as_bytes().to_vec();
            debug!("Read line of {} bytes", line.len());
            schematic.width = schematic.width.max(line.len());
            schematic.height += 1;
            schematic.flat_map = [schematic.flat_map, line].concat();
        }

        debug!("Total lines: {}", schematic.height);
        debug_assert_eq!(schematic.width * schematic.height, schematic.flat_map.len());
        schematic
    }

    fn at(&self, x: isize, y: isize) -> u8 {
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            self.flat_map.get(x + (y * self.width)).map_or(b'.', |c| *c)
        } else {
            b'.'
        }
    }

    fn at_idx(&self, at: usize) -> u8 {
        let x = at % self.width;
        let y = at / self.width;
        self.at(x as isize, y as isize)
    }

    fn neighbors(&self, pt: Match) -> Vec<u8> {
        let mut neighbors = vec![];
        let (start_x, start_y) = ((pt.start() % self.width) as isize, (pt.start() / self.width) as isize);
        let (end_x, end_y) = ((pt.end() % self.width) as isize, (pt.end() / self.width) as isize);
        debug!("start: {start_x}, {start_y}   end: {end_x}, {end_y}");
        for x in (start_x - 1)..=(end_x) {
            // row above
            neighbors.push(self.at(x, start_y - 1));
            // row below
            neighbors.push(self.at(x, start_y + 1));
        }
        // left and right neighbors
        neighbors.push(self.at(start_x - 1, start_y));
        neighbors.push(self.at(end_x, start_y));

        neighbors
    }
}
