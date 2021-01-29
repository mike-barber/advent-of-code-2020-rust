use eyre::{eyre, Result};
use std::collections::VecDeque;

struct Context {
    //cups: Vec<i8>,
    min: i8,
    max: i8,
}
impl Context {
    fn create<I>(cups: I) -> Result<Self>
    where
        I: Iterator<Item = i8>,
    {
        let cc: Vec<i8> = cups.collect();
        Ok(Context {
            min: *cc.iter().min().ok_or(eyre!("empty"))?,
            max: *cc.iter().max().ok_or(eyre!("empty"))?,
            //cups: cc,
        })
    }

    fn wrap(&self, i: i8) -> i8 {
        let range = self.max - self.min + 1;
        let mut modulo = (i - self.min) % range;
        if modulo < 0 {
            modulo += range
        }
        let res = modulo + self.min;
        res
    }
}

fn cup_right_of(cup: i8, state: &VecDeque<i8>) -> i8 {
    let loc = state.iter().position(|v| *v == cup).unwrap();
    let next_idx = (loc + 1) % state.len();
    *state.get(next_idx).unwrap()
}

// not very efficient
fn as_char(cup: i8) -> char {
    let s = format!("{}", cup);
    s.chars().next().unwrap()
}

pub fn run_part1() -> Result<()> {
    let input = "963275481";
    //let input = "389125467"; // test input

    let mut state: VecDeque<i8> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    println!("State: {:?}", state);

    let context = Context::create(state.iter().copied())?;

    let mut current_cup = *state.iter().next().unwrap();
    for round in 0..100 {
        // println!("round {} current cup: {} ---------------------", round, current_cup);

        let start_pos = state.iter().position(|v| *v == current_cup).unwrap();
        state.rotate_left(start_pos + 1);
        // println!("State: {:?}", state);

        let mut taken: VecDeque<_> = state.drain(0..3).collect();
        // println!("taken: {:?}", taken);
        // println!("state: {:?}", state);

        let destination_cup = (1..)
            .map(|i| context.wrap(current_cup - i))
            .find(|v| state.contains(v))
            .expect("could not find destination cup");
        // println!("destination cup: {}", destination_cup);

        let dest_pos = state.iter().position(|v| *v == destination_cup).unwrap();
        state.rotate_left(dest_pos + 1);
        // println!("state: {:?}", state);
        state.append(&mut taken);
        // println!("state: {:?} (appended)", state);

        let next_current_cup = cup_right_of(current_cup, &state);
        current_cup = next_current_cup;

        println!("round {} state {:?} next {}", round, state, current_cup);
    }
    println!("final current cup: {}", current_cup);
    let pos1 = state.iter().position(|v| *v == 1).unwrap();
    state.rotate_left(pos1);
    state.pop_front();
    println!("final state: {:?}", state);
    let final_state_string: String = state.iter().map(|cup| as_char(*cup)).collect();
    println!("final state string: {:?}", final_state_string);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1::Context;

    #[test]
    fn wrap_correct() {
        let context = Context::create([2, 3, 4, 5].iter().copied()).unwrap();
        assert_eq!(2, context.wrap(2));
        assert_eq!(4, context.wrap(4));
        assert_eq!(5, context.wrap(5));

        assert_eq!(5, context.wrap(1));
        assert_eq!(4, context.wrap(0));
        assert_eq!(3, context.wrap(-1));
        assert_eq!(2, context.wrap(-2));
        assert_eq!(5, context.wrap(-3));
    }
}
