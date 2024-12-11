use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn above(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum TileType {
    PipeVertical,
    PipeHorizontal,
    PipeTopLeft,
    PipeTopRight,
    PipeBottomLeft,
    PipeBottomRight,
    Ground,
    Start,
    SubSpace,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileType::PipeVertical => '|',
                TileType::PipeHorizontal => '-',
                TileType::PipeTopLeft => 'J',
                TileType::PipeTopRight => 'L',
                TileType::PipeBottomLeft => '7',
                TileType::PipeBottomRight => 'F',
                TileType::Ground => '.',
                TileType::Start => 'S',
                TileType::SubSpace => ' ',
            }
        )
    }
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '|' => TileType::PipeVertical,
            '-' => TileType::PipeHorizontal,
            'J' => TileType::PipeTopLeft,
            'L' => TileType::PipeTopRight,
            '7' => TileType::PipeBottomLeft,
            'F' => TileType::PipeBottomRight,
            '.' => TileType::Ground,
            'S' => TileType::Start,
            ' ' => TileType::SubSpace,
            _ => panic!("uh oh"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Behavior {
    Connection(Position, Position),
    Ground,
    Start,
    Subspace,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Tile {
    position: Position,
    behavior: Behavior,
    tile_type: TileType,
}

impl Tile {
    fn new(position: Position, tile_type: TileType) -> Self {
        use Behavior::*;
        Self {
            position,
            behavior: match tile_type {
                TileType::PipeVertical => Connection(position.above(), position.below()),
                TileType::PipeHorizontal => Connection(position.left(), position.right()),
                TileType::PipeTopLeft => Connection(position.above(), position.left()),
                TileType::PipeTopRight => Connection(position.above(), position.right()),
                TileType::PipeBottomLeft => Connection(position.left(), position.below()),
                TileType::PipeBottomRight => Connection(position.right(), position.below()),
                TileType::Ground => Ground,
                TileType::Start => Start,
                TileType::SubSpace => Subspace,
            },
            tile_type,
        }
    }

    fn connected(&self) -> Vec<Position> {
        use Behavior::*;
        match self.behavior {
            Connection(a, b) => vec![a, b],
            Start => vec![
                self.position.above(),
                self.position.below(),
                self.position.left(),
                self.position.right(),
            ],
            Ground | Subspace => Vec::new(),
        }
    }

    fn x(&self) -> i32 {
        self.position.x
    }

    fn y(&self) -> i32 {
        self.position.y
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn expand(&self) -> Grid {
        use TileType::*;
        let mut grid = Vec::new();
        for (y, row) in &self.0.iter().enumerate().collect::<Vec<_>>() {
            let mut row_a = Vec::new();
            let mut row_b = Vec::new();
            for (x, &tile) in row.iter().enumerate().collect::<Vec<_>>() {
                let t = Tile::new(
                    Position {
                        x: tile.x() * 2,
                        y: tile.y() * 2,
                    },
                    tile.tile_type,
                );
                row_a.push(t);
                row_a.push(match (t.tile_type, row.get(x + 1).map(|t| t.tile_type)) {
                    (PipeHorizontal | PipeTopRight | PipeBottomRight, _) => {
                        Tile::new(t.position.right(), PipeHorizontal)
                    }
                    (Start, Some(PipeHorizontal | PipeTopLeft | PipeBottomLeft)) => {
                        Tile::new(t.position.right(), PipeHorizontal)
                    }
                    _ => Tile::new(t.position.right(), SubSpace),
                });
                row_b.push(
                    match (t.tile_type, self.0.get(y + 1).map(|r| r[x].tile_type)) {
                        (PipeVertical | PipeBottomLeft | PipeBottomRight, _) => {
                            Tile::new(t.position.below(), PipeVertical)
                        }
                        (Start, Some(PipeVertical | PipeTopLeft | PipeTopRight)) => {
                            Tile::new(t.position.below(), PipeVertical)
                        }
                        _ => Tile::new(t.position.below(), SubSpace),
                    },
                );
                row_b.push(Tile::new(t.position.right().below(), SubSpace));
            }
            grid.push(row_a);
            grid.push(row_b);
        }
        Grid(grid)
    }

    fn escape(&self, pos: Position) -> Option<u64> {
        let (g_x, g_y) = (pos.x as usize, pos.y as usize);
        if !(0..self.0.len()).contains(&g_y) || !(0..self.0[0].len()).contains(&g_x) {
            return None;
        }
        if let TileType::Ground | TileType::SubSpace = self.0[g_y][g_x].tile_type {
        } else {
            return None;
        }
        let mut area = 0;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([pos]);
        while let Some(p) = queue.pop_front() {
            let (g_x, g_y) = (p.x as usize, p.y as usize);
            if !(0..self.0.len()).contains(&g_y) || !(0..self.0[0].len()).contains(&g_x) {
                return None;
            }
            if !seen.insert(p) {
                continue;
            }
            let tile = self.0[g_y][g_x];
            if let TileType::Ground = tile.tile_type {
                area += 1;
            }
            if let TileType::Ground | TileType::SubSpace = tile.tile_type {
                queue.extend([p.above(), p.below(), p.left(), p.right()]);
            }
        }
        Some(area)
    }

    fn are_connected(&self, pos_a: &Position, pos_b: &Position) -> bool {
        let (x_a, y_a) = (pos_a.x as usize, pos_a.y as usize);
        let (x_b, y_b) = (pos_b.x as usize, pos_b.y as usize);
        let (range_y, range_x) = (0..self.0.len(), 0..self.0[0].len());
        if !range_y.contains(&y_a)
            || !range_y.contains(&y_b)
            || !range_x.contains(&x_a)
            || !range_x.contains(&x_b)
        {
            return false;
        }
        self.0[y_a][x_a].connected().contains(pos_b) && self.0[y_b][x_b].connected().contains(pos_a)
    }

    fn isolate_loop(&mut self) -> &mut Self {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                let tile = self.0[y][x];
                let mut queue = VecDeque::new();
                match tile.tile_type {
                    TileType::Ground => {
                        queue.push_back((tile.position.above(), tile.position));
                        queue.push_back((tile.position.below(), tile.position));
                        queue.push_back((tile.position.left(), tile.position));
                        queue.push_back((tile.position.right(), tile.position));
                    }
                    TileType::Start => (),
                    _ => {
                        if tile
                            .connected()
                            .iter()
                            .any(|c| !self.are_connected(&tile.position, c))
                        {
                            queue.push_back((tile.position, tile.position));
                        }
                    }
                }
                while let Some((p, from)) = queue.pop_front() {
                    let (g_x, g_y) = (p.x as usize, p.y as usize);
                    if !(0..self.0.len()).contains(&g_y)
                        || !(0..self.0[0].len()).contains(&g_x)
                        || (p != from && !self.0[g_y][g_x].connected().contains(&from))
                        || self.0[g_y][g_x].tile_type == TileType::Start
                    {
                        continue;
                    }
                    queue.extend(
                        self.0[g_y][g_x]
                            .connected()
                            .iter()
                            .map(|&c| (c, p))
                            .collect::<Vec<_>>(),
                    );
                    self.0[g_y][g_x] = Tile::new(p, TileType::Ground);
                }
            }
        }
        self
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(f, "{}", tile.tile_type)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_01(input: &str) -> u64 {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    let tile = Tile::new((j, i).into(), c.into());
                    if let Behavior::Start = tile.behavior {
                        start = Some(tile.position);
                    }
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = start.unwrap();
    let mut distance_grid = vec![vec![0u64; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::from([(start, 0, start)]);
    let mut max_distance = 0;

    while let Some((pos, distance, prev)) = queue.pop_front() {
        let g_y = pos.y as usize;
        let g_x = pos.x as usize;
        distance_grid[g_y][g_x] = distance;
        if distance > max_distance {
            max_distance = distance;
        }
        queue.extend(
            grid[g_y][g_x]
                .connected()
                .iter()
                .filter(|&c| {
                    let g_y = c.y as usize;
                    let g_x = c.x as usize;
                    if !(0..distance_grid.len()).contains(&g_y)
                        || !(0..distance_grid[0].len()).contains(&g_x)
                    {
                        return false;
                    }
                    let tile = &grid[g_y][g_x];
                    distance_grid[g_y][g_x] < 1
                        && tile.position != prev
                        && tile.connected().contains(&pos)
                })
                .map(|&c| (c, distance + 1, pos)),
        );
    }

    max_distance
}

pub fn part_02(input: &str) -> u64 {
    let mut start = None;
    let grid = Grid(
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let tile = Tile::new((j, i).into(), c.into());
                        if let Behavior::Start = tile.behavior {
                            start = Some(tile.position);
                        }
                        tile
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .isolate_loop()
    .expand();

    let start = start
        .map(|Position { x, y }| Position { x: x * 2, y: y * 2 })
        .unwrap();

    for pos in [
        start.above(),
        start.below(),
        start.left(),
        start.right(),
        start.above().left(),
        start.above().right(),
        start.below().left(),
        start.below().right(),
    ] {
        if let Some(area) = grid.escape(pos) {
            return area;
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
                "{INPUT_PATH}::practice_01"
            )))),
            4
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_02"
            )))),
            8
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            6890
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_03"
            )))),
            4
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_04"
            )))),
            4
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_05"
            )))),
            8
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_06"
            )))),
            10
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            453
        );
    }
}
