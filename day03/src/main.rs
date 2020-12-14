use std::io;
use std::io::{Read};


fn main() -> io::Result<()> {
	/* just buffer the whole input file at once */
	let mut buf: Vec<u8> = Vec::new();
	let filesize = io::stdin().read_to_end(&mut buf)?;
	println!("Read {} bytes.", filesize);

	/* find width of first line and assume it is the width of all lines */
	let mapwidth = buf.iter().position(|&x| x == '\n' as u8).unwrap();
	println!("Map is {} bytes wide.", mapwidth);

	/* get a buffer with all newlines removed */
	let lines = buf.split(|x| *x == '\n' as u8);

	/* count the number of trees '#' along the path, where the path will be
	 * down a line, then right by 3, with the map repeating infinitely
	 * rightwards */
	let right = 3;
	let trees = lines
		.enumerate()
		.filter(|(i, line)| line.get(i*right % mapwidth) == Some(&('#' as u8)))
		.count();

	println!(
		"Found {} trees along path of down 1 and right {}",
		trees,
		right
	);

	Ok(())
}
