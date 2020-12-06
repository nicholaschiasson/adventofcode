use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_01(input: &String) -> u64 {
	input.split("\n\n").fold(0, |n, group| {
		n + group
			.lines()
			.flat_map(|answer| answer.chars())
			.collect::<HashSet<char>>()
			.len()
	}) as u64
}

pub fn part_02(input: &String) -> u64 {
	input.split("\n\n").fold(0, |n, group| {
		n + group
			.lines()
			.flat_map(|answer| answer.chars())
			.fold(HashMap::new(), |mut m, a| {
				m.insert(a, if let Some(v) = m.get(&a) { v + 1 } else { 1 });
				m
			})
			.iter()
			.filter(|(_, n)| **n == group.lines().map(|_| 0).collect::<Vec<u8>>().len())
			.map(|_| 0)
			.collect::<Vec<u8>>()
			.len()
	}) as u64
}
