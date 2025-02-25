use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct BagRule {
    quantity: u32,
    color: String,
}

fn read_entries(line: &str) -> (String, Vec<BagRule>) {
    let parts: Vec<&str> = line.split(" bags contain ").collect();
    let container = parts[0].to_string();
    
    if parts[1].contains("no other bags") {
        return (container, Vec::new());
    }

    let contents: Vec<BagRule> = parts[1]
        .split(", ")
        .map(|rule| {
            let words: Vec<&str> = rule.split_whitespace().collect();
            let quantity = words[0].parse().unwrap();
            let color = format!("{} {}", words[1], words[2]);
            BagRule { quantity, color }
        })
        .collect();

    (container, contents)
}

fn count_bags_inside(color: &str, rules: &HashMap<String, Vec<BagRule>>) -> u32 {
    let mut total = 0;
    
    if let Some(contents) = rules.get(color) {
        for bag in contents {
            // Add the bags directly inside
            total += bag.quantity;
            // Add the bags inside each of those bags
            total += bag.quantity * count_bags_inside(&bag.color, rules);
        }
    }
    
    total
}

fn main() {
    let file = File::open("input/day7.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut rules: HashMap<String, Vec<BagRule>> = HashMap::new();
    
    // Parse all rules
    for line in reader.lines() {
        let line = line.unwrap();
        let (container, contents) = read_entries(&line);
        rules.insert(container, contents);
    }
    
    let total_bags = count_bags_inside("shiny gold", &rules);
    println!("Number of bags required inside a single shiny gold bag: {}", total_bags);
}


