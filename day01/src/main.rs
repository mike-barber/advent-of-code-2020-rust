use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);
    let list: Vec<i32> = buffered
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    for i in 0..list.len() - 1 {
        for j in i + 1..list.len() {
            let vi = list[i];
            let vj = list[j];

            if vi + vj == 2020 {
                println!(
                    "vi = {}, vj = {}, vi + vj = {}, vi * vj = {}",
                    vi,
                    vj,
                    vi + vj,
                    vi * vj
                );
            }
        }
    }

    Ok(())
}
