use std::{fmt, str::FromStr};

#[derive(Clone, Debug)]
struct Board {
	cells: Vec<Vec<i32>>,
	checked: Vec<Vec<bool>>,
	columns: Vec<i32>,
	rows: Vec<i32>,
}

impl Board {
	fn check(&mut self, draw: i32) {
		if let Some(i) = self.cells.iter().flatten().position(|&c| c == draw) {
			let col = i % self.cells.len();
			let row = i / self.cells.len();
			self.checked[row][col] = true;
			self.columns[col] += 1;
			self.rows[row] += 1;
		}
	}

	fn is_winner(&self) -> bool {
		self.columns.contains(&(self.rows.len() as i32)) || self.rows.contains(&(self.columns.len() as i32))
	}

	fn unmarked(&self) -> Vec<i32> {
		self
			.cells
			.iter()
			.flatten()
			.enumerate()
			.filter(|(i, _)| !self.checked[i / self.cells.len()][i % self.cells.len()])
			.map(|(_, &c)| c)
			.collect::<Vec<_>>()
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			self
				.cells
				.iter()
				.enumerate()
				.map(|(i, r)| r
					.iter()
					.enumerate()
					.map(|(j, c)| if self.checked[i][j] {
						format!("[{}]", c)
					} else {
						c.to_string()
					})
					.collect::<Vec<_>>()
					.join(","))
				.collect::<Vec<_>>()
				.join("\n")
		)
	}
}

impl FromStr for Board {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let cells = s
			.lines()
			.map(|l| l.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>())
			.collect::<Vec<_>>();
		let len = cells.len();
		Ok(Self {
			cells,
			checked: vec![vec![false; len]; len],
			columns: vec![0; len],
			rows: vec![0; len],
		})
	}
}

pub fn part_01(input: &String) -> u64 {
	let input_lf = input.lines().collect::<Vec<_>>().join("\n");
	let draw = input_lf
		.split("\n\n")
		.take(1)
		.next()
		.unwrap()
		.split(',')
		.map(|d| d.parse::<i32>().unwrap())
		.collect::<Vec<_>>();
	let mut boards = input_lf
		.split("\n\n")
		.skip(1)
		.map(|b| b.parse::<Board>().unwrap())
		.collect::<Vec<_>>();
	draw
		.iter()
		.find_map(|&d| {
			for b in &mut boards {
				b.check(d);
				if b.is_winner() {
					return Some(b.unmarked().iter().sum::<i32>() * d);
				}
			}
			None
		})
		.unwrap() as u64
}

pub fn part_02(input: &String) -> u64 {
	let input_lf = input.lines().collect::<Vec<_>>().join("\n");
	let draw = input_lf
		.split("\n\n")
		.take(1)
		.next()
		.unwrap()
		.split(',')
		.map(|d| d.parse::<i32>().unwrap())
		.collect::<Vec<_>>();
	let mut boards = input_lf
		.split("\n\n")
		.skip(1)
		.map(|b| b.parse::<Board>().unwrap())
		.collect::<Vec<_>>();
	draw
		.iter()
		.fold(None, |a, &d| {
			let mut ret = a;
			for b in &mut boards {
				if b.is_winner() {
					continue;
				}
				b.check(d);
				if b.is_winner() {
					ret = Some(b.unmarked().iter().sum::<i32>() * d);
				}
			}
			ret
		})
		.unwrap() as u64
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 64084);
	}

	#[test]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 12833);
	}
}
