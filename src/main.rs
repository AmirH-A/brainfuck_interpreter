use std::io::{self, Read, Write};

fn execute(program: &str) {
    let mut tape: Vec<u8> = vec![0; 30000];
    let mut ptr: usize = 0;
    let mut pc: usize = 0;
    let program: Vec<char> = program.chars().collect();
    let mut jump_stack: Vec<usize> = Vec::new();

    while pc < program.len() {
        match program[pc] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                let mut input = [0];
                io::stdin().read_exact(&mut input).unwrap();
                tape[ptr] = input[0];
            }
            '[' => {
                if tape[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        pc += 1;
                        if program[pc] == '[' {
                            depth += 1;
                        } else if program[pc] == ']' {
                            depth -= 1;
                        }
                    }
                } else {
                    jump_stack.push(pc);
                }
            }
            ']' => {
                if let Some(jump_back) = jump_stack.pop() {
                    if tape[ptr] != 0 {
                        pc = jump_back;
                        continue;
                    }
                }
            }
            _ => {}
        }
        pc += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: bf <program>");
        return;
    }
    let program = &args[1];
    execute(program);
}
