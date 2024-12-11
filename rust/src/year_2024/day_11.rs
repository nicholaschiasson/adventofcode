use memoize::memoize;

#[memoize]
fn count_stones(stone: u64, blinks: u64) -> u64 {
    if blinks < 1 {
        return 1;
    }
    match (stone, stone.checked_ilog10()) {
        (0, _) => count_stones(1, blinks - 1),
        (_, Some(d)) if (d + 1) % 2 == 0 => {
            let factor = u64::pow(10, (d + 1) / 2);
            let lhs = stone / factor;
            let rhs = stone - (lhs * factor);
            count_stones(rhs, blinks - 1) + count_stones(lhs, blinks - 1)
        }
        _ => count_stones(stone * 2024, blinks - 1),
    }
}

pub fn part_01(input: &str) -> u64 {
    input
        .split_whitespace()
        .flat_map(|s| s.parse::<u64>())
        .fold(0, |stones, stone| stones + count_stones(stone, 25))
}

pub fn part_02(input: &str) -> u64 {
    input
        .split_whitespace()
        .flat_map(|s| s.parse::<u64>())
        .fold(0, |stones, stone| stones + count_stones(stone, 75))
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
            55312
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            185894
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            221632504974231
        );
    }
}
