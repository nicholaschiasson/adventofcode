use std::collections::{HashMap, HashSet};

#[allow(unused)]
struct PolymerTemplate {
    polymer: Vec<char>,
}

struct PolymerFormula {
    insertion_rules: HashMap<(char, char), char>,
    template: Vec<char>,
}

impl PolymerFormula {
    fn occurrences(&self) -> HashMap<char, usize> {
        let elements = self.template.iter().collect::<HashSet<_>>();
        elements.iter().fold(HashMap::new(), |mut m, &e| {
            m.insert(*e, self.template.iter().filter(|&c| c == e).count());
            m
        })
    }

    fn step(&mut self) {
        let mut new_template = Vec::new();
        for i in 0..self.template.len() {
            new_template.push(self.template[i]);
            if i < self.template.len() - 1 {
                new_template.push(
                    *self
                        .insertion_rules
                        .get(&(self.template[i], self.template[i + 1]))
                        .unwrap(),
                );
            }
        }
        self.template = new_template;
    }
}

pub fn part_01(input: &String) -> u64 {
    let mut formula = PolymerFormula {
        template: input
            .split("\n\n")
            .take(1)
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>(),
        insertion_rules: input.split("\n\n").last().unwrap().lines().fold(
            HashMap::new(),
            |mut m, r| {
                let (k, v) = r.split_once(" -> ").unwrap();
                m.insert(
                    (k.chars().next().unwrap(), k.chars().nth(1).unwrap()),
                    v.chars().next().unwrap(),
                );
                m
            },
        ),
    };
    for _ in 0..10 {
        formula.step();
    }
    let occurrences = formula.occurrences();
    (occurrences.values().max().unwrap() - occurrences.values().min().unwrap()) as u64
}

pub fn part_02(input: &String) -> u64 {
    let mut formula = PolymerFormula {
        template: input
            .split("\n\n")
            .take(1)
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>(),
        insertion_rules: input.split("\n\n").last().unwrap().lines().fold(
            HashMap::new(),
            |mut m, r| {
                let (k, v) = r.split_once(" -> ").unwrap();
                m.insert(
                    (k.chars().next().unwrap(), k.chars().nth(1).unwrap()),
                    v.chars().next().unwrap(),
                );
                m
            },
        ),
    };
    for _ in 0..40 {
        formula.step();
    }
    let occurrences = formula.occurrences();
    (occurrences.values().max().unwrap() - occurrences.values().min().unwrap()) as u64
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
            1588
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            3284
        );
    }

    #[ignore]
    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            2188189693529
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            0
        );
    }
}
