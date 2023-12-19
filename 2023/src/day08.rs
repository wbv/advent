//! Haunted Wasteland
//! -----------------
//!
//! You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching.
//! When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just
//! finished warning you about ghosts a few minutes ago.
//!
//! One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle
//! input) about how to navigate the desert. At least, you're pretty sure that's what they are; one
//! of the documents contains a list of left/right instructions, and the rest of the documents seem
//! to describe some kind of network of labeled nodes.

#![cfg(not(doctest))]

use std::collections::HashMap;

use super::*;

/// # Follow The Map
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

    let map = lines.filter_map(|line| {
        let line = line.expect("I/O error on line");
        let bytes: [u8; 16] = line.as_bytes().try_into().ok()?;
        let node = bytes[0..3].try_into().unwrap();
        let left = bytes[7..10].try_into().unwrap();
        let right = bytes[12..15].try_into().unwrap();
        Some((node, (left, right)))
    }).collect::<HashMap<Node, (Node, Node)>>();


    debug!("The Map: {map:?}");

    let mut node: Node = "AAA".try_into().unwrap();
    let end: Node = "ZZZ".try_into().unwrap();
    let mut instructions = instruction.iter().cycle();
    let mut steps = 0;

    while node != end {
        node = match instructions.next() {
            Some(&'L') => map[&node].0,
            Some(&'R') => map[&node].1,
            _ => panic!("BAD DIRECTION"),
        };
        steps += 1;
    }

    Ok(steps)
}

/// --- Part Two ---
///
/// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the
/// camel follow the instructions, but you've barely left your starting position. It's going to
/// take significantly more steps to escape!
///
/// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the
/// laws of spacetime? Only one way to find out.
///
/// After examining the maps a bit longer, your attention is drawn to a curious fact: the number of
/// nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd
/// probably just start at every node that ends with A and follow all of the paths at the same time
/// until they all simultaneously end up at nodes that end with Z.
///
/// For example:
/// ```
/// LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)
/// ```
///
/// Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow
/// each left/right instruction, use that instruction to simultaneously navigate away from both
/// nodes you're currently on. Repeat this process until all of the nodes you're currently on end
/// with Z. (If only some of the nodes you're on end with Z, they act like any other node and you
/// continue as normal.) In this example, you would proceed as follows:
///
/// - Step 0: You are at 11A and 22A.
/// - Step 1: You choose all of the left paths, leading you to 11B and 22B.
/// - Step 2: You choose all of the right paths, leading you to 11Z and 22C.
/// - Step 3: You choose all of the left paths, leading you to 11B and 22Z.
/// - Step 4: You choose all of the right paths, leading you to 11Z and 22B.
/// - Step 5: You choose all of the left paths, leading you to 11B and 22C.
/// - Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
///
/// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
///
/// Simultaneously start on every node that ends with A. How many steps does it take before you're
/// only on nodes that end with Z?
pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<u128> {
    let mut lines = input.lines();
    let instruction = lines.next()
        .expect("Premature end of input file")?
        .chars()
        .collect::<Vec<_>>();

    // grab the next line, verify it's empty
    debug_assert!(lines.next().expect("early EOF")?.is_empty());

    let map = lines.filter_map(|line| {
        let line = line.expect("I/O error on line");
        let bytes: [u8; 16] = line.as_bytes().try_into().ok()?;
        let node = bytes[0..3].try_into().unwrap();
        let left = bytes[7..10].try_into().unwrap();
        let right = bytes[12..15].try_into().unwrap();
        Some((node, (left, right)))
    }).collect::<HashMap<Node, (Node, Node)>>();

    debug!("The Map:");
    for (n, (l, r)) in map.iter() {
        debug!("  {n:?} = {l:?}, {r:?}");
    }

    // figure out each starting node
    let mut traversals = map.keys()
        .filter_map(|&n| if n.is_start() { Some(Traversal::new(&n)) } else { None })
        .collect::<Vec<Traversal>>();
    // our instructions form an infinite loop
    let mut instructions = instruction.iter().cycle();

    debug!("Instructions: {}", String::from_iter(instruction.iter()));
    debug!("Instructions Length: {}", instruction.len());

    // run our iterations until every pair has looped once
    let mut step = 0u128;
    while traversals.iter().any(|t| t.repeat.is_none()) {

        // step 1 = the location after the first traverse
        let dir = instructions.next().unwrap();
        step += 1;

        // for each starting node's traversal
        for t in traversals.iter_mut() {
            // move it along the map according to the current direction
            t.node = match &dir {
                'L' => map[&t.node].0,
                'R' => map[&t.node].1,
                _ => panic!("bad direction"),
            };

            // record if we visit an acceptable end-node
            if t.node.is_end() {
                t.ends.push(step);

                // if we see an end node twice, we assume it's a loop
                if t.ends.len() == 2 {
                    t.repeat = Some(Repeat {
                        start: t.ends[0],
                        end: t.ends[1],
                    });
                }
            }
        }
    }

    // verify an assumption about our input (to make calculation easier)
    let uniform_loops = traversals.iter().all(|t| t.repeat.is_some_and(|x| x.end - x.start == x.start));
    debug_assert!(uniform_loops);
    let lcm = traversals.iter_mut()
        .map(|t| t.repeat.unwrap().start)
        .reduce(|lcm, mut cur| {
            // fun fact, the loops are prime numbers times the instruction length
            // (but NOT in the example code, so we have to condition it here >.>)
            if cur % (instruction.len() as u128) == 0 && cur != instruction.len() as u128 {
                debug!("shortening cur {cur} to {}", cur / (instruction.len() as u128));
                cur /= instruction.len() as u128
            };
            lcm * cur
        });

    Ok(lcm.unwrap())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Repeat {
    start: u128,
    end: u128,
}

struct Traversal {
    node: Node,
    ends: Vec<u128>,
    repeat: Option<Repeat>,
}

impl Traversal {
    fn new(node: &Node) -> Self {
        Self {
            node: *node,
            ends: vec![],
            repeat: None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl TryFrom<&[u8]> for Node {
    type Error = String;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 3 {
            Err("invalid length".into())
        } else {
            Ok(Node(bytes.try_into().unwrap()))
        }
    }
}

impl TryFrom<&str> for Node {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 3 {
            Err(format!("invalid node length of {}", s.len()))
        } else {
            let node = s.as_bytes().try_into().unwrap();
            if s.chars().any(|ch| !ch.is_ascii()) {
                Err("non-ascii node character".into())
            } else {
                Ok(node)
            }
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let chars = self.0.map(|b| b as char);
        write!(f, "{}", String::from_iter(chars))
    }
}

impl Node {
    fn is_start(&self) -> bool {
        self.0[2] == b'A'
    }

    fn is_end(&self) -> bool {
        self.0[2] == b'Z'
    }
}
