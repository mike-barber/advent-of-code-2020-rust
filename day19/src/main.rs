use anyhow::{Result, anyhow};
use nom::{IResult, bytes::complete::tag, character::complete::{alpha1, anychar, one_of, space0}, combinator::{map_res, recognize}, multi::{many1, separated_list1}, sequence::{delimited, tuple}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuleId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rule {
    Literal(char),
    Either(Vec<RuleId>, Vec<RuleId>),
    Ordered(Vec<RuleId>),
}

// e.g. 1
fn rule_number(i: &str) -> IResult<&str, RuleId> {
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
    map_res(
        separated_list1(space0, rule_number),
        |nn| {
            let res: Result<Rule> = Ok(Rule::Ordered(nn));
            res
        })(i)
}

fn either(i: &str) -> IResult<&str, Rule> {
    map_res(
        tuple((ordered, many1(one_of(" |")), ordered)),
        |(aa,_,bb)| {
            match (aa,bb) {
                (Rule::Ordered(a), Rule::Ordered(b)) => Ok(Rule::Either(a,b)),
                _ => Err(anyhow!("missing Rule::Ordered on either"))
            }
        })(i)
}

fn rule(i: &str) -> IResult<&str, (RuleId, Rule)> {
    todo!();
}

fn main() {
    let example_input = [r#"0: 1 2"#, r#"1: "a"#, r#"2: 1 3 | 3 1"#, r#"3: "b""]"#];
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
