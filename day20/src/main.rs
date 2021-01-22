use eyre::{Result, WrapErr, eyre};
use ndarray::{Array1, Array2, arr1, arr2, array};

type Image = Array2::<char>;
type Coord = Array1::<i32>;


/// notes
/// consider
///  - https://docs.rs/eyre/0.6.5/eyre/ for fun, instead of related `anyhow`
///  - https://docs.rs/ndarray/0.14.0/ndarray/type.Array.html (used before)
///  - https://crates.io/crates/transpose (something to try, maybe a lighter option)
fn main() -> Result<()> {
    println!("Hello, world!");

    let coord: Coord = arr1(&[1i32,2]);
    let rotate1 = arr2(&[[0,-1],[1,0]]);

    println!("coord\n{}", coord);
    println!("rotate\n{}", rotate1);

    let res = &rotate1.dot(&coord);
    println!("rotated\n{}", res);

    println!("directly rotate...");

    let mat = arr2(&[[1i32,2,3],[4,5,6],[7,8,9]]);
    println!("mat\n{}",mat);

    let mat = mat * &rotate1;
    println!("mat\n{}",mat);

    let mat = mat * &rotate1;
    println!("mat\n{}",mat);

    let mat = mat * &rotate1;
    println!("mat\n{}",mat);

    



    Ok(())
}
