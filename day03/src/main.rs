use std::io;
use std::io::{Read};


/* count the number of trees ('#') along the path `right, down` on a map of
 * width `width` represented by the iterator across u8 slices of `lines` */
fn count_trees<'a, I>(lines: I, width: usize, right: usize, down: usize)
-> usize 
where
	I: Iterator<Item = &'a [u8]>,
{
	let trees =	lines
		.step_by(down)
		.enumerate()
		.filter(|(i, line)| line.get(i*right % width) == Some(&('#' as u8)))
		.count();
	println!(
		"{:4} trees found along path of down {} and right {}",
		trees,
		down,
		right
	);
	return trees;
}

fn main() -> io::Result<()> {
	/* just buffer the whole input file at once */
	let mut buf: Vec<u8> = Vec::new();
	let filesize = io::stdin().read_to_end(&mut buf)?;
	println!("Read {} bytes.", filesize);

	/* find width of first line and assume it is the width of all lines */
	let mapwidth = buf.iter().position(|&x| x == '\n' as u8).unwrap();
	println!("Map is {} bytes wide.", mapwidth);

	/* look at each line as a separate vec of u8's, no newlines */
	let lines = buf.split(|x| *x == '\n' as u8);

	/* count the number of trees '#' along the path, where the path will be
	 * down a line, then right by 3, with the map repeating infinitely
	 * rightwards */

	/* formulate paths as (right amount, down amount) pairs */
	let paths = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	let lines = buf.split(|x| *x == '\n' as u8);

	/* compute the product of the number of trees found in each path strat */
	let product = paths
		.iter()
		.map(|path| count_trees(buf.split(|x| *x == '\n' as u8), mapwidth, path.0, path.1))
		.fold(1, |acc, x| acc * x);

	println!("Product of all paths is {}", product);
	
	Ok(())
}
