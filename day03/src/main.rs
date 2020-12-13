use std::io;
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
	/* get standard input as a buffered reader */
	let buf = BufReader::new(io::stdin());

	Ok(())
}
