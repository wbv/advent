use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	// just buffer the whole input file at once
	let mut buf = String::new();
	let filesize = io::stdin().read_to_string(&mut buf)?;
	println!("Read {} bytes.", filesize);

	// vectorize each answer group into a string slice
	let answers: Vec<&str> = buf.split("\n\n").collect();

	// get a count for each unique character in each vector group
	let any = answers
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
				// filter out non-unique characters
				chars.sort();

				// by removing duplicate adjacents and counting
				chars.windows(2)
					.filter(|ch| ch[0] != ch[1])
					.count()
					+ 1 // plus the last item (which was unchecked)
			}
		});

	println!("Sum of any-Yes Answers: {}", any.sum::<usize>());

	// get a count for each unique character in each vector group
	let every = answers
		.iter()
		.map(|group| {
			// get the size of the answering group
			let group_size = group.split('\n').count();

			// filter out non-answer characters
			let chars = group.chars()
				.filter(|x| x.is_alphabetic());

			// aggregate totals for each group's answers as a map from key to
			// its frequency of appearence
			let mut map = chars.fold(HashMap::new(), |mut acc, x| {
					*acc.entry(x).or_insert(0) += 1;
					acc
				});
			// filter out answers that don't match the group size
			map.retain(|_, &mut v| v == group_size);

			// return how many of those answers there were
			map.len()
		});

	println!("Sum of all-Yes Answers: {}", every.sum::<usize>());

	Ok(())
}
