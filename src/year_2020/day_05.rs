fn get_seat_id(seat: &String) -> u64 {
	let row = seat.as_bytes()[..7]
		.iter()
		.fold(0, |r, b| (r | if *b == b'F' { 0 } else { 1 }) << 1)
		>> 1;
	let col = seat.as_bytes()[7..]
		.iter()
		.fold(0, |c, b| (c | if *b == b'L' { 0 } else { 1 }) << 1)
		>> 1;
	(row * 8) + col
}

pub fn part_01(input: &String) -> u64 {
	input.lines().fold(0, |id, seat| {
		let seat_id = get_seat_id(&seat.to_string());
		if seat_id > id {
			seat_id
		} else {
			id
		}
	})
}

pub fn part_02(input: &String) -> u64 {
	let mut ids: Vec<u64> = input.lines().map(|s| get_seat_id(&s.to_string())).collect();
	ids.sort();
	ids
		.iter()
		.enumerate()
		.filter(|(i, s)| i + 1 < ids.len() && ids[i + 1] - **s != 1)
		.map(|(_, s)| s + 1)
		.collect::<Vec<u64>>()[0]
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 896);
	}

	#[test]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 659);
	}
}
