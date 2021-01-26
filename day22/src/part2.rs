use crate::{deck::Deck, part1::SimpleGame};

pub enum Player {
    Player1,
    Player2
}

#[derive(Clone,PartialEq,Eq)]
pub struct GameState {
    player1: Deck,
    player2: Deck,
}

pub struct RecursiveGame {
    state: GameState,
    history: Vec<GameState>,
}


impl RecursiveGame {
    pub fn from_simple_game(game: &SimpleGame) -> Self {
        let state = GameState {
            player1: game.player1.clone(),
            player2: game.player2.clone()
        };
        RecursiveGame {
            state,
            history: Vec::new()
        }
    }

    pub fn play_game(&mut self) -> Player {
        todo!();
    }
}