use std::{
	collections::{HashMap, HashSet},
	ops::RangeInclusive,
};

type Rules = HashMap<String, Vec<RangeInclusive<u64>>>;

fn parse_input(input: &String) -> (Rules, Vec<u64>, Vec<Vec<u64>>) {
	let rejoined = input.lines().collect::<Vec<_>>().join("\n");
	let input_split = rejoined.split("\n\n").collect::<Vec<_>>();
	(
		parse_rules(&input_split[0].to_string()),
		parse_tickets(&input_split[1].to_string())[0].clone(),
		parse_tickets(&input_split[2].to_string()),
	)
}

fn parse_rules(rules: &String) -> Rules {
	rules.lines().fold(HashMap::new(), |mut m: Rules, rule: &str| {
		let r_split = rule.split(": ").collect::<Vec<_>>();
		r_split[1].split(" or ").for_each(|r| {
			let range = r.split('-').map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
			if let Some(ranges) = m.get_mut(&r_split[0].to_string()) {
				ranges.push(range[0]..=range[1]);
			} else {
				m.insert(r_split[0].to_string(), vec![range[0]..=range[1]]);
			}
		});
		return m;
	})
}

fn parse_tickets(tickets: &String) -> Vec<Vec<u64>> {
	tickets
		.lines()
		.skip(1)
		.map(|t| t.split(',').map(|f| f.parse::<u64>().unwrap()).collect::<Vec<_>>())
		.collect::<Vec<_>>()
}

pub fn part_01(input: &String) -> u64 {
	let (rules, _, tickets) = parse_input(input);
	tickets.iter().fold(0, |error_rate, ticket| {
		ticket.iter().fold(error_rate, |e, f| {
			if rules.values().all(|r| r.iter().all(|range| !range.contains(f))) {
				e + f
			} else {
				e
			}
		})
	})
}

pub fn part_02(input: &String) -> u64 {
	let (rules, my_ticket, tickets) = parse_input(input);
	let my_ticket_copy = my_ticket.clone();
	let mut fields = my_ticket
		.iter()
		.map(|_| rules.keys().cloned().collect::<HashSet<String>>())
		.collect::<Vec<_>>();

	tickets
		.iter()
		.filter(|ticket| {
			ticket
				.iter()
				.all(|f| rules.values().any(|r| r.iter().any(|range| range.contains(f))))
		})
		.chain(vec![my_ticket].iter())
		.for_each(|ticket| {
			ticket.iter().enumerate().for_each(|(i, f)| {
				for (name, rule) in &rules {
					if !fields[i].contains(name) {
						continue;
					}
					if !rule.iter().any(|range| range.contains(f)) {
						fields[i].remove(name);
					}
				}
			})
		});

	let mut field_sorted = fields.iter().enumerate().collect::<Vec<_>>();
	field_sorted.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

	let mut field_indices: HashMap<String, usize> = HashMap::new();
	for (i, names) in field_sorted {
		field_indices.insert(
			(*names.iter().find(|name| !field_indices.contains_key(*name)).unwrap()).clone(),
			i,
		);
	}

	field_indices
		.iter()
		.filter(|(k, _)| k.starts_with("departure "))
		.fold(1, |p, (_, i)| p as u64 * my_ticket_copy[*i])
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 25895);
	}

	#[test]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 5865723727753);
	}
}
