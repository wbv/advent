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
pub fn solve_part1<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    // part1 is just part2 but with a factor-of-2 expansion
    solve_part2(input, 2)
}

/// # Expansion by a Factor
///
/// The galaxies are much older (and thus much farther apart) than the researcher initially
/// estimated.
///
/// Now, instead of the expansion you did before, make each empty row or column one million times
/// larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty
/// column should be replaced with 1000000 empty columns.
///
/// (In the example above, if each empty row or column were merely 10 times larger, the sum of the
/// shortest paths between every pair of galaxies would be 1030. If each empty row or column were
/// merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be
/// 8410. However, your universe will need to expand far beyond these values.)
///
/// Starting with the same initial image, expand the universe according to these new rules, then
/// find the length of the shortest path between every pair of galaxies. What is the sum of these
/// lengths?
pub fn solve_part2<L: IntoIterator<Item = String>>(input: L, expansion_factor: usize) -> AdvInt {
    let universe = Universe::from(input);

    let empty_cols = universe.empty_cols();
    let empty_rows = universe.empty_rows();

    // calculate L1 ("taxicab geometry") distance for each pair of galaxies
    let mut distances = vec![];
    for i in 0..universe.galaxies.len() {
        let from = &universe.galaxies[i];
        for to in &universe.galaxies[(i+1)..] {
            let mut dist = from.0.abs_diff(to.0) as AdvInt + from.1.abs_diff(to.1) as AdvInt;
            debug!("   Distance (unexpanded) from {:?} to {:?} = {:?}", from, to, dist);

            // find all empty rows and columns between this pair of galaxies and multiply each
            // dimension of the L1 distance by those expanding rows/cols (times their factor)
            let cols_between = if from.0 < to.0 { from.0 + 1 .. to.0 } else { to.0 + 1 .. from.0 };
            let expanded_cols = empty_cols
                .iter()
                .filter(|&c| cols_between.contains(c))
                .count()
                * (expansion_factor - 1);
            debug!("   Extra cols: {}", expanded_cols);

            let rows_between = if from.1 < to.1 { from.1 + 1 .. to.1 } else { to.1 + 1 .. from.1 };
            let expanded_rows = empty_rows
                .iter()
                .filter(|&r| rows_between.contains(r))
                .count()
                * (expansion_factor - 1);
            debug!("   Extra rows: {}", expanded_cols);

            dist += expanded_cols + expanded_rows;
            debug!("-> Full Distance from {:?} to {:?} = {:?}", from, to, dist);
            distances.push(dist);
        }
    }

    distances.into_iter().sum()
}

type AdvInt = usize;

struct Universe {
    map: Vec<u8>,
    width: usize,
    height: usize,
    galaxies: Vec<(i32, i32)>,
}

impl Universe {
    fn from<L: IntoIterator<Item = String>>(lines: L) -> Self {
        let mut width = 0;
        let mut height = 0;
        let map = lines.into_iter()
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
            .filter(|(_, &b)| b == b'#')
            .map(|(i, _)| ((i % width) as i32, (i / width) as i32))
            .collect();

        Universe {
            map,
            width,
            height,
            galaxies,
        }
    }

    fn empty_rows(&self) -> Vec<i32> {
        (0..self.height)
            .filter(|&row| {
                self.map.iter()
                    .enumerate()
                    .filter(|(offset, _)| offset / self.width == row)
                    .all(|(_, &x)| x == b'.')
            })
            .map(|row| row as i32)
            .collect()
    }

    fn empty_cols(&self) -> Vec<i32> {
        (0..self.width)
            .filter(|&col| {
                self.map.iter()
                    .enumerate()
                    .filter(|(offset, _)| offset % self.width == col)
                    .all(|(_, &x)| x == b'.')
            })
            .map(|col| col as i32)
            .collect()
    }
}

testcase!(ex1, solve_part1, "example", 374);
testcase!(part1, solve_part1, "input", 9795148);
testcase!(ex2, solve_part2, "example", 1030, 10);
testcase!(ex3, solve_part2, "example", 8410, 100);
testcase!(part2, solve_part2, "input", 650672493820, 1_000_000);
