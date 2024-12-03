use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Do,
    DoNot,
    Mul,
}

impl Operator {
    fn len(&self) -> usize {
        self.to_string().len()
            + match self {
                Operator::Do => 2,
                Operator::DoNot => 2,
                Operator::Mul => 1,
            }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Do => "do",
                Operator::DoNot => "don't",
                Operator::Mul => "mul",
            }
        )
    }
}

pub fn part_01(input: &str) -> u64 {
    let mut i = 0;
    let mut operator = None;
    let mut left = None;
    let mut right = None;
    let mut sum = 0;
    'outer: while i < input.len() {
        match operator {
            Some(Operator::Mul) => {
                if let Some(l) = left {
                    if let Some(r) = right {
                        if let Some(b')') = input.as_bytes().get(i) {
                            sum += l * r;
                        }
                    } else if let Some(b',') = input.as_bytes().get(i) {
                        i += 1;
                        for j in (1..=3).rev() {
                            if let Some(Ok(n)) = input.get(i..i + j).map(|s| s.parse::<u64>()) {
                                right = Some(n);
                                i += j;
                                continue 'outer;
                            }
                        }
                    }
                } else {
                    for j in (1..=3).rev() {
                        if let Some(Ok(n)) = input.get(i..i + j).map(|s| s.parse::<u64>()) {
                            left = Some(n);
                            i += j;
                            continue 'outer;
                        }
                    }
                }
            }
            _ => {
                operator = match &input.get(i..i + 4) {
                    Some("mul(") => Some(Operator::Mul),
                    _ => None,
                };
                if let Some(op) = operator {
                    i += op.to_string().len() + 1;
                    continue;
                }
            }
        }
        operator = None;
        left = None;
        right = None;
        i += 1;
    }
    sum
}

pub fn part_02(input: &str) -> u64 {
    let mut i = 0;
    let mut do_op = true;
    let mut operator = None;
    let mut left = None;
    let mut right = None;
    let mut sum = 0;
    'outer: while i < input.len() {
        if let Some("do()") = input.get(i..i + Operator::Do.len()) {
            do_op = true;
            i += Operator::Do.len();
            operator = None;
        } else if let Some("don't()") = input.get(i..i + Operator::DoNot.len()) {
            do_op = false;
            i += Operator::DoNot.len();
            operator = None;
        }
        if do_op == true {
            match operator {
                Some(Operator::Mul) => {
                    if let Some(l) = left {
                        if let Some(r) = right {
                            if let Some(b')') = input.as_bytes().get(i) {
                                sum += l * r;
                            }
                        } else if let Some(b',') = input.as_bytes().get(i) {
                            i += 1;
                            for j in (1..=3).rev() {
                                if let Some(Ok(n)) = input.get(i..i + j).map(|s| s.parse::<u64>()) {
                                    right = Some(n);
                                    i += j;
                                    continue 'outer;
                                }
                            }
                        }
                    } else {
                        for j in (1..=3).rev() {
                            if let Some(Ok(n)) = input.get(i..i + j).map(|s| s.parse::<u64>()) {
                                left = Some(n);
                                i += j;
                                continue 'outer;
                            }
                        }
                    }
                }
                _ => {
                    operator = match &input.get(i..i + 4) {
                        Some("mul(") => Some(Operator::Mul),
                        _ => None,
                    };
                    if let Some(op) = operator {
                        i += op.len();
                        continue;
                    }
                }
            }
        }
        operator = None;
        left = None;
        right = None;
        i += 1;
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
            161
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            184511516
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_02"
            )))),
            48
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            90044227
        );
    }
}
