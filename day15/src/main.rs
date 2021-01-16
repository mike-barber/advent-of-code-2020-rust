use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    mem, todo,
};

struct RecentNumbers(usize, VecDeque<i32>);
impl RecentNumbers {
    fn create_with(capacity: usize, value: i32) -> Self {
        let mut vec = VecDeque::new();
        vec.push_back(value);
        Self(capacity, vec)
    }

    fn push(&mut self, value: i32) {
        let vec = &mut self.1;
        vec.push_back(value);
        if vec.len() > self.0 {
            vec.pop_front();
        }
    }

    // just return the iterator from the VecDeque
    fn iter(&self) -> impl Iterator<Item = &i32> {
        self.1.iter()
    }
}

#[derive(Debug, Clone, Copy)]
struct NumberSpoken {
    round: i32,
    prior_round: Option<i32>,
}

fn main() {
    //let starting_numbers = [0, 3, 6];
    let starting_numbers = [16,11,15,0,1,7];
    let rounds = 2020i32;

    let mut memory: HashMap<i32, NumberSpoken> = HashMap::new();
    let mut last_number_spoken = 0;

    for round in 1..=rounds {
        let num = {
            if round <= starting_numbers.len() as i32 {
                let num = starting_numbers[round as usize - 1];
                num
            } else {
                if let Some(m) = memory.get(&last_number_spoken) {
                    if let Some(prior) = m.prior_round {
                        m.round - prior
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        };
        // update memory 
        if let Some(m) = memory.get_mut(&num) {
            *m = NumberSpoken { 
                round,
                prior_round: Some(m.round)
            };
        } else {
            memory.insert(num, NumberSpoken { round, prior_round: None });
        }
        // record last number spoken
        last_number_spoken = num;
        println!("round {} -> num {}", round, num);
    }
}
