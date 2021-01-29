use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use day24::{fold_directions, parser::directions, Coord, Dir};
use eyre::{eyre, Result};
use strum::IntoEnumIterator;

fn parse_file(filename: &str) -> Result<Vec<Vec<Dir>>> {
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);

    let mut all_directions = Vec::new();
    for l in buffered.lines() {
        let l = l?;
        let dirs = directions(&l).map_err(|e| eyre!("parsing error {}", e))?;
        all_directions.push(dirs.1);
    }
    Ok(all_directions)
}

type FlipMap = HashMap<Coord, bool>;

fn flip_tiles(all_directions: &[Vec<Dir>]) -> FlipMap {
    let mut map = HashMap::new();
    for dirs in all_directions {
        let coord = fold_directions(dirs);
        let item = map.entry(coord).or_insert(false);
        *item ^= true;
    }
    map
}

fn count_black(flip_map: &FlipMap) -> usize {
    flip_map.values().filter(|&v| *v).count()
}

fn part1_results(filename: &str) -> Result<()> {
    let test_directions = parse_file(filename)?;
    // for dirs in &test_directions {
    //     let coord = fold_directions(&dirs);
    //     println!("{:?} from {:?}", coord, dirs);
    // }

    let flip_map = flip_tiles(&test_directions);
    println!("flips: {:?}", flip_map);
    println!("black tiles: {}", count_black(&flip_map));

    Ok(())
}

trait Evolution {
    fn problem_extents(&self) -> (Coord, Coord);
    fn count_black_adjacent(&self, pos: Coord) -> usize;
    fn calculate_flip_list(&self) -> Vec<(Coord, bool)>;
    fn evolve(&mut self);
}

impl Evolution for FlipMap {
    // calculate extents of the problem -- valid grid area for evolution
    fn problem_extents(&self) -> (Coord, Coord) {
        // find bounding box around existing tiles
        let extents = self
            .keys()
            .fold((Coord::default(), Coord::default()), |(low, high), v| {
                let new_low = low.fold(*v, i32::min);
                let new_high = high.fold(*v, i32::max);
                (new_low, new_high)
            });
        // expand by 1 so we can grow and flip new tiles
        (
            extents.0 + Coord::from([-1, -1]),
            extents.1 + Coord::from([1, 1]),
        )
    }

    fn count_black_adjacent(&self, pos: Coord) -> usize {
        Dir::iter()
            .filter(|d| {
                let c = pos + d.coord();
                match self.get(&c) {
                    Some(v) => *v,
                    None => false,
                }
            })
            .count()
    }

    // Run through entire extent of the map, every coordinate, not just
    // existing tiles. The map is sparse, so we need to consider the
    // tiles adjacent to ones that are populated too. Could look at tiles
    // strictly adjacent instead of the whole grid, but this is easy. Rust
    // is fast.
    fn calculate_flip_list(&self) -> Vec<(Coord, bool)> {
        let mut flip_list = Vec::new();

        let (low, high) = self.problem_extents();
        for x in low[0]..=high[0] {
            for y in low[1]..=high[1] {
                let c = Coord::from([x, y]);
                let v = self.get(&c).unwrap_or(&false);
                let adjacent = self.count_black_adjacent(c);
                match (v, adjacent) {
                    // black -> flip to white
                    (true, adj) if adj == 0 => flip_list.push((c, false)),
                    (true, adj) if adj > 2 => flip_list.push((c, false)),
                    // white -> flip to black
                    (false, adj) if adj == 2 => flip_list.push((c, true)),
                    _ => {} // no action
                }
            }
        }
        flip_list
    }

    fn evolve(&mut self) {
        // work out flip list then apply it afterwards for simultaneous transition
        let flip_list = self.calculate_flip_list();
        for (c, v) in flip_list {
            self.insert(c, v);
        }
    }
}

fn part2_results(filename: &str) -> Result<()> {
    let test_directions = parse_file(filename)?;
    let mut flip_map = flip_tiles(&test_directions);
    println!("black tiles: {}", count_black(&flip_map));

    for day in 1..=100 {
        flip_map.evolve();
        println!("day {} black tiles: {}", day, count_black(&flip_map));
    }

    Ok(())
}

fn main() -> Result<()> {
    // part 1
    part1_results("day24/example-input.txt")?;
    println!("\n\nactual...");
    part1_results("day24/input.txt")?;

    println!("\n\npart 2\n");
    part2_results("day24/example-input.txt")?;
    println!("\n\nactual...");
    part2_results("day24/input.txt")?;

    Ok(())
}
