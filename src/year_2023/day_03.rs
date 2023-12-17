use std::collections::HashMap;

pub fn part_01(input: &str) -> u64 {
	let mut num_map: HashMap<(isize, isize), u64> = HashMap::new();
	let symbols = input
		.lines()
		.enumerate()
		.filter_map(|(i, l)| {
			let line = l
				.chars()
				.enumerate()
				.filter_map(|(j, c)| {
					if let Some(num) = c.to_digit(10) {
						let mut k = j as isize - 1;
						let mut updated_num = None;
						while let Some(n) = num_map.get_mut(&(i as isize, k)) {
							*n = ((*n) * 10) + num as u64;
							updated_num = Some(*n);
							k -= 1;
						}
						num_map.insert((i as isize, j as isize), updated_num.unwrap_or(num as u64));
						None
					} else if c == '.' {
						None
					} else {
						Some((i as isize, j as isize))
					}
				})
				.collect::<Vec<_>>();
			(!line.is_empty()).then_some(line)
		})
		.flatten()
		.collect::<Vec<_>>();
	symbols.iter().fold(0, |sum, (i, j)| {
		sum
			+ (i - 1..=i + 1).fold(0, |sy, y| {
				let mut wide_num = false;
				sy + (j - 1..=j + 1).fold(0, |sx, x| {
					if let (Some(&n), wn) = (num_map.get(&(y, x)), wide_num) {
						wide_num = true;
						sx + (n * (!wn as u64))
					} else {
						wide_num = false;
						sx
					}
				})
			})
	})
}

pub fn part_02(input: &str) -> u64 {
	let mut num_map: HashMap<(isize, isize), u64> = HashMap::new();
	let symbols = input
		.lines()
		.enumerate()
		.filter_map(|(i, l)| {
			let line = l
				.chars()
				.enumerate()
				.filter_map(|(j, c)| {
					if let Some(num) = c.to_digit(10) {
						let mut k = j as isize - 1;
						let mut updated_num = None;
						while let Some(n) = num_map.get_mut(&(i as isize, k)) {
							*n = ((*n) * 10) + num as u64;
							updated_num = Some(*n);
							k -= 1;
						}
						num_map.insert((i as isize, j as isize), updated_num.unwrap_or(num as u64));
						None
					} else if c == '*' {
						Some((i as isize, j as isize))
					} else {
						None
					}
				})
				.collect::<Vec<_>>();
			(!line.is_empty()).then_some(line)
		})
		.flatten()
		.collect::<Vec<_>>();
	let mut sum = 0;
	for (i, j) in symbols {
		let mut adjacent = Vec::new();
		for y in i - 1..=i + 1 {
			let mut wide_num = false;
			for x in j - 1..=j + 1 {
				match (num_map.get(&(y, x)), wide_num) {
					(Some(&n), false) => {
						wide_num = true;
						adjacent.push(n);
					},
					(None, true) => {
						wide_num = false;
					},
					_ => (),
				}
			}
		}
		if adjacent.len() == 2 {
			sum += adjacent.iter().product::<u64>();
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			4361
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			539433
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			467835
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			75847567
		);
	}
}
