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

#[derive(Debug)]
struct SeatRange {
    row: (i32, i32),
    col: (i32, i32),
    _id: i32
}

fn search(mut seat: SeatRange, bit: char) -> SeatRange {
    let mut mid_row = seat.row.0 as f32 + (seat.row.1 as f32 - seat.row.0 as f32) / 2.0;
    let mut mid_col = seat.col.0 as f32 + (seat.col.1 as f32 - seat.col.0 as f32) / 2.0;

    mid_row = mid_row.floor() as f32;
    mid_col = mid_col.floor() as f32;

    match bit {
        'F' => seat.row.1 = mid_row as i32,
        'B' => seat.row.0 = mid_row as i32 + 1,
        'L' => seat.col.1 = mid_col as i32,
        'R' => seat.col.0 = mid_col as i32 + 1,
        _ => {}
    };

    seat
}

fn find_seat(partition: &str) -> i32 {
    let bits: Vec<char> = partition.chars().collect();
    let mut seat = SeatRange {
        row: (0, 127),
        col: (0, 7),
        _id: 0,
    };

    for bit in bits {
        seat = search(seat, bit);
    }

    // seat ID formula
    seat.row.0 * 8 + seat.col.0
}

fn main() {
    let encoded = read_entries("input/day5.txt");
    let mut seat_ids = vec![];

    for encode in &encoded {
        seat_ids.push(find_seat(encode))
    }

    // What is the highest seat ID on a boarding pass?
    let max_id = seat_ids.iter().max();
    match max_id {
        Some(max) => println!("Max ID: {:?}", max),
        None => println!("Not found"),
    }

    // Find missing seat ids in list
    let min_id = seat_ids.iter().min();
    match min_id {
        Some(_min_id) => (),
        None => println!("Not found"),
    }

    let mut missing_ids = vec![];

    for index in *min_id.unwrap()..*max_id.unwrap() {
        if !seat_ids.contains(&index) {
            missing_ids.push(index)
        }
    }

    println!("Missing ID: {:?}", missing_ids.pop().unwrap());
}

#[cfg(test)]
#[test]
fn works_with_samples() {
    let encoded = ["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

    let ids = [357, 567, 119, 820];
    let mut seat_ids = vec![];

    for encode in &encoded {
        seat_ids.push(find_seat(encode))
    }

    for (idx, _id) in seat_ids.iter().enumerate() {
        assert_eq!(seat_ids[idx], ids[idx]);
    }
}
