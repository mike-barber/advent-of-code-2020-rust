use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};

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

fn find_field_number_matching(field_spec: &FieldSpec, valid_tickets: &[Ticket]) -> Option<usize> {
    for i in 0..valid_tickets.first()?.0.len() {
        if valid_tickets.iter().all(|t| {
            let field_val = t.0[i];
            field_spec.ranges.contains(&field_val)
        }) {
            return Some(i);
        }
    }
    None
}

fn main() -> Result<()> {
    let problem_str = std::fs::read_to_string("day16/input.txt")?;
    let problem: Problem = problem_str.parse()?;
    println!("Problem: {:?}", problem);

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

    println!("Part 2 ---");
    let valid_nearby: Vec<Ticket> = problem
        .nearby_tickets
        .iter()
        .filter(|&t| problem.ticket_invalid_fields(t).count() == 0)
        .cloned()
        .collect();
    //println!("Valid nearby tickets: {:?}", valid_nearby);

    // let field_spec = problem.field_specs.iter().find(|f| f.name == "seat").unwrap();
    // let field_number = find_field_number_matching(field_spec, &valid_nearby).unwrap();
    // println!("field: {}, value: {}", field_number, problem.ticket.0[field_number]);

    let departure_fields: Vec<_> = problem
        .field_specs
        .iter()
        .filter(|&f| f.name.contains("departure"))
        .collect();

    let departure_field_numbers: Vec<_> = departure_fields
        .iter()
        .map(|&f| find_field_number_matching(f, &valid_nearby))
        .collect();

    let my_field_values: Option<Vec<_>> = departure_field_numbers
        .iter()
        .map(|n| n.map(|nn| problem.ticket.0[nn] as i64))
        .collect();

    let product: i64 = my_field_values
        .as_ref()
        .ok_or(anyhow!("did not find all fields"))?
        .iter()
        .product();

    println!("My field values: {:?}", &my_field_values);
    println!("Product: {}", product);

    Ok(())
}
