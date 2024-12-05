pub fn part_01(input: &str) -> u64 {
    let expenses: Vec<u64> = input
        .lines()
        .map(|e| e.parse::<u64>().expect("Failed to parse input"))
        .collect();
    for i in (1..expenses.len()).rev() {
        for j in 0..i {
            if expenses[i] + expenses[j] == 2020 {
                return expenses[i] * expenses[j];
            }
        }
    }
    0
}

pub fn part_02(input: &str) -> u64 {
    let expenses: Vec<u64> = input
        .lines()
        .map(|e| e.parse::<u64>().expect("Failed to parse input"))
        .collect();
    for i in (2..expenses.len()).rev() {
        for j in (1..i).rev() {
            let partial_sum = expenses[i] + expenses[j];
            if partial_sum <= 2020 {
                for k in 0..j {
                    if partial_sum + expenses[k] == 2020 {
                        return expenses[i] * expenses[j] * expenses[k];
                    }
                }
            }
        }
    }
    0
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
            514579
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            100419
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            241861950
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            265253940
        );
    }
}
