use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Evaluation {
    Preamble,
    Valid(i64),
    Invalid(i64),
}

fn number_is_valid(value: i64, buffer: &VecDeque<i64>) -> bool {
    for i in buffer.iter() {
        for j in buffer.iter() {
            if i != j && i + j == value {
                return true;
            }
        }
    }
    false
}

fn find_contiguous_slice(target: i64, numbers: &[i64]) -> Option<&[i64]> {
    for i0 in 0..numbers.len() - 1 {
        for i1 in i0 + 1..numbers.len() {
            let slice = &numbers[i0..i1];
            let mut acc = 0;
            // early breakout
            for v in slice {
                acc += v;
                if acc > target {
                    break;
                }
                if acc == target {
                    return Some(slice);
                }
            }
            // if slice.iter().sum::<i64>() == target {
            //     return Some(slice);
            // }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let preamble = 25; // use 5 for 'example-input.txt'
    let buffered = BufReader::new(File::open("input.txt")?);
    let mut numbers = Vec::new();
    for lr in buffered.lines() {
        let number: i64 = lr?.parse()?;
        numbers.push(number);
    }

    let eval = numbers.iter().scan(VecDeque::new(), |buffer, value| {
        let evaluation = {
            if buffer.len() < preamble {
                Evaluation::Preamble
            } else if number_is_valid(*value, buffer) {
                Evaluation::Valid(*value)
            } else {
                Evaluation::Invalid(*value)
            }
        };

        // push new value onto the buffer, and pop the old value off when
        // we're at the correct capacity
        buffer.push_back(*value);
        if buffer.len() > preamble {
            buffer.pop_front();
        }

        Some(evaluation)
    });

    // part 1
    let results: Vec<_> = eval.collect();
    let first_invalid = results
        .iter()
        .find_map(|v| {
            if let Evaluation::Invalid(x) = v {
                Some(x)
            } else {
                None
            }
        })
        .unwrap();
    println!("Part 1 -> results {:?}", results);
    println!("Part 1 -> first invalid: {:?}", first_invalid);

    // part 2
    // now find contiguous range that adds up to the first invalid
    if let Some(sl) = find_contiguous_slice(*first_invalid, &numbers) {
        println!("Found slice: {:?}", sl);
        let min = sl.iter().min().unwrap();
        let max = sl.iter().max().unwrap();
        println!("Sum of min {} and max {} -> {}", min, max, min + max);
    }

    Ok(())
}
