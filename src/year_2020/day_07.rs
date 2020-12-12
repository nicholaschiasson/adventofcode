use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
	parents: HashSet<String>,
	children: HashMap<String, u8>,
}

impl Rule {
	fn new() -> Self {
		Rule {
			parents: HashSet::new(),
			children: HashMap::new(),
		}
	}
}

impl Clone for Rule {
	fn clone(&self) -> Self {
		let mut rule = Rule::new();
		self.parents.iter().for_each(|p| {
			rule.parents.insert(p.to_owned());
		});
		self.children.iter().for_each(|(c, n)| {
			rule.children.insert(c.to_owned(), *n);
		});
		rule
	}
}

fn parse_rules(rules: &String) -> HashMap<String, Rule> {
	let r_tups = rules
		.lines()
		.filter(|l| !l.ends_with(" no other bags."))
		.map(|l| l.split(" bags contain "))
		.map(|mut s| {
			let p = s.next().unwrap();
			s.next()
				.unwrap()
				.trim_end_matches('.')
				.split(", ")
				.map(|c| {
					let mut r = c.split(' ');
					(
						p.to_string(),
						r.next().unwrap().parse::<u8>().unwrap_or(0),
						r.collect::<Vec<&str>>()
							.join(" ")
							.trim_end_matches('s')
							.trim_end_matches(" bag")
							.to_string(),
					)
				})
				.collect::<Vec<(String, u8, String)>>()
		})
		.flatten()
		.collect::<Vec<(String, u8, String)>>();

	let mut m: HashMap<String, Rule> = HashMap::new();
	for (p, n, c) in r_tups {
		let mut rule = match m.get_mut(&p) {
			Some(r) => r.clone(),
			None => Rule::new(),
		};
		rule.children.insert(c.clone(), n);
		m.insert(p.clone(), rule);

		rule = match m.get_mut(&c) {
			Some(r) => r.clone(),
			None => Rule::new(),
		};
		rule.parents.insert(p.clone());
		m.insert(c.clone(), rule);
	}
	m
}

fn get_parents(rules: &HashMap<String, Rule>, parents: &mut HashSet<String>, name: &String) {
	rules.get(name).unwrap().parents.iter().for_each(|p| {
		parents.insert(p.to_string());
		get_parents(rules, parents, p);
	});
}

fn count_children(rules: &HashMap<String, Rule>, name: &String) -> u64 {
	rules.get(name).unwrap().children.iter().fold(0, |n, (c, c_num)| {
		let num = *c_num as u64;
		n + num + num * count_children(rules, c)
	})
}

pub fn part_01(input: &String) -> u64 {
	let mut parents = HashSet::new();
	get_parents(&parse_rules(input), &mut parents, &"shiny gold".to_string());
	parents.len() as u64
}

pub fn part_02(input: &String) -> u64 {
	count_children(&parse_rules(input), &"shiny gold".to_string())
}
