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
        let trees = count_encountered_trees(&map, 3, 1);
        println!("Part1 -> trees = {}", trees);
    }

    // part 2
    {
        let paths = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let counts = paths
            .iter()
            .map(|(dx, dy)| count_encountered_trees(&map, *dx, *dy));
        let product:i64 = counts.map(|v| v as i64).product(); // overflows with i32
        println!("Part2 -> product = {}", product);
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

fn count_encountered_trees(map: &Vec<Vec<bool>>, dx: usize, dy: usize) -> i32 {
    let mut x = 0_usize;
    let mut y = 0_usize;
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
    trees
}
