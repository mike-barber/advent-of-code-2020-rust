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

    // Part 2 - second attempt
    {
        let id_offset: Vec<_> = ids
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| v.map(|v| (v, idx as i64)))
            .collect();

        let mut t_now = 0;
        loop {
            // find first possible next bus time
            let t_next_possible = id_offset
                .iter()
                .map(|(id, offset)| {
                    let t = t_now + offset;
                    (t / id + 1) * id
                })
                .min()
                .unwrap();

            // check if it matches all the busses
            t_now = t_next_possible;
            if id_offset.iter().all(|(id, offset)| {
                let t_effective = t_now + offset;
                t_effective % id == 0
            }) {
                println!("Found time: {}", t_now);
                break;
            }
        }
    }

    // Part 2
    {
        // aligned vectors of ids and offsets
        let effective_ids: Vec<i64> = ids.iter().filter_map(|x| *x).collect();
        let offsets: Vec<i64> = ids
            .iter()
            .enumerate()
            .filter_map(|(idx, b)| b.map(|_| idx as i64))
            .collect();
        println!("ids:     {:?}", &effective_ids);
        println!("offsets: {:?}", &offsets);
        let stride = effective_ids[0];

        // relying on the compiler to do some nice vectorisation here, rather than
        // testing every bus.
        let mut residuals = vec![0i64; effective_ids.len()];
        let mut t_now = 0;
        loop {
            residuals
                .iter_mut()
                .zip(&effective_ids)
                .zip(&offsets)
                .for_each(|((res, id), off)| {
                    let t_bus = t_now + off;
                    *res = t_bus % id;
                });
            if residuals.iter().all(|r| *r == 0) {
                println!("Found time: {}", t_now);
                break;
            }
            t_now += stride;
        }
    }

    Ok(())
}
