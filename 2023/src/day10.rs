//! Pipe Maze
//! ---------
//!
//! You use the hang glider to ride the hot air from Desert Island all the way up to the floating
//! metal island. This island is surprisingly cold and there definitely aren't any thermals to
//! glide on, so you leave your hang glider behind.
//!
//! You wander around for a while, but you don't find any people or animals. However, you do
//! occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction;
//! maybe you can find someone at the hot springs and ask them where the desert-machine parts are
//! made.
//!
//! The landscape here is alien; even the flowers and trees are made of metal. As you stop to
//! admire some metal grass, you notice something metallic scurry away in your peripheral vision
//! and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better
//! look, you'll need to get ahead of it.

#![cfg(not(doctest))]

use super::*;

/// # Furthest Distance
///
/// Scanning the area, you discover that the entire field you're standing on is densely packed with
/// pipes; it was hard to tell at first because they're the same metallic silver color as the
/// "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).
///
/// The pipes are arranged in a two-dimensional grid of tiles:
///
/// - `|` is a vertical pipe connecting north and south.
/// - `-` is a horizontal pipe connecting east and west.
/// - `L` is a 90-degree bend connecting north and east.
/// - `J` is a 90-degree bend connecting north and west.
/// - `7` is a 90-degree bend connecting south and west.
/// - `F` is a 90-degree bend connecting south and east.
/// - `.` is ground; there is no pipe in this tile.
/// - `S` is the starting position of the animal; there is a pipe on this tile, but your sketch
///   doesn't show what shape the pipe has.
///
/// Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the
/// animal is one large, continuous loop.
///
/// For example, here is a square loop of pipe:
///
/// ```
/// .....
/// .F-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// If the animal had entered this loop in the northwest corner, the sketch would instead look like
/// this:
///
/// ```
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the
/// adjacent pipes connect to it.
///
/// Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows
/// the same loop as above:
///
/// ```
/// -L|F7
/// 7S-7|
/// L|7||
/// -L-J|
/// L|-JF
/// ```
///
/// In the above diagram, you can still figure out which pipes form the main loop: they're the ones
/// connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every
/// pipe in the main loop connects to its two neighbors (including S, which will have exactly two
/// pipes connecting to it, and which is assumed to connect back to those two pipes).
///
/// Here is a sketch that contains a slightly more complex main loop:
///
/// ```
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
///
/// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
///
/// ```
/// 7-F7-
/// .FJ|7
/// SJLL7
/// |F--J
/// LJ.LJ
/// ```
///
/// If you want to get out ahead of the animal, you should find the tile in the loop that is
/// farthest from the starting position. Because the animal is in the pipe, it doesn't make sense
/// to measure this by direct distance. Instead, you need to find the tile that would take the
/// longest number of steps along the loop to reach from the starting point - regardless of which
/// way around the loop the animal went.
///
/// In the first example with the square loop:
///
/// ```
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// You can count the distance each tile in the loop is from the starting point like this:
///
/// ```
/// .....
/// .012.
/// .1.3.
/// .234.
/// .....
/// ```
///
/// In this example, the farthest point from the start is 4 steps away.
///
/// Here's the more complex loop again:
///
/// ```
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
///
/// Here are the distances for each tile on that loop:
///
/// ```
/// ..45.
/// .236.
/// 01.78
/// 14567
/// 23...
/// ```
///
/// Find the single giant loop starting at S. How many steps along the loop does it take to get
/// from the starting position to the point farthest from the starting position?
pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<AdvInt> {
    let mut maze = PipeMaze::from(input.lines().map(|l| l.expect("i/o error on lines")));
    Ok(maze.traverse())
}

/// # Enclosed Area
///
/// You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest
/// is within the area enclosed by the loop?
///
/// To determine whether it's even worth taking the time to search for such a nest, you should
/// calculate how many tiles are contained within the loop. For example:
///
/// ```
/// ...........
/// .S-------7.
/// .|F-----7|.
/// .||.....||.
/// .||.....||.
/// .|L-7.F-J|.
/// .|..|.|..|.
/// .L--J.L--J.
/// ...........
/// ```
///
/// The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast
/// (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same
/// loop again with those regions marked:
///
/// ```
/// ...........
/// .S-------7.
/// .|F-----7|.
/// .||OOOOO||.
/// .||OOOOO||.
/// .|L-7OF-J|.
/// .|II|O|II|.
/// .L--JOL--J.
/// .....O.....
/// ```
///
/// In fact, there doesn't even need to be a full tile path to the outside for tiles to count as
/// outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop
/// and O is still outside the loop:
///
/// ```
/// ..........
/// .S------7.
/// .|F----7|.
/// .||OOOO||.
/// .||OOOO||.
/// .|L-7F-J|.
/// .|II||II|.
/// .L--JL--J.
/// ..........
/// ```
///
/// In both of the above examples, 4 tiles are enclosed by the loop.
///
/// Here's a larger example:
///
/// ```
/// .F----7F7F7F7F-7....
/// .|F--7||||||||FJ....
/// .||.FJ||||||||L7....
/// FJL7L7LJLJ||LJ.L-7..
/// L--J.L7...LJS7F-7L7.
/// ....F-J..F7FJ|L7L7L7
/// ....L7.F7||L7|.L7L7|
/// .....|FJLJ|FJ|F7|.LJ
/// ....FJL-7.||.||||...
/// ....L---J.LJ.LJLJ...
/// ```
///
/// The above sketch has many random bits of ground, some of which are in the loop (I) and some of
/// which are outside it (O):
///
/// ```
/// OF----7F7F7F7F-7OOOO
/// O|F--7||||||||FJOOOO
/// O||OFJ||||||||L7OOOO
/// FJL7L7LJLJ||LJIL-7OO
/// L--JOL7IIILJS7F-7L7O
/// OOOOF-JIIF7FJ|L7L7L7
/// OOOOL7IF7||L7|IL7L7|
/// OOOOO|FJLJ|FJ|F7|OLJ
/// OOOOFJL-7O||O||||OOO
/// OOOOL---JOLJOLJLJOOO
/// ```
///
/// In this larger example, 8 tiles are enclosed by the loop.
///
/// Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's
/// another example with many bits of junk pipe lying around that aren't connected to the main loop
/// at all:
///
/// ```
/// FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJ7F7FJ-
/// L---JF-JLJ.||-FJLJJ7
/// |F|F-JF---7F7-L7L|7|
/// |FFJF7L7F-JF7|JL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L
/// ```
///
/// Here are just the tiles that are enclosed by the loop marked with I:
///
/// ```
/// FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJIF7FJ-
/// L---JF-JLJIIIIFJLJJ7
/// |F|F-JF---7IIIL7L|7|
/// |FFJF7L7F-JF7IIL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L
/// ```
///
/// In this last example, 10 tiles are enclosed by the loop.
///
/// Figure out whether you have time to search for the nest by calculating the area within the
/// loop. How many tiles are enclosed by the loop?
///
pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<AdvInt> {
    let mut maze = PipeMaze::from(input.lines().map(|l| l.expect("i/o error on lines")));
    maze.traverse();

    debug!(">>>>> Traverse done, performing longitudinal collision detection' <<<<<");
    let mut enclosed = 0;
    for y in 0..(maze.flatmap.len() / maze.width) as isize {
        // for each line, keep track of whether we've entered or exited the loop
        // by determining which way the loop entered and exited our line
        let mut crossed: Crossed = false.into();
        let mut inside = false;
        debug!("Examining Line: {y}");
        for x in 0..maze.width as isize {
            // if we crossed over the loop completely after leaving the last tile, we're either
            // gone from inside to outside, or outside to inside.
            if crossed.over() {
                warn!("CROSSED");
                inside = !inside;
                crossed = false.into();
            }
            // if we're on a loop tile, record which ways the pipe is connected
            if maze.get(x, y).depth.is_some() {
                let dirs = maze.connected_dirs(Coord { x, y });
                crossed.north ^= dirs.contains(&North);
                crossed.south ^= dirs.contains(&South);
                debug!("Pipe {dirs:?} => {crossed:?}");
            } else {
                // if we're not on a loop tile, forget any directions were crossed
                crossed = false.into();

                if inside {
                    info!(" -> TILE INSIDE");
                    enclosed += 1
                } else {
                    debug!("<-  tile outside");
                }
            }
        }
    }

    Ok(enclosed)
}

/// Numeric type used for the answer to this puzzle
type AdvInt = usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pipe {
    // Some(depth) from start pipe. None when not seen in traversal.
    depth: Option<AdvInt>,
    kind: u8,
}

impl Pipe {
    fn dirs(self: &Pipe) -> Vec<Direction> {
        match self.kind {
          b'|' => vec![North, South],
          b'-' => vec![East, West],
          b'L' => vec![North, East],
          b'J' => vec![North, West],
          b'7' => vec![South, West],
          b'F' => vec![South, East],
          b'.' => vec![],
          b'S' => vec![North, South, East, West],
          _ => panic!("bad pipe ({}) dirs loopup", self.kind as char),
        }
    }
}

impl From<u8> for Pipe {
    fn from(byte: u8) -> Pipe {
        match byte {
            b'|'
          | b'-'
          | b'L'
          | b'J'
          | b'7'
          | b'F'
          | b'.' => Pipe { kind: byte, depth: None },
            b'S' => Pipe { kind: byte, depth: Some(0) },
            _ => panic!("invalid pipe character: {}", byte as char),
        }
    }
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.kind as char)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

impl Direction {
    fn rev(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}


#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<Direction> for Coord {
    fn from(d: Direction) -> Coord {
        match d {
            North => Coord { x:  0, y: -1 },
            South => Coord { x:  0, y:  1 },
            East =>  Coord { x:  1, y:  0 },
            West =>  Coord { x: -1, y:  0 },
        }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


struct PipeMaze {
    width: usize,
    flatmap: Vec<Pipe>,
}

impl PipeMaze {
    fn from<B: Iterator<Item = String>>(lines: B) -> PipeMaze {
        let (mut width, mut height) = (0, 0);
        let mut flatmap = vec![];

        let lines = lines.map(|l| {
            l.as_bytes()
                .to_owned()
                .iter()
                .map(|&b| Pipe::from(b))
                .collect::<Vec<Pipe>>()
        });
        for mut line in lines {
            width = width.max(line.len());
            debug_assert_eq!(width, line.len()); // double-check for even map widths
            height += 1;
            flatmap.append(&mut line);
        }
        // double-check that flattened grid dimensions match 2D grid
        debug_assert_eq!(height * width, flatmap.len());

        // double-check that we only have a single start-pipe
        debug_assert_eq!(flatmap.iter().filter(|p| p.kind == b'S').count(), 1);

        debug!("the map:");
        for y in 0..height {
            let line = (0..width).map(|x| flatmap[y * width + x].kind as char).collect::<String>();
            debug!("  {}", line);
        }

        PipeMaze {
            width,
            flatmap,
        }
    }

    fn get(&self, x: isize, y: isize) -> Pipe {
        if x < 0 || y < 0 {
            return Pipe::from(b'.');
        }

        let i = (y as usize * self.width) + x as usize;
        match self.flatmap.get(i) {
            Some(&p) => p,
            _ => Pipe::from(b'.'),
        }
    }

    fn try_get_mut(&mut self, x: isize, y: isize) -> Option<&mut Pipe> {
        let i = (y as usize * self.width) + x as usize;
        self.flatmap.get_mut(i)
    }

    fn at(&self, coord: Coord) -> Pipe {
        self.get(coord.x, coord.y)
    }

    fn try_at_mut(&mut self, coord: Coord) -> Option<&mut Pipe> {
        self.try_get_mut(coord.x, coord.y)
    }

    fn start(&self) -> Coord {
        let index = self.flatmap.iter()
            .enumerate()
            .find(|(_, p)| p.kind == b'S')
            .expect("no start in maze")
            .0;
        Coord {
            x: (index % self.width) as isize,
            y: (index / self.width) as isize,
        }
    }

    fn connecting(&self, coord: Coord) -> Vec<Coord> {
        self.at(coord)
            .dirs()
            .into_iter()
            .filter_map(|dir| {
                let next = coord + dir.into();
                let here = self.at(coord);
                let there = self.at(next);
                debug!("Checking: {:?} at {:?} ({:?}) going {:?} = {:?} at {:?} ({:?})", here, coord, here.depth, dir, there, next, there.depth);
                if there.depth.is_none() && self.can_connect(coord, dir, next) {
                    info!("Connection: {:?} -> {:?}", coord, next);
                    Some(next)
                } else {
                    None
                }
            })
            .collect()
    }

    fn can_connect(&self, from: Coord, dir: Direction, to: Coord) -> bool {
        self.at(from).dirs().contains(&dir) && self.at(to).dirs().contains(&dir.rev())
    }

    fn connected_dirs(&self, coord: Coord) -> Vec<Direction> {
        let here = self.at(coord);
        if here.depth.is_none() {
            return vec![];
        }

        self.at(coord)
            .dirs()
            .into_iter()
            .filter(|&dir| {
                let neighbor = coord + dir.into();
                self.at(neighbor).depth.is_some() && self.can_connect(coord, dir, neighbor)
            })
            .collect()
    }

    /// Traverses the pipes, setting depth (distance from start) on all pipes visited.
    fn traverse(&mut self) -> usize {
        let start = self.start();

        let mut paths = self.connecting(start);
        let mut depth = 1;
        while !paths.is_empty() {
            debug!("==> DEPTH: {depth}");

            // visit each current coordinate in the path
            for coord in &paths {
                let here = self.try_at_mut(*coord).unwrap();
                here.depth = Some(depth);
                warn!("{:?} at {:?} is now seen with depth {:?}", self.at(*coord), coord, self.at(*coord).depth);
            }

            // for each coordinate, find non-traversed connecting coordinates and "traverse" them
            paths = paths.into_iter()
                .flat_map(|coord| self.connecting(coord))
                .collect::<Vec<_>>();

            depth += 1;
        }

        self.flatmap.iter().fold(0, |acc, p| p.depth.map_or(acc, |d| d.max(acc)))
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Crossed { north: bool, south: bool }

impl Crossed {
    fn over(&self) -> bool {
        self.north && self.south
    }
}

impl From<bool> for Crossed {
    fn from(value: bool) -> Self {
        Crossed { north: value, south: value }
    }
}
