use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BagRule {
    // quantity: u32,
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
            // let quantity = words[0].parse().unwrap();
            let color = format!("{} {}", words[1], words[2]);
            BagRule {  color }
        })
        .collect();

    (container, contents)
}

fn can_contain_shiny_gold(
    color: &str,
    rules: &HashMap<String, Vec<BagRule>>,
    memo: &mut HashSet<String>
) -> bool {
    if memo.contains(color) {
        return false;
    }
    
    if let Some(contents) = rules.get(color) {
        for bag in contents {
            if bag.color == "shiny gold" || can_contain_shiny_gold(&bag.color, rules, memo) {
                return true;
            }
        }
    }
    
    memo.insert(color.to_string());
    false
}

fn main() {
    let file = File::open("input/day7.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut rules: HashMap<String, Vec<BagRule>> = HashMap::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let (container, contents) = read_entries(&line);
        rules.insert(container, contents);
    }
    
    let mut count = 0;
    let mut memo = HashSet::new();
    
    for color in rules.keys() {
        if can_contain_shiny_gold(color, &rules, &mut memo) {
            count += 1;
        }
    }
    
    println!("Number of bag colors that can contain a shiny gold bag: {}", count);
}


