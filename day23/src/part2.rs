use eyre::{eyre, Result};
use std::{fmt::format, todo};

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

#[cfg(test)]
mod tests {
    use super::{CircularVec, FoundPos};

    fn create() -> CircularVec {
        CircularVec::create(&[7, 2, 5, 8, 9, 1, 3, 4, 6]) // 9 total
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
}
