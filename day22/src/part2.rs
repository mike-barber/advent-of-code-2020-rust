use std::usize;

use crate::{deck::Deck, part1::SimpleGame};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    player1: Deck,
    player2: Deck,
}

#[derive(Debug)]
pub struct RecursiveGame {
    state: GameState,
    history: Vec<GameState>,
}

impl RecursiveGame {
    pub fn from_simple_game(game: &SimpleGame) -> Self {
        let state = GameState {
            player1: game.player1.clone(),
            player2: game.player2.clone(),
        };
        RecursiveGame {
            state,
            history: Vec::new(),
        }
    }

    fn create_subgame(&self, num_cards_p1: usize, num_cards_p2: usize) -> Self {
        RecursiveGame {
            history: Vec::new(),
            state: GameState {
                player1: Deck {
                    cards: self
                        .state
                        .player1
                        .cards
                        .iter()
                        .copied()
                        .take(num_cards_p1)
                        .collect(),
                    ..self.state.player1
                },
                player2: Deck {
                    cards: self
                        .state
                        .player2
                        .cards
                        .iter()
                        .copied()
                        .take(num_cards_p2)
                        .collect(),
                    ..self.state.player2
                },
            },
        }
    }

    fn is_complete(&self) -> bool {
        self.state.player1.cards.is_empty() || self.state.player2.cards.is_empty()
    }

    pub fn play_game(&mut self) -> Player {
        while !self.is_complete() {
            // check history; player 1 wins if we've been in this state before.
            if self.history.contains(&self.state) {
                return Player::Player1;
            }

            // record history
            self.history.push(self.state.clone());

            // take cards
            let c1 = self.state.player1.cards.pop_front().unwrap();
            let c2 = self.state.player2.cards.pop_front().unwrap();

            // If both players have at least as many cards remaining in their deck as
            // the value of the card they just drew, the winner of the round is
            // determined by playing a new game of Recursive Combat (see below).
            let remaining_c1 = self.state.player1.cards.len() as i32;
            let remaining_c2 = self.state.player2.cards.len() as i32;
            let winner = if remaining_c1 >= c1 && remaining_c2 >= c2 {
                // play subgame
                let mut subgame = self.create_subgame(c1 as usize, c2 as usize);
                subgame.play_game()
            } else {
                // not enough cards to recurse: winner is the player with
                // the higher value card
                if c1 >= c2 {
                    Player::Player1
                } else {
                    Player::Player2
                }
            };

            // put cards on the winner's deck; winner's card first.
            match winner {
                Player::Player1 => {
                    self.state.player1.cards.push_back(c1);
                    self.state.player1.cards.push_back(c2);
                }
                Player::Player2 => {
                    self.state.player2.cards.push_back(c2);
                    self.state.player2.cards.push_back(c1);
                }
            }
        }

        if self.state.player1.cards.is_empty() {
            Player::Player2
        } else if self.state.player2.cards.is_empty() {
            Player::Player1
        } else {
            panic!("end of game: one player's deck should be empty")
        }
    }

    pub fn deck_for(&self, player: Player) -> &Deck {
        match player {
            Player::Player1 => &self.state.player1,
            Player::Player2 => &self.state.player2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;

    use super::{GameState, Player, RecursiveGame};

    #[test]
    fn validate_breakout() {
        // arrange
        let mut game = RecursiveGame {
            history: Vec::new(),
            state: GameState {
                player1: Deck {
                    player: 1,
                    cards: [43, 19].iter().copied().collect(),
                },
                player2: Deck {
                    player: 2,
                    cards: [2, 29, 14].iter().copied().collect(),
                },
            },
        };

        // assert
        assert_eq!(Player::Player1, game.play_game())
    }
}
