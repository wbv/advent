use std::io;
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
	/* get standard input as a buffered reader */
	let mut buf = BufReader::new(io::stdin());

	/* get the first line */
	let mut linebuf = String::new();
	buf.read_line(&mut linebuf).expect("Failed to read first line");
	let firstline = linebuf.trim();
	println!("{}", firstline);

	/* get the number of characters in the first line */
	//println!("{}", firstline.len());
	let mapwidth = firstline.len();
	println!("{}", mapwidth);

	Ok(())
}
