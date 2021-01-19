use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{all_consuming, map_res, recognize},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};

extern crate nom;

// nom reference
// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://docs.rs/nom/6.0.1/nom/

#[derive(Debug, Clone)]
enum Expression {
    Value(i64),
    Add,
    Mul,
    Terms(Vec<Expression>),
}

// calculate strictly left-to-right, as per problem spec,
// without obeying addition/multiplication precedence.
// TODO: can probably do this with a normal fold instead,
//       or more interestingly, with nom's fold.
fn reduce(terms: &[Expression]) -> Result<i64> {
    let mut value = 0i64;
    let mut prev_operator = None;
    for term in terms {
        let next = match (term, &prev_operator) {
            (Expression::Value(v), None) => (value + v, None),
            (Expression::Add, None) => (value, Some(Expression::Add)),
            (Expression::Mul, None) => (value, Some(Expression::Mul)),
            (Expression::Terms(t), None) => (value + reduce(&t)?, None),
            (Expression::Value(v), Some(Expression::Add)) => (value + v, None),
            (Expression::Value(v), Some(Expression::Mul)) => (value * v, None),
            (Expression::Terms(t), Some(Expression::Add)) => (value + reduce(&t)?, None),
            (Expression::Terms(t), Some(Expression::Mul)) => (value * reduce(&t)?, None),
            _ => return Err(anyhow!("Unexpected state")),
        };
        // remove when destructuring assignment is stabilised
        value = next.0;
        prev_operator = next.1;
    }
    Ok(value)
}

fn map_value(s: &str) -> Result<Expression> {
    Ok(Expression::Value(s.parse()?))
}
fn map_op(s: &str) -> Result<Expression> {
    match s {
        "+" => Ok(Expression::Add),
        "*" => Ok(Expression::Mul),
        _ => Err(anyhow!("unrecognised op")),
    }
}

fn parse_element(s: &str) -> IResult<&str, Expression> {
    let value = map_res(recognize(many1(one_of("1234567890"))), map_value);
    let operation = map_res(recognize(one_of("+*")), map_op);
    alt((value, operation, parse_parens))(s)
}

fn parse_parens(s: &str) -> IResult<&str, Expression> {
    delimited(tag("("), parse_expression, tag(")"))(s)
}

fn parse_expression(s: &str) -> IResult<&str, Expression> {
    fn map_terms(v: Vec<Expression>) -> Result<Expression, ()> {
        Ok(Expression::Terms(v))
    }
    map_res(separated_list1(tag(" "), parse_element), map_terms)(s)
}

fn parse_program(s: &str) -> Result<Vec<Expression>> {
    if let Ok(res) = all_consuming(parse_expression)(s) {
        if let Expression::Terms(terms) = res.1 {
            Ok(terms)
        } else {
            Err(anyhow!("wrong expression type returned"))
        }
    } else {
        Err(anyhow!("parsing failed"))
    }
}

pub fn main() -> Result<()> {
    //let example_string = "2 * 3 + (4 * 5)";

    // testing
    println!("{:?}", parse_program("1 + 3 * 5"));
    println!("{:?}", parse_program("1 + (3 * 4) + (5 + 6)"));
    println!("{:?}", parse_program("1 + (3 * (4 + 5) + (6 + 7))"));
    println!("{:?}", reduce(&parse_program("1 + 2")?));
    println!("{:?}", reduce(&parse_program("1 + 2 * 3")?));
    println!("{:?}", reduce(&parse_program("2 * (1 + 2)")?));

    let mut sum = 0i64;
    let problem_str = std::fs::read_to_string("day18/input.txt")?;
    for l in problem_str.lines() {
        let res = reduce(&parse_program(l)?)?;
        println!("result {} for {}", res, l);
        sum += res;
    }
    println!("Total: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_calculations_correct() {
        // 2 * 3 + (4 * 5) becomes 26.
        // 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
        // 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
        // ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
        assert_eq!(
            26,
            reduce(&parse_program("2 * 3 + (4 * 5)").unwrap()).unwrap()
        );
        assert_eq!(
            437,
            reduce(&parse_program("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap()).unwrap()
        );
        assert_eq!(
            12240,
            reduce(&parse_program("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap()).unwrap()
        );
        assert_eq!(
            13632,
            reduce(&parse_program("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap())
                .unwrap()
        );
    }
}
