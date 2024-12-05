pub fn part_01(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|c| {
                    c.parse::<u64>()
                        .expect("parse number of calories from string")
                })
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part_02(input: &str) -> u64 {
    let mut calories: Vec<u64> = input
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|c| {
                    c.parse::<u64>()
                        .expect("parse number of calories from string")
                })
                .sum()
        })
        .collect();
    calories.sort();
    calories.iter().rev().take(3).sum()
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
            24000
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            71924
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            45000
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            210406
        );
    }
}
