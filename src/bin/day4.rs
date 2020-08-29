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
    let mandatory = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for key in mandatory {
        if !data.contains(key) {
            return false;
        }
    }

    true
}

fn is_valid(fields: Vec<&str>) -> bool {
    let mut dict: HashMap<&str, &str> = HashMap::new();
    let mut keys: Vec<&str>;

    for field in fields {
        keys = field.split(":").collect();
        dict.insert(keys[0], keys[1]);
    }

    for (key, val) in &dict {
        let ret = match *key {
            "byr" => is_num_in_range(val, 1920, 2002),
            "iyr" => is_num_in_range(val, 2010, 2020),
            "eyr" => is_num_in_range(val, 2020, 2030),
            "hgt" => is_height(val),
            "hcl" => is_hair(val),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(val),
            "pid" => val.chars().count() == 9 && is_num_in_range(val, 0, 999999999),
            _ => true,
        };

        if !ret {
            return false;
        }
    }

    true
}

fn is_num_in_range(num: &str, lower: i32, upper: i32) -> bool {
    let val = num.parse::<i32>();
    let x = match val {
        Ok(x) => (x >= lower && x <= upper),
        Err(ref _e) => false,
    };

    return x;
}

fn is_height(hgt: &str) -> bool {
    let size = hgt.chars().count();

    return match size {
        5 => {
            let h = &hgt[..3];
            let sys = &hgt[3..];
            return sys == "cm" && is_num_in_range(h, 150, 193);
        }
        4 => {
            let h = &hgt[..2];
            let sys = &hgt[2..];
            return sys == "in" && is_num_in_range(h, 59, 76);
        }
        _ => false,
    };
}

fn is_hair(hcl: &str) -> bool {
    let hash = &hcl[..1];
    let alpha = &hcl[1..];

    match hash == "#" {
        false => {
            return false;
        }
        _ => {}
    };

    if alpha.chars().count() != 6 || !alpha.chars().all(char::is_alphanumeric) {
        return false;
    }

    true
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
