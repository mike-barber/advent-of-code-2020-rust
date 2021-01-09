use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);

    let rx = Regex::new("(\\d+)-(\\d+) ([a-z]{1}): ([a-z]+)")?;

    let mut part1_valid_count = 0_usize;
    let mut part2_valid_count = 0_usize;

    for line in buffered.lines() {
        let l = line?.to_string();
        let cap = rx.captures(&l).ok_or("failed to parse line")?;

        // part 1
        let min: usize = cap[1].parse()?;
        let max: usize = cap[2].parse()?;
        let ch: char = cap[3].parse()?;
        let password = &cap[4];

        let count = password.chars().filter(|c| *c == ch).count();
        if count >= min && count <= max {
            part1_valid_count += 1;
        }

        // part 2 -- indices (1-indexed)
        // character must appear at exactly one of the two indices
        let ix1: usize = cap[1].parse()?;
        let ix2: usize = cap[2].parse()?;
        let ch1 = password
            .chars()
            .nth(ix1 - 1)
            .expect("character not present at index");
        let ch2 = password
            .chars()
            .nth(ix2 - 1)
            .expect("character not present at index");

        if (ch1 == ch) ^ (ch2 == ch) {
            part2_valid_count += 1;
        }
    }
    println!("Part1 -> Valid passwords: {}", part1_valid_count);
    println!("Part2 -> Valid passwords: {}", part2_valid_count);

    Ok(())
}
