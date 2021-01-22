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
    L0,
    L90,
    L180,
    L270
}
impl Rotation {
    fn ident(c:&Coord, _dim:i32) -> Coord {
        Coord(c.0, c.1)
    }
    fn ccw90(c:&Coord, dim:i32) -> Coord {
        Coord(dim-1-c.1, c.0)
    }
    // TODO: direct matrices instead of nested calls
    fn ccw180(c:&Coord, dim:i32) -> Coord {
        Self::ccw90(&Self::ccw90(&c,dim),dim)
    }
    fn ccw270(c:&Coord, dim:i32) -> Coord {
        Self::ccw90(&Self::ccw90(&Self::ccw90(&c,dim),dim),dim)
    }
    
    fn get_function(&self) -> fn(&Coord, i32) -> Coord {
        match self {
            Rotation::L0 => Self::ident,
            Rotation::L90 => Self::ccw90,
            Rotation::L180 => Self::ccw180,
            Rotation::L270 => Self::ccw270
        }
    }

    // get next rotation, but stop after 270 degrees 
    fn next_stop(&self) -> Option<Rotation> {
        match self {
            Rotation::L0 => Some(Rotation::L90),
            Rotation::L90 => Some(Rotation::L180),
            Rotation::L180 => Some(Rotation::L270),
            Rotation::L270 => None
        }
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

    let f0 = Rotation::L0.get_function();
    let f1 = Rotation::L90.get_function();
    let f2 = Rotation::L180.get_function();
    let f3 = Rotation::L270.get_function();

    let c0 = f0(&c,dim);
    let c1 = f1(&c,dim);
    let c2 = f2(&c,dim);
    let c3 = f3(&c,dim);

    println!("{}",c0);
    println!("{}",c1);
    println!("{}",c2);
    println!("{}",c3);

    Ok(())
}
