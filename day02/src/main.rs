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

	let mut valid_count = 0;

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

		/* count the occurrences of letter in password */
		let mut num_chars = 0;
		for ch in password.chars() {
			if ch == letter {
				num_chars += 1;
			}
		}
	
		/* check if the letter count requirements are met */
		if num_chars >= min && num_chars <= max {
			valid_count += 1;
		}
	}

	/* report how many valid passwords we found */
	println!("{} valid passwords found", valid_count);
	Ok(())
}
