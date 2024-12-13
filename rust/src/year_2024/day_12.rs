use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);

fn fill_region(
    grid: &Grid,
    (x, y): Point,
    plant: char,
    visited: &mut HashSet<Point>,
) -> (u64, u64, u64) {
    if visited.contains(&(x, y)) {
        return (0, 0, 0);
    }
    visited.insert((x, y));

    let mut fences = 0;
    let mut area = 1;

    let mut neighbours = Vec::new();

    for i in 0..4 {
        let i = i * 2 + 1;
        let dx = i % 3;
        let dy = i / 3;
        if y + dy < 1 || x + dx < 1 {
            fences += 1;
            continue;
        }
        match grid.get(y + dy - 1).and_then(|row| row.get(x + dx - 1)) {
            Some(&p) if p == plant => {
                neighbours.push((x + dx - 1, y + dy - 1));
            }
            _ => fences += 1,
        }
    }

    let mut kernel = [[false; 3]; 3];
    for i in 0..9 {
        let dx = i % 3;
        let dy = i / 3;
        if x + dx < 1 || y + dy < 1 {
            continue;
        }
        match grid.get(y + dy - 1).and_then(|row| row.get(x + dx - 1)) {
            Some(&p) if p == plant => {
                kernel[dy][dx] = true;
            }
            _ => (),
        }
    }

    let mut corners = 0;
    corners += ((!kernel[0][1] && !kernel[1][0]) || (!kernel[0][0] && kernel[0][1] && kernel[1][0]))
        as u64;
    corners += ((!kernel[0][1] && !kernel[1][2]) || (!kernel[0][2] && kernel[0][1] && kernel[1][2]))
        as u64;
    corners += ((!kernel[2][1] && !kernel[1][0]) || (!kernel[2][0] && kernel[2][1] && kernel[1][0]))
        as u64;
    corners += ((!kernel[2][1] && !kernel[1][2]) || (!kernel[2][2] && kernel[2][1] && kernel[1][2]))
        as u64;

    for neighbour in neighbours {
        let (f, c, a) = fill_region(grid, neighbour, plant, visited);
        fences += f;
        corners += c;
        area += a;
    }

    (fences, corners, area)
}

pub fn part_01(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut cost = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &plant) in row.iter().enumerate() {
            let (fences, _, area) = fill_region(&grid, (x, y), plant, &mut visited);
            cost += fences * area;
        }
    }
    cost
}

pub fn part_02(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut cost = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &plant) in row.iter().enumerate() {
            let (_, sides, area) = fill_region(&grid, (x, y), plant, &mut visited);
            cost += sides * area;
        }
    }
    println!();
    cost
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
            140
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_2"
            )))),
            772
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_3"
            )))),
            1930
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            1424006
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_1"
            )))),
            80
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_2"
            )))),
            436
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_3"
            )))),
            1206
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_4"
            )))),
            236
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_5"
            )))),
            368
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            858684
        );
    }
}
