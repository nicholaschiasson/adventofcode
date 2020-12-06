pub fn part_01(input: &String) -> u64 {
	let expenses: Vec<u64> = input
		.split('\n')
		.map(|e| e.parse::<u64>().expect("Failed to parse input"))
		.collect();
	for i in (1..expenses.len()).rev() {
		for j in 0..i - 1 {
			if expenses[i] + expenses[j] == 2020 {
				return expenses[i] * expenses[j];
			}
		}
	}
	panic!("This is not good...")
}

pub fn part_02(input: &String) -> u64 {
	let expenses: Vec<u64> = input
		.split('\n')
		.map(|e| e.parse::<u64>().expect("Failed to parse input"))
		.collect();
	for i in (2..expenses.len()).rev() {
		for j in (1..i - 1).rev() {
			let partial_sum = expenses[i] + expenses[j];
			if partial_sum <= 2020 {
				for k in 0..j - 1 {
					if partial_sum + expenses[k] == 2020 {
						return expenses[i] * expenses[j] * expenses[k];
					}
				}
			}
		}
	}
	panic!("This is not good...")
}
