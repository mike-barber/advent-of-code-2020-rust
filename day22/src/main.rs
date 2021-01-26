use day22::{parser, part2::RecursiveGame};
use eyre::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/input.txt")?;

    // part 1
    {
        println!("Part 1 ----------");
        let mut game = parser::parse_input(&input)?;
        println!("game: {:?}", game);

        while !game.is_complete() {
            game.next_round();
            println!("game: {:?}", game);
        }

        println!("Player 1 score: {}", game.player1.score());
        println!("Player 2 score: {}", game.player2.score());
    }

    println!();

    // part 2
    {
        println!("Part 2 ----------");
        let simple_game = parser::parse_input(&input)?;
        let mut game = RecursiveGame::from_simple_game(&simple_game);

        let winner = game.play_game();
        let deck = game.deck_for(winner);
        println!("Winner {:?} {:?} score {}", winner, deck, deck.score());
    }

    Ok(())
}
