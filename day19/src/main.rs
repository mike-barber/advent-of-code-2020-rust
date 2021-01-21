use std::{collections::HashMap};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, one_of, space1},
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RuleId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Literal(char),
    Either((Vec<RuleId>, Vec<RuleId>)),
    Ordered(Vec<RuleId>),
}

// e.g. 1
fn rule_number(i: &str) -> IResult<&str, RuleId> {
    // map_res(
    //     delimited(space0, recognize(many1(one_of("1234567890"))), space0),
    //     |s: &str| s.parse().map(|n| RuleId(n)),
    // )(i)
    map_res(recognize(many1(one_of("1234567890"))), |s: &str| {
        s.parse().map(|n| RuleId(n))
    })(i)
}

// e.g. "a"
fn literal(i: &str) -> IResult<&str, Rule> {
    map_res(delimited(tag("\""), anychar, tag("\"")), |c: char| {
        let res: Result<Rule> = Ok(Rule::Literal(c));
        res
    })(i)
}

// e.g. 1 2
fn ordered(i: &str) -> IResult<&str, Rule> {
    map_res(separated_list1(space1, rule_number), |nn| {
        let res: Result<Rule> = Ok(Rule::Ordered(nn));
        res
    })(i)
}

// e.g. 1 2 | 2 3
fn either(i: &str) -> IResult<&str, Rule> {
    map_res(
        tuple((ordered, tag(" | "), ordered)),
        |(aa, _, bb)| match (aa, bb) {
            (Rule::Ordered(a), Rule::Ordered(b)) => Ok(Rule::Either((a, b))),
            _ => Err(anyhow!("missing Rule::Ordered on either")),
        },
    )(i)
}

fn rule(i: &str) -> IResult<&str, (RuleId, Rule)> {
    map_res(
        // note: either checked first
        tuple((rule_number, tag(": "), alt((literal, either, ordered)))),
        |(nn, _, rule)| {
            let res: Result<(RuleId, Rule)> = Ok((nn, rule));
            res
        },
    )(i)
}

#[derive(Debug)]
struct RuleSet(HashMap<RuleId, Rule>);
impl RuleSet {
    fn parse<'a, I>(lines: I) -> Result<Self>
    where
        I: Iterator<Item = &'a &'a str>,
    {
        let mut rules = HashMap::new();
        for l in lines {
            let (id, rule) = rule(l)
                .map_err(|e| anyhow!("parsing error: {}", e.to_string()))?
                .1;
            rules.insert(id, rule);
        }
        Ok(RuleSet(rules))
    }

    fn parse_from_file(path: &str) -> Result<Self> {
        let all = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = all.lines().collect();
        Self::parse(lines.iter())
    }

    fn rule(&self, id: &RuleId) -> Option<&Rule> {
        self.0.get(id)
    }

    fn evaluate_ordered_str<'a>(&self, i: &'a str, ids: &[RuleId]) -> Vec<(&'a str, String)> {
        // find *all* possible results
        match ids.len() {
            1 => {
                let r0 = self.rule(&ids[0]).unwrap();
                self.evaluate_rule_str(i, r0)
            }
            2 => {
                let r0 = self.rule(&ids[0]).unwrap();
                let r1 = self.rule(&ids[1]).unwrap();
                let mut results = Vec::new();
                for (rem0, found0) in self.evaluate_rule_str(i, r0) {
                    for (rem1, found1) in self.evaluate_rule_str(rem0, r1) {
                        results.push((rem1, format!("{}{}", found0, found1)))
                    }
                }
                results
            }
            3 => {
                let r0 = self.rule(&ids[0]).unwrap();
                let r1 = self.rule(&ids[1]).unwrap();
                let r2 = self.rule(&ids[2]).unwrap();
                let mut results = Vec::new();
                for (rem0, found0) in self.evaluate_rule_str(i, r0) {
                    for (rem1, found1) in self.evaluate_rule_str(rem0, r1) {
                        for (rem2, found2) in self.evaluate_rule_str(rem1, r2) {
                            results.push((rem2, format!("{}{}{}", found0, found1, found2)))
                        }
                    }
                }
                results
            }
            _ => panic!("unsupported list length: {:?}", ids),
        }
    }

    fn evaluate_rule_str<'a>(&self, i: &'a str, rule: &Rule) -> Vec<(&'a str, String)> {
        match rule {
            Rule::Literal(c) => {
                // match exact char -- single element vector returned
                let mut results = Vec::new();
                let matched: IResult<&str, char> = nom::character::complete::char(*c)(i);
                if let Ok((rem, found_char)) = matched {
                    results.push((rem, found_char.to_string()));
                }
                results
            }
            Rule::Ordered(ids) => self.evaluate_ordered_str(i, ids),
            Rule::Either((ids_a, ids_b)) => {
                // return list of results from both arms
                let mut results = Vec::new();
                results.append(&mut self.evaluate_ordered_str(i, ids_a));
                results.append(&mut self.evaluate_ordered_str(i, ids_b));
                results
            }
        }
    }

    fn evaluate_rule_complete<'a>(&self, i: &'a str, rule: &Rule) -> Option<(&'a str, String)> {
        self.evaluate_rule_str(i, rule)
            .into_iter()
            .find(|(rem, _)| rem.is_empty())
    }
}

fn test_rules(rules: &RuleSet, start_rule: RuleId, lines: &[&str]) -> Result<()> {
    let rule = rules
        .rule(&start_rule)
        .ok_or(anyhow!("missing rule {:?}", start_rule))?;
    let mut count = 0;
    for i in lines.iter() {
        let res = rules.evaluate_rule_complete(i, &rule).is_some();
        if res {
            count += 1;
        }
        //println!("{} => {}", i, res);
    }
    println!("Matching inputs: {}", count);
    Ok(())
}

// debugging code 
#[allow(dead_code)]
fn debug_test() -> Result<()> {
    let rules = RuleSet::parse_from_file("day19/example-rules-part2-b.txt")?;
    let id = RuleId(0);
    // 8 or 11 are the new self-referential ones
    //let id = RuleId(11);
    //let id = RuleId(42);
    //let id = RuleId(31);
    let rule = rules.rule(&id).unwrap();

    let mut count = 0;
    for l in std::fs::read_to_string("day19/example-input-part2.txt")?.lines() {
        let res = rules.evaluate_rule_str(l, &rule);
        println!("{} => {:?}", l, res);
        if let Some((rem, found)) = res.iter().find(|(rem, _)| rem.len() == 0) {
            println!("+ FOUND rem {} found {}", rem, found);
            assert_eq!(found, l);
            count += 1;
        } else {
            println!("- NOT FOUND: {}", l);
        }
    }
    println!("matching count: {}", count);
    Ok(())
}

fn main() -> Result<()> {
    // test inputs
    let example_rules = [
        r#"0: 4 1 5"#,
        r#"1: 2 3 | 3 2"#,
        r#"2: 4 4 | 5 5"#,
        r#"3: 4 5 | 5 4"#,
        r#"4: "a""#,
        r#"5: "b""#,
    ];
    let example_input = [
        r#"ababbb"#,
        r#"bababa"#,
        r#"abbbab"#,
        r#"aaabbb"#,
        r#"aaaabbb"#,
    ];
    test_rules(
        &RuleSet::parse(example_rules.iter())?,
        RuleId(0),
        &example_input,
    )?;

    // part 1 actual
    let lines_string = std::fs::read_to_string("day19/lines.txt")?;
    let lines: Vec<&str> = lines_string.lines().collect();
    println!("Part 1 ------------------------------------------------");
    {
        let rules = RuleSet::parse_from_file("day19/rules-part1.txt")?;
        test_rules(&rules, RuleId(0), &lines)?;
        // correct is 113
    }   

    // part 2 actual
    println!("Part 2 ------------------------------------------------");
    {
        let rules = RuleSet::parse_from_file("day19/rules-part2.txt")?;
        test_rules(&rules, RuleId(0), &lines)?; 
        // correct is 253
    }
   
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal() {
        assert_eq!(Rule::Literal('a'), literal("\"a\"").unwrap().1);
    }

    #[test]
    fn parse_number() {
        assert_eq!(RuleId(42), rule_number("42").unwrap().1);
    }
}
