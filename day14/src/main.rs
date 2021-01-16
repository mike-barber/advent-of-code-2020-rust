use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, bail, Result};
use regex::Regex;

// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
#[derive(Debug, Clone, Copy)]
struct Mask {
    set: u64,
    reset: u64,
}
impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & !self.reset | self.set
    }
}
lazy_static::lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"mask = ([01X]+)").unwrap();
}
impl FromStr for Mask {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_MASK.captures(s).ok_or(anyhow!("invalid format"))?;
        let mut set = 0u64;
        let mut reset = 0u64;
        for (i, c) in captures[1].chars().enumerate() {
            let loc = 35_u64 - i as u64;
            match c {
                'X' => {}
                '1' => {
                    set |= 1u64 << loc;
                }
                '0' => {
                    reset |= 1u64 << loc;
                }
                _ => return Err(anyhow!("Invalid bit spec: {}", c)),
            };
        }
        Ok(Mask { set, reset })
    }
}
impl Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mask set: {:b}, reset: {:b}", self.set, self.reset)
    }
}

// mem[8] = 11
#[derive(Debug, Copy, Clone)]
struct Instruction {
    addr: u64,
    value: u64,
}
lazy_static::lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}
impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_INSTRUCTION
            .captures(s)
            .ok_or(anyhow!("invalid format"))?;
        Ok(Instruction {
            addr: captures[1].parse()?,
            value: captures[2].parse()?,
        })
    }
}

fn main() -> Result<()> {
    let buffered = BufReader::new(File::open("day14/input.txt")?);

    let mut mask: Mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        .parse()
        .unwrap();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for lr in buffered.lines() {
        let l = lr?;
        if l.contains("mask") {
            mask = l.parse()?;
            println!("Mask: {}", &mask);
        } else if l.contains("mem") {
            let instruction: Instruction = l.parse()?;
            let value_masked = mask.apply(instruction.value);
            memory.insert(instruction.addr, value_masked);
            println!("Instruction: {:?}", instruction);
            println!("Memory {:?}", memory);
        } else {
            bail!("Invalid instruction");
        }
    }

    let memory_sum: u64 = memory.values().sum();
    println!("Sum of memory: {}", memory_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn mask_parse() {
        let mask: Mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse()
            .unwrap();
        assert_eq!(0b01000000, mask.set);
        assert_eq!(0b00000010, mask.reset);
    }

    #[test]
    fn mask_apply() {
        let mask: Mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse()
            .unwrap();
        assert_eq!(73, mask.apply(11));
        assert_eq!(101, mask.apply(101));
        assert_eq!(64, mask.apply(0));
    }

    #[test]
    fn instruction_parse() {
        let inst: Instruction = "mem[8] = 11".parse().unwrap();
        assert_eq!(8, inst.addr);
        assert_eq!(11, inst.value);
    }
}
