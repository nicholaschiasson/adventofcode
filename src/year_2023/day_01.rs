pub fn part_01(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u64))
                .collect::<Vec<_>>()
        })
        .map(|n| (n[0] * 10) + n[n.len() - 1])
        .sum()
}

pub fn part_02(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            (0..l.len())
                .filter_map(|i| {
                    let ss = &l[i..];
                    if ss.starts_with("one") {
                        Some(1)
                    } else if ss.starts_with("two") {
                        Some(2)
                    } else if ss.starts_with("three") {
                        Some(3)
                    } else if ss.starts_with("four") {
                        Some(4)
                    } else if ss.starts_with("five") {
                        Some(5)
                    } else if ss.starts_with("six") {
                        Some(6)
                    } else if ss.starts_with("seven") {
                        Some(7)
                    } else if ss.starts_with("eight") {
                        Some(8)
                    } else if ss.starts_with("nine") {
                        Some(9)
                    } else {
                        ss.chars().next().and_then(|c| c.to_digit(10))
                    }
                    .map(|n| n as u64)
                })
                .collect::<Vec<_>>()
        })
        .map(|n| (n[0] * 10) + n[n.len() - 1])
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
            142
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            54990
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            142
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_02"
            )))),
            281
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            54473
        );
    }
}
