use std::{any, collections::{btree_map::Range, HashSet}, ops::{Add, RangeInclusive}, str::FromStr, usize};

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i32, i32, i32);

impl Add<&Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: &Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}


#[derive(Debug, Default,Clone)]
struct Grid(HashSet<Coord>);

impl Grid {
    fn get(&self, coord: &Coord) -> bool {
        self.0.contains(coord)
    }
    fn set(&mut self, coord: &Coord, value: bool) {
        if value {
            self.0.insert(*coord);
        } else {
            self.0.remove(coord);
        }
    }
    fn range<F>(&self, f: F) -> RangeInclusive<i32>
    where
        F: Fn(&Coord) -> i32,
    {
        let min = self.0.iter().map(&f).min().unwrap();
        let max = self.0.iter().map(&f).max().unwrap();
        min..=max
    }
    fn range_x(&self) -> RangeInclusive<i32> {
        self.range(|c| c.0)
    }
    fn range_y(&self) -> RangeInclusive<i32> {
        self.range(|c| c.1)
    }
    fn range_z(&self) -> RangeInclusive<i32> {
        self.range(|c| c.2)
    }

    fn count_active_neighbours(&self, coord: &Coord) -> i32 {
        let mut count = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    // ignore self
                    if x == 0 && y == 0 && z == 0 {
                        continue; 
                    }
                    let c = Coord(x,y,z) + coord;
                    if self.get(&c) {
                        count += 1;
                    }                    
                }
            }
        }
        count
    }

    fn count_active_total(&self) -> i32 {
        self.0.len() as i32
    }

    fn step(&self) -> Self {
        let mut new_grid = Grid::default();
        for x in self.range_x().expand_1() {
            for y in self.range_y().expand_1() {
                for z in self.range_z().expand_1() {
                    let c = Coord(x,y,z);
                    let neighbours = self.count_active_neighbours(&c);
                    let new_state = match (self.get(&c), neighbours) {
                        (true, x) if x==2 || x==3 => true,
                        (false, x) if x==3 => true,
                        _ => false
                    };
                    new_grid.set(&c, new_state);
                }
            }
        }
        new_grid
    }
}

impl FromStr for Grid {
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
                    let coord = Coord(x as i32 - x_offset as i32, y as i32 - y_offset as i32, 0);
                    grid.set(&coord, true);
                }
            }
        }
        Ok(grid)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"")?;
        for z in self.range_z() {
            writeln!(f,"--- z={}",z)?;
            for y in self.range_y() {
                for x in self.range_x() {
                    write!(f, "{}", match self.get(&Coord(x,y,z)) {
                        true => '#',
                        false => '.'
                    })?;
                }
                writeln!(f,"")?;
            }
        }
        writeln!(f,"---")?;
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
