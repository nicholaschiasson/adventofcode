#[derive(Clone, Copy, Debug)]
struct Position {
	aim: i32,
	depth: i32,
	horizontal: i32,
}

impl Position {
	fn new() -> Self {
		Self {
			aim: 0,
			horizontal: 0,
			depth: 0,
		}
	}

	fn product(&self) -> i32 {
		self.depth * self.horizontal
	}

	fn forward(&mut self, x: i32) -> Self {
		self.horizontal += x;
		*self
	}

	fn down(&mut self, x: i32) -> Self {
		self.depth += x;
		*self
	}

	fn up(&mut self, x: i32) -> Self {
		self.depth -= x;
		*self
	}

	fn forward_aim(&mut self, x: i32) -> Self {
		self.horizontal += x;
		self.depth += self.aim * x;
		*self
	}

	fn down_aim(&mut self, x: i32) -> Self {
		self.aim += x;
		*self
	}

	fn up_aim(&mut self, x: i32) -> Self {
		self.aim -= x;
		*self
	}
}

pub fn part_01(input: &String) -> u64 {
	input
		.lines()
		.map(|x| x.split_whitespace())
		.map(|mut s| {
			(
				s.next().unwrap(),
				s.next().unwrap().parse::<i32>().expect("failed to parse integer"),
			)
		})
		.fold(Position::new(), |mut a, (c, x)| match c {
			"forward" => a.forward(x),
			"down" => a.down(x),
			"up" => a.up(x),
			_ => panic!("unknown command '{}'", c),
		})
		.product() as u64
}

pub fn part_02(input: &String) -> u64 {
	input
		.lines()
		.map(|x| x.split_whitespace())
		.map(|mut s| {
			(
				s.next().unwrap(),
				s.next().unwrap().parse::<i32>().expect("failed to parse integer"),
			)
		})
		.fold(Position::new(), |mut a, (c, x)| match c {
			"forward" => a.forward_aim(x),
			"down" => a.down_aim(x),
			"up" => a.up_aim(x),
			_ => panic!("unknown command '{}'", c),
		})
		.product() as u64
}
