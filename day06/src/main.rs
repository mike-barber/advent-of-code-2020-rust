use core::panic;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);

    let mut groups: Vec<Vec<String>> = Vec::new();
    groups.push(Vec::new());
    for l in buffered.lines() {
        let line = l?;
        if line.is_empty() {
            groups.push(Vec::new())
        } else {
            groups.last_mut().unwrap().push(line);
        }
    }

    let counts: Vec<_> = groups.iter().map(|g| {
        let found_chars = g.iter().fold(HashSet::new(), |mut set, l| {
            for c in l.chars() {
                set.insert(c);
            }
            set
        });
        found_chars.len()
    }).collect();

    println!("Counts: {:?}", counts);
    println!("Sum of counts: {}", counts.iter().sum::<usize>());

    Ok(())
}
