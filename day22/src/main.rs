use day22::parser;
use eyre::{eyre, Result};
use std::collections::VecDeque;


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
