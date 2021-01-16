use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("day13/input.txt")?);
    let mut lines = buffered.lines();

    let earliest: i64 = lines.next().ok_or("missing earliest")??.parse()?;
    let ids: Vec<Option<i64>> = lines
        .next()
        .ok_or("times")??
        .split(',')
        .map(|s| match s {
            "x" => None,
            _ => Some(s.parse().unwrap()),
        })
        .collect();

    println!("Earliest: {}, IDs: {:?}", &earliest, &ids);

    let mut next: Vec<_> = ids
        .iter()
        .filter_map(|ido| {
            ido.map(|id| {
                let next = (earliest / id + 1) * id;
                (id, next)
            })
        })
        .collect();
    next.sort_by_key(|(_, t)| *t);
    println!("Next busses: {:?}", next);

    let (next_id, next_time) = next.first().ok_or("no bus")?;
    let wait = next_time - earliest;
    println!("Next bus is {}, time is {}, id*wait = {}", next_id, next_time, next_id * wait);

    Ok(())
}
