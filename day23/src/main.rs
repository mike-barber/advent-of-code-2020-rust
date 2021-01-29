use eyre::Result;

mod part1;
mod part2;

fn main() -> Result<()> {
    part1::run_part1()?;

    part2::test_part1();
    part2::test_part2();
    part2::actual_part2();

    Ok(())
}
