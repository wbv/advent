use std::io;
use std::io::{Read};

fn main() -> io::Result<()> {
	/* just buffer the whole input file at once */
	let mut buf: Vec<u8> = Vec::new();
	let filesize = io::stdin().read_to_end(&mut buf)?;
	println!("Read {} bytes.", filesize);

	/* find first newline */
	let mapwidth = buf.iter().position(|&x| x == '\n' as u8).unwrap();
	println!("Map is {} bytes wide.", mapwidth);

	Ok(())
}
