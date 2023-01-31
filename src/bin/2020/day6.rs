use std::fs::File;
use std::io::{BufRead, BufReader};
//use itertools::Itertools;

fn read_entries(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut entries = Vec::new();

    for (_index, entry) in reader.lines().enumerate() {
        let entry = entry.unwrap();
        entries.push(entry)
    }
    entries
}

fn count_unique(line: &str) -> usize {
    let mut v: Vec<char> = line.chars().collect();
    v.sort_unstable();
    v.dedup();
    v.len()
}

fn main() {
    let entries = read_entries("input/day6.txt");
    let mut groups = vec![];
    let mut group = vec![];
    let mut counter = 0;

    for entry in &entries {
        if entry.is_empty() && !group.is_empty() {
            groups.push(group);
            group = vec![];
        } else {
            group.push(entry);
        }
    }

    let mut groups_str = vec![];
    for g in groups {
        groups_str.push(g.iter().fold(String::new(), |acc, st| acc + st));
        counter += count_unique(&groups_str[groups_str.len() - 1]);
    }

    println!("{:?}", counter);
}
