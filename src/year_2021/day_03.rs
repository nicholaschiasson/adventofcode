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
	todo!()
}
