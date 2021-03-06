use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

#[derive(Debug, Clone)]
struct FieldRange(Vec<RangeInclusive<i32>>);
impl FieldRange {
    fn create(ranges: Vec<RangeInclusive<i32>>) -> Self {
        FieldRange(ranges)
    }
    fn contains(&self, v: &i32) -> bool {
        self.0.iter().any(|r| r.contains(v))
    }
}

#[derive(Debug, Clone)]
struct FieldSpec {
    name: String,
    ranges: FieldRange,
}
impl FromStr for FieldSpec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_FIELD: Regex = Regex::new(r"([a-z ]+): (.*)").unwrap();
            static ref RE_RANGE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
        }

        let outer = RE_FIELD.captures(s).ok_or(anyhow!("parsing error"))?;
        let name = outer[1].to_string();

        let mut ranges = Vec::new();
        for range_str in outer[2].split("or") {
            let captures = RE_RANGE
                .captures(range_str)
                .ok_or(anyhow!("range parsing error"))?;
            let i0: i32 = captures[1].parse()?;
            let i1: i32 = captures[2].parse()?;
            ranges.push(i0..=i1);
        }

        Ok(FieldSpec {
            name,
            ranges: FieldRange::create(ranges),
        })
    }
}

#[derive(Debug, Clone)]
struct Ticket(Vec<i32>);
impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<i32>, _> = s.split(',').map(|v| v.parse()).collect();
        Ok(Ticket(values?))
    }
}

#[derive(Debug, Clone)]
struct Problem {
    field_specs: Vec<FieldSpec>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Problem {
    fn ticket_invalid_fields<'a>(&'a self, ticket: &'a Ticket) -> impl Iterator<Item = &i32> + 'a {
        let field_specs = &self.field_specs;
        ticket
            .0
            .iter()
            .filter(move |f| !field_specs.iter().any(|fs| fs.ranges.contains(f)))
    }
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut fields: Vec<FieldSpec> = Vec::new();
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }
            fields.push(l.parse()?);
        }

        // already ate the empty line above, just eat the title
        if lines.next().ok_or(anyhow!("input short"))? != "your ticket:" {
            return Err(anyhow!("missing your ticket"));
        }
        let ticket: Ticket = lines
            .next()
            .ok_or(anyhow!("missing your ticket"))?
            .parse()?;

        // eat the empty line and title
        lines.next().ok_or(anyhow!("input short"))?;
        if lines.next().ok_or(anyhow!("input short"))? != "nearby tickets:" {
            return Err(anyhow!("missing nearby tickets"));
        }
        let mut nearby_tickets: Vec<Ticket> = Vec::new();
        while let Some(l) = lines.next() {
            nearby_tickets.push(l.parse()?);
        }

        Ok(Problem {
            field_specs: fields,
            ticket,
            nearby_tickets,
        })
    }
}

fn find_field_numbers_possible(field_spec: &FieldSpec, valid_tickets: &[Ticket]) -> HashSet<usize> {
    let mut matches = HashSet::new();
    let num_fields = valid_tickets.first().map(|t| t.0.len()).unwrap_or(0);
    for i in 0..num_fields {
        if valid_tickets.iter().all(|t| {
            let field_val = t.0[i];
            field_spec.ranges.contains(&field_val)
        }) {
            matches.insert(i);
        }
    }
    matches
}

fn skip_nth_value<T>(source: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
    source
        .enumerate()
        .filter(move |(i, _v)| *i != n)
        .map(|(_i, v)| v)
}

fn main() -> Result<()> {
    let problem_str = std::fs::read_to_string("day16/input.txt")?;
    let problem: Problem = problem_str.parse()?;
    println!("Problem: {:?}", problem);

    //
    // Part 1
    //
    println!("Part 1 ---");
    for t in problem.nearby_tickets.iter() {
        let invalid_fields: Vec<_> = problem.ticket_invalid_fields(t).collect();
        println!("invalid fields: {:?}", invalid_fields);
    }
    let ticket_scanning_error_rate: i32 = problem
        .nearby_tickets
        .iter()
        .flat_map(|t| problem.ticket_invalid_fields(t))
        .sum();
    println!("ticket scanning error rate {}", ticket_scanning_error_rate);

    //
    // Part 1
    //
    println!("Part 2 ---");
    let valid_nearby: Vec<Ticket> = problem
        .nearby_tickets
        .iter()
        .filter(|&t| problem.ticket_invalid_fields(t).count() == 0)
        .cloned()
        .collect();
    println!("Valid nearby tickets: {:?}", valid_nearby);

    // find sets of fields that could match
    let all_field_numbers: Vec<_> = problem
        .field_specs
        .iter()
        .map(|f| find_field_numbers_possible(f, &valid_nearby))
        .collect();

    println!("All field numbers: {:?}", all_field_numbers);

    // now reduce each to a unique field -- one where we have only a single possible option
    // loop through all the field ranges
    //  - find a number that exists in only one set -> this is the only option for this set
    //  - remove the other numbers from this set
    //  - repeat
    //  - stop when all sets contain only a single entry
    let mut finished = false;
    let mut sets = all_field_numbers.clone();
    while !finished {
        for i in 0..sets.len() {
            // find a number in this set that is unique
            let unique_num_extract = {
                let set = &sets[i];
                set.iter()
                    .find(|&v| {
                        // skip self (checking the _reference_, not values)
                        // no other sets contain this value
                        skip_nth_value(sets.iter(), i).all(|s| !s.contains(v))
                    })
                    .copied() // needed to break reference (borrow checker)
            };
            // clear this set, and replace with only the unique number
            if let Some(num) = unique_num_extract {
                let mut_set = &mut sets[i];
                if mut_set.len() == 1 {
                    continue;
                }
                println!("Reducing set #{} {:?} -> {}", i, mut_set, num);
                mut_set.clear();
                mut_set.insert(num);
                continue; // start next loop
            }
        }
        // check if all sets are single elements
        if sets.iter().all(|s| s.len() == 1) {
            finished = true;
        }
    }

    println!("Unique sets: {:?}", sets);

    let my_field_values: Option<Vec<_>> = sets
        .iter()
        .map(|set| set.iter().nth(0).map(|nn| problem.ticket.0[*nn]))
        .collect();
    let my_field_values = my_field_values.unwrap();

    let departure_field_values: Vec<_> = problem
        .field_specs
        .iter()
        .zip(my_field_values.iter())
        .filter_map(|(fs, val)| {
            if fs.name.contains("departure") {
                Some(val)
            } else {
                None
            }
        })
        .collect();

    println!("My field values: {:?}", &my_field_values);
    println!("Departure field values: {:?}", &departure_field_values);

    let product: i64 = departure_field_values.iter().map(|&v| *v as i64).product();

    println!("Product: {}", product);

    Ok(())
}
