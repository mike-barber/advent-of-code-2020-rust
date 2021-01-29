use std::ops::{Add, AddAssign, Index};
use strum_macros::EnumIter;

pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Dir {
    NW,
    NE,
    E,
    SE,
    SW,
    W,
}

impl Dir {
    pub fn coord(&self) -> Coord {
        use Dir::*;
        match self {
            NW => Coord([0, 1]),
            SW => Coord([-1, -1]),
            W => Coord([-1, 0]),
            NE => Coord([1, 1]),
            E => Coord([1, 0]),
            SE => Coord([0, -1]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord([i32; 2]);

impl Coord {
    pub fn fold<F>(&self, other: Coord, op: F) -> Self
    where
        F: Fn(i32, i32) -> i32,
    {
        Coord([op(self.0[0], other.0[0]), op(self.0[1], other.0[1])])
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Coord([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl AddAssign<&Coord> for Coord {
    fn add_assign(&mut self, rhs: &Coord) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
    }
}

impl Index<usize> for Coord {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<[i32; 2]> for Coord {
    fn from(vals: [i32; 2]) -> Self {
        Coord(vals)
    }
}

impl From<&[i32; 2]> for Coord {
    fn from(vals: &[i32; 2]) -> Self {
        Coord(*vals)
    }
}

pub fn fold_directions(directions: &[Dir]) -> Coord {
    directions
        .iter()
        .fold(Coord::default(), |acc, d| acc + d.coord())
}

#[cfg(test)]
mod tests {

    use crate::*;
    use parser::directions;

    #[test]
    fn directions_return_to_origin() {
        let d = directions("nwwswee").unwrap().1;
        let c = fold_directions(&d);
        assert_eq!(Coord::default(), c);
    }

    #[test]
    fn directions_adjacent() {
        let d = directions("esew").unwrap().1;
        let c = fold_directions(&d);
        assert_eq!(Coord::from([0, -1]), c);
    }
}
