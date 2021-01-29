use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use day24::{Coord, Dir, fold_directions, parser::directions};
use eyre::{eyre, Result};

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

fn flip_tiles(all_directions: &Vec<Vec<Dir>>) -> FlipMap {
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

// impl FlipMap {

// }


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


fn main() -> Result<()> {
    
    part1_results("day24/example-input.txt")?;
    println!("actual...");
    part1_results("day24/input.txt")?;

    
    Ok(())
}
