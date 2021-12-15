use std::collections::VecDeque;

const MAX_ENERGY: u32 = 9;
const MIN_ENERGY: u32 = 0;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point(i32, i32);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DumboOctopus {
	energy: u32,
	position: Point,
}

impl DumboOctopus {
	fn energize(&mut self) -> bool {
		self.energy += 1;
		self.energy == MAX_ENERGY + 1
	}

	fn cooldown(&mut self) {
		if self.energy > MAX_ENERGY {
			self.energy = MIN_ENERGY;
		}
	}

	fn neighbours(&self) -> Vec<Point> {
		((self.position.1 - 1)..=(self.position.1 + 1))
			.map(|y| {
				((self.position.0 - 1)..=(self.position.0 + 1))
					.map(|x| Point(x, y))
					.collect::<Vec<_>>()
			})
			.flatten()
			.filter(|&p| p != self.position)
			.collect()
	}
}

fn step(octopuses: &mut Vec<Vec<DumboOctopus>>) -> u64 {
	let mut flashes = 0;
	let mut to_energize = octopuses.iter().flatten().map(|&o| o.position).collect::<VecDeque<_>>();
	while let Some(Point(x, y)) = to_energize.pop_front() {
		let o = &mut octopuses[y as usize][x as usize];
		if o.energize() {
			flashes += 1;
			for p in o.neighbours() {
				if p.1 >= 0
					&& p.0 >= 0
					&& (p.1 as usize) < octopuses.len()
					&& (p.0 as usize) < octopuses[p.1 as usize].len()
					&& octopuses[p.1 as usize][p.0 as usize].energy <= MAX_ENERGY
				{
					to_energize.push_back(p);
				}
			}
		}
	}
	for y in 0..octopuses.len() {
		for x in 0..octopuses[y].len() {
			octopuses[y][x].cooldown();
		}
	}
	flashes
}

pub fn part_01(input: &String) -> u64 {
	let mut octopuses = input
		.lines()
		.enumerate()
		.map(|(y, l)| {
			l.chars()
				.enumerate()
				.map(|(x, c)| DumboOctopus {
					energy: c.to_digit(10).unwrap(),
					position: Point(x as i32, y as i32),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	(0..100).fold(0, |f, _| f + step(&mut octopuses))
}

pub fn part_02(input: &String) -> u64 {
	let mut octopuses = input
		.lines()
		.enumerate()
		.map(|(y, l)| {
			l.chars()
				.enumerate()
				.map(|(x, c)| DumboOctopus {
					energy: c.to_digit(10).unwrap(),
					position: Point(x as i32, y as i32),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let mut steps = 1;
	while (step(&mut &mut octopuses) as usize) != octopuses.iter().flatten().count() {
		steps += 1;
	}
	steps
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			1656
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			1642
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			195
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			320
		);
	}
}
