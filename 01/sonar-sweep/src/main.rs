use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn next_number(lines: &mut Lines<BufReader<File>>) -> Option<usize> {
    if let Some(Ok(line)) = lines.next() {
        return line.trim().parse().ok();
    }
    return None;
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let file = BufReader::new(file);

    let mut grow_count = 0;
    let mut lines = file.lines();
    let mut last_val = next_number(&mut lines).unwrap();

    while let Some(value) = next_number(&mut lines) {
        if value > last_val {
            grow_count += 1;
        }
        last_val = value;
    }

    println!("{}", grow_count);
}
