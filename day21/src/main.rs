use std::{
    collections::{HashMap, HashSet},
    fs,
};

use day21::parse::parse_food;
use eyre::{eyre, Result};

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
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
    let foods = read_foods("day21/input.txt")?;
    println!("{:?}", foods);

    // -----------------------------
    // Part 1 -- find inert ingredients

    let all_allergens: HashSet<_> = foods.iter().flat_map(|f| f.allergens.iter()).collect();
    let all_ingredients: HashSet<_> = foods.iter().flat_map(|f| f.ingredients.iter()).collect();

    let mut possible_causes = HashMap::new();
    for allergen in all_allergens {
        let mut matching_foods = foods.iter().filter(|f| f.allergens.contains(allergen));
        let possible_ingredients = matching_foods
            .next()
            .map(|m| matching_foods.fold(m.ingredients.clone(), |acc, f| &acc & &f.ingredients));

        possible_causes.insert(allergen, possible_ingredients.unwrap());
    }

    println!("Possible causes {:?}", possible_causes);

    let all_ingredients_possible_causes: HashSet<_> = possible_causes
        .iter()
        .flat_map(|(_, ingred)| ingred.iter())
        .collect();
    let safe_ingredients: HashSet<_> = all_ingredients
        .iter()
        .filter(|&ingredient| !all_ingredients_possible_causes.contains(ingredient))
        .collect();

    println!("Safe ingredients: {:?}", safe_ingredients);
    let safe_ingredients_count = safe_ingredients.iter().fold(0, |acc, &&i| {
        let count = foods.iter().filter(|&f| f.ingredients.contains(i)).count();
        acc + count
    });
    println!(
        "Safe ingredients occur a total of {} times",
        safe_ingredients_count
    );

    // -----------------------------
    // Part 2 -- find which ingredients map to which allergens

    let mut possible_reduction = possible_causes.clone();
    loop {
        // create list of known unique causes (cloning them to avoid mutability collisions)
        let unique_causes: Vec<_> = possible_reduction
            .iter()
            .filter_map(|(&allergen, ingredients)| match ingredients.len() {
                1 => Some((allergen.clone(), ingredients.iter().next().unwrap().clone())),
                _ => None,
            })
            .collect();

        // remove each of these ingredients from the list of other possible causes of allergens
        for (unique_allergen, unique_ingredient) in unique_causes.iter() {
            for (&allergen, ingredients) in possible_reduction.iter_mut() {
                if allergen != unique_allergen {
                    ingredients.remove(unique_ingredient);
                }
            }
        }

        // stop when all allergens have a unique cause
        if possible_reduction
            .iter()
            .all(|(_, ingredients)| ingredients.len() == 1)
        {
            break;
        }
    }

    // collect and sort by allergen
    let mut causes: Vec<_> = possible_reduction
        .iter()
        .map(|(&allergen, ingredients)| (allergen.clone(), ingredients.iter().next().unwrap()))
        .collect();
    causes.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    println!("Unique causes: {:?}", causes);

    // and present as a canonical list of allergens
    let canonical = itertools::join(causes.iter().map(|(_, ingred)| ingred), ",");
    println!("Canonical result:\n{}", canonical);

    Ok(())
}
