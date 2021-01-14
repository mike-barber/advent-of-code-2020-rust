use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
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
                "nop" => Ok(Instruction::Nop(arg.parse()?)),
                "acc" => Ok(Instruction::Acc(arg.parse()?)),
                "jmp" => Ok(Instruction::Jmp(arg.parse()?)),
                _ => Err("unrecognised instruction"),
            };
            Ok(instruction?)
        })
        .collect();

    let instructions = instructions?;

    println!("Instructions: {:?}", instructions);

    // part 1 -- find completed state for example program
    let part1_state = run_program(&instructions);
    println!("Part 1 -> Completed with state {:?}", part1_state);

    // part 2 -- find the mutated program for which the terminal state is pc == instructions.len
    //           i.e. the next instruction after the end of the program
    for mutate_index in 0..instructions.len() {
        let original_instruction = &instructions[mutate_index];
        let mut mutated = instructions.clone();
        mutated[mutate_index] = match original_instruction {
            Instruction::Nop(x) => Instruction::Jmp(*x),
            Instruction::Jmp(x) => Instruction::Nop(*x),
            Instruction::Acc(x) => Instruction::Acc(*x),
        };

        // run the mutated program
        let terminal_state = run_program(&mutated);
        if terminal_state.pc == instructions.len() {
            println!(
                "Found a working program. Terminated with {:?}",
                terminal_state
            );
            break;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct State {
    pc: usize,
    acc: i32,
}
impl State {
    fn next(&self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Nop(_) => State {
                pc: self.pc + 1,
                ..*self
            },
            Instruction::Acc(x) => State {
                pc: self.pc + 1,
                acc: self.acc + x,
            },
            Instruction::Jmp(x) => State {
                pc: (self.pc as i32 + x) as usize,
                ..*self
            },
        }
    }
}

fn run_program(instructions: &[Instruction]) -> State {
    let mut visited = vec![false; instructions.len()];
    let mut state = State { pc: 0, acc: 0 };

    while state.pc < instructions.len() && !visited[state.pc] {
        let instruction = &instructions[state.pc];
        visited[state.pc] = true;
        //println!("state {:?} next instruction {:?}", state, instruction);
        state = state.next(instruction);
    }

    state
}
