use std::io;
use std::io::{Read};

fn main() -> io::Result<()> {
	// just buffer the whole input file at once
	let mut buf = String::new();
	let filesize = io::stdin().read_to_string(&mut buf)?;
	println!("Read {} bytes.", filesize);

	// vectorize each passport into a string slice
	let passports = buf
		.split("\n\n")
		.collect::<Vec<&str>>();

	// filter out the passports without all required fields
	let complete_passports = passports.into_iter()
	                                  .filter(|x| has_required_fields(*x))
	                                  .collect::<Vec<_>>();

	println!("Found {} passports with all required fields.",
	         complete_passports.len());

	// count the total number of passports which are actually valid
	let num_valid = complete_passports.iter()
	                                  .map(|x| is_valid_passport(x))
	                                  .filter(|x| *x == true)
	                                  .count();

	println!("Found {} actually valid passports.", num_valid);

	Ok(())
}


/// returns true if all required fields are present
fn has_required_fields(passport: &str) -> bool {
	const REQ_FIELDS: [&'static str; 7] =
		[ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];

	// pull the first three letters (the field) from each key-val entry
	let fields = passport.split_whitespace()
	                     .map(|x| x.get(0..3).unwrap_or("n/a"))
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
	return true;
}

/// checks validity of a passport assuming it has already been checked for
/// having all the required fields
fn is_valid_passport(passport: &str) -> bool {

	// pull the first three letters (the field) from each key-val entry
	let fields = passport.split_whitespace()
	                     .collect::<Vec<&str>>();

	// check for validity of each field found
	for f in fields {
		let valid_field: bool = match f.get(0..3).unwrap_or("n/a") {
			"byr" => is_valid_byr(f),
			"iyr" => is_valid_iyr(f),
			"eyr" => is_valid_eyr(f),
			"hgt" => is_valid_hgt(f),
			"hcl" => is_valid_hcl(f),
			"ecl" => is_valid_ecl(f),
			"pid" => is_valid_pid(f),
			// all other fields are ignored (e.g. cid)
			_     => true,
		};

		if ! valid_field {
			return false;
		}
	}

	// if we got here, all fields were valid
	return true;
}



/// validity check for `byr` field
fn is_valid_byr(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val: usize = field.get(4..)
	                      .unwrap_or("0")
	                      .parse()
	                      .unwrap_or(0);

	return val >= 1920 && val <= 2002;
}

/// validity check for `iyr` field
fn is_valid_iyr(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val: usize = field.get(4..)
	                      .unwrap_or("0")
	                      .parse()
	                      .unwrap_or(0);

	return val >= 2010 && val <= 2020;
}

/// validity check for `eyr` field
fn is_valid_eyr(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val: usize = field.get(4..)
	                      .unwrap_or("0")
	                      .parse()
	                      .unwrap_or(0);

	return val >= 2020 && val <= 2030;
}

/// validity check for `hgt` field
fn is_valid_hgt(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val = field.get(4..)
	               .unwrap_or("0");

	// check for valid height given the units
	if val.contains("cm") {
		let hgt: usize = val.split("cm")
		                    .nth(0)
		                    .unwrap_or("0")
		                    .parse()
		                    .unwrap_or(0);

		return hgt >= 150 && hgt <= 193;
	} else if val.contains("in") {
		let hgt: usize = val.split("in")
		                    .nth(0)
		                    .unwrap_or("0")
		                    .parse()
		                    .unwrap_or(0);

		return hgt >= 59 && hgt <= 76;
	} else {
		return false;
	}
}

/// validity check for `hcl` field
fn is_valid_hcl(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val = field.get(4..)
	               .unwrap_or("0");

	// must start with a #
	if val.chars().nth(0) != Some('#') {
		return false;
	}

	// must be followed by 6 hex digits
	let num_str = val.get(1..7).unwrap_or("xxxxxx");
	let num_hex = usize::from_str_radix(num_str, 16);

	return match num_hex {
		Ok(num) => num <= 0xffffff,
		Err(_) => false,
	};
}

/// validity check for `ecl` field
fn is_valid_ecl(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val = field.get(4..)
	               .unwrap_or("n/a");

	return match val {
		"amb" => true,
		"blu" => true,
		"brn" => true,
		"gry" => true,
		"grn" => true,
		"hzl" => true,
		"oth" => true,
		_     => false,
	};
}

/// validity check for `pid` field
fn is_valid_pid(field: &str) -> bool {
	// unpack value from the field, using invalid defaults on failure
	let val_str = field.get(4..)
	                   .unwrap_or("0");

	// check length
	if val_str.len() != 9 {
		return false;
	}

	// use invalid default on parse failure
	let val: usize = val_str.parse().unwrap_or(9_999_999_999);

	return val < 1_000_000_000;
}
