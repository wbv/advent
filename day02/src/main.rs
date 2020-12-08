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

		/* parse each matched component and print them out */
		println!("{} to {} of {} inside {}", min, max, letter, password);
	}
	
	Ok(())
}
