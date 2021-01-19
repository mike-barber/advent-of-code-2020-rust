use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{one_of, space0},
    combinator::{all_consuming, map_res, recognize},
    multi::{fold_many0, many1},
    sequence::{delimited, pair},
    IResult,
};

extern crate nom;

// nom reference
// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://docs.rs/nom/6.0.1/nom/

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
enum Expression {
    Value(i64),
    Expr(Box<Expression>, Operation, Box<Expression>),
}

impl Expression {
    fn reduce(&self) -> Result<i64> {
        match &self {
            Expression::Value(v) => Ok(*v),
            Expression::Expr(l, op, r) => {
                let lv = l.as_ref().reduce()?;
                let rv = r.as_ref().reduce()?;
                let res = match op {
                    Operation::Add => lv + rv,
                    Operation::Mul => lv * rv,
                };
                Ok(res)
            }
        }
    }
}

fn map_value(s: &str) -> Result<Expression> {
    Ok(Expression::Value(s.parse()?))
}
fn map_op(s: &str) -> Result<Operation> {
    match s {
        "+" => Ok(Operation::Add),
        "*" => Ok(Operation::Mul),
        _ => Err(anyhow!("unrecognised op")),
    }
}

// delimited with space0: we eat any space around the expression and ignore it.
fn value(s: &str) -> IResult<&str, Expression> {
    delimited(
        space0,
        map_res(recognize(many1(one_of("1234567890"))), map_value),
        space0,
    )(s)
}
fn parens(s: &str) -> IResult<&str, Expression> {
    delimited(
        space0,
        delimited(tag("("), multiplication_terms, tag(")")),
        space0,
    )(s)
}
fn operation_add(s: &str) -> IResult<&str, Operation> {
    delimited(space0, map_res(recognize(char('+')), map_op), space0)(s)
}
fn operation_mul(s: &str) -> IResult<&str, Operation> {
    delimited(space0, map_res(recognize(char('*')), map_op), space0)(s)
}

// addition takes priority in this special world
fn addition_terms(s: &str) -> IResult<&str, Expression> {
    let (s, first) = alt((value, parens))(s)?;
    fold_many0(
        pair(operation_add, alt((value, parens))),
        first,
        |acc, (op, val)| Expression::Expr(Box::new(acc), op, Box::new(val)),
    )(s)
}

// multiplication is lower priority, so refers to addition so that we ensure that all the additions
// are completed before multiplication.
fn multiplication_terms(s: &str) -> IResult<&str, Expression> {
    let (s, first) = addition_terms(s)?; // refers to addition
    fold_many0(
        pair(operation_mul, addition_terms), // refers to addition
        first,
        |acc, (op, val)| Expression::Expr(Box::new(acc), op, Box::new(val)),
    )(s)
}

fn parse_program(s: &str) -> Result<Expression> {
    if let Ok((_, res)) = all_consuming(multiplication_terms)(s) {
        Ok(res)
    } else {
        Err(anyhow!("parsing failed"))
    }
}

pub fn main() -> Result<()> {
    // examples
    println!("{:?}", multiplication_terms("1+2"));
    println!("{:?}", multiplication_terms("1+2+3*4"));
    println!("{:?}", multiplication_terms("1*3+2"));
    println!("{:?}", multiplication_terms("1 * 2 + 3"));
    println!("{:?}", multiplication_terms("1 + (2 * 3)"));
    // testing
    println!("{:?}", parse_program("1 + 3 * 5"));
    println!("{:?}", parse_program("1 + (3 * 4) + (5 + 6)"));
    println!("{:?}", parse_program("1 + (3 * (4 + 5) + (6 + 7))"));
    println!("{:?}", parse_program("1 + 3 * 5")?.reduce());
    println!("{:?}", parse_program("1 + (3 * 4) + (5 + 6)")?.reduce());
    println!(
        "{:?}",
        parse_program("1 + (3 * (4 + 5) + (6 + 7))")?.reduce()
    );

    // actual problem
    let mut sum = 0i64;
    let problem_str = std::fs::read_to_string("day18/input.txt")?;
    for l in problem_str.lines() {
        let res = parse_program(l)?.reduce()?;
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
        let cases = [
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for (prog, val) in cases.iter() {
            assert_eq!(*val, parse_program(prog).unwrap().reduce().unwrap());
        }
    }
}
