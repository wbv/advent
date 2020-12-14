use std::io;
use std::io::{Read};

fn main() -> io::Result<()> {
	// just buffer the whole input file at once
	let mut buf = String::new();
	let filesize = io::stdin().read_to_string(&mut buf)?;
	println!("Read {} bytes.", filesize);

	let passports = buf
		.split("\n\n")
		.collect::<Vec<&str>>();

	let num_valid = passports.iter()
	                         .map(|x| is_valid_passport(x))
	                         .filter(|x| *x == true)
	                         .count();

	println!("Found {} valid passports", num_valid);

	Ok(())
}


fn is_valid_passport(passport: &str) -> bool {
	const REQ_FIELDS: [&'static str; 7] =
		[ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];

	// pull the first three letters (the field) from each key-val entry
	let fields = passport.split_whitespace()
	                     .map(|x| x.get(0..3).expect("Bad field"))
	                     .collect::<Vec<&str>>();

	// fail check if any required field is not found anywhere in the passport
	for f in &REQ_FIELDS {
		if ! fields.iter()
		           .map(|x| x.contains(f))
		           .fold(false, |x, acc| x || acc) {
			return false;
		}
	}

	// if we got here, then all required fields were found
	true
}
