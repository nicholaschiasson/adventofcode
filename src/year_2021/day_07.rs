pub fn part_01(input: &String) -> u64 {
	let mut positions = input.split(',').map(|f| f.parse::<i32>().unwrap()).collect::<Vec<_>>();
	positions.sort();
	let target = positions[positions.len() / 2];
	positions.iter().fold(0, |f, &p| f + (target - p).abs()) as u64
}

pub fn part_02(input: &String) -> u64 {
	let positions = input.split(',').map(|f| f.parse::<i32>().unwrap()).collect::<Vec<_>>();
	let target = (positions.iter().map(|&p| p as f32).sum::<f32>() / positions.len() as f32).floor() as i32;
	(target..=target + 1)
		.map(|t| {
			positions.iter().fold(0, |f, &p| {
				let distance = (t - p).abs();
				f + (distance + (distance * (distance - 1) / 2))
			})
		})
		.min()
		.unwrap() as u64
}
