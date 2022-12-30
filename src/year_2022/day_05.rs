fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
	let rows = stacks.lines().rev().skip(1).collect::<Vec<_>>();
	let mut stacks = Vec::new();
	for r in rows {
		for (i, c) in r.chars().skip(1).step_by(4).enumerate().filter(|(_, c)| !c.is_whitespace()) {
			if i + 1 > stacks.len() {
				stacks.push(Vec::new());
			}
			stacks[i].push(c);
		}
	}
	stacks
}

fn parse_instruction(instruction: &str) -> Vec<usize> {
	instruction
		.split(|c: char| c.is_alphabetic() || c.is_whitespace())
		.filter(|c| !c.is_empty())
		.map(|c| c.parse::<usize>().expect("numeric instructions"))
		.collect::<Vec<_>>()
}

fn collect_top_crates(stacks: &Vec<Vec<char>>) -> String {
	stacks
		.iter()
		.filter(|s| !s.is_empty())
		.map(|s| s.last().expect("stack has last element").to_string())
		.collect::<Vec<_>>()
		.join("")
}

pub fn part_01(input: &String) -> String {
	let (stacks, instructions) = input.split_once("\n\n").expect("correctly formatted input");
	let mut stacks = parse_stacks(stacks);

	for i in instructions.lines() {
		let instruction = parse_instruction(i);
		for _ in 0..instruction[0] {
			let c = stacks[instruction[1] - 1].pop().expect("stack not empty");
			stacks[instruction[2] - 1].push(c);
		}
	}

	collect_top_crates(&stacks)
}

pub fn part_02(input: &String) -> String {
	let (stacks, instructions) = input.split_once("\n\n").expect("correctly formatted input");
	let mut stacks = parse_stacks(stacks);

	for i in instructions.lines() {
		let instruction = parse_instruction(i);
		let s1_len = stacks[instruction[1] - 1].len();
		let s2_ext = stacks[instruction[1] - 1].split_off(s1_len - instruction[0]);
		stacks[instruction[2] - 1].extend(s2_ext);
	}

	collect_top_crates(&stacks)
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			"CMZ"
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			"SHQWSRBDL"
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			"MCD"
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			"CDTQZHBRS"
		);
	}
}
