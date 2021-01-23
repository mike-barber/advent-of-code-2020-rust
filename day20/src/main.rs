use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use eyre::{eyre, Result, WrapErr};
use ndarray::{arr1, arr2, array, Array1, Array2};

//type Image = Array2::<char>;
//type Coord = Array1::<i32>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coord(i32, i32);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

enum Rotation {
    R0,
    R90,
    R180,
    R270,
}
impl Rotation {
    fn ident(c: &Coord, _dim: i32) -> Coord {
        Coord(c.0, c.1)
    }
    fn cw90(c: &Coord, dim: i32) -> Coord {
        Coord(dim - 1 - c.1, c.0)
    }
    // TODO: direct matrices instead of nested calls
    fn cw180(c: &Coord, dim: i32) -> Coord {
        Self::cw90(&Self::cw90(&c, dim), dim)
    }
    fn cw270(c: &Coord, dim: i32) -> Coord {
        Self::cw90(&Self::cw90(&Self::cw90(&c, dim), dim), dim)
    }

    fn get_function(&self) -> fn(&Coord, i32) -> Coord {
        match self {
            Rotation::R0 => Self::ident,
            Rotation::R90 => Self::cw90,
            Rotation::R180 => Self::cw180,
            Rotation::R270 => Self::cw270,
        }
    }

    // get next rotation, but stop after 270 degrees
    fn next_stop(&self) -> Option<Rotation> {
        match self {
            Rotation::R0 => Some(Rotation::R90),
            Rotation::R90 => Some(Rotation::R180),
            Rotation::R180 => Some(Rotation::R270),
            Rotation::R270 => None,
        }
    }
}

struct Tile {
    image: Array2<char>,
    dim: i32,
}

impl Tile {
    fn get(&self, c: &Coord) -> Option<char> {
        self.image.get((c.0 as usize, c.1 as usize)).map(|ch| *ch)
    }

    fn rotated(&self, rotation:Rotation) -> RotatedTile {
        RotatedTile {
            tile: self, 
            rotation
        }
    }
}

struct TileIterator<'a> {
    tile: &'a RotatedTile<'a>,
    current: Coord,
    inc: Coord,
}
impl<'a> Iterator for TileIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.tile.get(&self.current);
        self.current += self.inc;
        res
    }
}

#[derive(Debug, Clone, Copy)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

struct RotatedTile<'a> {
    tile: &'a Tile,
    rotation: Rotation,
}

impl<'a> RotatedTile<'a> {
    fn get(&self, c: &Coord) -> Option<char> {
        let c = self.rotation.get_function()(&c, self.tile.dim);
        self.tile.get(&c)
    }

    fn row_iter(&self, row:i32) -> TileIterator {
        TileIterator {
            tile: self,
            current: Coord(row, 0),
            inc: Coord(0, 1),
        }
    }
    fn col_iter(&self, col:i32) -> TileIterator {
        TileIterator {
            tile: self,
            current: Coord(0, col),
            inc: Coord(1, 0),
        }
    }

    fn edge_iter(&self, edge: Edge) -> TileIterator {
        let far_index = self.tile.dim - 1;
        match edge {
            Edge::Top => self.row_iter(0),
            Edge::Bottom => self.row_iter(far_index),
            Edge::Left => self.col_iter(0), 
            Edge::Right => self.col_iter(far_index)
        }
    }
}

impl<'a> Display for RotatedTile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.tile.dim {
            writeln!(f)?;
            for c in 0..self.tile.dim {
                let ch = self.get(&Coord(r, c)).ok_or_else(|| std::fmt::Error)?;
                write!(f, "{}", ch)?;
            }
        }
        Ok(())
    }
}

/// notes
/// consider
///  - https://docs.rs/eyre/0.6.5/eyre/ for fun, instead of related `anyhow`
///  - https://docs.rs/ndarray/0.14.0/ndarray/type.Array.html (used before)
///  - https://crates.io/crates/transpose (something to try, maybe a lighter option)
fn main() -> Result<()> {
    println!("Hello, world!");

    let coord = arr1(&[1i32, 2]);
    let rotate1 = arr2(&[[0, -1], [1, 0]]);

    println!("coord\n{}", coord);
    println!("rotate\n{}", rotate1);

    let res = &rotate1.dot(&coord);
    println!("rotated\n{}", res);

    let dim = 3;
    let c = Coord(1, 2);

    let f0 = Rotation::R0.get_function();
    let f1 = Rotation::R90.get_function();
    let f2 = Rotation::R180.get_function();
    let f3 = Rotation::R270.get_function();

    let c0 = f0(&c, dim);
    let c1 = f1(&c, dim);
    let c2 = f2(&c, dim);
    let c3 = f3(&c, dim);

    println!("{}", c0);
    println!("{}", c1);
    println!("{}", c2);
    println!("{}", c3);

    let tile = Tile {
        dim: 3,
        image: arr2(&[['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]),
    };
    let t0 = tile.rotated(Rotation::R0);
    let t1 = tile.rotated(Rotation::R90);
    let t2 = tile.rotated(Rotation::R180);
    let t3 = tile.rotated(Rotation::R270);
    
    println!("{}", &t0);
    println!("{}", &t1);
    println!("{}", &t2);
    println!("{}", &t3);




    Ok(())
}

#[cfg(test)] 
mod tests {
    use super::*;

    // all chars:
    //  123
    //  456
    //  789
    fn create_tile() -> Tile {
        Tile {
            dim: 3,
            image: arr2(&[['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]),
        }
    }

    #[test]
    fn rotation_correct() {
        let tile = create_tile();
        let t0 = tile.rotated(Rotation::R0);
        let t1 = tile.rotated(Rotation::R90);
        let t2 = tile.rotated(Rotation::R180);
        let t3 = tile.rotated(Rotation::R270);

        itertools::assert_equal("123".chars(), t0.row_iter(0));
        itertools::assert_equal("741".chars(), t1.row_iter(0));
        itertools::assert_equal("987".chars(), t2.row_iter(0));
        itertools::assert_equal("369".chars(), t3.row_iter(0));
    }

    #[test]
    fn edge_correct() {
        let tile = create_tile();
        let t0 = tile.rotated(Rotation::R0);

        itertools::assert_equal("123".chars(), t0.edge_iter(Edge::Top));
        itertools::assert_equal("789".chars(), t0.edge_iter(Edge::Bottom));
        itertools::assert_equal("147".chars(), t0.edge_iter(Edge::Left));
        itertools::assert_equal("369".chars(), t0.edge_iter(Edge::Right));
    }
}
