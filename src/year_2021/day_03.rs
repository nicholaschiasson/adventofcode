pub fn part_01(input: &String) -> u64 {
	todo!()
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
		let common_bit = if count - ones == ones { None } else if count - ones > ones { Some('0') } else { Some('1') };
		oxygen_lines = oxygen_lines
			.iter()
			.filter(|&l| if let None = common_bit {
				if let Some('1') = l.chars().nth(i) { true } else { false }
			 } else {
				 common_bit == l.chars().nth(i)
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
		let common_bit = if count - ones == ones { None } else if count - ones > ones { Some('0') } else { Some('1') };
		c02_lines = c02_lines
			.iter()
			.filter(|&l| if let None = common_bit {
				if let Some('0') = l.chars().nth(i) { true } else { false }
			 } else {
				 common_bit != l.chars().nth(i)
			 })
			 .map(|&l| l)
			 .collect::<Vec<_>>();
		i += 1;
	}
	u64::from_str_radix(oxygen_lines[0], 2).unwrap() * u64::from_str_radix(c02_lines[0], 2).unwrap()
}
