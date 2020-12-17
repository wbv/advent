use std::io;
use std::io::{Read};

fn main() -> io::Result<()> {
	// just buffer the whole input file at once
	let mut buf = String::new();
	let filesize = io::stdin().read_to_string(&mut buf)?;
	println!("Read {} bytes.", filesize);

	// vectorize each answer group into a string slice
	let ans: Vec<&str> = buf.split("\n\n").collect();

	// get a count for each unique character in each vector group
	let counts: Vec<usize> = ans
		.iter()
		.map(|group| {
			// empty groups are have one unique character
			if group.is_empty() {
				0
			} else {
				// filter out non-answer characters
				let mut chars: Vec<char> = group.chars()
					.filter(|x| x.is_alphabetic() )
					.collect();
				// filter out non-unique characters (sort + remove adjacent)
				chars.sort();
				chars.windows(2)
					.filter(|ch| ch[0] != ch[1])
					.count()
				+ 1
			}
		})
		.collect();

	println!("Sum of Answers: {}", counts.iter().sum::<usize>());

	Ok(())
}
