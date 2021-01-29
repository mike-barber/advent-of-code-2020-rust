use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Node {
    addr: usize, 
    next_addr: Option<usize>,
    prev_addr: Option<usize>,
    value: i32,
}

struct CircularList {
    nodes: Vec<Node>,
    value_map: HashMap<i32, usize>
}

impl CircularList {
    fn create(values: &[i32]) -> Self {
        let mut nodes = Vec::new();
        let mut value_map = HashMap::new();
        let mut prev_addr = None;
        for &v in values {
            // add node
            let addr = nodes.len(); // next 
            let node = Node {
                addr,
                value: v,
                next_addr: None,
                prev_addr: prev_addr.clone(),
            };
            nodes.push(node);
            // link prev node to this
            if let Some(p) = prev_addr {
                nodes[p].next_addr = Some(addr);
            }
            // add to value map (and ensure it is unique)
            assert!(value_map.insert(v, addr).is_none());
            // update prev_addr to current
            prev_addr = Some(addr);
        }
        // link end node back to first and return list
        nodes.last_mut().unwrap().next_addr = Some(0);
        nodes.first_mut().unwrap().prev_addr = Some(nodes.last().unwrap().addr);
        CircularList {
            nodes,
            value_map
        }
    }

    fn get(&self, addr: usize) -> Option<&Node> {
        self.nodes.get(addr)
    }

    fn get_mut(&mut self, addr: usize) -> Option<&mut Node> {
        self.nodes.get_mut(addr)
    }

    fn find_value(&self, value:i32) -> Option<usize> {
        self.value_map.get(&value).copied()
    }

    fn move_chain(&mut self, chain_start: usize, chain_length: usize, attach_after: usize) {
        // walk along chain to find end address
        let chain_end = (0..chain_length-1).fold(chain_start, |acc,_| self.get(acc).unwrap().next_addr.unwrap());

        // remove chain and rejoin
        {
            let prev = self.get(chain_start).unwrap().prev_addr.unwrap();
            let next = self.get(chain_end).unwrap().next_addr.unwrap();
            self.get_mut(prev).unwrap().next_addr = Some(next);
            self.get_mut(next).unwrap().prev_addr = Some(prev);
        }

        // splice chain in again
        {
            let prev = attach_after;
            let next = self.get(attach_after).unwrap().next_addr.unwrap();
            self.get_mut(prev).unwrap().next_addr = Some(chain_start);
            self.get_mut(next).unwrap().prev_addr = Some(chain_end);
            self.get_mut(chain_start).unwrap().prev_addr = Some(prev);
            self.get_mut(chain_end).unwrap().next_addr = Some(next);
        }
    }

    fn copy_values(&self, destination: &mut[i32], chain_start: usize) {
        let mut next_addr = chain_start;
        for d in destination.iter_mut() {
            let node = self.get(next_addr).unwrap();
            *d = node.value;
            next_addr = node.next_addr.unwrap();
        }
    }

    fn copy_all_values(&self, chain_start: usize) -> Vec<i32> {
        let mut res = vec![0; self.nodes.len()];
        self.copy_values(&mut res, chain_start);
        res
    }
}



const CHAIN_LENGTH: usize = 3;
struct Game {
    state: CircularList,
    current_value: i32,
    current_addr: usize,
    min: i32,
    max: i32,
    buffer: Vec<i32>
}

impl Game {
    fn create_internal(initial_vector: Vec<i32>) -> Game {
        let current_value = *initial_vector.get(0).unwrap();
        let min = *initial_vector.iter().min().unwrap();
        let max = *initial_vector.iter().max().unwrap();
        Game {
            state: CircularList::create(&initial_vector),
            current_addr: 0,
            current_value,
            min,
            max,
            buffer: vec![0; CHAIN_LENGTH]
        }
    }

    fn create_part1(initial_vector: &[i32]) -> Game {
        let v: Vec<i32> = initial_vector.iter().copied().collect();
        Self::create_internal(v)
    }

    // initialise for part 2 -- take the initial vector, then proceed by adding 1 until we
    // reach the total number of cups specified.
    fn create_part2(initial_vector: &[i32], total_cups: usize) -> Game {
        let remaining_start = initial_vector.iter().max().unwrap() + 1;
        let remaining_range = remaining_start..;

        let v: Vec<i32> = initial_vector
            .iter()
            .copied()
            .chain(remaining_range)
            .take(total_cups)
            .collect();

        Self::create_internal(v)
    }

    fn find_next_lowest_number(&self, start_number: i32, exclude: &[i32]) -> i32 {
        let mut num = start_number;
        loop {
            num -= 1;
            if num < self.min {
                num = self.max;
            }
            if !exclude.contains(&num) {
                return num;
            }
        }
    }

    fn play_round(&mut self) {
        //use std::time::Instant;

        // let t0 = Instant::now();

        // extract values in chain, starting at the next list location after the current node
        let chain_start = self.state.get(self.current_addr).unwrap().next_addr.unwrap();
        self.state.copy_values(&mut self.buffer, chain_start);

        // find destination cup value and position
        let dest_cup_value = self.find_next_lowest_number(self.current_value, &self.buffer);
        let dest_cup_addr = self
            .state
            .find_value(dest_cup_value)
            .expect("could not find destination cup position");

        // move chain from where it is to after the destination cup
        self.state.move_chain(chain_start, CHAIN_LENGTH, dest_cup_addr);

        // get the next cup after the current one (this might have changed after the move)
        let next_addr = self.state.get(self.current_addr).unwrap().next_addr.unwrap();
        let next_value = self.state.get(next_addr).unwrap().value;
        
        // store new state
        self.current_addr = next_addr;
        self.current_value = next_value;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // display after cup "1" by convention
        let addr = self.state.find_value(1).ok_or(std::fmt::Error)?;
        let mut vals = vec![0; 10];
        self.state.copy_values(&mut vals, addr);
        write!(f, "pos {} value {} state {:?}", self.current_addr, self.current_value, vals)
    }
}

pub fn vec_from_chars(s: &str) -> Vec<i32> {
    s.chars().map(|c| c.to_string().parse().unwrap()).collect()
}

pub fn test_part1() {
    let init = vec_from_chars("389125467");
    let mut game = Game::create_part1(&init);

    println!("start -> game {}", game);
    for round in 0..10 {
        game.play_round();
        println!("round {} game {}", round, game);
    }
    println!("Final result {:?}", game.state.copy_all_values(game.state.find_value(1).unwrap()))
}

fn run_part2(init_state: &str) -> i64 {
    let init = vec_from_chars(init_state);
    let mut game = Game::create_part2(&init, 1_000_000);

    println!("start -> game {}", game);
    for round in 0..10_000_000 {
        game.play_round();
        if round % 1_000_000 == 0 {
            println!("round {} game {}", round, game);
        }
    }
    println!("final game {}", game);

    let one_addr = game.state.find_value(1).unwrap();
    let mut res = vec![0;3];
    game.state.copy_values(&mut res, one_addr);

    println!("result: {:?}", res);
    let product = res[1] as i64 * res[2] as i64;
    println!("product of [1]*[2] => {}", product); 
    product
}

pub fn test_part2() {
    let product = run_part2("389125467");
    assert_eq!(149245887792, product);
}

pub fn actual_part2() {
    let product = run_part2("963275481");
    println!("final result {}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create() -> CircularList {
        CircularList::create(&[7, 2, 5, 8, 9, 1, 3, 4, 6]) // 9 total
    }

    fn assert_circular_eq(expected: &[i32], actual: &[i32], message: &str) {
        assert_eq!(expected.len(), actual.len(), "length");
        let actual_circular = CircularList::create(actual);
        let start_addr = actual_circular.find_value(expected[0]).unwrap();
        let matched = actual_circular.copy_all_values(start_addr);
        assert_eq!(expected, &matched, "element mismatch {}", message);
    }

    #[test]
    fn find_correct() {
        let cv = create();
        assert_eq!(None, cv.find_value(1000));
        assert_eq!(Some(0), cv.find_value(7));
        assert_eq!(Some(4), cv.find_value(9));
        assert_eq!(Some(6), cv.find_value(3));
        assert_eq!(Some(8), cv.find_value(6));
    }

    #[test]
    fn moves_left_correct() {
        // starting from position [4] == 9(,1,3)
        let from_addr = 4;
        let cases = [
            (7, vec![7, 9, 1, 3, 2, 5, 8, 4, 6]),
            (2, vec![7, 2, 9, 1, 3, 5, 8, 4, 6]),
            (5, vec![7, 2, 5, 9, 1, 3, 8, 4, 6]),
            (4, vec![7, 2, 5, 8, 4, 9, 1, 3, 6]),
            (6, vec![7, 2, 5, 8, 4, 6, 9, 1, 3]),
        ];

        for (after_value, expected) in cases.iter() {
            // arrange
            let mut cv = create();
            
            // act
            let after_addr = cv.find_value(*after_value).unwrap();
            cv.move_chain(from_addr, 3, after_addr);

            // assert
            assert_circular_eq(expected, &cv.copy_all_values(0), &format!("after {} expect {:?}", after_value, expected));
        }
    }
    
    #[test]
    fn copy_to_buffer_correct() {
        // arrange
        let from_addr = 7;
        let cv = create();

        // act
        let mut buffer = vec![0; 3];

        // assert
        cv.copy_values(&mut buffer, from_addr);

        assert_eq!(vec![4, 6, 7], buffer);
    }

    #[test]
    fn test_part1_expected() {
        let expected_sequence = [
            [3, 2, 8, 9, 1, 5, 4, 6, 7],
            [3, 2, 5, 4, 6, 7, 8, 9, 1],
            [7, 2, 5, 8, 9, 1, 3, 4, 6],
            [3, 2, 5, 8, 4, 6, 7, 9, 1],
            [9, 2, 5, 8, 4, 1, 3, 6, 7],
            [7, 2, 5, 8, 4, 1, 9, 3, 6],
            [8, 3, 6, 7, 4, 1, 9, 2, 5],
            [7, 4, 1, 5, 8, 3, 9, 2, 6],
            [5, 7, 4, 1, 8, 3, 9, 2, 6],
            [5, 8, 3, 7, 4, 1, 9, 2, 6],
        ];
        let init = vec_from_chars("389125467");
        let mut game = Game::create_part1(&init);

        for (i, expected) in expected_sequence.iter().enumerate() {
            let round = i + 1;
            game.play_round();
            //assert_eq!(expected.to_vec(), game.state.0, "checking round {}", round);
            assert_circular_eq(
                expected,
                &game.state.copy_all_values(0),
                &format!("checking round {}", round),
            );
        }
    }
}
