use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() -> io::Result<()> {
	// just buffer the whole input file at once
	let mut buf = String::new();
	let filesize = io::stdin().read_to_string(&mut buf)?;
	println!("Read {} bytes.", filesize);

	// store all bags which can contain my bag
	let mut have_my_bag = HashSet::new();
	let my_bag = String::from("shiny gold");

	// add my bag to the set
	have_my_bag.insert(my_bag);

	let lines = buf.split("\n");

	// fill the set with all bags which can contain my bag
	// TODO: this doesn't actually solve my problem

	for line in lines {
		match get_bags(line) {
			Some((parent, children)) => {
				for child in &children {
					if have_my_bag.contains(child) {
						have_my_bag.insert(parent.to_string());
					}
				}
			},

			None => continue,
		};
	}

	Ok(())
}

fn get_bags<'a>(line: &'a str) -> Option<(&'a str, Vec<String>)> {
	// extract the containing bag (parent) and contained bags (children)
	let mut declaration = line.split("bags contain");
	let parent = declaration.next()?.trim();
	let children: Vec<&str> = declaration.next()?
		.split(char::is_whitespace)
		.collect();

	// parse out all contained bags from "children" section
	let mut contained_bags = Vec::new();

	for child in children.windows(4) {
		if child[0].parse::<usize>().is_ok() {
			let bag = child[1].to_owned() + " " + child[2];
			contained_bags.push(bag);
		}
	}

	return Some((parent, contained_bags));
}
