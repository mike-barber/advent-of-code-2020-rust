use crate::deck::Deck;

#[derive(Debug, Clone)]
pub struct SimpleGame {
    pub player1: Deck,
    pub player2: Deck,
}

impl SimpleGame {
    pub fn next_round(&mut self) {
        if self.is_complete() {
            return;
        }

        let c1 = self.player1.cards.pop_front().unwrap();
        let c2 = self.player2.cards.pop_front().unwrap();
        match c1.cmp(&c2) {
            std::cmp::Ordering::Greater => {
                self.player1.cards.push_back(c1);
                self.player1.cards.push_back(c2);
            },
            std::cmp::Ordering::Less => {
                self.player2.cards.push_back(c2);
                self.player2.cards.push_back(c1);
            },
            std::cmp::Ordering::Equal => {
                panic!("repeated cards")
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        self.player1.cards.is_empty() || self.player2.cards.is_empty()
    }
}
