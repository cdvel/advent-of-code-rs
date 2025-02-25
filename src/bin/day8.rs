use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    operation: String,
    argument: i32,
}

fn main() {
    let file = File::open("input/day8.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut instructions: Vec<Instruction> = Vec::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        instructions.push(Instruction {
            operation: parts[0].to_string(),
            argument: parts[1].parse().unwrap(),
        });
    }
    
    let mut accumulator = 0;
    let mut position = 0;
    let mut visited = HashSet::new();
    
    while !visited.contains(&position) {
        visited.insert(position);
        let instruction = &instructions[position];
        
        match instruction.operation.as_str() {
            "acc" => {
                accumulator += instruction.argument;
                position += 1;
            },
            "jmp" => {
                position = (position as i32 + instruction.argument) as usize;
            },
            "nop" => {
                position += 1;
            },
            _ => panic!("Unknown operation: {}", instruction.operation),
        }
    }
    
    println!("Value in accumulator before loop: {}", accumulator);
}
