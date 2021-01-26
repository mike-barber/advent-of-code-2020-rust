use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    pub player: i32,
    pub cards: VecDeque<i32>,
}

impl Deck {
    pub fn score(&self) -> i32 {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let value = i as i32 + 1;
                value * c
            })
            .sum()
    }
}
