use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn count_trees(slopes: Vec<String>, right: i32, down: i32) -> i32 {
    let mut counter = 0;
    let mut col_index = 0;
    let mut line_counter = 0;

    for s in slopes {
        if down > 1 && line_counter % 2 != 0 {
            line_counter += 1;
            continue;
        }

        let mut road: Vec<char> = s.chars().collect();
        while road.len() <= col_index {
            road.extend(road.clone());
        }
        if road[col_index] == '#' {
            counter += 1;
        }
        col_index += right as usize;
        line_counter += 1;
    }
    counter
}

fn main() {
    let slopes = read_entries("input/day3.txt");
    let mut trees = vec![];
    let tuples = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for tp in tuples {
        trees.push(count_trees(slopes.clone(), tp.0, tp.1));
    }
    let mut product: i64 = 1;

    for tree in trees.iter() {
        product *= *tree as i64;
    }

    println!("{:?} = {:?}", trees, product);
}
