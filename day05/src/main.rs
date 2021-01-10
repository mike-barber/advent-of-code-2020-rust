use core::panic;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);

    let seats_maybe: Result<Vec<Seat>, _> = buffered
        .lines()
        .map(|l_maybe| match l_maybe {
            Ok(l) => Ok(parse_seat(&l)),
            Err(err) => Err(err),
        })
        .collect();

    let seats = seats_maybe?;
    for s in &seats {
        println!("{:?}", s);
    }

    // part 1 -- max id
    let max = seats.iter().map(|s| s.id).max().ok_or("no max")?;
    println!("max id {}", max);

    // part 2 -- find missing seat (not first or last row)
    let occupied_ids: HashSet<i32> = seats.iter().map(|s| s.id).collect();
    for r in 1..=126 {
        for c in 0..=7 {
            let test_id = seat_id(r, c);
            if !occupied_ids.contains(&test_id)
                && occupied_ids.contains(&(test_id - 1))
                && occupied_ids.contains(&(test_id + 1))
            {
                println!("Found empty seat: {} {} => {}", r, c, test_id);
            }
        }
    }

    Ok(())
}

fn parse_seat(l: &str) -> Seat {
    let row = {
        let mut r1 = 0;
        let mut r2 = 127;
        for v in l.chars().take(7) {
            let mid = (r1 + r2) / 2;
            let ofs = (r1 + r2) % 2;
            if v == 'F' {
                // lower half
                r2 = mid;
            } else if v == 'B' {
                // upper half
                r1 = mid + ofs;
            } else {
                panic!("row unexpected instruction {}", v);
            }
            println!("instruction {} -> {} {}", v, r1, r2);
        }
        r1
    };

    let col = {
        let mut c1 = 0;
        let mut c2 = 7;
        for v in l.chars().skip(7).take(3) {
            let mid = (c1 + c2) / 2;
            let ofs = (c1 + c2) % 2;
            if v == 'L' {
                // lower half
                c2 = mid;
            } else if v == 'R' {
                // upper half
                c1 = mid + ofs;
            } else {
                panic!("col unexpected instruction {}", v);
            }
        }
        c1
    };

    Seat {
        row,
        col,
        id: seat_id(row, col),
    }
}

fn seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}
