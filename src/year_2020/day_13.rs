use lexical::parse_partial;

pub fn part_01(input: &String) -> u64 {
	let (earliest_departure, i) = parse_partial::<u64, _>(input.as_bytes()).unwrap();
	let (time, id) = input.as_str()[i + 1..]
		.split(',')
		.filter(|s| s != &"x")
		.map(|s| s.parse::<u64>().unwrap())
		.fold((2 * earliest_departure, 0), |(t, i), id| {
			let time = earliest_departure + (id - (earliest_departure % id));
			if time < t {
				(time, id)
			} else {
				(t, i)
			}
		});
	(time - earliest_departure) * id
}

pub fn part_02(_input: &String) -> u64 {
	// let (_, i) = parse_partial::<u64, _>(input.as_bytes()).unwrap();
	// let ids = input.as_str()[i + 1..].split(',').collect::<Vec<&str>>();
	// // Brute force. Does not work since the answer is above 100 trillion.
	// for i in 0.. {
	// 	let t = i * ids[0].parse::<u64>().unwrap();
	// 	for j in 1..ids.len() {
	// 		if ids[j] == "x" {
	// 			continue;
	// 		}
	// 		if (t + j as u64) % ids[j].parse::<u64>().unwrap() != 0 {
	// 			break;
	// 		}
	// 		if j == ids.len() - 1 {
	// 			return t;
	// 		}
	// 	}
	// }
	// 0

	// // Trying to implement chinese remainder algorithm
	// let n = ids
	// 	.iter()
	// 	.filter(|s| *s != &"x")
	// 	.map(|s| s.parse::<u64>().unwrap())
	// 	.fold(1, |p, i| p * i);
	// ids
	// 	.iter()
	// 	.enumerate()
	// 	.fold((0u64, 1u64), |(x, q), (offset, id_str)| {
	// 		if id_str == &"x" {
	// 			return (x, q);
	// 		}
	// 		let id = id_str.parse::<u64>().unwrap();
	// 		let mut i = 0;
	// 		loop {
	// 			if (x + i * q * id) % id == offset as u64 {
	// 				break;
	// 			}
	// 			i += 1;
	// 		}
	// 		(x + i * q * id, q * id)
	// 	}).0;
	panic!("DEFEAT")
}
