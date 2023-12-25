#![cfg(not(doctest))]

use std::collections::HashSet;

/// # Point of Incidence
///
/// With your help, the hot springs team locates an appropriate spring which launches you neatly
/// and precisely up to the edge of Lava Island.
///
/// There's just one problem: you don't see any lava.
///
/// You do see a lot of ash and igneous rock; there are even what look like gray mountains
/// scattered around. After a while, you make your way to a nearby cluster of mountains only to
/// discover that the valley between them is completely full of large mirrors. Most of the mirrors
/// seem to be aligned in a consistent way; perhaps you should head in that direction?
///
/// As you move through the valley of mirrors, you find that several of them have fallen from the
/// large metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of
/// the fallen mirrors have lodged into the ash at strange angles. Because the terrain is all one
/// color, it's hard to tell where it's safe to walk or where you're about to run into a mirror.

use super::*;
type AdvInt = usize;

/// You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle
/// input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors
/// are!
///
/// For example:
///
/// ```
/// #.##..##.
/// ..#.##.#.
/// ##......#
/// ##......#
/// ..#.##.#.
/// ..##..##.
/// #.#.##.#.
/// ```
///
/// ```
/// #...##..#
/// #....#..#
/// ..##..###
/// #####.##.
/// #####.##.
/// ..##..###
/// #....#..#
/// ```
///
/// To find the reflection in each pattern, you need to find a perfect reflection across either a
/// horizontal line between two rows or across a vertical line between two columns.
///
/// In the first pattern, the reflection is across a vertical line between two columns; arrows on
/// each of the two columns point at the line between the columns:
///
/// ```
/// 123456789
///     ><
/// #.##..##.
/// ..#.##.#.
/// ##......#
/// ##......#
/// ..#.##.#.
/// ..##..##.
/// #.#.##.#.
///     ><
/// 123456789
/// ```
///
/// In this pattern, the line of reflection is the vertical line between columns 5 and 6. Because
/// the vertical line is not perfectly in the middle of the pattern, part of the pattern (column 1)
/// has nowhere to reflect onto and can be ignored; every other column has a reflected column
/// within the pattern and must match exactly: column 2 matches column 9, column 3 matches 8, 4
/// matches 7, and 5 matches 6.
///
/// The second pattern reflects across a horizontal line instead:
///
/// ```
/// 1 #...##..# 1
/// 2 #....#..# 2
/// 3 ..##..### 3
/// 4v#####.##.v4
/// 5^#####.##.^5
/// 6 ..##..### 6
/// 7 #....#..# 7
/// ```
///
/// This pattern reflects across the horizontal line between rows 4 and 5. Row 1 would reflect with
/// a hypothetical row 8, but since that's not in the pattern, row 1 doesn't need to match
/// anything. The remaining rows match: row 2 matches row 7, row 3 matches row 6, and row 4 matches
/// row 5.
///
/// To summarize your pattern notes, add up the number of columns to the left of each vertical line
/// of reflection; to that, also add 100 multiplied by the number of rows above each horizontal
/// line of reflection. In the above example, the first pattern's vertical line has 5 columns to
/// its left and the second pattern's horizontal line has 4 rows above it, a total of 405.
///
/// Find the line of reflection in each of the patterns in your notes. What number do you get after
/// summarizing all of your notes?
pub fn solve_part1<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    input.into_iter()
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|p| v_mirrors(p).get(0).unwrap_or(&0) + h_mirrors(p).get(0).unwrap_or(&0) * 100)
        .sum()
}

/// # Smudges
///
/// You resume walking through the valley of mirrors and - SMACK! - run directly into one.
/// Hopefully nobody was watching, because that must have been pretty embarrassing.
///
/// Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one `.`
/// or `#` should be the opposite type.
///
/// In each pattern, you'll need to locate and fix the smudge that causes a different reflection
/// line to be valid. (The old reflection line won't necessarily continue being valid after the
/// smudge is fixed.)
///
/// Here's the above example again:
///
/// ```
/// #.##..##.
/// ..#.##.#.
/// ##......#
/// ##......#
/// ..#.##.#.
/// ..##..##.
/// #.#.##.#.
/// ```
///
/// ```
/// #...##..#
/// #....#..#
/// ..##..###
/// #####.##.
/// #####.##.
/// ..##..###
/// #....#..#
/// ```
///
/// The first pattern's smudge is in the top-left corner. If the top-left `#` were instead `.`, it
/// would have a different, horizontal line of reflection:
///
/// ```
/// 1 ..##..##. 1
/// 2 ..#.##.#. 2
/// 3v##......#v3
/// 4^##......#^4
/// 5 ..#.##.#. 5
/// 6 ..##..##. 6
/// 7 #.#.##.#. 7
/// ```
///
/// With the smudge in the top-left corner repaired, a new horizontal line of reflection between
/// rows 3 and 4 now exists. Row 7 has no corresponding reflected row and can be ignored, but every
/// other row matches exactly: row 1 matches row 6, row 2 matches row 5, and row 3 matches row 4.
///
/// In the second pattern, the smudge can be fixed by changing the fifth symbol on row 2 from `.`
/// to `#`:
///
/// ```
/// 1v#...##..#v1
/// 2^#...##..#^2
/// 3 ..##..### 3
/// 4 #####.##. 4
/// 5 #####.##. 5
/// 6 ..##..### 6
/// 7 #....#..# 7
/// ```
///
/// Now, the pattern has a different horizontal line of reflection between rows 1 and 2.
///
/// Summarize your notes as before, but instead use the new different reflection lines. In this
/// example, the first pattern's new horizontal line has 3 rows above it and the second pattern's
/// new horizontal line has 1 row above it, summarizing to the value 400.
///
/// In each pattern, fix the smudge and find the different line of reflection. What number do you
/// get after summarizing the new reflection line in each pattern in your notes?
pub fn solve_part2<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    input.into_iter()
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|pat| -> usize {
            // calculate which mirrors we already had
            let init_v = v_mirrors(pat);
            let init_h = h_mirrors(pat);
            let mut new_v = HashSet::<usize>::new();
            let mut new_h = HashSet::<usize>::new();

            // for each position (row, col) in a pattern
            for row in 0..pat.len() {
                for col in 0..pat.get(0).map_or(0, |line| line.len()) {

                    // create a copy of the original pattern with that position "de-smudged"
                    let mut smudged = pat.to_vec();
                    let mut newrow = smudged[row].clone().into_bytes();
                    newrow.get_mut(col)
                        .map(|ch| *ch = match ch {
                            b'#' => b'.',
                            b'.' => b'#',
                            _ => panic!("mutating invalid character"),
                        });
                    smudged[row] = String::from_utf8(newrow).unwrap();

                    // find any new mirroring points (excluding ones we already knew about)
                    v_mirrors(&smudged)
                        .into_iter()
                        .filter(|x| !init_v.contains(x))
                        .for_each(|x| {
                            debug!("smudge: {row}, {col} --> new vertical mirror found at {x}");
                            new_v.insert(x);
                        });
                    h_mirrors(&smudged)
                        .into_iter()
                        .filter(|x| !init_h.contains(x))
                        .for_each(|x| {
                            debug!("smudge: {row}, {col} --> new horizontal mirror found at {x}");
                            new_h.insert(x);
                        });
                }
            }
            // return the calculation for any new mirror edges
            new_v.into_iter().sum::<usize>() + (100 * new_h.into_iter().sum::<usize>())
        })
        .sum()
}

fn v_mirrors(pat: &[String]) -> Vec<usize> {
    let width = pat.get(0).map(|line| line.len()).unwrap_or(0);
    // for each column that could be a mirror point
    (1..width).filter(|&col| {
        // verify that on all lines
        pat.iter().all(|line| {
            // the left and right characters are mirrored
            let (left, right) = line.split_at(col);
            left.chars()
                .rev()
                .zip(right.chars())
                .all(|(l, r)| l == r)
        })
    }).collect()
}

fn h_mirrors(pat: &[String]) -> Vec<usize> {
    let height = pat.len();
    // for each row that could be a mirror point
    (1..height).filter(|&row| {
        // verify that lines above and below are mirrors
        let (above, below) = pat.split_at(row);
        above.iter()
            .rev()
            .zip(below.iter())
            .all(|(a, b)| a == b)
    }).collect()
}


testcase!(ex1, solve_part1, "example1", 5);
testcase!(ex2, solve_part1, "example2", 400);
testcase!(part1, solve_part1, "input", 33122);
testcase!(ex3, solve_part2, "example1", 300);
testcase!(ex4, solve_part2, "example2", 100);
testcase!(part2, solve_part2, "input", 32312);
