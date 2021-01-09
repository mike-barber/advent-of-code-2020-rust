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

    // part 1
    for i in 0..list.len() - 1 {
        for j in i + 1..list.len() {
            let vi = list[i];
            let vj = list[j];

            if vi + vj == 2020 {
                println!(
                    "part1 -> vi = {}, vj = {}, vi + vj = {}, vi * vj = {}",
                    vi,
                    vj,
                    vi + vj,
                    vi * vj
                );
            }
        }
    }

    // part 2
    for i in 0..list.len() - 2 {
        for j in i + 1..list.len() - 1 {
            for k in j + 1..list.len() {
                let vi = list[i];
                let vj = list[j];
                let vk = list[k];

                if vi + vj + vk == 2020 {
                    println!(
                        "part2 -> vi = {}, vj = {}, vk = {}, sum = {}, product = {}",
                        vi,
                        vj,
                        vk,
                        vi + vj + vk,
                        vi * vj * vk
                    );
                }
            }
        }
    }

    Ok(())
}
