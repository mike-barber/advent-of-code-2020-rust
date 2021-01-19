use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{all_consuming, map_res, recognize},
    fold_many1,
    multi::{fold_many0, many1, separated_list1},
    sequence::{delimited, pair},
    IResult,
};

extern crate nom;

// nom reference
// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://docs.rs/nom/6.0.1/nom/

#[derive(Debug, Clone)]
enum Expression {
    Value(i64),
    Expr(Box<Expression>, Operation, Box<Expression>),
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mul,
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

fn value(s: &str) -> IResult<&str, Expression> {
    map_res(recognize(many1(one_of("1234567890"))), map_value)(s)
}
fn parens(s: &str) -> IResult<&str, Expression> {
    delimited(tag("("), multiplication_terms, tag(")"))(s)
}
fn operation_add(s: &str) -> IResult<&str, Operation> {
    map_res(recognize(one_of("+")), map_op)(s)
}
fn operation_mul(s: &str) -> IResult<&str, Operation> {
    map_res(recognize(one_of("*")), map_op)(s)
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

// multiplication is lower priority, so refers to addition
fn multiplication_terms(s: &str) -> IResult<&str, Expression> {
    let (s, first) = addition_terms(s)?; // refers to addition

    fold_many0(
        pair(operation_mul, alt((value, parens))),
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
    //let example_string = "2 * 3 + (4 * 5)";

    println!("{:?}", multiplication_terms("1*3+2"));

    // testing
    println!("{:?}", parse_program("1 + 3 * 5"));
    println!("{:?}", parse_program("1 + (3 * 4) + (5 + 6)"));
    println!("{:?}", parse_program("1 + (3 * (4 + 5) + (6 + 7))"));
    // println!("{:?}", reduce(&parse_program("1 + 2")?));
    // println!("{:?}", reduce(&parse_program("1 + 2 * 3")?));
    // println!("{:?}", reduce(&parse_program("2 * (1 + 2)")?));

    // let mut sum = 0i64;
    // let problem_str = std::fs::read_to_string("day18/input.txt")?;
    // for l in problem_str.lines() {
    //     let res = reduce(&parse_program(l)?)?;
    //     println!("result {} for {}", res, l);
    //     sum += res;
    // }
    // println!("Total: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn reference_calculations_correct() {
    //     // 2 * 3 + (4 * 5) becomes 26.
    //     // 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
    //     // 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
    //     // ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
    //     assert_eq!(
    //         26,
    //         reduce(&parse_program("2 * 3 + (4 * 5)").unwrap()).unwrap()
    //     );
    //     assert_eq!(
    //         437,
    //         reduce(&parse_program("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap()).unwrap()
    //     );
    //     assert_eq!(
    //         12240,
    //         reduce(&parse_program("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap()).unwrap()
    //     );
    //     assert_eq!(
    //         13632,
    //         reduce(&parse_program("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap())
    //             .unwrap()
    //     );
    // }
}
