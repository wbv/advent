use std::io;
use std::io::{BufReader, BufRead};


/// for each line of input, check if the password field matches requirements
/// laid out for each policy (old and new)
///
/// input line format:
/// ```
///	    a-b x: y
/// ```
/// where
/// ```
///     a, b  are numeric indicies
///     x     is an alphabetic letter
///     y     is a password (contiguous sequence of alphabetic letters)
/// ```
///
/// old policy: `a` and `b` are the lowest and highest number of times a given
/// letter `x` must appear iff the password `y` is valid 
///
/// new policy: a password `y` is valid iff exactly one instance of letter `y`
/// appears at indices `a` and `b` in `y`
fn main() -> io::Result<()> {
	/* get standard input as a buffered reader */
	let buf = BufReader::new(io::stdin());

	/* count the valid passwords under the old and new policies */
	let (old_count, new_count) = buf
		.lines()
		.map(|line| is_valid_password(line.unwrap()))
		.fold((0, 0), |x, acc| (x.0 + acc.0, x.1 + acc.1));

	/* report how many valid passwords we found */
	println!("{} valid passwords under old policy found", old_count);
	println!("{} valid passwords under new policy found", new_count);

	Ok(())
}


/// takes a password line and returns a 1 (valid) or 0 (invalid) for both of the
/// password policies (old, new) 
fn is_valid_password(line: String) -> (usize, usize) {
	let mut components = line.split_whitespace();

	// the range (lo - hi) being checked under old policy
	// which is also the indices (lo = first, hi = second) being checked under
	// the new policy
	let mut range = components.next().unwrap().split('-');
	let lo = range.next().unwrap().parse::<usize>().unwrap();
	let hi = range.next().unwrap().parse::<usize>().unwrap();

	// the letter being checked under both policies
	let letter = components.next().unwrap().chars().nth(0).unwrap();

	// obtain the password text
	let password = components.next().unwrap().trim();

	let mut num_chars = 0;        // old policy counter
	let mut char_pos_matches = 0; // new policy counter

	for (i, ch) in password.chars().enumerate() {
		// old policy
		if ch == letter {
			num_chars += 1;
		}
		// new policy - mind the 1-indexed location */
		if i == (lo - 1) && ch == letter {
			char_pos_matches += 1;
		}
		if i == (hi - 1) && ch == letter {
			char_pos_matches += 1;
		}
	}

	// old policy: check if the letter count requirements are met
	let old_valid = if num_chars >= lo && num_chars <= hi { 1 } else { 0 };

	// new policy: only count matches at the two indices
	let new_valid = if char_pos_matches == 1 { 1 } else { 0 };

	return (old_valid, new_valid);
}
