pub fn part_01(input: &str) -> u64 {
    let reports: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|n| n.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .map(|l| l.windows(2).map(|w| (w[0] - w[1])).collect())
        .collect();

    let mut not_safe = 0;
    for r in &reports {
        let mut sign = 0;
        for &d in r {
            let s = d.signum();
            if sign == 0 {
                sign = s;
            }
            if s == 0 || s != sign || !(-3..=3).contains(&d) {
                not_safe += 1;
                break;
            }
        }
    }
    (reports.len() - not_safe) as u64
}

pub fn part_02(input: &str) -> u64 {
    let reports: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|n| n.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut safe = 0;
    for report in &reports {
        let mut rev = report.clone();
        rev.reverse();
        if is_safe(report) || is_safe(&rev) {
            safe += 1;
        }
    }
    safe
}

fn is_safe(report: &Vec<i64>) -> bool {
    let mut sign = 0;
    let mut damper = None;
    let mut previous = None;
    for &level in report {
        if let Some(p) = previous {
            let diff: i64 = p - level;
            let s = diff.signum();
            let pre_damper_sign = sign;
            if sign == 0 {
                sign = s;
            }
            if s == 0 || s != sign || !(-3..=3).contains(&diff) {
                if damper.is_none() {
                    damper = Some(level);
                    sign = pre_damper_sign;
                    continue;
                } else {
                    return false;
                }
            }
        }
        previous = Some(level);
    }
    true
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
            2
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            252
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            4
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            324
        );
    }
}
