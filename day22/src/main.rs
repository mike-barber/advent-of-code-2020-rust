use eyre::{eyre, Result};
use std::collections::VecDeque;

pub mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::*,
        combinator::{all_consuming, map, recognize},
        multi::*,
        sequence::*,
        IResult,
    };

    use crate::Deck;

    fn parse_deck(i: &str) -> IResult<&str, Deck> {
        map(
            tuple((
                recognize(tag("Player ")),
                digit1,
                recognize(tag(":")),
                recognize(multispace1),
                separated_list1(multispace1, digit1),
            )),
            |(_, player, _, _, cards)| Deck {
                player: player.parse(),
                cards: cards.iter().map(|c| c.parse()).collect(),
            },
        )(i)
    }

    pub fn parse_input(i: &str) -> eyre::Result<Vec<Deck>> {
        todo!();
    }
}

struct Deck {
    player: i32,
    cards: VecDeque<i32>,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/example-input.txt")?;

    let decks = parser::parse_input(&input)?;

    Ok(())
}
