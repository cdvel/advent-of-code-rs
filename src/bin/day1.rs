use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let magic_number = 2020;
    let filename = "input/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut records: Vec<i32> = Vec::new();

    for (_index, record) in reader.lines().enumerate() {
        let record = record.unwrap().parse::<i32>().unwrap();
        records.push(record);
    }

    'outer: for f_record in records.clone() {
        for s_record in records.clone() {
            if f_record + s_record == magic_number {
                println!("{} * {} = {}", f_record, s_record, f_record * s_record);
                break 'outer;
            }
        }
    }
}
