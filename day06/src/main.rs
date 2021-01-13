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

    // part 1 -- which questions *anyone* answered yes to in a group
    let counts: Vec<_> = groups
        .iter()
        .map(|g| {
            let found_chars = g.iter().fold(HashSet::new(), |mut set, l| {
                for c in l.chars() {
                    set.insert(c);
                }
                set
            });
            found_chars.len()
        })
        .collect();

    println!(
        "Part1 -> Counts: {:?}, sum: {}",
        counts,
        counts.iter().sum::<usize>()
    );

    // part 2 -- which questions *everyone* in a group answered yes to
    let counts_everyone: Vec<_> = groups
        .iter()
        .map(|g| {
            let sets = g.iter().map(|l| l.chars().collect::<HashSet<char>>());
            let intersection = sets.fold(None, |acc_option, set| match acc_option {
                None => Some(set.clone()),
                Some(acc) => Some(&acc & &set),
            });

            match intersection {
                None => 0,
                Some(ii) => ii.len(),
            }
        })
        .collect();
    println!(
        "Part2 -> Counts: {:?}, sum: {}",
        counts_everyone,
        counts_everyone.iter().sum::<usize>()
    );

    Ok(())
}
