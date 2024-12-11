use std::collections::HashMap;

pub fn part_01(input: &str) -> u64 {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut p| {
            (
                p.next().map(|n| n.parse::<u64>()),
                p.next().map(|n| n.parse::<u64>()),
            )
        })
        .unzip();
    let mut left: Vec<_> = left.into_iter().flatten().flatten().collect();
    let mut right: Vec<_> = right.into_iter().flatten().flatten().collect();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l.abs_diff(r)))
        .sum()
}

pub fn part_02(input: &str) -> u64 {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut p| {
            (
                p.next().map(|n| n.parse::<u64>()),
                p.next().map(|n| n.parse::<u64>()),
            )
        })
        .unzip();
    let left: Vec<_> = left.into_iter().flatten().flatten().collect();
    let right: Vec<_> = right.into_iter().flatten().flatten().collect();
    let occurrences: HashMap<u64, u64> = right.into_iter().fold(HashMap::new(), |mut m, n| {
        *m.entry(n).or_insert(0) += 1;
        m
    });
    left.into_iter()
        .flat_map(|n| occurrences.get(&n).or(Some(&0)).map(|&o| n * o))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            11
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            1530215
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            31
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            26800609
        );
    }
}
