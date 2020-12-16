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
