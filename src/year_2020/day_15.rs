use std::collections::HashMap;

fn elf_game(nums: Vec<u64>, nth: u64) -> u64 {
	let mut last = *nums.last().unwrap();
	let mut ages = HashMap::new();

	for (i, n) in nums.iter().enumerate() {
		ages.insert(*n, i as u64 + 1);
	}

	for i in nums.len() as u64 + 1..=nth {
		let next = if let Some(age) = ages.get(&last) {
			if *age < i - 1 {
				i - 1 - age
			} else {
				0
			}
		} else {
			0
		};
		ages.insert(last, i - 1);
		last = next;
	}
	last
}

pub fn part_01(input: &String) -> u64 {
	let nums = input.split(',').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
	elf_game(nums, 2020)
}

pub fn part_02(input: &String) -> u64 {
	let nums = input.split(',').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
	elf_game(nums, 30000000)
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&String::from("0,3,6")), 436);
		assert_eq!(super::part_01(&String::from("1,3,2")), 1);
		assert_eq!(super::part_01(&String::from("2,1,3")), 10);
		assert_eq!(super::part_01(&String::from("1,2,3")), 27);
		assert_eq!(super::part_01(&String::from("2,3,1")), 78);
		assert_eq!(super::part_01(&String::from("3,2,1")), 438);
		assert_eq!(super::part_01(&String::from("3,1,2")), 1836);
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 517);
	}

	#[test]
	fn part_02() {
		// // Keep these all commented out since they each take ~50 seconds to finish
		// assert_eq!(day::part_02(&String::from("0,3,6")), 175594);
		// assert_eq!(day::part_02(&String::from("1,3,2")), 2578);
		// assert_eq!(day::part_02(&String::from("2,1,3")), 3544142);
		// assert_eq!(day::part_02(&String::from("1,2,3")), 261214);
		// assert_eq!(day::part_02(&String::from("2,3,1")), 6895259);
		// assert_eq!(day::part_02(&String::from("3,2,1")), 18);
		// assert_eq!(day::part_02(&String::from("3,1,2")), 362);
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 1047739);
	}
}
