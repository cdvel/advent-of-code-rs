use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day9.txt").expect("Failed to read input file");
    let numbers: Vec<u64> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();

    let preamble_size = 25;
    let invalid_number = find_first_invalid_number(&numbers, preamble_size)
        .expect("Failed to find an invalid number");
    
    println!("p1: The first invalid number is {}", invalid_number);


    let mut contiguous_set = Vec::new();
    let mut current_sum = 0;

    for num in numbers {
        contiguous_set.push(num);
        current_sum += num;

        while current_sum > invalid_number {
            current_sum -= contiguous_set.remove(0);
        }

        if current_sum == invalid_number {
            let min = contiguous_set.iter().min().unwrap();
            let max = contiguous_set.iter().max().unwrap();
            let sum = min + max;
            println!("p2: The sum of the smallest and largest numbers in the contiguous set is {}", sum);
            break;
        }
    }
    
}

fn find_first_invalid_number(numbers: &[u64], preamble_size: usize) -> Option<u64> {
    for i in preamble_size..numbers.len() {
        let current = numbers[i];
        let window = &numbers[i - preamble_size..i];
        
        if !is_valid_number(current, window) {
            return Some(current);
        }
    }
    None
}

fn is_valid_number(number: u64, previous_numbers: &[u64]) -> bool {
    let mut seen = HashSet::new();
    
    for &prev in previous_numbers {
        // numbers must be different and summation equals target
        if prev < number && seen.contains(&(number - prev)) {
            return true;
        }
        seen.insert(prev);
    }
    
    false
}
