pub fn part_01(input: &String) -> u64 {
	let mut gamma_bits = Vec::<u32>::new();
	let mut epsilon_bits = Vec::<u32>::new();
	input.lines().for_each(|d| {
		d.chars()
			.map(|b| b.to_digit(10).expect("failed to parse bit"))
			.enumerate()
			.for_each(|(i, b)| {
				let is_gamma = b != 0;
				if gamma_bits.len() <= i {
					gamma_bits.push(if is_gamma { 1 } else { 0 });
					epsilon_bits.push(if is_gamma { 0 } else { 1 });
				} else {
					gamma_bits[i] += if is_gamma { 1 } else { 0 };
					epsilon_bits[i] += if is_gamma { 0 } else { 1 };
				}
			});
	});
	let (gamma, epsilon) = gamma_bits
		.iter()
		.zip(epsilon_bits.iter())
		.rev()
		.enumerate()
		.fold((0, 0), |(ag, ae), (i, (g, e))| {
			let c = g > e;
			(ag | if c { 1 } else { 0 } << i, ae | if c { 0 } else { 1 } << i)
		});
	gamma * epsilon
}

pub fn part_02(input: &String) -> u64 {
	let mut oxygen_lines = input.lines().collect::<Vec<_>>();
	let mut c02_lines = input.lines().collect::<Vec<_>>();
	let mut i = 0;
	let len = oxygen_lines[0].len();
	while i < len && oxygen_lines.len() > 1 {
		let ones = oxygen_lines
			.iter()
			.fold(0, |a, l| a + if Some('1') == l.chars().nth(i) { 1 } else { 0 });
		let count = oxygen_lines.len();
		let common_bit = if count - ones == ones {
			None
		} else if count - ones > ones {
			Some('0')
		} else {
			Some('1')
		};
		oxygen_lines = oxygen_lines
			.iter()
			.filter(|&l| {
				if let None = common_bit {
					if let Some('1') = l.chars().nth(i) {
						true
					} else {
						false
					}
				} else {
					common_bit == l.chars().nth(i)
				}
			})
			.map(|&l| l)
			.collect::<Vec<_>>();
		i += 1;
	}
	i = 0;
	while i < len && c02_lines.len() > 1 {
		let ones = c02_lines
			.iter()
			.fold(0, |a, l| a + if Some('1') == l.chars().nth(i) { 1 } else { 0 });
		let count = c02_lines.len();
		let common_bit = if count - ones == ones {
			None
		} else if count - ones > ones {
			Some('0')
		} else {
			Some('1')
		};
		c02_lines = c02_lines
			.iter()
			.filter(|&l| {
				if let None = common_bit {
					if let Some('0') = l.chars().nth(i) {
						true
					} else {
						false
					}
				} else {
					common_bit != l.chars().nth(i)
				}
			})
			.map(|&l| l)
			.collect::<Vec<_>>();
		i += 1;
	}
	u64::from_str_radix(oxygen_lines[0], 2).unwrap() * u64::from_str_radix(c02_lines[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))), 198);
		assert_eq!(super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))), 2743844);
	}

	#[test]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))), 230);
		assert_eq!(super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))), 6677951);
	}
}
