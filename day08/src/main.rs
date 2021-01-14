use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("input.txt")?);

    let instructions: Result<Vec<Instruction>, Box<dyn Error>> = buffered
        .lines()
        .map(|lr| {
            let line = lr?;
            let mut split = line.split(" ");
            let inst = split.next().ok_or("missing instruction")?;
            let arg = split.next().ok_or("missing instruction argument")?;

            let instruction = match inst {
                "nop" => Ok(Instruction::Nop),
                "acc" => Ok(Instruction::Acc(arg.parse()?)),
                "jmp" => Ok(Instruction::Jmp(arg.parse()?)),
                _ => Err("unrecognised instruction"),
            };
            Ok(instruction?)
        })
        .collect();

    println!("Instructions: {:?}", instructions);

    // part 1
    run_program_stop_on_repeat(&instructions?);

    Ok(())
}

fn run_program_stop_on_repeat(instructions: &[Instruction]) {
    #[derive(Debug)]
    struct State {
        pc: i32,
        acc: i32,
    };

    let mut visited = vec![false; instructions.len()];
    let mut state = State { pc: 0, acc: 0 };

    while !visited[state.pc as usize] {
        let instruction = &instructions[state.pc as usize];
        visited[state.pc as usize] = true;
        println!("state {:?} next instruction {:?}", state, instruction);
        state = match instruction {
            Instruction::Nop => State {
                pc: state.pc + 1,
                ..state
            },
            Instruction::Acc(x) => State {
                pc: state.pc + 1,
                acc: state.acc + x,
            },
            Instruction::Jmp(x) => State {
                pc: state.pc + x,
                ..state
            },
        };
    }

    println!("Completed with state {:?}", state);
}
