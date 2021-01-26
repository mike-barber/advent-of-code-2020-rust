use crate::deck::Deck;

pub struct GameState {
    player1: Deck,
    player2: Deck,
}

pub struct RecursiveGame {
    state: GameState,
    history: Vec<GameState>,
}
