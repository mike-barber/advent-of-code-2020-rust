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

fn number_is_valid(value:i64, buffer:&VecDeque<i64>) -> bool {
    for i in buffer.iter() {
        for j in buffer.iter() {
            if i != j && i + j == value {
                return true;
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("input.txt")?);
    let mut numbers = Vec::new();
    for lr in buffered.lines() {
        let number: i64 = lr?.parse()?;
        numbers.push(number);
    }

    let preamble = 25;
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

    let results: Vec<_> = eval.collect();
    println!("Part 1 -> results {:?}", results);
    println!("Part 1 -> first invalid: {:?}", results.iter().find(|&v| match v {
        Evaluation::Invalid(_) => true,
        _ => false
    }));

    Ok(())
}
