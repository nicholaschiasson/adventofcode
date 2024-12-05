use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Clone, Copy, Debug)]
struct Bounds(i64, i64);

// Don't care about making sure max is higher than min, but this is bad
impl Bounds {
    fn expand(&mut self, x: i64) {
        if x > self.max() {
            self.1 = x;
        }
        if x < self.min() {
            self.0 = x;
        }
    }

    fn max(&self) -> i64 {
        self.1
    }

    fn min(&self) -> i64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point3(i64, i64, i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point4(i64, i64, i64, i64);

trait Point: Clone + Copy + Eq + Hash + Sized {
    fn new(x: i64, y: i64) -> Self;
    fn x(&self) -> i64;
    fn y(&self) -> i64;
    fn z(&self) -> i64;
    fn w(&self) -> Option<i64>;
    fn _set_x(&mut self, x: i64);
    fn _set_y(&mut self, y: i64);
    fn set_z(&mut self, z: i64);
    fn set_w(&mut self, w: i64);
}

impl Point for Point3 {
    fn new(x: i64, y: i64) -> Self {
        Self(x, y, 0)
    }
    fn x(&self) -> i64 {
        self.0
    }
    fn y(&self) -> i64 {
        self.1
    }
    fn z(&self) -> i64 {
        self.2
    }
    fn w(&self) -> Option<i64> {
        None
    }
    fn _set_x(&mut self, x: i64) {
        self.0 = x;
    }
    fn _set_y(&mut self, y: i64) {
        self.1 = y;
    }
    fn set_z(&mut self, z: i64) {
        self.2 = z;
    }
    fn set_w(&mut self, _: i64) {}
}

impl Point for Point4 {
    fn new(x: i64, y: i64) -> Self {
        Self(x, y, 0, 0)
    }
    fn x(&self) -> i64 {
        self.0
    }
    fn y(&self) -> i64 {
        self.1
    }
    fn z(&self) -> i64 {
        self.2
    }
    fn w(&self) -> Option<i64> {
        Some(self.3)
    }
    fn _set_x(&mut self, x: i64) {
        self.0 = x;
    }
    fn _set_y(&mut self, y: i64) {
        self.1 = y;
    }
    fn set_z(&mut self, z: i64) {
        self.2 = z;
    }
    fn set_w(&mut self, w: i64) {
        self.3 = w;
    }
}

#[derive(Debug)]
struct Space<T: Point> {
    bounds_x: Bounds,
    bounds_y: Bounds,
    bounds_z: Bounds,
    bounds_w: Bounds,
    grid: HashMap<T, bool>,
}

impl<T: Point> Space<T> {
    fn new() -> Self {
        Self {
            bounds_x: Bounds(0, 0),
            bounds_y: Bounds(0, 0),
            bounds_z: Bounds(0, 0),
            bounds_w: Bounds(0, 0),
            grid: HashMap::new(),
        }
    }

    fn insert(&mut self, point: T, active: bool) -> Option<bool> {
        self.bounds_x.expand(point.x());
        self.bounds_y.expand(point.y());
        self.bounds_z.expand(point.z());
        if let Some(w) = point.w() {
            self.bounds_w.expand(w)
        }
        self.grid.insert(point, active)
    }

    fn count_active_neighbours(&self, point: &T) -> u64 {
        let mut n = 0;
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                for z_off in -1..=1 {
                    for w_off in -1..=1 {
                        if x_off != 0
                            || y_off != 0
                            || z_off != 0
                            || (point.w().is_some() && w_off != 0)
                        {
                            let mut p = T::new(point.x() + x_off, point.y() + y_off);
                            p.set_z(point.z() + z_off);
                            if let Some(w) = point.w() {
                                p.set_w(w + w_off);
                            }
                            if let Some(active) = self.grid.get(&p) {
                                if *active {
                                    n += 1;
                                }
                            }
                            if point.w().is_none() {
                                break;
                            }
                        }
                    }
                }
            }
        }
        n
    }

    fn cycle(&mut self) {
        let mut new_bounds_x = Bounds(0, 0);
        let mut new_bounds_y = Bounds(0, 0);
        let mut new_bounds_z = Bounds(0, 0);
        let mut new_bounds_w = Bounds(0, 0);
        let mut new_grid = HashMap::new();
        for x in self.bounds_x.min() - 1..=self.bounds_x.max() + 1 {
            for y in self.bounds_y.min() - 1..=self.bounds_y.max() + 1 {
                for z in self.bounds_z.min() - 1..=self.bounds_z.max() + 1 {
                    for w in self.bounds_w.min() - 1..=self.bounds_w.max() + 1 {
                        let mut p = T::new(x, y);
                        p.set_z(z);
                        p.set_w(w);
                        let active_neighbours = self.count_active_neighbours(&p);
                        if let Some(active) = self.grid.get(&p) {
                            if *active {
                                if active_neighbours == 2 || active_neighbours == 3 {
                                    new_grid.insert(p, true);
                                } else {
                                    new_grid.insert(p, false);
                                }
                            } else if active_neighbours == 3 {
                                new_grid.insert(p, true);
                            } else {
                                new_grid.insert(p, false);
                            }
                        } else if active_neighbours == 3 {
                            new_grid.insert(p, true);
                        } else {
                            new_grid.insert(p, false);
                        }
                        new_bounds_x.expand(x);
                        new_bounds_y.expand(y);
                        new_bounds_z.expand(z);
                        new_bounds_w.expand(w);
                        if p.w().is_none() {
                            break;
                        }
                    }
                }
            }
        }
        self.bounds_x = new_bounds_x;
        self.bounds_y = new_bounds_y;
        self.bounds_z = new_bounds_z;
        self.bounds_w = new_bounds_w;
        self.grid = new_grid;
    }
}

impl<T: Point> Display for Space<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.bounds_w.min()..=self.bounds_w.max() {
            for z in self.bounds_z.min()..=self.bounds_z.max() {
                if T::new(0, 0).w().is_none() {
                    writeln!(f, "z={}", z)?;
                } else {
                    writeln!(f, "z={}, w={}", z, w)?;
                }
                for y in self.bounds_y.min()..=self.bounds_y.max() {
                    for x in self.bounds_x.min()..=self.bounds_x.max() {
                        let mut p = T::new(x, y);
                        p.set_z(z);
                        p.set_w(w);
                        write!(
                            f,
                            "{}",
                            if let Some(active) = self.grid.get(&p) {
                                if *active {
                                    "#"
                                } else {
                                    "."
                                }
                            } else {
                                "."
                            }
                        )?;
                    }
                    writeln!(f)?;
                }
                writeln!(f)?;
            }
            if T::new(0, 0).w().is_none() {
                break;
            }
        }
        Ok(())
    }
}

fn parse_input<T: Point>(input: &str) -> Space<T> {
    let mut space = Space::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            space.insert(T::new(x as i64, y as i64), c == '#');
        })
    });
    space
}

pub fn part_01(input: &str) -> u64 {
    let mut space = parse_input::<Point3>(input);
    for _ in 0..6 {
        space.cycle();
    }
    space
        .grid
        .values()
        .fold(0, |sum, active| if *active { sum + 1 } else { sum })
}

pub fn part_02(input: &str) -> u64 {
    let mut space = parse_input::<Point4>(input);
    for _ in 0..6 {
        space.cycle();
    }
    space
        .grid
        .values()
        .fold(0, |sum, active| if *active { sum + 1 } else { sum })
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
            112
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            269
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            848
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            1380
        );
    }
}
