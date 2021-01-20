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

    fn evaluate_ordered<'a>(&self, i: &'a str, ids: &[RuleId]) -> IResult<&'a str, ()> {
        let mut remaining: &str = i;
        for id in ids {
            let rule = self.0.get(id).unwrap();
            let res = self.evaluate_rule(remaining, rule)?;
            remaining = res.0
        }
        Ok((remaining, ()))
    }

    fn evaluate_rule<'a>(&self, i: &'a str, rule: &Rule) -> IResult<&'a str, ()> {
        use nom::character::complete::char;
        fn result_ok() -> Result<()> {
            Ok(())
        }
        match rule {
            Rule::Literal(c) => map_res(char(*c), |_| result_ok())(i),
            Rule::Ordered(ids) => {
                let mut remaining: &str = i;
                for id in ids {
                    let rule = self.0.get(id).unwrap();
                    let res = self.evaluate_rule(remaining, rule)?;
                    remaining = res.0
                }
                Ok((remaining, ()))
            }
            Rule::Either((a, b)) => {
                if let Ok(res_a) = self.evaluate_ordered(i, a) {
                    Ok(res_a)
                } else if let Ok(res_b) = self.evaluate_ordered(i, b) {
                    Ok(res_b)
                } else {
                    Err(nom::Err::Failure(nom::error::Error::new(
                        i,
                        nom::error::ErrorKind::TooLarge,
                    )))
                }
            }
        }
    }

    // check to see if the supplied line matches rule 0
    fn matches_rule_0(&self, i: &str) -> bool {
        let rule = self.0.get(&RuleId(0)).unwrap();
        if let Ok(res) = self.evaluate_rule(i, rule) {
            // needs to be a complete match -- no remaining input
            if res.0.is_empty() {
                return true;
            }
        }
        false
    }
}


fn part1(rules: &[&str], lines: &[&str]) -> Result<()> {
    let rules = RuleSet::parse(rules.iter());
    //println!("{:?}", &rules);

    let rules = rules?;
    let mut count = 0;
    for i in lines.iter() {
        let res = rules.matches_rule_0(i);
        if res {
            count += 1;
        }
        println!("{} => {}", i, res);
    }
    println!("Matching inputs: {}", count);
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
    part1(&example_rules, &example_input)?;

    // part 1 actual
    println!("Part 1 ------------------------------------------------");
    {
        let rules_string = std::fs::read_to_string("day19/rules-part1.txt")?;
        let lines_string = std::fs::read_to_string("day19/lines.txt")?;
        let rules: Vec<&str> = rules_string.lines().collect();
        let lines: Vec<&str> = lines_string.lines().collect();
        part1(&rules, &lines)?;
    }

    // part 2 actual
    println!("Part 2 ------------------------------------------------");
    {
        let rules_string = std::fs::read_to_string("day19/rules-part2.txt")?;
        let lines_string = std::fs::read_to_string("day19/lines.txt")?;
        let rules: Vec<&str> = rules_string.lines().collect();
        let lines: Vec<&str> = lines_string.lines().collect();
        part1(&rules, &lines)?;
        // 141 is the WRONG answer
    }

    fn part2_test(rules_path: &str, lines_path: &str) -> Result<()> {
        let rules_string = std::fs::read_to_string(rules_path)?;
        let lines_string = std::fs::read_to_string(lines_path)?;
        let rules: Vec<&str> = rules_string.lines().collect();
        let lines: Vec<&str> = lines_string.lines().collect();
        part1(&rules, &lines)?;
        Ok(())
    }

    println!("Part 2 test A -------------");
    part2_test("day19/example-rules-part2-a.txt", "day19/example-input-part2.txt")?;
    println!("Part 2 test B -------------");
    part2_test("day19/example-rules-part2-b.txt", "day19/example-input-part2.txt")?;
    println!("Part 2 test B these should all match -------------");
    part2_test("day19/example-rules-part2-b.txt", "day19/example-input-part2-all-match.txt")?;

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
