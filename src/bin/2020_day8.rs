use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    operation: String,
    argument: i32,
}

fn run_program(instructions: &[Instruction]) -> (bool, i32) {
    let mut accumulator = 0;
    let mut position = 0;
    let mut visited = HashSet::new();
    
    while position < instructions.len() && !visited.contains(&position) {
        visited.insert(position);
        let instruction = &instructions[position];
        
        match instruction.operation.as_str() {
            "acc" => {
                accumulator += instruction.argument;
                position += 1;
            },
            "jmp" => position = (position as i32 + instruction.argument) as usize,
            "nop" => position += 1,
            _ => panic!("Unknown operation: {}", instruction.operation),
        }
    }
    
    // Return whether program terminated successfully and final accumulator value
    (position == instructions.len(), accumulator)
}

fn main() {
    let file = File::open("input/day8.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);
    
    let mut instructions: Vec<Instruction> = Vec::new();
    
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        instructions.push(Instruction {
            operation: parts[0].to_string(),
            argument: parts[1].parse().expect("Failed to parse argument"),
        });
    }
    
    // p1, find accumulator before loop
    let (_, acc) = run_program(&instructions);
    println!("Part 1 - Value in accumulator before loop: {}", acc);
    
    // p2, try changing each jmp/nop instruction
    for i in 0..instructions.len() {
        let original_op = instructions[i].operation.clone();
        
        // Only try switching jmp and nop instructions
        match original_op.as_str() {
            "jmp" => instructions[i].operation = "nop".to_string(),
            "nop" => instructions[i].operation = "jmp".to_string(),
            _ => continue,
        }
        
        // Test if fixed program terminates
        let (terminates, acc) = run_program(&instructions);
        if terminates {
            println!("Part 2 - Value in accumulator after fixing program: {}", acc);
            break;
        }
        
        // undo
        instructions[i].operation = original_op;
    }
}

