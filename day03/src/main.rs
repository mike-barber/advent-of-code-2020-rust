use core::panic;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);

    let map: Vec<Vec<bool>> = buffered.lines().map(|l| parse_line(&l.unwrap())).collect();
    //println!("{:?}", map);

    // part 1
    {
        let mut x = 0_usize;
        let mut y = 0_usize;

        let dx = 3_usize;
        let dy = 1_usize;

        let mut trees = 0;
        loop {
            x += dx;
            y += dy;

            if y >= map.len() {
                break;
            }

            if encountered_tree(&map, x, y) {
                trees += 1;
            }
        }

        println!("Part1 -> trees = {}", trees);
    }

    Ok(())
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|ch| match ch {
            '.' => false,
            '#' => true,
            _ => panic!("unexpected char"),
        })
        .collect()
}

fn encountered_tree(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let row = &map[y];
    let x_mod = x % row.len();
    row[x_mod]
}
