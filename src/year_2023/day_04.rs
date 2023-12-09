use std::collections::HashSet;

pub fn part_01(input: &str) -> u64 {
	input
		.lines()
		.map(|l| {
			l.split_once(": ")
				.unwrap()
				.1
				.split_once("|")
				.map(|(winning, have)| {
					(
						HashSet::<u64>::from_iter(winning.split_whitespace().map(|n| n.parse::<u64>().unwrap())),
						have.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>(),
					)
				})
				.unwrap()
		})
		.fold(0, |score, (winning, have)| {
			score + (have.iter().fold(1, |multiplier, n| multiplier << winning.contains(n) as u8) >> 1)
		})
}

pub fn part_02(input: &str) -> u64 {
	let n_cards = input.lines().count();
	input
		.lines()
		.map(|l| {
			l.split_once(": ")
				.unwrap()
				.1
				.split_once("|")
				.map(|(winning, have)| {
					(
						HashSet::<u64>::from_iter(winning.split_whitespace().map(|n| n.parse::<u64>().unwrap())),
						have.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>(),
					)
				})
				.unwrap()
		})
		.enumerate()
		.fold(vec![1; n_cards], |mut cards, (i, (winning, have))| {
			let score = have.iter().fold(0, |s, n| s + winning.contains(n) as usize);
			for j in i + 1..=i + score {
				if j >= cards.len() {
					break;
				}
				cards[j] += cards[i];
			}
			cards
		})
		.iter()
		.sum()
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			13
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			26443
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			30
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			6284877
		);
	}
}
