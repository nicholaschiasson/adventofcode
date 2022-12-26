use regex::Regex;

#[derive(Debug)]
struct Passport {
	byr: Option<String>,
	iyr: Option<String>,
	eyr: Option<String>,
	hgt: Option<String>,
	hcl: Option<String>,
	ecl: Option<String>,
	pid: Option<String>,
	cid: Option<String>,
}

impl Passport {
	fn new() -> Self {
		Passport {
			byr: None,
			iyr: None,
			eyr: None,
			hgt: None,
			hcl: None,
			ecl: None,
			pid: None,
			cid: None,
		}
	}

	fn has_valid_byr(&self) -> bool {
		if let Some(byr) = &self.byr {
			return match byr.parse::<u32>() {
				Ok(v) => v >= 1920 && v <= 2002,
				_ => false,
			};
		}
		false
	}

	fn has_valid_iyr(&self) -> bool {
		if let Some(iyr) = &self.iyr {
			return match iyr.parse::<u32>() {
				Ok(v) => v >= 2010 && v <= 2020,
				_ => false,
			};
		}
		false
	}

	fn has_valid_eyr(&self) -> bool {
		if let Some(eyr) = &self.eyr {
			return match eyr.parse::<u32>() {
				Ok(v) => v >= 2020 && v <= 2030,
				_ => false,
			};
		}
		false
	}

	fn has_valid_hgt(&self) -> bool {
		if let Some(hgt) = &self.hgt {
			let r = Regex::new(r"^(\d+)(cm|in)$").unwrap();
			return match r.captures(hgt) {
				Some(c) => match c.get(2).map(|m| m.as_str()) {
					Some("cm") => {
						let h = c.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
						h >= 150 && h <= 193
					},
					Some("in") => {
						let h = c.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
						h >= 59 && h <= 76
					},
					_ => false,
				},
				_ => false,
			};
		}
		false
	}

	fn has_valid_hcl(&self) -> bool {
		if let Some(hcl) = &self.hcl {
			let r = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
			return r.is_match(hcl);
		}
		false
	}

	fn has_valid_ecl(&self) -> bool {
		if let Some(ecl) = &self.ecl {
			let r = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
			return r.is_match(ecl);
		}
		false
	}

	fn has_valid_pid(&self) -> bool {
		if let Some(pid) = &self.pid {
			let r = Regex::new(r"^\d{9}$").unwrap();
			return r.is_match(pid);
		}
		false
	}

	fn is_valid(&self) -> bool {
		self.has_valid_byr()
			&& self.has_valid_iyr()
			&& self.has_valid_eyr()
			&& self.has_valid_hgt()
			&& self.has_valid_hcl()
			&& self.has_valid_ecl()
			&& self.has_valid_pid()
	}

	fn has_required_fields(&self) -> bool {
		self.byr.is_some()
			&& self.iyr.is_some()
			&& self.eyr.is_some()
			&& self.hgt.is_some()
			&& self.hcl.is_some()
			&& self.ecl.is_some()
			&& self.pid.is_some()
	}
}

fn parse_passport(passport: &String) -> Passport {
	passport.replace(' ', "\n").lines().fold(Passport::new(), |mut p, l| {
		let mut kvp = l.split(':');
		match kvp.next() {
			Some("byr") => p.byr = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("iyr") => p.iyr = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("eyr") => p.eyr = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("hgt") => p.hgt = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("hcl") => p.hcl = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("ecl") => p.ecl = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("pid") => p.pid = Some(kvp.next().expect("Failed to get value").to_string()),
			Some("cid") => p.cid = Some(kvp.next().expect("Failed to get value").to_string()),
			_ => {},
		};
		p
	})
}

fn parse_passport_batch(batch: &String) -> Vec<Passport> {
	batch.split("\n\n").map(|p| parse_passport(&p.to_string())).collect()
}

pub fn part_01(input: &String) -> u64 {
	parse_passport_batch(input)
		.iter()
		.fold(0, |v, p| v + if p.has_required_fields() { 1 } else { 0 })
}

pub fn part_02(input: &String) -> u64 {
	parse_passport_batch(input)
		.iter()
		.fold(0, |v, p| v + if p.is_valid() { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			2
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			213
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			2
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			147
		);
	}
}
