use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone)]
enum Instruction {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = s.split_at(1);
        let val: i32 = value.parse().map_err(|_| "unable to parse value")?;
        match instruction {
            "N" => Ok(Self::N(val)),
            "E" => Ok(Self::E(val)),
            "S" => Ok(Self::S(val)),
            "W" => Ok(Self::W(val)),
            "L" => Ok(Self::L(val)),
            "R" => Ok(Self::R(val)),
            "F" => Ok(Self::F(val)),
            _ => Err(format!("unknown instruction: {}", instruction)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord(i32, i32);

impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Mul<i32> for Coord {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

impl Coord {
    fn left_one(&self) -> Self {
        Coord(-self.1, self.0)
    }
    fn right_one(&self) -> Self {
        Coord(self.1, -self.0)
    }
    fn rotate_op<F>(&self, degrees: i32, op: F) -> Self
    where
        F: Fn(&Coord) -> Coord,
    {
        let num = degrees / 90;
        let mut c = *self;
        for _ in 0..num {
            c = op(&c);
        }
        c
    }
    fn left(&self, degrees: i32) -> Self {
        self.rotate_op(degrees, Self::left_one)
    }
    fn right(&self, degrees: i32) -> Self {
        self.rotate_op(degrees, Self::right_one)
    }
}

#[derive(Debug, Clone)]
struct State {
    location: Coord,
    direction: Coord,
}

impl State {
    fn apply_instruction(&self, instruction: &Instruction) -> Self {
        match *instruction {
            Instruction::N(v) => State {
                location: self.location + Coord(0, v),
                ..*self
            },
            Instruction::E(v) => State {
                location: self.location + Coord(v, 0),
                ..*self
            },
            Instruction::S(v) => State {
                location: self.location + Coord(0, -v),
                ..*self
            },
            Instruction::W(v) => State {
                location: self.location + Coord(-v, 0),
                ..*self
            },
            Instruction::L(v) => State {
                direction: self.direction.left(v),
                ..*self
            },
            Instruction::R(v) => State {
                direction: self.direction.right(v),
                ..*self
            },
            Instruction::F(v) => State {
                location: self.location + self.direction * v,
                ..*self
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("day12/input.txt")?);
    let instructions: Result<Vec<_>, _> = buffered
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Instruction>())
        .collect();
    let instructions = instructions?;

    println!("Instructions: {:?}", &instructions);

    let initial_state = State {
        location: Coord(0, 0),
        direction: Coord(1, 0),
    };
    let result = instructions.iter().fold(initial_state, |state,i| state.apply_instruction(i));
    println!("Result: {:?}", result);
    println!("Manhattan distance: {:?}", result.location.0.abs() + result.location.1.abs());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_left() {
        let coord = Coord(2, 1);
        assert_eq!(Coord(-1, 2), coord.left_one());
        assert_eq!(Coord(-2, -1), coord.left_one().left_one());
        assert_eq!(Coord(1, -2), coord.left_one().left_one().left_one());
        assert_eq!(coord, coord.left_one().left_one().left_one().left_one());

        assert_eq!(Coord(-1, 2), coord.left(90));
        assert_eq!(Coord(-2, -1), coord.left(180));
        assert_eq!(Coord(1, -2), coord.left(270));
    }

    #[test]
    fn coord_right() {
        let coord = Coord(2, 1);
        assert_eq!(Coord(1, -2), coord.right_one());
        assert_eq!(Coord(-2, -1), coord.right_one().right_one());
        assert_eq!(Coord(-1, 2), coord.right_one().right_one().right_one());
        assert_eq!(coord, coord.right_one().right_one().right_one().right_one());

        assert_eq!(Coord(1, -2), coord.right(90));
        assert_eq!(Coord(-2, -1), coord.right(180));
        assert_eq!(Coord(-1, 2), coord.right(270));
    }
}
