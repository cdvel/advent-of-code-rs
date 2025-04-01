use std::fs;

fn main() {
    let mut layout = read_layout("input/day11.txt");
    let mut new_layout;
    // let mut current = String::new();

    loop {
        new_layout = layout.clone();
        for i in 0..layout.len() {
            // println!("{:?}", layout[i]);

            for j in 0..layout[i].len() {
                // println!("{:?}", adjacent_occupied(&layout, i, j)) == 0;
                if layout[i][j] == 'L' && adjacent_occupied(&layout, i, j) == 0 {
                    new_layout[i][j] = '#';
                }

                if layout[i][j] == '#' && adjacent_occupied(&layout, i, j) > 3 {
                    new_layout[i][j] = 'L';
                }
            }
        }

        if layout == new_layout {
            break; // No changes, we're done
        }

        layout = new_layout.clone();
    }

    println!(
        "{:?}",
        layout.iter().flatten().filter(|&&c| c == '#').count()
    );
}
fn adjacent_occupied(layout: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut occupied = 0;

    for x in i.saturating_sub(1)..=i + 1 {
        for y in j.saturating_sub(1)..=j + 1 {
            if x == i && y == j {
                continue;
            } // same seat
            if x < layout.len() && y < layout[x].len() && layout[x][y] == '#' {
                occupied += 1;
            } // found occupied seat
        }
    }

    return occupied;
}
fn read_layout(filepath: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(filepath).expect("Failed to read file");
    let layout = content.lines().map(|line| line.chars().collect()).collect();
    layout
}
