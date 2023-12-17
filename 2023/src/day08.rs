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

    let mut traversals = map.keys()
        .filter_map(|&n| if n.is_start() { Some(Traversal::new(&n)) } else { None })
        .collect::<Vec<Traversal>>();
    // let mut instructions = instruction.iter().cycle();

    debug!("Instructions: {}", String::from_iter(instruction.iter()));
    debug!("Instructions Length: {}", instruction.len());

    for iloop in 0.. {
        for (instr_idx, instr) in instruction.iter().enumerate() {

            // for each starting node's traversal
            for t in traversals.iter_mut() {
                // move it along the map according to the current direction
                t.node = match &instr {
                    'L' => map[&t.node].0,
                    'R' => map[&t.node].1,
                    _ => panic!("bad direction"),
                };

                // record if we visit an acceptable end-node
                if t.node.is_end() {
                    // track end-nodes uniquely by which instruction in the cycle of instructions
                    // we found them at, noting how many cycles through all instructions we've
                    // been through at each visit
                    t.ends.entry(End { instr_idx, node: t.node })
                        .and_modify(|iloop_idxs| iloop_idxs.push(iloop))
                        .or_insert(vec![iloop]);

                    // let next_node = match instruction[(instr_idx + 1) % instruction.len()] {
                    //     'L' => map[&t.node].0,
                    //     'R' => map[&t.node].1,
                    //     _ => panic!("predictive bad direction"),
                    // };
                    // debug!("BTW: traversal {:?} has next move ({:?} --> {:?})", t.start_node, t.node, next_node);
                }
            }
        }

        debug!("");
        debug!("---> instruction loop = {iloop}");
        for t in traversals.iter() {
            debug!("Node: {:?}", t.start_node);
            for (end, loop_idxs) in t.ends.iter() {
                // if we find an end-node that has been visited twice on the same instruction then
                // we know that all other end-nodes (if any) for this traversal will repeat on the
                // same pattern (at the same distance).
                if loop_idxs.len() > 1 {
                    debug!("  LOOP AT {:?} | instr_idx: {:?} ... {:?}", end.node, end.instr_idx, loop_idxs);
                    let cycle_start = loop_idxs[0];
                    let cycle_len = loop_idxs[1] - loop_idxs[0];
                    debug!("               | (cycle start: {cycle_start}, cycle length: {cycle_len})");
                    let loop_start = end.instr_idx + (loop_idxs[0] * instruction.len());
                    let loop_len = (loop_idxs[1] - loop_idxs[0]) * instruction.len();
                    debug!("               | (step start: {loop_start}, step length: {loop_len})");
                } else {
                    debug!("   end at {:?} | instr_idx: {:?} ... {:?}", end.node, end.instr_idx, loop_idxs);
                    let cycle_start = loop_idxs[0];
                    debug!("               | (cycle start: {cycle_start})");
                    let loop_start = end.instr_idx + (loop_idxs[0] * instruction.len());
                    debug!("               | (step start: {loop_start})");
                }
            }
        }

        // once all traversals have found the "true" loop (i.e. we land twice on the same
        // node/instruction-#-of-cycle pair), we can stop the sequential search
        if traversals.iter().all(|t| t.ends.len() > 0 && t.ends.values().all(|end| end.len() > 1)) {
            break;
        }
    }

    // find instruction indices all (cycle_idxs) upon which we found a loop
    let mut cycle_idxs = traversals.iter()
        .map(|t| t.ends.keys().map(|e| e.instr_idx))
        .flatten()
        .collect::<Vec<_>>();
    cycle_idxs.sort();
    cycle_idxs.dedup();
    debug!("Unique cycle indices: {cycle_idxs:?}");

    // find which idxs aren't met by a loop in every traversal
    cycle_idxs.retain(|&idx| {
        traversals.iter().all(|t| t.ends.keys().any(|e| e.instr_idx == idx))
    });
    debug!("Unique AND APPLICABLE cycle indices: {cycle_idxs:?}");

    // remove those from the traversals list
    for t in traversals.iter_mut() {
        t.ends.retain(|end, _| cycle_idxs.contains(&end.instr_idx));
    }

    debug!("Remaining traversals:");
    for t in traversals.iter() {
        debug!(" {:?}", t.start_node);
        for (e, ps) in t.ends.iter() {
            debug!("  {:?} : {:?}", e, ps);
        }
    }

    // pretty sure we only have one to check, but make sure of it first
    debug_assert_eq!(cycle_idxs.len(), 1);
    let cycle_idx = cycle_idxs[0];
    let lcm = traversals.iter()
        .map(|t| {
            let ends = t.ends.keys()
                .filter(|e| e.instr_idx == cycle_idx)
                .collect::<Vec<_>>();
            // verify there's only one loop at this cycle index per traversal
            debug_assert_eq!(ends.len(), 1);
            let cycle = t.ends[ends[0]].clone();

            // also make sure that the loops happen exactly one before the entire sequence starts
            // over
            let first = (cycle[0] + 1) as u128;
            let size = (cycle[1] - cycle[0]) as u128;
            debug!("LOOP on cycles: {:?} (size {:?})", first, size);
            debug_assert_eq!(first, size);
            first
        })
        .reduce(|mut lcm, mut cur| {
            debug!("doing LCM: {lcm} and {cur}");
            // INEFFICIENT LCM BELOW
            let lcm_step = lcm;
            let cur_step = cur;
            while lcm != cur {
                if lcm < cur {
                    lcm += lcm_step;
                } else {
                    cur += cur_step;
                }
            }

            lcm
    });

    let lcm = lcm.unwrap();
    let steps = (lcm - 1) * (instruction.len() as u128) + cycle_idx as u128;
    Ok(steps)

    //let ans = traversals.iter().map(|t|
    //    t.ends.values()
    //        .map(|idxs| idxs[1] - idxs[0])
    //        .collect::<Vec<_>>())
    //    .reduce(|lcm, current| {
    //    // INEFFICIENT SOLUTION: find the first step where 'all' current elements overlap
    //    let mut left = lcm;
    //    let mut right = current;
    //    while left != right {
    //        if left < right {
    //            left += all.num_elements;
    //        } else {
    //            right += cur.num_elements;
    //        }
    //    }
    //    debug!("{left} == {right}");
    //    let start = left;
    //    let num_elements = start - all.start;
    //    debug!("with num_elements {num_elements}");
    //    Loop { start, num_elements }
    //});

    // while traversals.iter().any(|t| t.path_loop.is_none()) {
    //     //debug!("TRAVERSAL ITERATION = {:?}", traversals.iter().map(|t| t.node).collect::<Vec<_>>());
    //     let next = instructions.next();
    //     for t in traversals.iter_mut().filter(|t| t.path_loop.is_none()) {
    //         t.node = match next {
    //             Some(&'L') => map[&t.node].0,
    //             Some(&'R') => map[&t.node].1,
    //             _ => panic!("BAD DIRECTION"),
    //         };
    //         //debug!("node {:?} has seen {:?}", t.node, t.seen);

    //         // check if we're on an end-node
    //         if t.node.is_end() {
    //             // check if we visited this end-node already
    //             for (i, n) in t.seen.iter().enumerate() {
    //                 if t.node == *n {
    //                     let path_loop = Loop {
    //                         start: i as isize,
    //                         num_elements: t.seen.len() as isize - i as isize,
    //                     };
    //                     debug!("TERMINAL LOOP: {:?} to {:?}", n, t.node);
    //                     debug!("(which is): {:?}", path_loop);
    //                     debug!("Sanity: that loop is {:?}", t.seen);
    //                     t.path_loop = Some(path_loop);
    //                     break;
    //                 }
    //             }
    //         }
    //         t.seen.push(t.node);
    //     }
    // }

    // let paths = traversals.iter().map(|t| t.path_loop.unwrap()).collect::<Vec<_>>();
    // debug!(">>> {paths:?}");

    // let ans = paths.into_iter().reduce(|all, cur| {
    //     debug!("reducing {cur:?} into {all:?}");

    //     // INEFFICIENT SOLUTION: find the first step where 'all' current elements overlap
    //     let mut left = all.start;
    //     let mut right = cur.start;
    //     while left != right {
    //         if left < right {
    //             left += all.num_elements;
    //         } else {
    //             right += cur.num_elements;
    //         }
    //     }
    //     debug!("{left} == {right}");
    //     let start = left;
    //     let num_elements = start - all.start;
    //     debug!("with num_elements {num_elements}");
    //     Loop { start, num_elements }
    // });


    // // start is the first time everyone "collectively loops"
    // Ok(ans.map(|x| x.start ).unwrap_or(0))
}

// #[derive(Clone, Copy, Debug)]
// struct Loop {
//     start: isize,
//     num_elements: isize,
// }

#[derive(PartialEq, Eq, Hash, Debug)]
struct End {
    instr_idx: usize,
    node: Node,
}

struct Traversal {
    start_node: Node,
    node: Node,
    // seen: Vec<Node>,
    // path_loop: Option<Loop>,
    ends: HashMap<End, Vec<usize>>,
}

impl Traversal {
    fn new(node: &Node) -> Self {
        Self {
            start_node: *node,
            node: *node,
            // seen: vec![*node],
            // path_loop: None,
            ends: HashMap::new(),
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
