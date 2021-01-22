use std::fmt::Display;

use eyre::{Result, WrapErr, eyre};
use ndarray::{Array1, Array2, arr1, arr2, array};

type Image = Array2::<char>;
//type Coord = Array1::<i32>;


struct Coord(i32,i32);
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}


enum Rotation {
    R0,
    R90,
    R180,
    R270
}
impl Rotation {
    fn ident(c:&Coord, _dim:i32) -> Coord {
        Coord(c.0, c.1)
    }
    fn cw90(c:&Coord, dim:i32) -> Coord {
        Coord(dim-1-c.1, c.0)
    }
    // TODO: direct matrices instead of nested calls
    fn cw180(c:&Coord, dim:i32) -> Coord {
        Self::cw90(&Self::cw90(&c,dim),dim)
    }
    fn cw270(c:&Coord, dim:i32) -> Coord {
        Self::cw90(&Self::cw90(&Self::cw90(&c,dim),dim),dim)
    }
    
    fn get_function(&self) -> fn(&Coord, i32) -> Coord {
        match self {
            Rotation::R0 => Self::ident,
            Rotation::R90 => Self::cw90,
            Rotation::R180 => Self::cw180,
            Rotation::R270 => Self::cw270
        }
    }

    // get next rotation, but stop after 270 degrees 
    fn next_stop(&self) -> Option<Rotation> {
        match self {
            Rotation::R0 => Some(Rotation::R90),
            Rotation::R90 => Some(Rotation::R180),
            Rotation::R180 => Some(Rotation::R270),
            Rotation::R270 => None
        }
    }
}

struct Tile {
    image: Array2<char>,
    dim: i32
}
impl Tile {
    fn get(&self, c: &Coord) -> char {
        *self.image.get((c.0 as usize, c.1 as usize)).expect("dimension exceeded")
    }
}

struct RotatedTile<'a> {
    tile: &'a Tile,
    rotation: Rotation
}

impl<'a> RotatedTile<'a> {
    fn get(&self, c: &Coord) -> char {
        let c = self.rotation.get_function()(&c, self.tile.dim);
        self.tile.get(&c)
    }
}

impl<'a> Display for RotatedTile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.tile.dim {
            writeln!(f)?;
            for c in 0..self.tile.dim {
                let ch = self.get(&Coord(r,c));
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

    let coord= arr1(&[1i32,2]);
    let rotate1 = arr2(&[[0,-1],[1,0]]);

    println!("coord\n{}", coord);
    println!("rotate\n{}", rotate1);

    let res = &rotate1.dot(&coord);
    println!("rotated\n{}", res);

    
    let dim = 3;
    let c = Coord(1,2);

    let f0 = Rotation::R0.get_function();
    let f1 = Rotation::R90.get_function();
    let f2 = Rotation::R180.get_function();
    let f3 = Rotation::R270.get_function();

    let c0 = f0(&c,dim);
    let c1 = f1(&c,dim);
    let c2 = f2(&c,dim);
    let c3 = f3(&c,dim);

    println!("{}",c0);
    println!("{}",c1);
    println!("{}",c2);
    println!("{}",c3);


    let tile = Tile {
        dim: 3,
        image: arr2(&[['1','2','3'],['4','5','6'],['7','8','9']])
    };
    let t0 = RotatedTile { tile: &tile, rotation: Rotation::R0 };
    let t1 = RotatedTile { tile: &tile, rotation: Rotation::R90 };
    let t2 = RotatedTile { tile: &tile, rotation: Rotation::R180 };
    let t3 = RotatedTile { tile: &tile, rotation: Rotation::R270 };
    println!("{}", &t0);
    println!("{}", &t1);
    println!("{}", &t2);
    println!("{}", &t3);



    Ok(())
}
