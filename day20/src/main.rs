use std::{collections::HashMap, fmt::Display, hash::Hash, ops::{Add, AddAssign}, println, writeln};

use eyre::{eyre, Result, WrapErr};
use ndarray::{arr1, arr2, Array, Array2, ShapeBuilder};

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    // fn next_stop(&self) -> Option<Rotation> {
    //     match self {
    //         Rotation::R0 => Some(Rotation::R90),
    //         Rotation::R90 => Some(Rotation::R180),
    //         Rotation::R180 => Some(Rotation::R270),
    //         Rotation::R270 => None,
    //     }
    // }
    fn all() -> &'static [Rotation] {
        use Rotation::*;
        &[R0, R90, R180, R270]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Id(i32);
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id{}", self.0)
    }
}


struct Tile {
    image: Array2<char>,
    dim: i32,
    id: Id,
}
impl Tile {
    fn get(&self, c: &Coord) -> Option<char> {
        self.image.get((c.0 as usize, c.1 as usize)).map(|ch| *ch)
    }

    fn rotated(&self, rotation: Rotation) -> RotatedTile {
        RotatedTile {
            tile: self,
            rotation,
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rotated(Rotation::R0).fmt(f)
    }
}
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Tile-----------------------")?;
        Display::fmt(&self, f)?;
        writeln!(f, "---------------------------")?;
        Ok(())
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}
impl Edge {
    fn adjacent(&self) -> Self {
        use Edge::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
    fn all() -> &'static [Edge] {
        &[Edge::Top, Edge::Bottom, Edge::Left, Edge::Right]
    }
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

    fn row_iter(&self, row: i32) -> TileIterator {
        TileIterator {
            tile: self,
            current: Coord(row, 0),
            inc: Coord(0, 1),
        }
    }
    fn col_iter(&self, col: i32) -> TileIterator {
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
            Edge::Right => self.col_iter(far_index),
        }
    }
}

impl<'a> Display for RotatedTile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.tile.dim {
            for c in 0..self.tile.dim {
                let ch = self.get(&Coord(r, c)).ok_or_else(|| std::fmt::Error)?;
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
// debug == display; too verbose otherwise
impl<'a> std::fmt::Debug for RotatedTile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "RotatedTile----------------")?;
        writeln!(f, "{:?}", self.rotation)?;
        Display::fmt(&self, f)?;
        writeln!(f, "---------------------------")?;
        Ok(())
    }
}

#[derive(Debug)]
struct TileMap<'a> {
    tiles: HashMap<Id, TileRelation<'a>>,
    all_ids: Vec<Id>,
}

#[derive(Debug)]
struct TileRelation<'a> {
    tile: &'a Tile,
    rotated: Option<RotatedTile<'a>>,
    neighbours: HashMap<Edge, Id>,
}

#[derive(Debug)]
struct FoundRelation {
    ref_edge: Edge,
    other_id: Id,
    other_new_rotation: Option<Rotation>,
}

impl<'a> TileMap<'a> {
    fn create(tiles: &'a [Tile]) -> Self {
        let mut tile_map = HashMap::new();
        for (i, t) in tiles.iter().enumerate() {
            let relation = TileRelation {
                tile: t,
                rotated: None,
                neighbours: HashMap::new(),
            };
            tile_map.insert(t.id, relation);
        }
        let all_ids = tile_map.keys().copied().collect();
        TileMap {
            tiles: tile_map,
            all_ids,
        }
    }

    // solve once, and return true if something was done
    fn solve_one(&self, all_ids: &[Id], reference_id: Id) -> Option<FoundRelation> {
        let ref_relation = self.tiles.get(&reference_id).unwrap();
        let ref_rotated = ref_relation.rotated.as_ref().unwrap();
        for this_edge in Edge::all() {
            // already have a relationship; skip
            if ref_relation.neighbours.contains_key(this_edge) {
                continue;
            }
            // find a new relationship
            for other_id in all_ids {
                if *other_id == reference_id {
                    continue; // skip self
                }
                let other_rel = self.tiles.get(other_id).unwrap();
                if let Some(other_rotated) = &other_rel.rotated {
                    // already rotated -- just check and associate if found
                    let adjacent_edge = this_edge.adjacent();
                    if !other_rel.neighbours.contains_key(&adjacent_edge)
                        && itertools::equal(
                            ref_rotated.edge_iter(*this_edge),
                            other_rotated.edge_iter(adjacent_edge),
                        )
                    {
                        return Some(FoundRelation {
                            other_id: other_rel.tile.id,
                            ref_edge: *this_edge,
                            other_new_rotation: None,
                        });
                    }
                } else {
                    // not rotated -- find a rotation
                    let adjacent_edge = this_edge.adjacent();
                    for rotation in Rotation::all() {
                        let other_rotated = other_rel.tile.rotated(*rotation);
                        if itertools::equal(
                            ref_rotated.edge_iter(*this_edge),
                            other_rotated.edge_iter(adjacent_edge),
                        ) {
                            return Some(FoundRelation {
                                other_id: other_rel.tile.id,
                                ref_edge: *this_edge,
                                other_new_rotation: Some(*rotation),
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn solve(&mut self) {
        let all_ids = self.all_ids.clone();
        
        // lock rotation of first tile (need to start somewhere)
        {
            let t = self.tiles.get_mut(&all_ids[0]).unwrap();
            t.rotated = Some(t.tile.rotated(Rotation::R0));
        }

        loop {
            let mut changed = false;
            for this_id in all_ids.iter() {
                // skip this if the rotation is not set yet
                if self.tiles.get(this_id).unwrap().rotated.is_none() {
                    println!("Skipping {} as rotation not set", this_id);
                    continue;
                }
                // otherwise find a new relationship and record it
                if let Some(new_relation) = self.solve_one(&all_ids, *this_id) {
                    println!("new relation: {} => {:?}", this_id, new_relation);
    
                    // this -> other
                    let this_rel = self.tiles.get_mut(&this_id).unwrap();
                    this_rel.neighbours.insert(new_relation.ref_edge, new_relation.other_id);
                    
                    // other -> this (and set rotation if found)
                    let other_rel = self.tiles.get_mut(&new_relation.other_id).unwrap();
                    other_rel.neighbours.insert(new_relation.ref_edge.adjacent(), *this_id);
                    if let Some(new_rotation) = new_relation.other_new_rotation {
                        other_rel.rotated = Some(other_rel.tile.rotated(new_rotation))
                    }

                    // record change occurred 
                    changed = true;
                }
            }
            // converged
            if !changed {
                break;
            }
        }

        // found 
        println!("{:?}", &self);
    }
}

// quick n dirty
fn parse_tiles(path: &str, dim: i32) -> Result<Vec<Tile>> {
    let mut tiles = Vec::new();
    let contents = std::fs::read_to_string(path)?;
    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        let mut l = line;
        if l.is_empty() {
            l = lines.next().unwrap();
        }

        let id: i32 = l.replace("Tile ", "").replace(":", "").parse()?;
        let mut image: Array2<char> = Array::from_elem((dim as usize, dim as usize), 'X');
        for r in 0..dim {
            let row_str = lines.next().ok_or_else(|| eyre!("expected tile row"))?;
            for (c, ch) in row_str.chars().enumerate() {
                let elem = image.get_mut((r as usize, c)).ok_or(eyre!("dim error"))?;
                *elem = ch;
            }
        }

        let tile = Tile {
            image,
            id: Id(id),
            dim,
        };
        tiles.push(tile);
    }

    Ok(tiles)
}

/// notes
/// consider
///  - https://docs.rs/eyre/0.6.5/eyre/ for fun, instead of related `anyhow`
///  - https://docs.rs/ndarray/0.14.0/ndarray/type.Array.html (used before)
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
        id: Id(1234),
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

    let tiles = parse_tiles("day20/example-input.txt", 10)?;
    for t in &tiles {
        println!("id {:?}\n{}", t.id, &t);
    }

    let mut tile_map = TileMap::create(&tiles);
    tile_map.solve();

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
            id: Id(1234),
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
