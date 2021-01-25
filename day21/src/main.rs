use std::fs;

use day21::parse::parse_food;
use eyre::{Result, eyre};

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}
impl Food {
    fn parse_line(i: &str) -> Result<Food> {
        let (_rem, res) = parse_food(i).map_err(|_| eyre!("parsing error"))?;
        Ok(Food {
            ingredients: res.0.iter().map(|s| s.to_string()).collect(),
            allergens: res.1.iter().map(|s| s.to_string()).collect(),
        })
    }
}

fn read_foods(path: &str) -> Result<Vec<Food>> {
    let contents = fs::read_to_string(path)?;
    let mut foods = Vec::new();
    for l in contents.lines() {
        foods.push(Food::parse_line(l)?);
    }
    Ok(foods)
}

fn main() -> Result<()> {
    let foods = read_foods("day21/example-input.txt")?;
    println!("{:?}", foods);

    Ok(())
}
