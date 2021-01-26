use eyre::{eyre, Result};
use std::collections::VecDeque;

pub mod parser {
    use eyre::eyre;
    use nom::{IResult, bytes::complete::tag, character::complete::*, combinator::{all_consuming, map, map_res, recognize}, multi::*, sequence::*};

    use crate::{Deck, Game};


    fn num(i:&str) -> IResult<&str, i32> {
        map_res(recognize(digit1), |d:&str| d.parse())(i)
    }

    fn player(i:&str) -> IResult<&str, i32> {
        delimited(tag("Player "), num, tag(":"))(i)
    }

    fn cards(i:&str) -> IResult<&str, Vec<i32>> {
        separated_list1(multispace1, num)(i)
    }

    fn parse_deck(i: &str) -> IResult<&str, Deck> {
        map(tuple((player, multispace1, cards)), |(pl,_space, cards)| {
            Deck {
                player: pl,
                cards: cards.iter().copied().collect()
            }
        })(i)
    }

    pub fn parse_input(i: &str) -> eyre::Result<Game> {
        if let Ok((_rem, mut res)) = all_consuming(separated_list1(multispace1, parse_deck))(i) {
            let d2 = res.pop().ok_or(eyre!("missing deck 2"))?;
            let d1 = res.pop().ok_or(eyre!("missing deck 1"))?;
            Ok(Game{
                player1: d1,
                player2: d2,
            })
        } else {
            Err(eyre!("failed to parse"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    player: i32,
    cards: VecDeque<i32>,
}

#[derive(Debug, Clone)]
pub struct Game {
    player1: Deck,
    player2: Deck
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/example-input.txt")?;

    let game = parser::parse_input(&input)?;
    println!("game: {:?}", game);

    Ok(())
}
