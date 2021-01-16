use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct NumberSpoken {
    round: i32,
    prior_round: Option<i32>,
}

fn run_game(round: i32, starting_numbers: &[i32]) -> impl Iterator<Item = i32> {
    struct State {
        memory: HashMap<i32, NumberSpoken>,
        last_number_spoken: i32,
    };

    let starting_numbers = starting_numbers.to_vec();
    let rounds = 1..=round;
    rounds.scan(
        State {
            memory: HashMap::new(),
            last_number_spoken: 0,
        },
        move |state, round| {
            let num = {
                if round <= starting_numbers.len() as i32 {
                    let num = starting_numbers[round as usize - 1];
                    num
                } else {
                    if let Some(m) = state.memory.get(&state.last_number_spoken) {
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
            if let Some(m) = state.memory.get_mut(&num) {
                *m = NumberSpoken {
                    round,
                    prior_round: Some(m.round),
                };
            } else {
                state.memory.insert(
                    num,
                    NumberSpoken {
                        round,
                        prior_round: None,
                    },
                );
            }
            // record last number spoken
            state.last_number_spoken = num;
            Some(num)
        },
    )
}

fn main() {
    println!("Example ----");
    {
        let numbers: Vec<_> = run_game(10, &[0, 3, 6]).collect();
        println!("{:?}", numbers);
    }

    println!("Part 1 ----");
    println!(
        "Final number: {:?}",
        run_game(2020, &[16, 11, 15, 0, 1, 7]).last()
    );

    println!("Part 2 ----");
    println!(
        "Final number: {:?}",
        run_game(30000000, &[16, 11, 15, 0, 1, 7]).last()
    );
}
