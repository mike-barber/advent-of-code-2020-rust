use eyre::{eyre, Result};
use std::{collections::VecDeque, fmt::format};

mod part1;
mod part2;

fn main() -> Result<()> {
    part1::run_part1()?;

    Ok(())
}
