#![cfg(not(doctest))]


//! # Haunted Wasteland
//!
//! You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching.
//! When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just
//! finished warning you about ghosts a few minutes ago.
//!
//! One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle
//! input) about how to navigate the desert. At least, you're pretty sure that's what they are; one
//! of the documents contains a list of left/right instructions, and the rest of the documents seem
//! to describe some kind of network of labeled nodes.

use std::collections::HashMap;

use regex::Regex;

use super::*;

/// ## Follow The Map
///
/// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps
/// if you have the camel follow the same instructions, you can escape the haunted wasteland!
///
/// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is
/// where you are now, and you have to follow the left/right instructions until you reach ZZZ.
///
/// This format defines each node of the network individually. For example:
///
/// ```
/// RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting with AAA, you need to look up the next element based on the next left/right
/// instruction in your input. In this example, start with AAA and go right (R) by choosing the
/// right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following
/// the left/right instructions, you reach ZZZ in 2 steps.
///
/// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat
/// the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on.
/// For example, here is a situation that takes 6 steps to reach ZZZ:
///
/// ```
/// LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<isize> {
    let mut lines = input.lines();
    let instruction = lines.next()
        .expect("Premature end of input file")?
        .chars()
        .collect::<Vec<_>>();

    // grab the next line, verify it's empty
    debug_assert!(lines.next().expect("early EOF")?.is_empty());

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let map = lines.map(|s| {
        let line = s.unwrap();
        let (_, dirs) = re.captures(line.as_str()).unwrap().extract();
        debug_assert_eq!(dirs.len(), 3);
        let [start, left, right] = dirs.map(|word| word.try_into().expect("invalid direction"));
        (start, (left, right))
    }).collect::<HashMap<Node, (Node, Node)>>();

    debug!("Directions (repeated 10x): {:?}", instruction);
    debug!("The Map: {map:?}");

    let mut node: Node = "AAA".try_into().unwrap();

    todo!("Continue this later: traverse the map")
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Node([char; 3]);
impl TryFrom<&[u8]> for Node {
    type Error = ();
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 3 {
            Err(())
        } else {
            let chars = [bytes[0] as char, bytes[1] as char, bytes[2] as char];
            Ok(Node(chars))
        }
    }
}

impl TryFrom<&str> for Node {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 3 {
            Err(())
        } else {
            let mut chars = s.chars();
            let array = [chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap()];
            Ok(Node(array))
        }
    }
}

pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<isize> {
    let mut lines = input.lines();

    unimplemented!()
}
