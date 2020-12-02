use std::{env, fs};

mod days;

use days::*;

fn main() {
	// Day 01
	let input_file_01 = env::current_dir()
		.expect("Failed to get current directory")
		.join("rsrc/input_01.txt");
	let input_01 = fs::read_to_string(input_file_01).expect("Failed to read file.");
	println!("Day 01");
	println!("{}", day_01::part_01(&input_01));
	println!("{}", day_01::part_02(&input_01));
	println!();

	// Day 02
	let input_file_02 = env::current_dir()
		.expect("Failed to get current directory")
		.join("rsrc/input_02.txt");
	let input_02 = fs::read_to_string(input_file_02).expect("Failed to read file.");
	println!("Day 02");
	println!("{}", day_02::part_01(&input_02));
	println!("{}", day_02::part_02(&input_02));
	println!();
}
