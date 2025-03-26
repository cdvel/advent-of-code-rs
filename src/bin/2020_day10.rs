use std::fs;

fn main() {
    // Read adapters from file and sort them
    let adapters = read_input_file_and_sort("input/day10.txt");
    let mut differences = [0, 0, 0]; // Index 0, 1, 2 represents differences of 1, 2, 3 jolts

    for i in 0..adapters.len() - 1 {
        let diff = adapters[i + 1] - adapters[i];
        add_difference(diff, &mut differences);
    }

    // multiply the count of 1-jolt differences by the count of 3-jolt differences
    let result = differences[0] * differences[2];
    println!(
        "The product of 1-jolt and 3-jolt differences is: {}",
        result
    );

    let sequences = count_sequences(&adapters);
    println!("Total number of distinct sequences: {}", sequences);
}

fn count_sequences(adapters: &[i64]) -> i64 {
    // dp represents ways to reach i-th adapter
    let mut dp = vec![0; adapters.len()];
    dp[0] = 1;

    for i in 1..adapters.len() {
        // look back at prev adapters
        for j in (0..i).rev() {
            let diff = adapters[i] - adapters[j];
            if diff > 3 {
                //difference too large, stop looking since adapters are sorted
                break;
            }
            // since the diff is OK, add this solution to the total ways to reach i-th adapter
            dp[i] += dp[j];
        }
    }

    // The last element = total ways to reach the device
    dp[adapters.len() - 1]
}

fn read_input_file_and_sort(file_path: &str) -> Vec<i64> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");

    let mut adapters: Vec<i64> = content
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    adapters.push(0); // first adapter, outlet base case
    adapters.sort();

    let max_adapter = adapters[adapters.len() - 1];
    adapters.push(max_adapter + 3);
    adapters.sort();
    adapters
}

fn add_difference(diff: i64, differences: &mut [i64; 3]) {
    if diff >= 1 && diff <= 3 {
        differences[diff as usize - 1] += 1;
    }
}
