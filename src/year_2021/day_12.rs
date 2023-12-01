use std::collections::HashMap;

fn parse_caves(input: &String) -> HashMap<&str, Vec<&str>> {
	input.lines().fold(HashMap::<&str, Vec<&str>>::new(), |mut m, l| {
		if let Some((a, b)) = l.split_once("-") {
			if let Some(c) = m.get_mut(a) {
				c.push(b);
			} else {
				m.insert(a, vec![b]);
			}
			if let Some(c) = m.get_mut(b) {
				c.push(a);
			} else {
				m.insert(b, vec![a]);
			}
			m
		} else {
			panic!("Failed to parse cave connection {}", l)
		}
	})
}

fn dfs_count_paths(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str, extra_visits: usize) -> u64 {
	let mut trail = vec![(start, graph.get(start).unwrap().clone())];
	let mut visits = HashMap::new();
	let mut extras = extra_visits;
	graph.iter().for_each(|(c, _)| {
		visits.insert(*c, 0);
	});
	visits.insert(start, 1);
	let mut paths = 0;
	while let Some((c, n)) = trail.last_mut() {
		let cave = *c;
		if *c == end {
			paths += 1;
		}
		if *c == end || n.is_empty() {
			trail.pop();
			let n_visits = *visits.get(cave).unwrap();
			visits.insert(cave, n_visits - 1);
			if cave != end && cave.to_lowercase() == cave && n_visits > 1 && extras < extra_visits {
				extras += 1;
			}
			continue;
		}
		if let Some(next) = n.pop() {
			let n_visits = *visits.get(next).unwrap();
			visits.insert(next, n_visits + 1);
			if next.to_lowercase() == next && n_visits > 0 && extras > 0 {
				extras -= 1;
			}
			trail.push((
				next,
				graph
					.get(next)
					.unwrap()
					.iter()
					.filter(|&c| *c != start && (&(*c.to_uppercase()) == *c || *visits.get_mut(*c).unwrap() < 1 || extras > 0))
					.map(|&c| c)
					.collect(),
			));
		}
	}
	paths
}

pub fn part_01(input: &String) -> u64 {
	dfs_count_paths(&parse_caves(input), "start", "end", 0)
}

pub fn part_02(input: &String) -> u64 {
	dfs_count_paths(&parse_caves(input), "start", "end", 1)
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			10
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_02", INPUT_PATH)))),
			19
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_03", INPUT_PATH)))),
			226
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			5228
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			36
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_02", INPUT_PATH)))),
			103
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_03", INPUT_PATH)))),
			3509
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			131228
		);
	}
}
