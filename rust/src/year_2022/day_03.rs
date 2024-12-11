use std::collections::HashSet;

pub fn part_01(input: &str) -> u64 {
    let priorities = [
        (b'a'..=b'z').collect::<Vec<_>>(),
        (b'A'..=b'Z').collect::<Vec<_>>(),
    ]
    .concat();
    input
        .lines()
        .map(|r| {
            (priorities
                .iter()
                .position(|&i| {
                    i == r[..r.len() / 2]
                        .bytes()
                        .collect::<HashSet<_>>()
                        .intersection(&r[r.len() / 2..].bytes().collect::<HashSet<_>>())
                        .cloned()
                        .next()
                        .expect("item in both compartments of rucksack")
                })
                .expect("item having priority")
                + 1) as u64
        })
        .sum()
}

pub fn part_02(input: &str) -> u64 {
    let priorities = [
        (b'a'..=b'z').collect::<Vec<_>>(),
        (b'A'..=b'Z').collect::<Vec<_>>(),
    ]
    .concat();
    input
        .lines()
        .collect::<Vec<_>>()
        .windows(3)
        .step_by(3)
        .map(|s| {
            *s[0]
                .bytes()
                .collect::<HashSet<_>>()
                .intersection(&s[1].bytes().collect::<HashSet<_>>())
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&s[2].bytes().collect::<HashSet<_>>())
                .next()
                .expect("badge in rucksack")
        })
        .map(|b| {
            (priorities
                .iter()
                .position(|&i| i == b)
                .expect("item having priority")
                + 1) as u64
        })
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
                "{}::practice_01",
                INPUT_PATH
            )))),
            157
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            8109
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            70
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            2738
        );
    }
}
