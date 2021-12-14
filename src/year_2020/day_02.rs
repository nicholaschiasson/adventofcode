struct OccurrenceLimits {
	maximum: i32,
	minimum: i32,
}

struct PasswordPolicy {
	authority: PolicyAuthority,
	character: char,
	limits: OccurrenceLimits,
}

enum PolicyAuthority {
	OfficialTobogganCorporate,
	SledRental,
}

fn parse_policy(input: &String, authority: PolicyAuthority) -> PasswordPolicy {
	let policy_str = input.split(": ").collect::<Vec<&str>>()[0];
	let limits = policy_str.split(' ').collect::<Vec<&str>>()[0]
		.split('-')
		.collect::<Vec<&str>>();
	let character = policy_str.split(' ').collect::<Vec<&str>>()[1];
	PasswordPolicy {
		authority,
		character: character.parse().expect("Failed to parse policy character."),
		limits: OccurrenceLimits {
			maximum: limits[1].parse().expect("Failed to parse policy maximum limit."),
			minimum: limits[0].parse().expect("Failed to parse policy minimum limit."),
		},
	}
}

fn parse_password(input: &String) -> String {
	input.split(": ").collect::<Vec<&str>>()[1].into()
}

fn validate_password(password: &String, policy: &PasswordPolicy) -> bool {
	match policy.authority {
		PolicyAuthority::SledRental => {
			let occurrences = password.chars().fold(0, |a, c| a + if c == policy.character { 1 } else { 0 });
			occurrences >= policy.limits.minimum && occurrences <= policy.limits.maximum
		}
		PolicyAuthority::OfficialTobogganCorporate => {
			let char_arr: Vec<char> = password.chars().collect();
			(policy.limits.minimum as usize <= char_arr.len()
				&& char_arr[policy.limits.minimum as usize - 1] == policy.character
				&& char_arr[policy.limits.maximum as usize - 1] != policy.character)
				|| (policy.limits.maximum as usize <= char_arr.len()
					&& char_arr[policy.limits.minimum as usize - 1] != policy.character
					&& char_arr[policy.limits.maximum as usize - 1] == policy.character)
		}
	}
}

pub fn part_01(input: &String) -> u64 {
	let password_list: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
	let mut valid_passwords = 0;
	for p in password_list {
		let policy = parse_policy(&p, PolicyAuthority::SledRental);
		let password = parse_password(&p);
		valid_passwords += if validate_password(&password, &policy) { 1 } else { 0 };
	}
	valid_passwords
}

pub fn part_02(input: &String) -> u64 {
	let password_list: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
	let mut valid_passwords = 0;
	for p in password_list {
		let policy = parse_policy(&p, PolicyAuthority::OfficialTobogganCorporate);
		let password = parse_password(&p);
		valid_passwords += if validate_password(&password, &policy) { 1 } else { 0 };
	}
	valid_passwords
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))), 2);
		assert_eq!(super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))), 456);
	}

	#[test]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))), 1);
		assert_eq!(super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))), 308);
	}
}
