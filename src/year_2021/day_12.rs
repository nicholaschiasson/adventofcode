use std::collections::HashMap;

fn dfs_count_paths(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> u64 {
	let mut trail = vec![(start, graph.get(start).unwrap().clone())];
	let mut visits = HashMap::new();
	graph.iter().for_each(|(c, _)| {
		visits.insert(*c, 0);
	});
	visits.insert(start, 1);
	let mut paths = 0;
	while let Some((c, n)) = trail.last_mut() {
		let cave = c.clone();
		if *c == end {
			paths += 1;
		}
		if *c == end || n.is_empty() {
			trail.pop();
			visits.insert(cave, visits.get(cave).unwrap() - 1);
			continue;
		}
		if let Some(next) = n.pop() {
			trail.push((
				next,
				graph
					.get(next)
					.unwrap()
					.iter()
					.filter(|&c| *visits.get_mut(*c).unwrap() < 1 || &(*c.to_uppercase()) == *c)
					.map(|&c| c)
					.collect(),
			));
			visits.insert(next, visits.get(next).unwrap() + 1);
		}
	}
	paths
}

pub fn part_01(input: &String) -> u64 {
	let caves = input.lines().fold(HashMap::<&str, Vec<&str>>::new(), |mut m, l| {
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
	});
	dfs_count_paths(&caves, "start", "end")
}

pub fn part_02(_input: &String) -> u64 {
	todo!()
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
			0
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_02", INPUT_PATH)))),
			0
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_03", INPUT_PATH)))),
			0
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			0
		);
	}
}
