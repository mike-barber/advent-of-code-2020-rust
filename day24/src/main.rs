#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    NW,
    NE,
    E,
    SE,
    SW,
    W,
}
pub type Coord = [i32; 2];

mod parser {
    use multi::many1;
    use nom::{IResult, branch::*, bytes::complete::tag, character::complete::*, combinator::*, multi};
    use crate::Dir::*;
    use crate::Dir;

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
            assert_eq!(vec![NW,W,SW,E,E], parsed);
        }
    }

}

fn main() {
    println!("Hello, world!");
}

