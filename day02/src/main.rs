extern crate regex;

use regex::Regex;

use std::io;
use std::io::{BufReader, BufRead};


/* for each line of input, check if the password field matches requirements */
/* TODO: add more detailed explanation */
fn main() -> io::Result<()> {
	/* get standard input as a buffered reader */
	let buf = BufReader::new(io::stdin());

	/* create a regular expression to capture our inputs */
	let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

	/* count the valid passwords under the old and new policies */
	let mut old_valid_count = 0;
	let mut new_valid_count = 0;

	for line in buf.lines() {
		/* capture each component of the password line */
		let text = line.expect("Bad input string").to_string();
		let captured = re.captures(&text).expect("Failed RegEx capture");

		let min = captured
			.get(1).expect("couldn't find lower bound").as_str()
			.parse::<usize>().expect("invalid lower bound");
		let max = captured
			.get(2).expect("couldn't find upper bound").as_str()
			.parse::<usize>().expect("invalid upper bound");
		let letter = captured
			.get(3).expect("couldn't find required character").as_str()
			.parse::<char>().expect("invalid required character");
		let password = captured
			.get(4).expect("couldn't find password string").as_str();

		/* count the occurrences of letter in password for old policy */
		let mut num_chars = 0;
		/* count the character position matches for new policy */
		let mut char_pos_matches = 0;

		for (i, ch) in password.chars().enumerate() {
			/* old policy */
			if ch == letter {
				num_chars += 1;
			}
			/* new policy - mind the 1-indexed location */
			if i == (min-1) && ch == letter {
				char_pos_matches += 1;
			}
			if i == (max-1) && ch == letter {
				char_pos_matches += 1;
			}
		}
	
		/* old policy: check if the letter count requirements are met */
		if num_chars >= min && num_chars <= max {
			old_valid_count += 1;
		}

		/* new policy: only count single location matches */
		if char_pos_matches == 1 {
			new_valid_count += 1;
		}
	}

	/* report how many valid passwords we found */
	println!("{} valid passwords under old policy found", old_valid_count);
	println!("{} valid passwords under new policy found", new_valid_count);
	Ok(())
}
