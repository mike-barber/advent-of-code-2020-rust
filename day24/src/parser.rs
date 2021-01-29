use crate::Dir;
use crate::Dir::*;
use multi::many1;
use nom::{branch::*, bytes::complete::tag, combinator::*, multi, IResult};

// returns a parsing function for the given direction
fn parse_dir(tag_str: &str, dir: Dir) -> impl Fn(&str) -> IResult<&str, Dir> {
    let tt = tag_str.to_string();
    move |i| map(tag(tt.as_str()), |_| dir)(i)
}

fn direction(i: &str) -> IResult<&str, Dir> {
    alt((
        parse_dir("e", E),
        parse_dir("se", SE),
        parse_dir("sw", SW),
        parse_dir("w", W),
        parse_dir("nw", NW),
        parse_dir("ne", NE),
    ))(i)
}

pub fn directions(i: &str) -> IResult<&str, Vec<Dir>> {
    many1(direction)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions_parsed_correctly() {
        let input = "nwwswee";
        let parsed = directions(input).unwrap().1;
        assert_eq!(vec![NW, W, SW, E, E], parsed);
    }
}
