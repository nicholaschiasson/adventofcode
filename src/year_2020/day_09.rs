use std::collections::VecDeque;

fn find_outlier(sequence: &Vec<i64>, preamble: usize) -> i64 {
	let mut prev_five = sequence.iter().take(preamble).map(|n| *n).collect::<VecDeque<_>>();
	*sequence
		.iter()
		.skip(preamble)
		.find(|n| {
			if prev_five.iter().any(|p| prev_five.contains(&(*n - p)) && *n - p != *p) {
				prev_five.pop_front();
				prev_five.push_back(**n);
				return false;
			}
			true
		})
		.unwrap()
}

pub fn part_01(input: &String, preamble: usize) -> u64 {
	let sequence = input.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
	find_outlier(&sequence, preamble) as u64
}

pub fn part_02(input: &String, preamble: usize) -> u64 {
	let sequence = input.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
	let outlier = find_outlier(&sequence, preamble);
	let outlier_idx = sequence.iter().position(|n| *n == outlier).unwrap();
	for i in 0..(outlier_idx - 2) {
		let mut sum = sequence[i];
		let mut largest = sum;
		let mut smallest = sum;
		for j in (i + 1)..(outlier_idx - 1) {
			sum += sequence[j];
			if sequence[j] > largest {
				largest = sequence[j];
			}
			if sequence[j] < smallest {
				smallest = sequence[j];
			}
			if sum > outlier {
				break;
			}
			if sum == outlier {
				return (smallest + largest) as u64;
			}
		}
	}
	0
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(
				&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH))),
				5
			),
			127
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH))), 25),
			3199139634
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(
				&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH))),
				5
			),
			62
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH))), 25),
			438559930
		);
	}
}
