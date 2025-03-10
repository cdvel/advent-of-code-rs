use std::collections::HashSet;
use std::fs;

fn main() {
    // Read adapters from file and sort them
    let adapters = read_input_file("input/day10.txt");

    // Count differences between consecutive adapters
    let mut differences = [0, 0, 0]; // Index 0, 1, 2 represents differences of 1, 2, 3 jolts

    for i in 0..adapters.len() - 1 {
        let diff = adapters[i + 1] - adapters[i];
        add_difference(diff, &mut differences);
    }

    // Calculate result - multiply the count of 1-jolt differences by the count of 3-jolt differences
    let result = differences[0] * differences[2];
    println!(
        "The product of 1-jolt and 3-jolt differences is: {}",
        result
    );
}

fn read_input_file(file_path: &str) -> Vec<i32> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");

    let mut adapters: Vec<i32> = content
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    adapters.push(0);
    adapters.sort();

    let max_adapter = adapters[adapters.len() - 1];
    adapters.push(max_adapter + 3);
    adapters.sort();
    adapters
}

fn add_difference(diff: i32, differences: &mut [i32; 3]) {
    if diff >= 1 && diff <= 3 {
        differences[diff as usize - 1] += 1;
    }
}
