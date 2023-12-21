//! # Cosmic Expansion
//!
//! You continue following signs for "Hot Springs" and eventually come across an observatory. The
//! Elf within turns out to be a researcher studying cosmic expansion using the giant telescope
//! here.
//!
//! He doesn't know anything about the missing machine parts; he's only visiting for this research
//! project. However, he confirms that the hot springs are the next-closest area likely to have
//! people; he'll even take you straight there once he's done with today's observation analysis.
//!
//! Maybe you can help him with the analysis to speed things up?



#![cfg(not(doctest))]

use std::collections::HashMap;

use super::*;

/// # Simple Expansion
///
/// The researcher has collected a bunch of data and compiled the data into a single giant image
/// (your puzzle input). The image includes empty space (.) and galaxies (#). For example:
///
/// ```
/// ...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....
/// ```
///
/// The researcher is trying to figure out the sum of the lengths of the shortest path between
/// every pair of galaxies. However, there's a catch: the universe expanded in the time it took the
/// light from those galaxies to reach the observatory.
///
/// Due to something involving gravitational effects, only some space expands. In fact, the result
/// is that any rows or columns that contain no galaxies should all actually be twice as big.
///
/// In the above example, three columns and two rows contain no galaxies:
///
/// ```
///    v  v  v
///  ...#......
///  .......#..
///  #.........
/// >..........<
///  ......#...
///  .#........
///  .........#
/// >..........<
///  .......#..
///  #...#.....
///    ^  ^  ^
/// ```
///
/// These rows and columns need to be twice as big; the result of cosmic expansion therefore looks
/// like this:
///
/// ```
/// ....#........
/// .........#...
/// #............
/// .............
/// .............
/// ........#....
/// .#...........
/// ............#
/// .............
/// .............
/// .........#...
/// #....#.......
/// ```
///
/// Equipped with this expanded universe, the shortest path between every pair of galaxies can be
/// found. It can help to assign every galaxy a unique number:
///
/// ```
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// ............6
/// .............
/// .............
/// .........7...
/// 8....9.......
/// ```
///
/// In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair
/// doesn't matter. For each pair, find any shortest path between the two galaxies using only steps
/// that move up, down, left, or right exactly one . or # at a time. (The shortest path between two
/// galaxies is allowed to pass through another galaxy.)
///
/// For example, here is one of the shortest paths between galaxies 5 and 9:
///
/// ```
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// .##.........6
/// ..##.........
/// ...##........
/// ....##...7...
/// 8....9.......
/// ```
///
/// This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy
/// 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other
/// example shortest path lengths:
///
/// - Between galaxy 1 and galaxy 7: 15
/// - Between galaxy 3 and galaxy 6: 17
/// - Between galaxy 8 and galaxy 9: 5
///
/// In this example, after expanding the universe, the sum of the shortest path between all 36
/// pairs of galaxies is 374.
///
/// Expand the universe, then find the length of the shortest path between every pair of galaxies.
/// What is the sum of these lengths?
pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<AdvInt> {
    let lines = input.lines().map(|l| l.expect("i/o error reading input file"));
    let universe = Universe::from(lines).expanded();

    let mut distances = vec![];
    for i in 0..universe.galaxies.len() {
        let from = &universe.galaxies[i];
        for to in &universe.galaxies[(i+1)..] {
            // calculate L1 ("taxicab geometry") distance for each pair of galaxies
            let dist = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
            debug!("DISTANCE from {:?} to {:?} = {:?}", from, to, dist);
            distances.push(dist as usize);
        }
    }

    Ok(distances.into_iter().sum())
}

pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<AdvInt> {
    todo!()
}

type AdvInt = usize;

struct Universe {
    map: Vec<u8>,
    width: usize,
    height: usize,
    galaxies: Vec<(i32, i32)>,
}

impl Universe {
    fn from<L: Iterator<Item = String>>(lines: L) -> Self {
        let mut width = 0;
        let mut height = 0;
        let map = lines
            .flat_map(|s| {
                width = width.max(s.len());
                height += 1;
                s.as_bytes().to_vec()
            })
            .collect::<Vec<u8>>();

        // verify correct flattening of the 2D map
        debug_assert_eq!(width * height, map.len());

        let galaxies = map
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| {
                (b == b'#').then(|| ((i % width) as i32, (i / width) as i32))
            })
            .collect();

        Universe {
            map,
            width,
            height,
            galaxies,
        }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || x >= self.width as i32 {
            b'.'
        } else if y < 0 || y >= self.height as i32 {
            b'.'
        } else {
            let i = self.to_offset(x, y);
            self.map[i]
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut u8> {
        let i = self.to_offset(x, y);
        self.map.get_mut(i)
    }

    fn to_offset(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
    }

    fn expanded(&self) -> Self {
        let mut empty_rows = vec![];
        for row in 0..self.height {
            let row_empty = self.map
                .iter()
                .enumerate()
                .filter(|(d, _)| d / self.width == row)
                .all(|(_, &x)| x == b'.');
            if row_empty {
                empty_rows.push(row);
            }
        }

        let mut empty_cols = vec![];
        for col in 0..self.width {
            let col_empty = self.map
                .iter()
                .enumerate()
                .filter(|(d, _)| d % self.width == col)
                .all(|(_, &x)| x == b'.');
            if col_empty {
                empty_cols.push(col);
            }
        }

        debug!("EXPANSION: found {} empty rows, {} empty cols", empty_rows.len(), empty_cols.len());
        debug!("EXPANSION: rows {:?}, cols {:?} ", empty_rows, empty_cols);

        debug!("Going from:");
        for row in self.map.chunks_exact(self.width).map(|x| String::from_utf8(x.to_vec())) {
            debug!("{}", row.unwrap());
        }

        // copy each row
        let mut newmap = vec![];
        for row in 0..self.height {
            // copy each column in a row
            let mut newrow = vec![];
            for col in 0..self.width {
                // if the column is empty, insert an extra space
                if empty_cols.contains(&col) {
                    newrow.push(b'.');
                }
                newrow.push(self.get(col as i32, row as i32));
            }

            // if the row is duplicated, insert it twice
            if empty_rows.contains(&row) {
                newmap.push(String::from_utf8(newrow.clone()).expect("non utf8 row"));
            }
            newmap.push(String::from_utf8(newrow).expect("non utf8 row"));
        }

        debug!("");
        debug!("To:");
        for row in &newmap {
            debug!("{}", row);
        }

        Universe::from(newmap.into_iter())
    }
}
