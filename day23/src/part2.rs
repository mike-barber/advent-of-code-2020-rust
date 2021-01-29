use eyre::{eyre, Result};
use itertools::chain;
use std::{collections::HashMap, fmt::{format, Display}, todo, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FoundPos {
    Left(usize),
    Right(usize),
}

#[derive(Debug)]
struct CircularVec(Vec<i32>);

impl CircularVec {
    fn create(values: &[i32]) -> Self {
        CircularVec(values.iter().copied().collect())
    }
    fn create_vec(values: Vec<i32>) -> Self {
        CircularVec(values)
    }

    // rotate the vector so that the ref_pos is in the middle; returns new ref_pos
    fn conditional_rotate_centre(&mut self, ref_pos: usize, tolerance: usize) -> usize {
        let mid = self.0.len() / 2;
        if ref_pos > mid {
            if ref_pos - mid > tolerance {
                print!("*");
                self.0.rotate_left(ref_pos - mid);
                return mid;
            }
            
        } else if ref_pos < mid {
            if mid - ref_pos > tolerance {
                print!("*");
                self.0.rotate_right(mid - ref_pos);
                return mid;
            }
        }
        ref_pos // unchanged
    }

    // scan left and right from current position to find value
    fn find_value_pos(&self, start_pos: usize, value: i32) -> Option<FoundPos> {
        for i in 1.. {
            let left_idx = start_pos.checked_sub(i);
            let right_idx = start_pos.checked_add(i);

            let left_opt = left_idx.map(|j| self.0.get(j)).flatten();
            let right_opt = right_idx.map(|j| self.0.get(j)).flatten();

            if let Some(left) = left_opt {
                if *left == value {
                    return Some(FoundPos::Left(left_idx.unwrap()));
                }
            }

            if let Some(right) = right_opt {
                if *right == value {
                    return Some(FoundPos::Right(right_idx.unwrap()));
                }
            }

            if left_opt.is_none() && right_opt.is_none() {
                break;
            }
        }
        None
    }

    fn wrapped(&self, idx: usize) -> usize {
        idx % self.0.len()
    }

    fn get_wrapped(&self, idx: usize) -> Option<&i32> {
        let iw = self.wrapped(idx);
        self.0.get(iw)
    }

    fn get_mut_wrapped(&mut self, idx: usize) -> Option<&mut i32> {
        let iw = self.wrapped(idx);
        self.0.get_mut(iw)
    }

    fn copy_to_buffer(&self, buffer: &mut [i32], start: usize) {
        for i in 0..buffer.len() {
            buffer[i] = *self.get_wrapped(start + i).expect("copy invalid index");
        }
    }

    fn insert_buffer_after(&mut self, buffer: &[i32], from_pos: usize, after_pos: FoundPos) {
        match after_pos {
            FoundPos::Left(after) => {
                let shifts = from_pos - after - 1;
                let dest_end = from_pos + buffer.len() - 1;
                for i in 0..shifts {
                    let dst = dest_end - i;
                    let src = dest_end - i - buffer.len();
                    let val = self.get_wrapped(src).unwrap();
                    *self.get_mut_wrapped(dst).unwrap() = *val;
                }
                for i in 0..buffer.len() {
                    let dst = after + 1 + i;
                    *self.get_mut_wrapped(dst).unwrap() = buffer[i];
                }
            }
            FoundPos::Right(after) => {
                let shifts = after - from_pos - 3 + 1;
                for i in 0..shifts {
                    let dst = from_pos + i;
                    let src = from_pos + i + buffer.len();
                    let val = self.get_wrapped(src).unwrap();
                    *self.get_mut_wrapped(dst).unwrap() = *val;
                }
                for i in 0..buffer.len() {
                    let dst = from_pos + shifts + i;
                    *self.get_mut_wrapped(dst).unwrap() = buffer[i];
                }
            }
        }
    }
}

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
            // add to value map (and ensure it is unique)
            assert!(value_map.insert(v, addr).is_none());
            // update prev_addr to current
            prev_addr = Some(addr);    
        }
        // link end node back to first and return list
        nodes.last_mut().unwrap().next_addr = Some(0);
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

    fn next_addr(&self, node: &Node) -> Option<usize> {
        node.next_addr
    }

    fn prev(&self, node: &Node) -> Option<usize> {
        node.prev_addr
    }

    fn find_value(&self, value:i32) -> Option<usize> {
        self.value_map.get(&value).copied()
    }

    fn move_chain(&mut self, chain_start: usize, chain_length: usize, attach_after: usize) {
        // walk along chain to find end address
        let chain_end = (0..chain_length).fold(chain_start, |acc,_| self.get(acc).unwrap().next_addr.unwrap());

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



const BUF_SIZE: usize = 3;
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
            buffer: vec![0; BUF_SIZE]
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
        use std::time::Instant;

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
        self.state.move_chain(chain_start, BUF_SIZE, dest_cup_addr);

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
        let values = self.state.copy_values(&mut vals, addr);
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

pub fn test_part2() {
    let init = vec_from_chars("389125467");
    let mut game = Game::create_part2(&init, 1_000_000);

    println!("start -> game {}", game);
    for round in 0..1_000_000 {
        game.play_round();
        if round % 10_000 == 0 {
            println!("round {} game {}", round, game);
        }
    }
    println!("final game {}", game);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create() -> CircularVec {
        CircularVec::create(&[7, 2, 5, 8, 9, 1, 3, 4, 6]) // 9 total
    }

    fn assert_circular_eq(expected: &[i32], actual: &[i32], message: &str) {
        let expected = CircularVec::create(expected);
        let actual = CircularVec::create(actual);
        assert_eq!(expected.0.len(), actual.0.len(), "length");
        let len = expected.0.len();
        let mut any_match = false;
        for offset in 0..len {
            if (0..len)
                .into_iter()
                .all(|i| expected.get_wrapped(i) == actual.get_wrapped(i + offset))
            {
                any_match = true;
                break;
            }
        }
        assert!(
            any_match,
            "{} no match found {:?} vs {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn find_correct() {
        let cv = create();
        let start_pos = 5; // = 1

        assert_eq!(None, cv.find_value_pos(start_pos, 1000));
        assert_eq!(Some(FoundPos::Left(0)), cv.find_value_pos(start_pos, 7));
        assert_eq!(Some(FoundPos::Left(4)), cv.find_value_pos(start_pos, 9));
        assert_eq!(Some(FoundPos::Right(6)), cv.find_value_pos(start_pos, 3));
        assert_eq!(Some(FoundPos::Right(8)), cv.find_value_pos(start_pos, 6));
    }

    #[test]
    fn moves_left_correct() {
        // starting from position [4] == 9(,1,3)
        let from_pos = 4;
        let cases = [
            (7, vec![7, 9, 1, 3, 2, 5, 8, 4, 6]),
            (2, vec![7, 2, 9, 1, 3, 5, 8, 4, 6]),
            (5, vec![7, 2, 5, 9, 1, 3, 8, 4, 6]),
        ];

        for (after_value, expected) in cases.iter() {
            // arrange
            let mut cv = create();
            assert_eq!(*cv.get_wrapped(from_pos).unwrap(), 9);
            let after_loc = cv.find_value_pos(from_pos, *after_value).unwrap();

            // act
            let buffer = [9, 1, 3];
            cv.insert_buffer_after(&buffer, from_pos, after_loc);

            // assert
            assert_eq!(*expected, cv.0);
        }
    }

    #[test]
    fn moves_right_correct() {
        // starting from position [4] == 9(,1,3)
        let from_pos = 4;
        let cases = [
            (4, vec![7, 2, 5, 8, 4, 9, 1, 3, 6]),
            (6, vec![7, 2, 5, 8, 4, 6, 9, 1, 3]),
        ];

        for (after_value, expected) in cases.iter() {
            // arrange
            let mut cv = create();
            assert_eq!(*cv.get_wrapped(from_pos).unwrap(), 9);
            let after_loc = cv.find_value_pos(from_pos, *after_value).unwrap();

            // act
            let buffer = [9, 1, 3];
            cv.insert_buffer_after(&buffer, from_pos, after_loc);

            // assert
            assert_eq!(*expected, cv.0);
        }
    }

    #[test]
    fn moves_left_correct_wrap() {
        // starting from position [7] == 4(,6,7)
        let from_pos = 7;
        let cases = [
            (2, vec![3, 2, 4, 6, 7, 5, 8, 9, 1]),
            (5, vec![3, 2, 5, 4, 6, 7, 8, 9, 1]),
        ];

        for (after_value, expected) in cases.iter() {
            // arrange
            let mut cv = create();
            assert_eq!(*cv.get_wrapped(from_pos).unwrap(), 4);
            let after_loc = cv.find_value_pos(from_pos, *after_value).unwrap();

            // act
            let buffer = [4, 6, 7];
            cv.insert_buffer_after(&buffer, from_pos, after_loc);

            // assert
            assert_eq!(*expected, cv.0);
        }
    }

    #[test]
    fn copy_to_buffer_correct() {
        // arrange
        let from_pos = 7;
        let cv = create();

        // act
        let mut buffer = vec![0; 3];

        // assert
        cv.copy_to_buffer(&mut buffer, from_pos);

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

        let mut buffer = [0; BUF_SIZE];
        for (i, expected) in expected_sequence.iter().enumerate() {
            let round = i + 1;
            game.play_round(&mut buffer);
            //assert_eq!(expected.to_vec(), game.state.0, "checking round {}", round);
            assert_circular_eq(
                expected,
                &game.state.0,
                &format!("checking round {}", round),
            );
        }
    }
}
