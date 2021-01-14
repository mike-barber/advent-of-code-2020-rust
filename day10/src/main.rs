use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

struct AdaptersProblem {
    adapters: Vec<i32>,
    final_joltage: i32
}
impl AdaptersProblem {
 
    fn create(adapters: Vec<i32>) -> Self {
        let final_joltage = *adapters.iter().max().unwrap() + 3;
        AdaptersProblem {
            adapters,
            final_joltage
        }
    }
 
    fn find_chain(&self) -> Option<Vec<i32>> {
        let bag: HashSet<i32> = self.adapters.iter().copied().collect();
        self.find_chain_internal(vec![0], bag)
    }

    fn find_chain_internal(&self, chain: Vec<i32>, bag: HashSet<i32>) -> Option<Vec<i32>> {
        if bag.is_empty() {
            let mut new_chain = chain.clone();
            new_chain.push(self.final_joltage);
            Some(new_chain)
        } else {
            // find all adapters that can connect to the start
            let start = chain.last().unwrap();
            let chain_option = bag.iter().find_map(|&adapt| {
                let diff = adapt - start;
                // try all possible adapters
                if diff >= 1 && diff <= 3 {
                    let mut new_chain = chain.clone();
                    let mut new_bag = bag.clone();
                    new_chain.push(adapt);
                    new_bag.remove(&adapt);
                    self.find_chain_internal(new_chain, new_bag)
                } else {
                    None
                }
            });
            chain_option
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("input.txt")?);
    let mut adapters = Vec::new();
    for l in buffered.lines() {
        let joltage: i32 = l?.parse()?;
        adapters.push(joltage);
    }

    let problem = AdaptersProblem::create(adapters);
    let chain = problem.find_chain().unwrap();
    println!("Chain: {:?}", chain);

    let diffs: Vec<_> = chain.iter().zip(chain.iter().skip(1)).map(|(&a,&b)| b-a).collect();
    println!("Diffs: {:?}", diffs);

    let diffs_1 = diffs.iter().filter(|&&v| v == 1).count();
    let diffs_3 = diffs.iter().filter(|&&v| v == 3).count();
    println!("n(1): {}, n(3): {}, n(1)*n(3): {}", diffs_1, diffs_3, diffs_1 * diffs_3);

    Ok(())
}
