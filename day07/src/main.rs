use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]
struct ParseError(String);
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError({})", self.0)
    }
}
impl Error for ParseError {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bag(String);

#[derive(Debug)]
struct BagQuantity {
    number: i32,
    bag: Bag,
}

#[derive(Debug)]
struct BagRule {
    bag: Bag,
    contains: Vec<BagQuantity>,
}

// recursively find bags that can contain this bag
fn containers_for(bag: &Bag, rules: &[BagRule]) -> HashSet<Bag> {
    let mut set = HashSet::new();
    for rule in rules
        .iter()
        .filter(|&r| r.contains.iter().any(|qty| &qty.bag == bag))
    {
        // direct container of this bag
        set.insert(rule.bag.clone());
        // containers of this container (recursive)
        for b in containers_for(&rule.bag, rules) {
            set.insert(b);
        }
    }
    set
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("input.txt")?);

    // try to change this to use nom -- would be much cooler
    let regex_bag_qty = Regex::new("(\\d+) ([a-z ]+?) bag")?;

    let mut rules = Vec::new();
    for l in buffered.lines() {
        let line = l?;

        // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        let outer: Vec<_> = line.split("bags contain").collect();
        if outer.len() != 2 {
            return Err(Box::new(ParseError("Outer split failed".to_string())));
        }

        // left hand side -- bag
        let bag = Bag(outer[0].trim().to_string());

        // right hand side -- list of bags it contains
        let mut bag_quantities = Vec::new();
        for contained in outer[1].split(",") {
            if let Some(captures) = regex_bag_qty.captures(contained) {
                bag_quantities.push(BagQuantity {
                    bag: Bag(captures[2].into()),
                    number: captures[1].parse().unwrap(),
                })
            }
        }

        let rule = BagRule {
            bag,
            contains: bag_quantities,
        };
        println!("Rule: {:?}", rule);
        rules.push(rule);
    }

    // part 1 -- find out what bags can eventually hold my bag
    {
        let my_bag = Bag("shiny gold".into());
        let containers = containers_for(&my_bag, &rules);
        println!("Part 1 -> Containers: {:?}", containers);
        println!("Part 1 -> Number: {}", containers.len());
    }

    Ok(())
}
