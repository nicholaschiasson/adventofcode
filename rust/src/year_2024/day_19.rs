use std::collections::{HashMap, HashSet};

fn num_possible_designs<'a>(
    memo: &mut HashMap<&'a str, u64>,
    towels: &HashSet<&str>,
    design: &'a str,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&n) = memo.get(design) {
        return n;
    }

    let num_possibilities = towels
        .iter()
        .filter(|&towel| design.starts_with(towel))
        .fold(0, |possibilities, towel| {
            possibilities + num_possible_designs(memo, towels, &design[towel.len()..])
        });

    memo.insert(design, num_possibilities);

    num_possibilities
}

pub fn part_01(input: &str) -> u64 {
    let mut possible_designs = 0;
    let mut memo = HashMap::new();

    if let Some((towels, designs)) = input.split_once("\n\n") {
        let towels = towels.split(',').map(str::trim).collect::<HashSet<_>>();
        possible_designs += designs
            .lines()
            .map(|design| num_possible_designs(&mut memo, &towels, design))
            .filter(|&n| n > 0)
            .count() as u64;
    }

    possible_designs
}

pub fn part_02(input: &str) -> u64 {
    let mut possible_designs = 0;
    let mut memo = HashMap::new();

    if let Some((towels, designs)) = input.split_once("\n\n") {
        let towels = towels.split(',').map(str::trim).collect::<HashSet<_>>();
        possible_designs += designs
            .lines()
            .map(|design| num_possible_designs(&mut memo, &towels, design))
            .sum::<u64>()
    }

    possible_designs
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_1"
            )))),
            6
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            251
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_1"
            )))),
            16
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            616957151871345
        );
    }
}
