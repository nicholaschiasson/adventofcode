use std::collections::{HashMap, HashSet};

pub fn part_01(input: &str) -> u64 {
    let mut antennae: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = input.len() as isize;

    for (y, l) in input.iter().enumerate() {
        let width = l.len() as isize;
        let y = y as isize;
        for (x, &c) in l.iter().enumerate() {
            if c == '.' {
                continue;
            }
            let x = x as isize;
            let antennae = antennae.entry(c).or_default();
            for &(ax, ay) in antennae.iter() {
                let (dx, dy) = (x - ax, y - ay);
                let (x1, y1) = (ax - dx, ay - dy);
                let (x2, y2) = (x + dx, y + dy);
                if (0..width).contains(&x1) && (0..height).contains(&y1) {
                    antinodes.insert((x1, y1));
                }
                if (0..width).contains(&x2) && (0..height).contains(&y2) {
                    antinodes.insert((x2, y2));
                }
            }
            antennae.push((x, y));
        }
    }

    antinodes.len() as u64
}

pub fn part_02(input: &str) -> u64 {
    let mut antennae: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = input.len() as isize;

    for (y, l) in input.iter().enumerate() {
        let width = l.len() as isize;
        let y = y as isize;
        for (x, &c) in l.iter().enumerate() {
            if c == '.' {
                continue;
            }
            let x = x as isize;
            let antennae = antennae.entry(c).or_default();
            for &(ax, ay) in antennae.iter() {
                let (dx, dy) = (x - ax, y - ay);
                for i in 0.. {
                    let mut any = false;
                    let (x1, y1) = (ax - dx * i, ay - dy * i);
                    let (x2, y2) = (x + dx * i, y + dy * i);
                    if (0..width).contains(&x1) && (0..height).contains(&y1) {
                        antinodes.insert((x1, y1));
                        any = true;
                    }
                    if (0..width).contains(&x2) && (0..height).contains(&y2) {
                        antinodes.insert((x2, y2));
                        any = true;
                    }
                    if !any {
                        break;
                    }
                }
            }
            antennae.push((x, y));
        }
    }

    antinodes.len() as u64
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
            14
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            313
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            34
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            1064
        );
    }
}
