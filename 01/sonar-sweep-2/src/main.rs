use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn next_number(lines: &mut Lines<BufReader<File>>) -> Option<usize> {
    if let Some(Ok(line)) = lines.next() {
        return line.trim().parse().ok();
    }
    return None;
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let mut grow_count = 0;
    let mut last_sum = 0;
    let mut queue = VecDeque::new();

    while let Some(value) = next_number(&mut lines) {
        queue.push_back(value);
        if queue.len() < 4 {
            last_sum += value;
            continue;
        }
        let first = queue.pop_front().unwrap();
        let sum = last_sum - first + value;
        if sum > last_sum {
            grow_count += 1;
        }
        last_sum = sum;
    }

    println!("{}", grow_count);
}
