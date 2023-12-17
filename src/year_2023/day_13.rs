use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Reflection {
	None,
	Vertical(u64),
	Horizontal(u64),
}

impl Reflection {
	fn set_value(&mut self, value: u64) {
		*self = match self {
			Reflection::None => Reflection::None,
			Reflection::Vertical(_) => Self::Vertical(value),
			Reflection::Horizontal(_) => Self::Horizontal(value),
		}
	}

	fn value(&self) -> Option<u64> {
		match self {
			Reflection::Vertical(0) | Reflection::Horizontal(0) | Reflection::None => None,
			Reflection::Vertical(n) => Some(*n),
			Reflection::Horizontal(n) => Some(*n),
		}
	}
}

trait AlmostEq {
	type Tolerance;
	fn almost_eq(&self, rhs: &Self) -> Vec<Self::Tolerance>;
}

impl<T> AlmostEq for T
where
	T: AsRef<str> + Display,
{
	type Tolerance = usize;

	fn almost_eq(&self, rhs: &Self) -> Vec<Self::Tolerance> {
		let lhs = self.to_string();
		let rhs = rhs.to_string();
		let mut tolerated = Vec::new();
		let mut zipped = lhs.chars().zip(rhs.chars());
		for i in 0..(lhs.len().max(rhs.len())) {
			if let Some((lhc, rhc)) = zipped.next() {
				if lhc != rhc {
					tolerated.push(i);
				}
			} else {
				tolerated.push(i);
			}
		}
		tolerated
	}
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Valley {
	rows: Vec<String>,
	columns: Vec<String>,
}

impl Valley {
	fn find_reflection(&mut self, smudge_tolerance: usize) -> Reflection {
		let mut column_stack: Vec<usize> = Vec::new();
		let mut row_stack: Vec<usize> = Vec::new();
		let mut column_reflection = Reflection::Vertical(0);
		let mut row_reflection = Reflection::Horizontal(0);
		let mut column_tolerated: Vec<(usize, usize)> = Vec::new();
		let mut row_tolerated: Vec<(usize, usize)> = Vec::new();
		for index in 0..(self.columns.len() - 1) {
			column_stack.push(index);
			for j in (index + 1)..self.columns.len() {
				if (j - index) > column_stack.len() {
					break;
				}
				let i = column_stack.len() - (j - index);
				let tolerated = self.columns[i].almost_eq(&self.columns[j]);
				if tolerated.len() <= smudge_tolerance - column_tolerated.len() - row_tolerated.len() {
					column_reflection.set_value(index as u64 + 1);
					for t in tolerated {
						column_tolerated.push((i, t));
						self.flip(i, t);
					}
				} else {
					column_reflection.set_value(0);
					while let Some((x, y)) = column_tolerated.pop() {
						self.flip(x, y);
					}
					break;
				}
			}
			if !column_reflection.value().is_none() {
				if column_tolerated.len() + row_tolerated.len() >= smudge_tolerance {
					break;
				} else {
					column_reflection.set_value(0);
				}
			}
		}
		for index in 0..(self.rows.len() - 1) {
			row_stack.push(index);
			for j in (index + 1)..self.rows.len() {
				if (j - index) > row_stack.len() {
					break;
				}
				let i = row_stack.len() - (j - index);
				let tolerated = self.rows[i].almost_eq(&self.rows[j]);
				if tolerated.len() <= smudge_tolerance - column_tolerated.len() - row_tolerated.len() {
					row_reflection.set_value(index as u64 + 1);
					for t in tolerated {
						row_tolerated.push((t, i));
						self.flip(t, i);
					}
				} else {
					row_reflection.set_value(0);
					while let Some((x, y)) = row_tolerated.pop() {
						self.flip(x, y);
					}
					break;
				}
			}
			if !row_reflection.value().is_none() {
				if column_tolerated.len() + row_tolerated.len() >= smudge_tolerance {
					break;
				} else {
					row_reflection.set_value(0);
				}
			}
		}
		match (
			column_reflection.value(),
			row_reflection.value(),
			column_tolerated.len() == smudge_tolerance,
			row_tolerated.len() == smudge_tolerance,
		) {
			(None, None, _, _) => Reflection::None,
			(Some(_), _, true, _) => column_reflection,
			(_, Some(_), _, true) => row_reflection,
			(Some(a), Some(b), _, _) => {
				if a < b {
					column_reflection
				} else {
					row_reflection
				}
			},
			(Some(_), _, _, _) => column_reflection,
			(_, Some(_), _, _) => row_reflection,
		}
	}

	fn flip(&mut self, x: usize, y: usize) {
		if let Some(row) = self.rows.get_mut(y) {
			match row.chars().nth(x) {
				Some('.') => row.replace_range(x..=x, "#"),
				Some('#') => row.replace_range(x..=x, "."),
				_ => (),
			}
		}
		if let Some(column) = self.columns.get_mut(x) {
			match column.chars().nth(y) {
				Some('.') => column.replace_range(y..=y, "#"),
				Some('#') => column.replace_range(y..=y, "."),
				_ => (),
			}
		}
	}
}

impl Display for Valley {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.rows {
			writeln!(f, "{row}")?;
		}
		Ok(())
	}
}

impl FromStr for Valley {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			rows: s.lines().map(String::from).collect::<Vec<_>>(),
			columns: s.lines().fold(Vec::new(), |mut columns, line| {
				line.chars().enumerate().for_each(|(i, c)| {
					if let Some(s) = columns.get_mut(i) {
						*s = format!("{}{c}", *s);
					} else {
						columns.push(c.to_string());
					}
				});
				columns
			}),
		})
	}
}

pub fn part_01(input: &str) -> u64 {
	input
		.split("\n\n")
		.map(|valley| valley.parse::<Valley>())
		.flatten()
		.fold(0, |s, mut valley| {
			s + match valley.find_reflection(0) {
				Reflection::Vertical(r) => r,
				Reflection::Horizontal(r) => r * 100,
				Reflection::None => 0,
			}
		})
}

pub fn part_02(input: &str) -> u64 {
	input
		.split("\n\n")
		.map(|valley| valley.parse::<Valley>())
		.flatten()
		.fold(0, |s, mut valley| {
			s + match valley.find_reflection(1) {
				Reflection::Vertical(r) => r,
				Reflection::Horizontal(r) => r * 100,
				Reflection::None => 0,
			}
		})
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			405
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_02")))),
			709
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_03")))),
			15
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			33047
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			400
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_02")))),
			1400
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_03")))),
			15
		);

		/*
		This is every answer I got trying to fix my algorithm. Feel my frustration.
		- 12687
		- 22303
		- 22920
		- 23110
		- 23906
		- 24959
		- 25271
		- 28601
		- 28806
		- 29008
		- 33047
		- 34179
		- 34992
		- 36974
		 */
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			28806
		);
	}
}
