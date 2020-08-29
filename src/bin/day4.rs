use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_entries(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    let mut passport = String::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line == "" {
            entries.push(passport.trim().to_string());
            passport = String::new();
        } else {
            passport = format!("{} {}", passport, line);
        }
    }
    entries
}

fn is_complete(data: &str) -> bool {
    for key in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]{
        if !data.contains(key) {
            return false;
        }
    }
    true
}

fn is_valid(fields: Vec<&str>) -> bool {
    let mut dict: HashMap<&str, &str> = HashMap::new();
    let mut keys: Vec<&str>;
    let mut validity: bool = true;

    for field in fields {
        keys = field.split(":").collect();
        dict.insert(keys[0], keys[1]);
    }

    for (key, val) in &dict {
        validity = match *key {
            "byr" => in_range(val, 1920, 2002),
            "iyr" => in_range(val, 2010, 2020),
            "eyr" => in_range(val, 2020, 2030),
            "hgt" => is_height(val),
            "hcl" => is_hair(val),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(val),
            "pid" => val.chars().count() == 9 && in_range(val, 0, 999999999),
            _ => true,
        };

        if !validity {
            break;
        }
    }
    validity
}

fn in_range(num: &str, lower: i32, upper: i32) -> bool {
    return match num.parse::<i32>() {
        Ok(x) => (x >= lower && x <= upper),
        Err(ref _e) => false,
    };
}

fn is_height(hgt: &str) -> bool {
    return match hgt.chars().count(){
        5 => return &hgt[3..] == "cm" && in_range(&hgt[..3], 150, 193),
        4 => return &hgt[2..] == "in" && in_range(&hgt[..2], 59, 76),
        _ => false,
    };
}

fn is_hair(hcl: &str) -> bool {
    return hcl[..1] == *"#" && hcl[1..].chars().count() == 6 && hcl[1..].chars().all(char::is_alphanumeric);
}

fn main() {
    let passports = read_entries("input/day4.txt");
    let mut complete = Vec::new();
    let mut valid = Vec::new();

    for pass in passports {
        if is_complete(&pass) {
            complete.push(pass.clone());
        }
    }
    println!("complete no. = {:?}", complete.len());

    for passport in complete {
        let tmp: Vec<&str> = passport.split(" ").collect();
        if is_valid(tmp) {
            valid.push(passport.clone());
        }
    }
    println!("valid no. = {:?}", valid.len());
}
