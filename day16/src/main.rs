use std::{
    ops::{Range, RangeInclusive},
    str::FromStr,
    todo,
};

use anyhow::{anyhow, bail, Result};

trait RangeContains<T> {
    fn contains(&self, v: &T) -> bool;
}

#[derive(Debug, Clone)]
struct FieldRange(Vec<Range<i32>>);
impl FieldRange {
    fn create(ranges: &[Range<i32>]) -> Self {
        FieldRange(ranges.to_vec())
    }
}
// hook trait up to existing range types
impl RangeContains<i32> for RangeInclusive<i32> {
    fn contains(&self, v: &i32) -> bool {
        self.contains(v)
    }
}
// and do so for my own type too
impl RangeContains<i32> for FieldRange {
    fn contains(&self, v: &i32) -> bool {
        self.0.iter().all(|r| r.contains(v))
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
        todo!()
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

        lines.next().ok_or(anyhow!("input short"))?;
        if lines.next().ok_or(anyhow!("input short"))? != "your ticket:" {
            return Err(anyhow!("missing your ticket"));
        }

        let ticket: Ticket = lines
            .next()
            .ok_or(anyhow!("missing your ticket"))?
            .parse()?;

        lines.next().ok_or(anyhow!("input short"))?;
        if lines.next().ok_or(anyhow!("input short"))? != "your ticket:" {
            return Err(anyhow!("missing your ticket"));
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

fn main() -> Result<()> {
    println!("Hello, world!");

    let r1 = 1..=10;
    let r2 = 20..=30;

    let r = r1.chain(r2);

    println!("r: {:?}", r);
    println!("r: {:?}", r);

    Ok(())
}
