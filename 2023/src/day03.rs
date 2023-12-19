//! Gear Ratios
//! -----------
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

#![cfg(not(doctest))]

use super::*;
use std::{collections::HashMap, iter::repeat};
use regex::bytes::Regex;

/// # Just the Parts
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
    let sch = Schematic::from_bufread(input);
    let sum = sch.all_nums.iter()
        .filter(|&num| num.near_symbol)
        .map(|num| num.value)
        .sum();
    Ok(sum)
}

/// # Product of Gear Ratios
///
/// The engineer finds the missing part and installs it in the engine! As the engine springs to
/// life, you jump in the closest gondola, finally ready to ascend to the water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the
/// gondola has a phone labeled "help", so you pick it up and the engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the window. There stands
/// the engineer, holding a phone in one hand and waving with the other. You're going so slowly
/// that you haven't even left the station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any
/// `*` symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of
/// multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all up so that the
/// engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
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
/// In this schematic, there are two gears. The first is in the top left; it has part numbers 467
/// and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is
/// 451490. (The `*` adjacent to 617 is not a gear because it is only adjacent to one part number.)
/// Adding up all of the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<usize> {
    let sch = Schematic::from_bufread(input);
    let sum = sch.gears.values()
        .filter(|g| g.nums.len() == 2)
        .map(|g| g.nums[0].value * g.nums[1].value)
        .sum();
    Ok(sum)
}

#[derive(Default, Debug)]
struct Schematic {
    flat_map: Vec<u8>,
    width: usize,
    height: usize,
    all_nums: Vec<Number>,
    gears: HashMap<Coordinate, Gear>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
struct Number {
    value: usize,
    near_symbol: bool,
}

#[derive(Debug, Default)]
struct Gear {
    nums: Vec<Number>,
}


impl Schematic {
    pub fn from_bufread<B: BufRead>(buf: B) -> Self {
        let mut schematic = Schematic::default();
        let mut lines = buf.lines();
        while let Some(Ok(line)) = lines.next() {
            let line = line.as_bytes().to_vec();
            schematic.width = schematic.width.max(line.len());
            schematic.height += 1;
            schematic.flat_map = [schematic.flat_map, line].concat();
        }

        // verify that our computed dimensions represent our data as a rectangle
        debug_assert_eq!(schematic.width * schematic.height, schematic.flat_map.len());

        // find each number in the file
        let re = Regex::new("[0-9]+").unwrap();
        for num in re.find_iter(&schematic.flat_map) {
            // compute coordinates of the found number
            let start_x = (num.start() % schematic.width) as isize;
            let end_x = (num.end() % schematic.width) as isize;
            let y = (num.start() / schematic.width) as isize;

            // compute coordinates of all of its neighbors
            let toprow = ((start_x - 1)..=(end_x)).zip(repeat(y - 1));
            let middle = [(start_x - 1, y), (end_x, y)].into_iter();
            let bottom = ((start_x - 1)..=(end_x)).zip(repeat(y + 1));
            let neighbors = toprow.chain(middle).chain(bottom);

            // determine whether it's next to a symbol
            let near_symbol = neighbors.clone()
                .any(|(x, y)| schematic.at(x, y) != b'.');
            // and save its integral value
            let value = String::from_utf8(num.as_bytes().to_owned())
                .unwrap()
                .parse::<usize>()
                .unwrap();

            // check for gears in its neighborhood
            let number = Number { value, near_symbol};
            let gears: Vec<_> = neighbors.filter(|(x, y)| schematic.at(*x, *y) == b'*').collect();
            for (x, y) in gears {
                // add it to the list of that gear's neighboring numbers (for the gear that we found)
                schematic.gears.entry(Coordinate { x, y })
                    .and_modify(|g| g.nums.push(number))
                    .or_insert(Gear { nums: vec![number] });
            }

            // store the parsed number and its attributes
            schematic.all_nums.push(number);
        }

        schematic
    }

    fn at(&self, x: isize, y: isize) -> u8 {
        // bounds check x and y (out-of-bounds = default value of b'.')
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            self.flat_map.get(x + (y * self.width)).map_or(b'.', |c| *c)
        } else {
            b'.'
        }
    }
}
