use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::one_of, combinator::{map_res, recognize}, multi::{many1, separated_list1}, number::complete::le_i64, sequence::terminated};
use anyhow::{Result, anyhow};

extern crate nom;

// nom reference
// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://docs.rs/nom/6.0.1/nom/

#[derive(Debug,Clone)]
enum Expression {
    Value(i64),
    Add,
    Mul,
    Term(Vec<Expression>)
}

fn map_value(s:&str) -> Result<Expression> {
    Ok(Expression::Value(s.parse()?))
}
fn map_op(s:&str) -> Result<Expression> {
    match s {
        "+" => Ok(Expression::Add),
        "*" => Ok(Expression::Mul),
        _ => Err(anyhow!("unrecognised op"))
    }
}

fn expression(input: &str) -> IResult<&str, Vec<Expression>> {
    let value = map_res(
        recognize(many1(one_of("1234567890"))),
        map_value);
    let operation = map_res(recognize(one_of("+*")), map_op);
    let alternatives = alt((value,operation));
    let mut term = separated_list1(tag(" "), alternatives);
    
    term(input)
}


fn main() {
    //let example_string = "2 * 3 + (4 * 5)";

    println!("{:?}", expression("1 + 3 * 5"));


}

#[cfg(test)]
mod tests
{
    use super::*;


}