use std::collections::VecDeque;

fn holiday_ascii_string_helper(s: &str) -> u8 {
	s.bytes().fold(0_u16, |h, b| ((h + b as u16) * 17) % 256) as u8
}

fn holiday_ascii_string_helper_manual_arrangement_procedure(s: &str) -> [(VecDeque<&str>, VecDeque<u8>); 256] {
	const DEQUE: (VecDeque<&str>, VecDeque<u8>) = (VecDeque::new(), VecDeque::new());
	let mut boxes = [DEQUE; 256];
	for step in s.split(',') {
		match step.split_once(['=', '-']) {
			Some((label, focal_length)) => {
				match (
					boxes.get_mut(holiday_ascii_string_helper(label) as usize),
					focal_length.parse::<u8>(),
				) {
					(Some((labels, focal_lengths)), Ok(fl)) => {
						if let Some(i) = labels.iter().position(|&l| l == label) {
							focal_lengths[i] = fl;
						} else {
							labels.push_back(label);
							focal_lengths.push_back(fl);
						}
					},
					(Some((labels, focal_lengths)), _) => {
						if let Some(i) = labels.iter().position(|&l| l == label) {
							labels.remove(i);
							focal_lengths.remove(i);
						}
					},
					_ => (),
				}
			},
			_ => panic!("gross. you call that trash input?"),
		}
	}
	boxes
}

pub fn part_01(input: &str) -> u64 {
	input
		.split(',')
		.fold(0, |s, step| s + holiday_ascii_string_helper(step) as u64)
}

pub fn part_02(input: &str) -> u64 {
	holiday_ascii_string_helper_manual_arrangement_procedure(input)
		.iter()
		.enumerate()
		.fold(0, |sum, (box_number, (_, b))| {
			sum
				+ b.iter().enumerate().fold(0, |focal_power, (slot_number, &lens)| {
					focal_power + ((1 + box_number as u64) * (1 + slot_number) as u64 * lens as u64)
				})
		})
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01("HASH"), 52);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			1320
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			504449
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			145
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			262044
		);
	}
}
