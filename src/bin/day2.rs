use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_entries(filename: &str) -> Vec<String>{

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    
    for (_index, entry) in reader.lines().enumerate(){
        let entry = entry.unwrap();
        entries.push(entry);
    }
    entries
}

fn main() {
    let entries = read_entries("input/day2.txt");
    let mut counter = 0;

    for e in entries{

        let mut pair = e.split(": ");
        let policy = pair.next().unwrap();
        let password = pair.next().unwrap();
        
        let mut pol = policy.split(' ');
        let range = pol.next().unwrap();
        let letter = pol.next().unwrap();

        let mut ran = range.split('-');
        let minr = ran.next().unwrap().parse::<i32>().unwrap() as usize;
        let maxr = ran.next().unwrap().parse::<i32>().unwrap() as usize;
        
        let count = password.matches(letter).count();

        if minr <= count && count <= maxr {
            counter += 1;
        }
    }
    println!("match count = {:?}", counter);
}
