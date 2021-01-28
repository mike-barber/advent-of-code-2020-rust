use eyre::{eyre, Result};
use std::{collections::VecDeque, fmt::format};

mod part1;

fn main() -> Result<()> {
    part1::run_part1()?;

    Ok(())
}
