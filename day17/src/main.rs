use anyhow::Result;
use itertools::{cons_tuples, Itertools};
use std::{
    collections::HashSet,
    iter,
    ops::{Add, RangeInclusive},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Coord3(i32, i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Coord4(i32, i32, i32, i32);

impl Add<&Coord3> for Coord3 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Coord3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&Coord4> for Coord4 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Coord4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl From<(i32,i32)> for Coord3 {
    fn from(c: (i32,i32)) -> Self {
        Coord3(c.0, c.1, 0)
    }
}
impl From<(i32,i32)> for Coord4 {
    fn from(c: (i32,i32)) -> Self {
        Coord4(c.0, c.1, 0, 0)
    }
}

trait Coord: Sized + std::hash::Hash + Eq + From<(i32,i32)> {
    fn bounding_box(set: &HashSet<Self>, expand: i32) -> (Self, Self);
    // TODO: Not sure if Box<dyn xx> is required; research needed.
    fn neighbours(&self) -> Box<dyn Iterator<Item = Self>>;
    fn space(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>>;
}

impl Coord for Coord3 {
    fn bounding_box(set: &HashSet<Self>, expand: i32) -> (Self, Self) {
        (
            Coord3(
                set.iter().map(|c| c.0).min().unwrap(),
                set.iter().map(|c| c.1).min().unwrap(),
                set.iter().map(|c| c.2).min().unwrap(),
            ),
            Coord3(
                set.iter().map(|c| c.0).max().unwrap(),
                set.iter().map(|c| c.1).max().unwrap(),
                set.iter().map(|c| c.2).max().unwrap(),
            ),
        )
    }

    fn neighbours(&self) -> Box<dyn Iterator<Item = Self>> {
        let skip = vec![0, 0, 0];
        let ii = (0..3)
            .map(|_| -1..1)
            .multi_cartesian_product()
            .filter(|v| v != &skip)
            .map(|v| Coord3(v[0], v[1], v[2]));
        Box::new(ii)
    }

    fn space(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        let ii = (min.0..=max.0)
            .cartesian_product(min.1..=max.1)
            .cartesian_product(min.2..=max.2)
            .map(|((a, b), c)| Coord3(a, b, c));
        Box::new(ii)
    }
}

impl Coord for Coord4 {
    fn bounding_box(set: &HashSet<Self>, expand: i32) -> (Self, Self) {
        (
            Coord4(
                set.iter().map(|c| c.0).min().unwrap(),
                set.iter().map(|c| c.1).min().unwrap(),
                set.iter().map(|c| c.2).min().unwrap(),
                set.iter().map(|c| c.3).min().unwrap(),
            ),
            Coord4(
                set.iter().map(|c| c.0).max().unwrap(),
                set.iter().map(|c| c.1).max().unwrap(),
                set.iter().map(|c| c.2).max().unwrap(),
                set.iter().map(|c| c.3).max().unwrap(),
            ),
        )
    }

    fn neighbours(&self) -> Box<dyn Iterator<Item = Self>> {
        let skip = vec![0, 0, 0, 0];
        let ii = (0..4)
            .map(|_| -1..1)
            .multi_cartesian_product()
            .filter(|v| v != &skip)
            .map(|v| Coord4(v[0], v[1], v[2], v[3]));
        Box::new(ii)
    }

    fn space(min: Self, max: Self) -> Box<dyn Iterator<Item = Self>> {
        let ii = (min.0..=max.0)
            .cartesian_product(min.1..=max.1)
            .cartesian_product(min.2..=max.2)
            .cartesian_product(min.3..=max.3)
            .map(|(((a, b), c), d)| Coord4(a, b, c, d));
        Box::new(ii)
    }
}

#[derive(Debug, Default, Clone)]
struct Grid<TC>(HashSet<TC>)
where
    TC: Coord;

impl<TC> Grid<TC>
where
    TC: Coord,
{
    fn get(&self, coord: &TC) -> bool {
        self.0.contains(coord)
    }
    fn set(&mut self, coord: &TC, value: bool) {
        if value {
            self.0.insert(*coord);
        } else {
            self.0.remove(coord);
        }
    }

    fn count_active_neighbours(&self, coord: &TC) -> i32 {
        let mut count = 0;
        for c in coord.neighbours() {
            if self.get(&c) {
                count += 1;
            }
        }
        count
    }

    fn count_active_total(&self) -> i32 {
        self.0.len() as i32
    }
}

impl Grid<Coord3> {
    fn step(&self) -> Self {
        let mut new_grid = Grid::default();
        let (min, max) = Coord3::bounding_box(&self.0, 1);
        for c in Coord3::space(min, max) {
            let neighbours = self.count_active_neighbours(&c);
            let new_state = match (self.get(&c), neighbours) {
                (true, x) if x == 2 || x == 3 => true,
                (false, x) if x == 3 => true,
                _ => false,
            };
            new_grid.set(&c, new_state);
        }
        new_grid
    }
}

impl<TC> FromStr for Grid<TC> where TC: Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        let lines: Vec<_> = s.lines().collect();
        let x_offset = lines[0].len() / 2 + 1;
        let y_offset = lines.len() / 2 + 1;
        for y in 0..lines.len() {
            let line: Vec<_> = lines[y].chars().collect();
            for x in 0..line.len() {
                if line[x] == '#' {
                    let coord = (x as i32 - x_offset as i32, y as i32 - y_offset as i32);
                    grid.set(&coord.into(), true);
                }
            }
        }
        Ok(grid)
    }
}

impl std::fmt::Display for Grid<Coord3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        // TODO use: itertools group_by
        for z in self.range_z() {
            writeln!(f, "--- z={}", z)?;
            for y in self.range_y() {
                for x in self.range_x() {
                    write!(
                        f,
                        "{}",
                        match self.get(&Coord(x, y, z)) {
                            true => '#',
                            false => '.',
                        }
                    )?;
                }
                writeln!(f, "")?;
            }
        }
        writeln!(f, "---")?;
        Ok(())
    }
}

trait RangeExpand {
    fn expand_1(&self) -> RangeInclusive<i32>;
}

impl RangeExpand for RangeInclusive<i32> {
    fn expand_1(&self) -> Self {
        let min = self.clone().min().unwrap() - 1;
        let max = self.clone().max().unwrap() + 1;
        min..=max
    }
}

fn main() -> Result<()> {
    let problem_str = std::fs::read_to_string("day17/input.txt")?;
    let init_grid: Grid = problem_str.parse()?;
    println!("Grid: {:?}", init_grid);

    let mut grid = init_grid.clone();
    println!("Grid {}", grid);
    for iteration in 1..=6 {
        grid = grid.step();
        println!("Iteration {} Grid {}", iteration, grid);
    }

    println!("Active after 6 cycles: {}", grid.count_active_total());

    Ok(())
}
