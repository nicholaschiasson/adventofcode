use std::fmt::Display;

enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> Option<u64> {
        if lhs < rhs {
            return None;
        }
        match self {
            Operator::Add => Some(lhs - rhs),
            Operator::Mul => {
                if lhs % rhs == 0 {
                    lhs.checked_div(rhs)
                } else {
                    None
                }
            }
            Operator::Cat => {
                let difference = lhs - rhs;
                let denominator = u64::pow(10, rhs.ilog10() + 1);
                if difference % denominator == 0 {
                    difference.checked_div(denominator)
                } else {
                    None
                }
            }
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Mul => "*",
                Operator::Cat => "||",
            }
        )
    }
}

fn is_possible(target: u64, operators: &[Operator], operands: &[u64]) -> bool {
    if let Some((&last, ops)) = operands.split_last() {
        operators.iter().any(|operator| {
            if let Some(t) = operator.apply(target, last) {
                is_possible(t, operators, ops)
            } else {
                false
            }
        })
    } else {
        target == 0
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
                if is_possible(target, &[Mul, Add], &operands) {
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
                if is_possible(target, &[Mul, Cat, Add], &operands) {
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
