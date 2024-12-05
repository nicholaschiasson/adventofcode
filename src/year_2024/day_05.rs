use std::collections::{HashMap, HashSet};

type Rules = HashMap<u64, HashSet<u64>>;
type Updates = Vec<Vec<u64>>;

pub fn part_01(input: &str) -> u64 {
    let input: Option<(Rules, Updates)> = input.split_once("\n\n").map(|(rules, updates)| {
        (
            rules.lines().fold(HashMap::new(), |mut r: Rules, pages| {
                if let Some((Ok(p1), Ok(p2))) = pages
                    .split_once('|')
                    .map(|(p1, p2)| (p1.parse::<u64>(), p2.parse::<u64>()))
                {
                    r.entry(p1).or_default().insert(p2);
                }
                r
            }),
            updates
                .lines()
                .map(|l| {
                    l.split(',')
                        .flat_map(|page| page.parse::<u64>())
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    });

    let mut sum = 0;

    if let Some((rules, updates)) = input {
        'outer: for update in &updates {
            let mut seen = HashSet::new();
            for &page in update {
                for p in &seen {
                    if let Some(true) = rules.get(&page).map(|rule| rule.contains(p)) {
                        continue 'outer;
                    }
                }
                seen.insert(page);
            }
            sum += update[update.len() / 2];
        }
    }

    sum
}

pub fn part_02(input: &str) -> u64 {
    let input: Option<(Rules, Updates)> = input.split_once("\n\n").map(|(rules, updates)| {
        (
            rules.lines().fold(HashMap::new(), |mut r: Rules, pages| {
                if let Some((Ok(p1), Ok(p2))) = pages
                    .split_once('|')
                    .map(|(p1, p2)| (p1.parse::<u64>(), p2.parse::<u64>()))
                {
                    r.entry(p1).or_default().insert(p2);
                }
                r
            }),
            updates
                .lines()
                .map(|l| {
                    l.split(',')
                        .flat_map(|page| page.parse::<u64>())
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    });

    let mut sum = 0;

    if let Some((rules, updates)) = input {
        for update in &updates {
            let mut seen = Vec::new();
            let mut ordered = true;
            'outer: for &page in update {
                for i in 0..seen.len() {
                    if let Some(true) = rules.get(&page).map(|rule| rule.contains(&seen[i])) {
                        ordered = false;
                        seen.insert(i, page);
                        continue 'outer;
                    }
                }
                seen.push(page);
            }
            if !ordered {
                sum += seen[seen.len() / 2];
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
            143
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            5588
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            123
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            5331
        );
    }
}
