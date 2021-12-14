use std::collections::HashMap;

pub fn part_01(input: &String) -> u64 {
	let mut adapters = input.lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();
	adapters.sort();
	let mut differences = [0u64; 3];
	for i in 0..adapters.len() {
		let difference = if i == 0 { adapters[i] } else { adapters[i] - adapters[i - 1] };
		differences[difference as usize - 1] += 1;
	}
	differences[0] * (differences[2] + 1)
}

pub fn part_02(input: &String) -> u64 {
	let mut adapters = input.lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();
	adapters.push(0);
	adapters.push(adapters.iter().max().unwrap() + 3);
	adapters.sort_by(|a, b| b.cmp(a));
	let mut options = HashMap::new();
	options.insert(adapters[0], 1);
	for a in &adapters[1..] {
		options.insert(
			*a,
			(1..=3).fold(0, |n, i| {
				if options.contains_key(&(a + i)) {
					n + options[&(a + i)]
				} else {
					n
				}
			}),
		);
	}
	options[&0]
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			35
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_02", INPUT_PATH)))),
			220
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			1914
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			8
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_02", INPUT_PATH)))),
			19208
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			9256148959232
		);
	}
}
