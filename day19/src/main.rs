use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, anychar, one_of, space0, space1}, combinator::{map_res, recognize}, multi::{many1, separated_list1}, sequence::{delimited, tuple}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuleId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Literal(char),
    Either(Vec<RuleId>, Vec<RuleId>),
    Ordered(Vec<RuleId>),
}

struct RuleSet(HashMap<usize, Rule>);

// e.g. 1
fn rule_number(i: &str) -> IResult<&str, RuleId> {
    map_res(
        delimited(space0, recognize(many1(one_of("1234567890"))), space0),
        |s: &str| s.parse().map(|n| RuleId(n)),
    )(i)
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
        tuple((ordered, many1(one_of(" | ")), ordered)),
        |(aa, _, bb)| match (aa, bb) {
            (Rule::Ordered(a), Rule::Ordered(b)) => Ok(Rule::Either(a, b)),
            _ => Err(anyhow!("missing Rule::Ordered on either")),
        },
    )(i)
}

fn rule(i: &str) -> IResult<&str, (RuleId, Rule)> {
    map_res(
        tuple((rule_number, tag(": "), alt((literal, ordered, either)))),
        |(nn, _, rule)| {
            let res: Result<(RuleId, Rule)> = Ok((nn, rule));
            res
        }
    )(i)
}

fn main() {
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

    for l in example_rules.iter() {
        let rule = rule(l);
        println!("{} => {:?}", l, rule);
    }
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
