#![cfg(not(doctest))]

use std::io::Lines;

use regex::Regex;

use super::*;

pub fn solve_part1<B: BufRead>(input: B) -> std::io::Result<isize> {
    let mut lines = input.lines();

    // list of seeds
    let seeds = lines.next().unwrap().unwrap()
        .split_once(':').unwrap().1
        .split_whitespace()
        .map(|word| isize::from_str_radix(word, 10).unwrap())
        .collect::<Vec<_>>();
    debug!("Seeds: {:?}", seeds);

    lines.next(); // skip empty line

    let almanac = Almanac::from_lines(&mut lines);

    let min_loc = seeds.iter()
        .map(|&s| almanac.translations.get_loc(s))
        .reduce(|acc, loc| acc.min(loc));

    Ok(min_loc.unwrap())
}

pub fn solve_part2<B: BufRead>(input: B) -> std::io::Result<isize> {
    let mut lines = input.lines();

    // list of seeds
    let seeds = lines.next().unwrap().unwrap()
        .split_once(':').unwrap().1
        .split_whitespace()
        .map(|word| isize::from_str_radix(word, 10).unwrap())
        .collect::<Vec<_>>();
    let seeds = seeds.as_slice().chunks(2)
        .collect::<Vec<_>>();
    debug!("Seeds: {:?}", seeds);

    lines.next(); // skip empty line

    let almanac = Almanac::from_lines(&mut lines);

    let min_loc = seeds.iter()
        .map(|&range| {
            debug_assert_eq!(range.len(), 2);
            almanac.discontinuities.iter()
                .filter(|&d| (range[0]..(range[0] + range[1])).contains(d))
        })
        .flatten()
        .map(|&val| val)
        .chain(seeds.clone()
            .into_iter()
            .map(|range| range[0])
        ).map(|s| almanac.translations.get_loc(s))
        .reduce(|acc, loc| acc.min(loc));

    Ok(min_loc.unwrap())
}

#[derive(Debug)]
struct Offset {
    start: isize,
    end: isize,
    amount: isize,
}

struct Translations {
    inner: [Vec<Offset>; 7],
}

impl Translations {
    pub fn get_loc(&self, seed: isize) -> isize {
        let mut result = seed;
        for trans in &self.inner {
            for offset in trans {
                if result >= offset.start && result < offset.end {
                    result = result + offset.amount;
                    break;
                }
            }
        }
        debug!("Overall: {seed} --> {result}");
        result
    }
}

struct Almanac {
    translations: Translations,
    discontinuities: Vec<isize>
}

impl Almanac {
    pub fn from_lines<B: BufRead>(lines: &mut Lines<B>) -> Self {
        // number-matcher
        let re = Regex::new("([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

        // all translation tables
        let mut translations = Translations {
            inner: [
                Vec::<Offset>::new(), // seed-to-soil
                Vec::<Offset>::new(), // soil-to-fertilizer
                Vec::<Offset>::new(), // fertilizer-to-water
                Vec::<Offset>::new(), // water-to-light
                Vec::<Offset>::new(), // light-to-temperature
                Vec::<Offset>::new(), // temperature-to-humidity
                Vec::<Offset>::new(), // humidity-to-location
            ]
        };

        for trans in &mut translations.inner {
            lines.next(); // skip header line
            while let Some(Ok(line)) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let (_, [to, from, len]) = re.captures(line.as_str())
                    .map(|x| x.extract()).unwrap();
                let (to, from, len) = (
                    isize::from_str_radix(to, 10).unwrap(),
                    isize::from_str_radix(from, 10).unwrap(),
                    isize::from_str_radix(len, 10).unwrap()
                );

                let offset = Offset {
                    start: from,
                    end: from + len,
                    amount: to - from,
                };
                trans.push(offset);
            }
        }

        // compute discontinuities by applying translations inversely
        let mut discontinuities = Vec::<isize>::new();
        //let rtrans = translations.inner.iter().rev();
        for trans in translations.inner.iter().rev() {
            // first, reverse-translate all existing steps
            for disc in &mut discontinuities {
                // search for any applicable translations, apply at most one
                for t in trans {
                    if *disc >= (t.start + t.amount) && *disc < (t.end + t.amount) {
                        *disc -= t.amount;
                        break;
                    }
                }
            }
            // add new discontinuities from the current translation
            for t in trans {
                discontinuities.push(t.start);
                discontinuities.push(t.end);
            }
        }

        Self { translations, discontinuities }
    }
}
