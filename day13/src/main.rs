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

    // Part 1
    {
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
        println!(
            "Next bus is {}, time is {}, id*wait = {}",
            next_id,
            next_time,
            next_id * wait
        );
        println!("-----");
    }

    // Part 2 with LCM -- brute linear search proved infeasible and I needed to take a more iterative
    // approach. Divide and conquer. As per notes.txt.
    {
        // get ids and offsets
        let mut id_offset: Vec<_> = ids
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| v.map(|v| (v, idx as i64)))
            .collect();
        id_offset.sort_by_key(|(id, _offset)| *id);
        id_offset.reverse();

        // locate the next number in the sequence [start + N*stride] that matches some [M*id + offset].
        fn find_next_number(start: i64, stride: i64, id: i64, offset: i64) -> i64 {
            itertools::iterate(start, |v| v + stride)
                .find(|v| (v + offset) % id == 0)
                .unwrap()
        }

        // now iteratively include the busses, starting with the first bus; work out the lowest common multiplier
        // as the required stride for each next search.
        let starts: Vec<_> = id_offset
            .iter()
            .scan((0i64, 1i64), |(start, stride), (id, offset)| {
                let next = find_next_number(*start, *stride, *id, *offset);
                let next_stride = num::Integer::lcm(stride, id);
                println!(
                    "id {} offset {} start {} stride {} -> next {} next_stride {}",
                    id, offset, start, stride, next, next_stride
                );
                *start = next;
                *stride = next_stride;
                Some(next)
            })
            .collect();

        println!("Starts: {:?}", starts);
        println!("Earliest time for all busses: {}", starts.last().unwrap());
    }

    Ok(())
}
