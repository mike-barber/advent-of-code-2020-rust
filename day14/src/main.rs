use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, bail, Result};
use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"mask = ([01X]+)").unwrap();
}

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

// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// for part 2,
// 0 -> unchanged
// 1 -> set to 1
// X -> floating, permitted to take all values
#[derive(Debug, Clone)]
struct MaskAddress {
    set_mask: u64,
    floating_bits: Vec<u8>,
}

impl MaskAddress {
    fn addresses_iter(&self, address: u64) -> MaskAddressIterator {
        MaskAddressIterator {
            floating_bit_values: 0u64,
            floating_bit_last: (1u64 << self.floating_bits.len()) - 1,
            completed: false,
            mask_address: &self,
            address,
        }
    }
}
impl FromStr for MaskAddress {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_MASK.captures(s).ok_or(anyhow!("invalid format"))?;
        let mut set_mask = 0u64;
        let mut floating_bits = Vec::new();
        for (i, c) in captures[1].chars().enumerate() {
            let loc = 35_u64 - i as u64;
            match c {
                'X' => floating_bits.push(loc as u8),
                '1' => set_mask |= 1u64 << loc,
                '0' => (),
                _ => return Err(anyhow!("Invalid bit spec: {}", c)),
            };
        }
        // ensure floating bit addresses are sorted smallest to largest
        floating_bits.sort();
        Ok(MaskAddress {
            set_mask,
            floating_bits,
        })
    }
}

struct MaskAddressIterator<'a> {
    floating_bit_values: u64,
    floating_bit_last: u64,
    completed: bool,
    mask_address: &'a MaskAddress,
    address: u64,
}
impl<'a> Iterator for MaskAddressIterator<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        // take address, apply set mask
        let mut addr = self.address;
        addr |= self.mask_address.set_mask;

        // now extract bits from floating_bit_values and place them in the address
        let mut shift_extract = self.floating_bit_values;
        for loc in self.mask_address.floating_bits.iter() {
            // reset bit in address, then extract and apply our bit
            addr &= !(1u64 << loc);
            addr |= (shift_extract & 1) << loc;
            // shift right, ditching last bit
            shift_extract >>= 1;
        }

        // increment out counter
        if self.floating_bit_values == self.floating_bit_last {
            self.completed = true;
        } else {
            self.floating_bit_values += 1;
        }

        Some(addr)
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
    let lines = {
        let buffered = BufReader::new(File::open("day14/input.txt")?);
        let mut lines = Vec::new();
        for l in buffered.lines() {
            lines.push(l?);
        }
        lines
    };

    // part 1
    {
        println!("Part 1 ------");
        let mut mask: Mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
            .parse()
            .unwrap();
        let mut memory: HashMap<u64, u64> = HashMap::new();
        for l in lines.iter() {
            if l.contains("mask") {
                mask = l.parse()?;
            //println!("Mask: {}", &mask);
            } else if l.contains("mem") {
                let instruction: Instruction = l.parse()?;
                let value_masked = mask.apply(instruction.value);
                memory.insert(instruction.addr, value_masked);
            //println!("Instruction: {:?}", instruction);
            //println!("Memory {:?}", memory);
            } else {
                bail!("Invalid instruction");
            }
        }
        let memory_sum: u64 = memory.values().sum();
        println!("Sum of memory: {}", memory_sum);
    }

    // part 2
    {
        println!("Part 2 ------");
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut mask: Option<MaskAddress> = None;
        for l in lines.iter() {
            if l.contains("mask") {
                mask = Some(l.parse()?);
            } else if l.contains("mem") {
                let instruction: Instruction = l.parse()?;
                let addr_mask = mask.as_ref().ok_or(anyhow!("mask not set yet"))?;
                for addr in addr_mask.addresses_iter(instruction.addr) {
                    memory.insert(addr, instruction.value);
                }
            }
        }
        println!("Set {} total memory addresses", memory.len());
        let memory_sum: u64 = memory.values().sum();
        println!("Sum of memory: {}", memory_sum);
    }

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

    #[test]
    fn mask_address_iterate() {
        //let mask: MaskAddress = "mask = 000000000000000000000000000XX0010XXX".parse()?;
        let mask: MaskAddress = "mask = 000000000000000000000000000000000XXX"
            .parse()
            .unwrap();
        // all floating, should replace all of these with iterated values
        itertools::assert_equal(0u64..=0b111, mask.addresses_iter(0b000));
        itertools::assert_equal(0u64..=0b111, mask.addresses_iter(0b101));
        itertools::assert_equal(0u64..=0b111, mask.addresses_iter(0b111));
    }

    #[test]
    fn mask_address_set_and_iterate() {
        let mask: MaskAddress = "mask = 00000000000000000000000000010000000X"
            .parse()
            .unwrap();
        // all floating, should replace all of these with iterated values
        let addr = 0xF000;
        itertools::assert_equal(
            [addr + 256, addr + 256 + 1].iter().copied(),
            mask.addresses_iter(addr),
        );
    }
}
