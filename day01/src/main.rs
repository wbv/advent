use std::io;
use std::io::{BufReader, BufRead};

/* 1) print the product of two values which sum to 2020 */
/* 2) print the product of three values which sum to 2020 */
fn main() -> io::Result<()> {
	/* get standard input as a buffered reader */
	let buf = BufReader::new(io::stdin());

	/* extract each line into a usize vector */
	let vals: Vec<usize> = buf
		.lines()
		.map(|x| x.expect("Bad text line input").trim().parse::<usize>())
		.filter_map(Result::ok)
		.collect();

	/* if sum of pair of vals is 2020, print their product */
	for (i, a) in vals.iter().enumerate() {
		for b in vals[i+1..].iter() {
			if a + b == 2020 {
				println!("{} * {} = {}", a, b, a * b);
			}
		}
	}

	/* if sum of a triple of vals is 2020, print their product */
	for (i, a) in vals.iter().enumerate() {
		for (j, b) in vals[i+1..].iter().enumerate() {
			for c in vals[j+1..].iter() {
				if a + b + c == 2020 {
					println!("{} * {} * {} = {}", a, b, c, a * b * c);
				}
			}
		}
	}

	Ok(())
}
