use std::io;
use std::io::{Read};

fn main() -> io::Result<()> {
	// store all input in a string
	let mut buf = String::new();
	let _ = io::stdin().read_to_string(&mut buf)?;

	// get a vector for all boarding pass strings in the input file
	let passes: Vec<&str> = buf.split("\n").collect();

	let mut pass_ids: Vec<u16> = passes
		.iter()
		.filter_map(|pass| bsp_to_id(pass).ok())
		.collect();

	match pass_ids.iter().max() {
		Some(max) => println!("Max pass ID: {}", max),
		None      => println!("No passes found"),
	};

	pass_ids.sort();

	for (a, b) in pass_ids.iter().zip(pass_ids.iter().skip(1)) {
		if *b == *a + 2 {
			println!("Your pass is {}", *a + 1);
			break;
		}
	}

	return Ok(());
}

/// convert a BSP code to an ID
fn bsp_to_id(pass: &str) -> Result<u16, &str> {
	if pass.len() < 10 {
		return Err("String too short");
	}

	let mut id = 0u16;
	let mut chars = pass.chars();

	for i in (0..10).rev() {
		id += match chars.next().unwrap() {
			'B'|'R'=> Ok(2_u16.pow(i)),
			'F'|'L'=> Ok(0),
			_      => Err("Bad direction"),
		}?;
	}

	return Ok(id);
}
