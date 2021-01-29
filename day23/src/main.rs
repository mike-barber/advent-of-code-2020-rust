use eyre::Result;

mod game;

fn main() -> Result<()> {
    game::test_part1();
    game::test_part2();
    game::actual_part2();

    Ok(())
}
