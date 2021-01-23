use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign},
    println, writeln,
};
use eyre::{eyre, Result};
use ndarray::{arr1, arr2, azip, s, Array, Array2};

// Note -- could simplify this quite a bit. 
// Didn't realise how easy the second part of the problem
// would be, so the zero-allocation rotations stuff is
// a bit of an overkill. Could just create a physically
// rotated new image each time, and use the built-in
// ndarray functions more for slices, etc.
//
// Also could use a hashmap<Edge=>Id> instead and match 
// up all the edges like this first, before doing 
// any rotations. 
//
// Enough time spent on the problem. Works. Moving on... 

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coord(i32, i32);
impl Coord {
    fn rotate(&self, r: &Rotation, dim: i32) -> Self {
        r.apply(&self, dim)
    }
}
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

// rotate and/or flip
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Rotation {
    R0,
    R90,
    R180,
    R270,
    F0,
    F90,
    F180,
    F270,
}
impl Rotation {
    // 90 degree rotation
    fn cw90(c: &Coord, dim: i32) -> Coord {
        Coord(dim - 1 - c.1, c.0)
    }
    // vertical flip (rows)
    fn flip(c: &Coord, dim: i32) -> Coord {
        Coord(dim - 1 - c.0, c.1)
    }

    fn apply(&self, c: &Coord, dim: i32) -> Coord {
        let r = |c| Self::cw90(c, dim);
        let f = |c| Self::flip(c, dim);
        match self {
            // rotate only
            Rotation::R0 => *c,
            Rotation::R90 => r(c),
            Rotation::R180 => r(&r(c)),
            Rotation::R270 => r(&r(&r(c))),
            // flip and rotate
            Rotation::F0 => f(c),
            Rotation::F90 => f(&r(c)),
            Rotation::F180 => f(&r(&r(c))),
            Rotation::F270 => f(&r(&r(&r(c)))),
        }
    }

    fn all() -> &'static [Rotation] {
        use Rotation::*;
        &[R0, R90, R180, R270, F0, F90, F180, F270]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Id(i32);
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id{}", self.0)
    }
}

#[derive(Clone)]
struct Tile {
    image: Array2<char>,
    dim: i32,
    id: Id,
}
impl Tile {
    fn get(&self, c: &Coord) -> Option<char> {
        self.image.get((c.0 as usize, c.1 as usize)).copied()
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
        self.tile.get(&c.rotate(&self.rotation, self.tile.dim))
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

    fn render(&self, range: &std::ops::Range<i32>) -> Tile {
        let mut image = Array2::from_elem((range.len(), range.len()), 'X');
        for (ro, r) in range.clone().enumerate() {
            for (co, c) in range.clone().enumerate() {
                let src = self.get(&Coord(r, c)).unwrap();
                let dst = image.get_mut((ro, co)).unwrap();
                *dst = src;
            }
        }
        Tile {
            dim: range.len() as i32,
            id: self.tile.id,
            image,
        }
    }

    fn render_borderless_tile(&self) -> Tile {
        let range = 1..self.tile.dim - 1;
        self.render(&range)
    }

    fn render_tile(&self) -> Tile {
        let range = 0..self.tile.dim;
        self.render(&range)
    }
}

impl<'a> Display for RotatedTile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.tile.dim {
            for c in 0..self.tile.dim {
                let ch = self.get(&Coord(r, c)).ok_or(std::fmt::Error)?;
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
        for t in tiles.iter() {
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

    fn get_relation(&self, id: &Id) -> Option<&TileRelation> {
        self.tiles.get(id)
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
                    this_rel
                        .neighbours
                        .insert(new_relation.ref_edge, new_relation.other_id);

                    // other -> this (and set rotation if found)
                    let other_rel = self.tiles.get_mut(&new_relation.other_id).unwrap();
                    other_rel
                        .neighbours
                        .insert(new_relation.ref_edge.adjacent(), *this_id);
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
                let elem = image
                    .get_mut((r as usize, c))
                    .ok_or_else(|| eyre!("dim error"))?;
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
    {
        let coord = arr1(&[1i32, 2]);
        let rotate1 = arr2(&[[0, -1], [1, 0]]);

        println!("coord\n{}", coord);
        println!("rotate\n{}", rotate1);

        let res = &rotate1.dot(&coord);
        println!("rotated\n{}", res);

        let dim = 3;

        let tile = Tile {
            id: Id(1234),
            dim,
            image: arr2(&[['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]),
        };
        for r in Rotation::all() {
            println!("Rotate {:?} =>\n{}", r, tile.rotated(*r));
        }
    }

    // ----------------
    // Part 1
    //

    let dim = 10;
    let tiles = parse_tiles("day20/input.txt", dim)?;
    for t in &tiles {
        println!("id {:?}\n{}", t.id, &t);
    }

    let mut tile_map = TileMap::create(&tiles);
    tile_map.solve();

    // check all are oriented
    assert!(tile_map.tiles.values().all(|tr| tr.rotated.is_some()));

    // find the 4 corners
    let corners: Vec<_> = tile_map
        .tiles
        .values()
        .filter(|&tr| tr.neighbours.len() == 2)
        .collect();
    for tr in corners.iter() {
        println!("{} has {:?}", tr.tile.id, tr.neighbours);
    }
    let product: i64 = corners.iter().map(|tr| tr.tile.id.0 as i64).product();
    println!("product: {}", product);

    // ----------------
    // Part 1 - arrange the tiles
    //

    // get top row of tiles, then create a big tile with rendered contents
    let top_row = {
        let mut top_row = Vec::new();
        let top_left = tile_map
            .tiles
            .iter()
            .find(|(_, tr)| {
                !tr.neighbours.contains_key(&Edge::Left) && !tr.neighbours.contains_key(&Edge::Top)
            })
            .unwrap();
        let mut current = top_left.1;
        top_row.push(current);
        while let Some(next) = current.neighbours.get(&Edge::Right) {
            current = tile_map.get_relation(next).unwrap();
            top_row.push(current);
        }
        top_row
    };
    println!("{:?}", top_row);

    // ----------------
    // Part 2 - find the seamonster
    //

    // copy little tiles into one large tile
    let rendered_dim = dim as usize - 2;
    let dest_dim = rendered_dim * top_row.len();
    let mut big_tile = Tile {
        dim: dest_dim as i32,
        id: Id(0),
        image: Array::from_elem((dest_dim, dest_dim), 'X'),
    };
    let mut row = top_row;
    let mut row_number = 0;
    loop {
        // copy into the big tile
        for (col_number, tr) in row.iter().enumerate() {
            let rendered = tr.rotated.as_ref().unwrap().render_borderless_tile();
            let r0 = row_number * rendered_dim;
            let r1 = r0 + rendered_dim;
            let c0 = col_number * rendered_dim;
            let c1 = c0 + rendered_dim;
            let mut slice = big_tile.image.slice_mut(s![r0..r1, c0..c1]);
            slice.assign(&rendered.image);
        }
        // and get next row (until none are left)
        let next_row: Option<Vec<&TileRelation>> = row
            .iter()
            .map(|tr| {
                let x = tr
                    .neighbours
                    .get(&Edge::Bottom)
                    .map(|id| tile_map.get_relation(id).unwrap());
                x
            })
            .collect();
        if let Some(next_row) = next_row {
            row = next_row;
            row_number += 1;
        } else {
            break;
        }
    }
    println!("Big tile\n{}", big_tile.rotated(Rotation::F0));

    // construct our seamonster
    let seamonster = {
        let seamonster_data = [
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ];
        let cols = seamonster_data.first().unwrap().len();
        let rows = seamonster_data.len();
        let char_vec: Vec<_> = seamonster_data.iter().flat_map(|&l| l.chars()).collect();
        Array::from_shape_vec((rows, cols), char_vec)?
    };
    println!("{:?}", seamonster);

    // find the seamonster (kind of like convolution) -- try all rotations until
    // we find some :)
    // array.windows would have been perfect if there was a mutable version, but sadly no.
    for rot in Rotation::all() {
        let search_tile = big_tile.rotated(*rot).render_tile();
        let mut destination_tile = search_tile.clone();
        let mut seamonster_count = 0;
        for r_start in 0..=(dest_dim - seamonster.dim().0) {
            for c_start in 0..=(dest_dim - seamonster.dim().1) {
                let window = s![
                    r_start..(r_start + seamonster.dim().0),
                    c_start..(c_start + seamonster.dim().1)
                ];
                let src = search_tile.image.slice(&window);
                let mut dst = destination_tile.image.slice_mut(&window);
                let mut found_seamonster = true;
                azip!((&m in &seamonster, &s in &src) {
                    // and detect if any part of the seamonster is missing
                    if m=='#' && s!='#' {
                        found_seamonster = false;
                    }
                });
                if found_seamonster {
                    seamonster_count += 1;
                    // set destination seamonster elements to 'O'
                    azip!((&m in &seamonster, d in &mut dst) {
                        if m == '#' {
                            *d = 'O'
                        }
                    })
                }
            }
        }

        // found 'em
        if seamonster_count > 0 {
            let sea_roughness = destination_tile
                .image
                .iter()
                .filter(|&ch| *ch == '#')
                .count();

            println!(
                "Found {} seamonsters; roughness is {}; result is\n{}",
                seamonster_count, sea_roughness, destination_tile
            );
            break;
        }
    }

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
