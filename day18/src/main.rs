use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::one_of, combinator::recognize, multi::{many1, separated_list1}, number::complete::le_i64, sequence::terminated};

extern crate nom;

// nom reference
// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://docs.rs/nom/6.0.1/nom/

enum Expression {
    Value(i64),
    Add,
    Mul,
    Term(Vec<Expression>)
}


fn expression(input: &str) -> IResult<&str, Vec<&str>> {
    let value = recognize(many1(one_of("1234567890")));
    let add = recognize(tag("+"));
    let mul = recognize(tag("-"));
    let alternatives = alt((value,add,mul));
    let mut term = separated_list1(tag(" "), alternatives);
    
    term(input)
}


fn main() {
    //let example_string = "2 * 3 + (4 * 5)";

    println!("{:?}", expression("1 + 3"));


}

#[cfg(test)]
mod tests
{
    use super::*;


}