#![cfg(not(doctest))]

//! # Hot Springs
//!
//! You finally reach the hot springs! You can see steam rising from secluded areas attached to the
//! primary, ornate building.
//!
//! As you turn to enter, the researcher stops you. "Wait - I thought you were looking for the hot
//! springs, weren't you?" You indicate that this definitely looks like hot springs to you.
//!
//! "Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."
//!
//! You look in the direction the researcher is pointing and suddenly notice the massive metal
//! helixes towering overhead. "This way!"
//!
//! It only takes you a few more steps to reach the main gate of the massive fenced-off area
//! containing the springs. You go through the gate and into a small administrative building.
//!
//! "Hello! What brings you to the hot springs today? Sorry they're not very hot right now; we're
//! having a lava shortage at the moment." You ask about the missing machine parts for Desert
//! Island.
//!
//! "Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment, not
//! until we get more lava to heat our forges. And our springs. The springs aren't very springy
//! unless they're hot!"
//!
//! "Say, could you go up and see why the lava stopped flowing? The springs are too cold for normal
//! operation, but we should be able to find one springy enough to launch you up there!"
//!
//! There's just one problem - many of the springs have fallen into disrepair, so they're not
//! actually sure which springs would even be safe to use! Worse yet, their condition records of
//! which springs are damaged (your puzzle input) are also damaged! You'll need to help them repair
//! the damaged records.

use super::*;
type AdvInt = usize;

/// In the giant field just outside, the springs are arranged into rows. For each row, the
/// condition records show every spring and whether it is operational (.) or damaged (#). This is
/// the part of the condition records that is itself damaged; for some springs, it is simply
/// unknown (?) whether the spring is operational or damaged.
///
/// However, the engineer that produced the condition records also duplicated some of this
/// information in a different format! After the list of springs for a given row, the size of each
/// contiguous group of damaged springs is listed in the order those groups appear in the row. This
/// list always accounts for every damaged spring, and each number is the entire size of its
/// contiguous group (that is, groups are always separated by at least one operational spring: ####
/// would always be 4, never 2,2).
///
/// So, condition records with no unknown spring conditions might look like this:
///
/// ```
/// #.#.### 1,1,3
/// .#...#....###. 1,1,3
/// .#.###.#.###### 1,3,1,6
/// ####.#...#... 4,1,1
/// #....######..#####. 1,6,5
/// .###.##....# 3,2,1
/// ```
///
/// However, the condition records are partially damaged; some of the springs' conditions are
/// actually unknown (?). For example:
///
/// ```
/// ???.### 1,1,3
/// .??..??...?##. 1,1,3
/// ?#?#?#?#?#?#?#? 1,3,1,6
/// ????.#...#... 4,1,1
/// ????.######..#####. 1,6,5
/// ?###???????? 3,2,1
/// ```
///
/// Equipped with this information, it is your job to figure out how many different arrangements of
/// operational and broken springs fit the given criteria in each row.
///
/// In the first line (???.### 1,1,3), there is exactly one way separate groups of one, one, and
/// three broken springs (in that order) can appear in that row: the first three unknown springs
/// must be broken, then operational, then broken (#.#), making the whole row #.#.###.
///
/// The second line is more interesting: .??..??...?##. 1,1,3 could be a total of four different
/// arrangements. The last ? must always be broken (to satisfy the final contiguous group of three
/// broken springs), and each ?? must hide exactly one of the two broken springs. (Neither ?? could
/// be both broken springs or they would form a single contiguous group of two; if that were true,
/// the numbers afterward would have been 2,3 instead.) Since each ?? can either be #. or .#, there
/// are four possible arrangements of springs.
///
/// The last line is actually consistent with ten different arrangements! Because the first number
/// is 3, the first and second ? must both be . (if either were #, the first number would have to
/// be 4 or higher). However, the remaining run of unknown spring conditions have many different
/// ways they could hold groups of two and one broken springs:
///
/// ```
/// ?###???????? 3,2,1
/// .###.##.#...
/// .###.##..#..
/// .###.##...#.
/// .###.##....#
/// .###..##.#..
/// .###..##..#.
/// .###..##...#
/// .###...##.#.
/// .###...##..#
/// .###....##.#
/// ```
///
/// In this example, the number of possible arrangements for each row is:
///
/// - `???.### 1,1,3` - 1 arrangement
/// - `.??..??...?##. 1,1,3` - 4 arrangements
/// - `?#?#?#?#?#?#?#? 1,3,1,6` - 1 arrangement
/// - `????.#...#... 4,1,1` - 1 arrangement
/// - `????.######..#####. 1,6,5` - 4 arrangements
/// - `?###???????? 3,2,1` - 10 arrangements
///
/// Adding all of the possible arrangement counts together produces a total of 21 arrangements.
///
/// For each row, count all of the different arrangements of operational and broken springs that
/// meet the given criteria. What is the sum of those counts?
pub fn solve_part1<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    let mut sum = 0;

    for line in input {
        let record: Vec<&str> = line.split_whitespace().collect();
        let (row, groups) = (record[0], record[1]);
        let groups: Vec<usize> = groups.split(',').map(|n| n.parse().unwrap()).collect();
        sum += count_arrangements(row, groups);
    }

    sum
}

pub fn solve_part2<L: IntoIterator<Item = String>>(input: L) -> AdvInt {
    todo!()
}

fn perms(groups: &[usize], len: usize) -> Vec<Vec<usize>> {
    inner_perms(groups, len, 0)
}
fn inner_perms(groups: &[usize], len: usize, depth: usize) -> Vec<Vec<usize>> {
    debug!("{0:1$}inner_perms(): len = {len:2} | {groups:?}", "", 4*depth);
    // no groups, no remaining length ==> no permutations
    if len == 0 || groups.len() == 0 {
        return vec![];
    }
    // minimum amount of length required to fill the rest of `len`
    let min_len = groups.iter().sum::<usize>() + (groups.len() - 1);
    if min_len > len {
        return vec![];
    }

    if groups.len() == 1 {
        let remaining = len.saturating_sub(groups[0]);
        return (0..=remaining).map(|p| vec![p]).collect();
    } else {
        // groups.len() > 1, so apply recursion
        let remaining = len.saturating_sub(groups[0] + 1);
        let mut perms = vec![];
        for i in 0.. {
            let inner = inner_perms(&groups[1..], remaining.saturating_sub(i), depth + 1);
            debug!("{0:1$}==> {inner:?}", "", 4*(depth+1));
            match inner.len() {
                0 => break,
                _ => {
                    let mut this = inner.into_iter().map(|p| [vec![i], p].concat()).collect();
                    perms.append(&mut this);
                },
            }
        }
        perms
    }
}

fn draw_perm(groups: &[usize], len: usize, perm: &[usize]) -> String {
    debug_assert_eq!(groups.len(), perm.len());
    let mut drawn = 0;
    let mut drawing = String::new();
    for i in 0..groups.len() {
        for _ in 0..(perm[i]) {
            drawing.push('.');
            drawn += 1;
        }
        for _ in 0..(groups[i]) {
            drawing.push('#');
            drawn += 1;
        }
        // all springs except the last must be followed by a space
        if i != groups.len() - 1 {
            drawing.push('.');
            drawn += 1;
        }
    }
    for _ in 0..(len.saturating_sub(drawn)) {
        drawing.push('.');
    }
    debug!("{}", drawing);
    drawing
}

fn draw_perms(groups: &[usize], len: usize, perms: &[Vec<usize>]) {
    for perm in perms {
        draw_perm(&groups, len, perm);
    }
}

fn can_perm_fit(line: &str, groups: &[usize], permstr: &str) -> bool {
    let line_bytes = line.as_bytes();
    let perm_bytes = permstr.as_bytes();

    if line_bytes.len() != perm_bytes.len() {
        return false;
    }

    for (&l, &p) in line_bytes.iter().zip(perm_bytes.iter()) {
        match (l, p) {
            (b'#', b'.') => return false,
            (b'.', b'#') => return false,
            (_, _) => continue,
        }
    }

    return true;
}

fn count_arrangements(line: &str, groups: Vec<usize>) -> usize {
    let len = line.len();
    let perms = perms(&groups, len);

    let mut fits = 0;
    for perm in perms {
        let permstr = draw_perm(&groups, len, &perm);
        let can = can_perm_fit(&line, &groups, &permstr);
        if can {
            debug!("FITS!");
            fits += 1;
        } else {
            debug!(" doesnt fit");
        }
        debug!("     {permstr}");
        debug!("  in {line}");
        debug!("");
    }

    fits
}

#[test]
fn permute_ex1() {
    log_init();
    let groups = vec![3, 2, 1];
    let len = 10;
    let p = perms(&groups, len);
    debug!("TESTING: {p:?}");
    draw_perms(&groups, len, &p);
}

#[test]
fn permute_ex2() {
    log_init();
    let groups = vec![2, 5, 3, 4];
    let len = 23;
    let p = perms(&groups, len);
    debug!("TESTING: {p:?}");
    draw_perms(&groups, len, &p);
}

#[test]
fn permute_fit1() {
    log_init();
    let line = String::from("???.###");
    let groups = [1,1,3];
    let len = line.len();
    let perms = perms(&groups, len);

    let mut fits = 0;
    for perm in perms {
        let permstr = draw_perm(&groups, len, &perm);
        let can = can_perm_fit(&line, &groups, &permstr);
        if can {
            debug!("FITS!");
            fits += 1;
        } else {
            debug!(" doesnt fit");
        }
        debug!("     {permstr}");
        debug!("  in {line}");
        debug!("");
    }

    debug_assert_eq!(fits, 1);
}

#[test]
fn permute_fit2() {
    log_init();
    let line = String::from(".??..??...?##.");
    let groups = [1,1,3];
    let len = line.len();
    let perms = perms(&groups, len);

    let mut fits = 0;
    for perm in perms {
        let permstr = draw_perm(&groups, len, &perm);
        let can = can_perm_fit(&line, &groups, &permstr);
        if can {
            debug!("FITS!");
            fits += 1;
        } else {
            debug!(" doesnt fit");
        }
        debug!("     {permstr}");
        debug!("  in {line}");
        debug!("");
    }

    debug_assert_eq!(fits, 4);
}

testcase!(ex1, solve_part1, "example", 21);
testcase!(part1, solve_part1, "input", 6949);
testcase!(ex2, solve_part2, "example", 525152);
testcase!(part2, solve_part2, "input", 0);
