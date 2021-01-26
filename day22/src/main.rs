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
        if let Ok((_rem, mut res)) = separated_list1(multispace1, parse_deck)(i) {
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

impl Deck {
    fn score(&self) -> i32 {
        self.cards.iter().rev().enumerate().map(|(i,c)| {
            let value = i as i32 + 1;
            value * c
        }).sum()
    }
}


#[derive(Debug, Clone)]
pub struct Game {
    player1: Deck,
    player2: Deck
}

impl Game {
    fn next_round(&mut self) {
        if self.is_complete() {
            return;
        }

        let c1 = self.player1.cards.pop_front().unwrap();
        let c2 = self.player2.cards.pop_front().unwrap();
        if c1 > c2 {
            self.player1.cards.push_back(c1);
            self.player1.cards.push_back(c2);
        } else if c2 > c1 {
            self.player2.cards.push_back(c2);
            self.player2.cards.push_back(c1);
        } else {
            panic!("repeated cards")
        }
    }

    fn is_complete(&self) -> bool {
        self.player1.cards.is_empty() || self.player2.cards.is_empty()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/input.txt")?;

    let mut game = parser::parse_input(&input)?;
    println!("game: {:?}", game);

    while !game.is_complete() {
        game.next_round();
        println!("game: {:?}", game);
    }

    println!("Player 1 score: {}", game.player1.score());
    println!("Player 2 score: {}", game.player2.score());

    Ok(())
}
