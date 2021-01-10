use core::panic;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    todo,
};

struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);

    let seats: Result<Vec<_>,_> = buffered
        .lines()
        .map(|l_maybe| match l_maybe {
            Ok(l) => Ok(parse_seat(&l)),
            Err(err) => Err(err),
        })
        .collect();

        

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
                r2 = mid - ofs;
            } else if v == 'B' {
                // upper half
                r1 = mid + ofs;
            } else {
                panic!("unexpected instruction");
            }
        }
        r1
    };

    let col = {
        let mut c1 = 0;
        let mut c2 = 7;
        for v in l.chars().skip(7).take(3) {
            let mid = (c1 + c2) / 2;
            let ofs = (c1 + c2) % 2;
            if v == 'F' {
                // lower half
                c2 = mid - ofs;
            } else if v == 'B' {
                // upper half
                c1 = mid + ofs;
            } else {
                panic!("unexpected instruction");
            }
        }
        c1
    };

    Seat {
        row,
        col,
        id: row * 8 + col,
    }
}
