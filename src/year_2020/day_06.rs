use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_01(input: &String) -> u64 {
	input
		.split("\n\n")
		.fold(0, |n, group| {
			n + group
				.lines()
				.flat_map(|answer| answer.chars())
				.collect::<HashSet<char>>()
				.len()
		}) as u64
}

pub fn part_02(input: &String) -> u64 {
	input
		.split("\n\n")
		.fold(0, |n, group| {
			n + group
				.lines()
				.flat_map(|answer| answer.chars())
				.fold(HashMap::new(), |mut m, a| {
					m.insert(a, if let Some(v) = m.get(&a) { v + 1 } else { 1 });
					m
				})
				.iter()
				.filter(|(_, n)| **n == group.lines().count())
				.count()
		}) as u64
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			11
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			6763
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			6
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			3512
		);
	}
}
