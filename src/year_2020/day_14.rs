use std::collections::HashMap;

use regex::Regex;

fn apply_bitmask(num: u64, bitmask: &[u8]) -> u64 {
	bitmask.iter().enumerate().fold(num, |x, (i, b)| match b {
		b'0' => clear_bit(x, i as u64),
		b'1' => set_bit(x, i as u64),
		_ => x,
	})
}

fn clear_bit(num: u64, bit: u64) -> u64 {
	num & !(1 << bit)
}

fn set_bit(num: u64, bit: u64) -> u64 {
	num | (1 << bit)
}

pub fn part_01(input: &String) -> u64 {
	let mut mask = String::new();
	let mut mem = HashMap::new();
	input.lines().for_each(|l| {
		if l.starts_with("mask") {
			mask = Regex::new(r"^mask = ([X0-1]+)$")
				.unwrap()
				.captures(l)
				.unwrap()
				.get(1)
				.unwrap()
				.as_str()
				.chars()
				.rev()
				.collect::<String>();
		} else {
			let captures = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap().captures(l).unwrap();
			mem.insert(
				captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
				apply_bitmask(captures.get(2).unwrap().as_str().parse::<u64>().unwrap(), mask.as_bytes()),
			);
		}
	});
	mem.iter().fold(0, |sum, (_, v)| sum + v)
}

pub fn part_02(input: &String) -> u64 {
	todo!()
}
