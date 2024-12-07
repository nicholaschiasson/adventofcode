enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn operate(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
            Self::Cat => lhs * u64::pow(10, rhs.ilog10() + 1) + rhs,
        }
    }
}

fn is_possible(target: u64, accumulator: u64, operands: &[u64], operators: &[Operator]) -> bool {
    if let Some((&first, ops)) = operands.split_first() {
        operators
            .iter()
            .any(|op| is_possible(target, op.operate(accumulator, first), ops, operators))
    } else {
        accumulator == target
    }
}

pub fn part_01(input: &str) -> u64 {
    use Operator::*;
    let mut sum = 0;
    for callibration in input.lines().collect::<Vec<_>>() {
        if let Some((target, operands)) = callibration.split_once(": ") {
            let target = target.parse::<u64>();
            let operands = operands
                .split(' ')
                .flat_map(|o| o.parse::<u64>())
                .collect::<Vec<_>>();
            if let Ok(target) = target {
                if is_possible(target, 0, &operands, &[Add, Mul]) {
                    sum += target;
                }
            }
        }
    }
    sum
}

pub fn part_02(input: &str) -> u64 {
    use Operator::*;
    let mut sum = 0;
    for callibration in input.lines().collect::<Vec<_>>() {
        if let Some((target, operands)) = callibration.split_once(": ") {
            let target = target.parse::<u64>();
            let operands = operands
                .split(' ')
                .flat_map(|o| o.parse::<u64>())
                .collect::<Vec<_>>();
            if let Ok(target) = target {
                if is_possible(target, 0, &operands, &[Add, Mul, Cat]) {
                    sum += target;
                }
            }
        }
    }
    sum
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
            3749
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            12940396350192
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            11387
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            106016735664498
        );
    }
}
