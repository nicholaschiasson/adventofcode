use num::{Integer, ToPrimitive};

#[derive(Clone, Copy, Debug)]
struct Signal<'a>(&'a str);

impl Signal<'_> {
	fn possible_digits(&self) -> Vec<u8> {
		match self.0.len() {
			2 => vec![1],
			3 => vec![7],
			4 => vec![4],
			5 => vec![2, 3, 5],
			6 => vec![0, 6, 9],
			7 => vec![8],
			_ => panic!("Invalid signal digit"),
		}
	}

	fn segments(&self) -> Vec<u8> {
		self.0.bytes().map(|b| b - b'a').collect()
	}
}

impl Into<u8> for Signal<'_> {
	fn into(self) -> u8 {
		let mut s: Vec<u8> = self.0.bytes().collect();
		s.sort();
		match s.as_slice() {
			b"abcefg" => 0,
			b"cf" => 1,
			b"acdeg" => 2,
			b"acdfg" => 3,
			b"bcdf" => 4,
			b"abdfg" => 5,
			b"abdefg" => 6,
			b"acf" => 7,
			b"abcdefg" => 8,
			b"abcdfg" => 9,
			_ => panic!("Invalid signal digit"),
		}
	}
}

impl<T: Integer + ToPrimitive> From<T> for Signal<'_> {
	fn from(d: T) -> Self {
		match d.to_u8().unwrap() {
			0 => Self("abcefg"),
			1 => Self("cf"),
			2 => Self("acdeg"),
			3 => Self("acdfg"),
			4 => Self("bcdf"),
			5 => Self("abdfg"),
			6 => Self("abdefg"),
			7 => Self("acf"),
			8 => Self("abcdefg"),
			9 => Self("abcdfg"),
			_ => panic!("Invalid signal digit"),
		}
	}
}

#[derive(Clone, Debug)]
struct SignalMapping {
	segments: Vec<Vec<char>>,
}

impl SignalMapping {
	fn new() -> Self {
		Self {
			segments: (0..7).map(|_| "abcdefg".chars().collect()).collect(),
		}
	}

	fn calibrate(&self, signal: &str, digit: u8) -> Self {
		let digit_segments = Signal::from(digit).segments();
		let segments = self
			.segments
			.iter()
			.enumerate()
			.map(|(i, s)| {
				s.iter()
					.filter(|&c| {
						if digit_segments.contains(&(i as u8)) {
							signal.contains(&c.to_string())
						} else {
							!signal.contains(&c.to_string())
						}
					})
					.map(|&c| c)
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		if segments.iter().all(|s| !s.is_empty()) {
			return Self { segments, ..*self };
		}
		self.clone()
	}

	fn map_signal(&self, signal: Signal) -> u8 {
		let segments = self.segments.iter().flatten().map(|&c| c).collect::<Vec<_>>();
		Signal(
			&signal
				.0
				.chars()
				.map(|c| (segments.iter().position(|&s| s == c).unwrap() as u8 + b'a') as char)
				.collect::<String>(),
		)
		.into()
	}
}

pub fn part_01(input: &String) -> u64 {
	input
		.lines()
		.map(|l| {
			l.split(" | ")
				.last()
				.unwrap()
				.split_whitespace()
				.filter(|&d| [2, 3, 4, 7].contains(&d.len()))
				.count() as u64
		})
		.sum()
}

pub fn part_02(input: &String) -> u64 {
	let mappings: Vec<SignalMapping> = input
		.lines()
		.map(|l| {
			let mut signals = l.split(" | ").nth(0).unwrap().split_whitespace().collect::<Vec<_>>();
			signals.sort_by(|&a, &b| a.len().cmp(&b.len()));
			signals.iter().fold(SignalMapping::new(), |mapping, &s| {
				Signal(s)
					.possible_digits()
					.iter()
					.fold(mapping, |m, &d| m.calibrate(s, d).clone())
			})
		})
		.collect();
	input
		.lines()
		.enumerate()
		.map(|(i, l)| {
			l.split(" | ")
				.last()
				.unwrap()
				.split_whitespace()
				.map(|d| mappings[i].map_signal(Signal(d)).to_string())
				.collect::<String>()
		})
		.map(|n| n.parse::<u64>().unwrap())
		.sum::<u64>()
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 445);
	}

	#[test]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 1043101);
	}
}
