use std::io;
use std::io::{Read};

fn main() -> io::Result<()> {
	// store all input in a string
	let mut buf = String::new();
	let _ = io::stdin().read_to_string(&mut buf)?;

	// get a vector for all boarding pass strings in the input file
	let passes: Vec<&str> = buf.split("\n").collect();

	for pass in passes {
		let id = match bsp_to_id(pass) {
			Ok(val) => val,
			Err(e)  => 0u16,
		};
	}

	return Ok(());
}

/// convert a BSP code to an ID
fn bsp_to_id(pass: &str) -> Result<u16, String> {
	let mut id = 0u16;
	let mut chars = pass.chars();
	for i in (3..10).rev() {
		let dir = match chars.next().expect("String too short") {
			'B' => Ok(2_u16.pow(i)),
			'F' => Ok(0),
			_   => Err("Bad dir"),
		};

		id += dir?;
	}

	println!("{}", id);
	return Ok(id);
}
