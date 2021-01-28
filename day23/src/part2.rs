use eyre::{eyre, Result};
use std::fmt::format;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum FoundPos {
    Left(usize),
    Right(usize)
}

#[derive(Debug)]
struct CircularVec(Vec<i32>);

impl CircularVec {
    fn create(values: &[i32]) -> Self {
        CircularVec(values.iter().copied().collect())
    }

    // scan left and right from current position to find value
    fn find_value_pos(&self, start_pos: usize, value:i32) -> Option<FoundPos> {
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

    fn copy_to_buffer(&self, buffer: &mut[i32], start: usize) {
        for i in 0..buffer.len() {
            buffer[i] = *self.get_wrapped(start+i).expect("copy invalid index");
        }
    }

    fn insert_buffer_after(&mut self, buffer: &[i32], from_pos:usize, after_pos:FoundPos) {
        match after_pos {
            FoundPos::Left(after) => {
                let distance = from_pos - after;
                let moves = distance - buffer.len();
                let dest_start = from_pos + buffer.len();
                for i in 0..moves {
                    let dst = dest_start - i;
                    let src = dest_start - i - distance;
                    let val = self.get_wrapped(src).unwrap();
                    *self.get_mut_wrapped(dst).unwrap() = *val;
                }
            },
            FoundPos::Right(after) => {
            }
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::{CircularVec, FoundPos};


    fn create() -> CircularVec {
        CircularVec::create(&[7,2,5,8,9,1,3,4,6]) // 9 total
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

    }
}



