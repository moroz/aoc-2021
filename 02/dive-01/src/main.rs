use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_line(line: &str) -> Option<(isize, isize)> {
    let mut iter = line.split_ascii_whitespace();

    if let Some(keyword) = iter.next() {
        if let Some(digits) = iter.next() {
            if let Ok(number) = digits.parse::<isize>() {
                match keyword {
                    "forward" => return Some((number, 0)),
                    "down" => return Some((0, number)),
                    "up" => return Some((0, -number)),
                    _ => return None,
                }
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let file = BufReader::new(file);

    let mut x: isize = 0;
    let mut y: isize = 0;

    for line in file.lines() {
        if let Some((x_offset, y_offset)) = parse_line(&line.unwrap()) {
            x += x_offset;
            y += y_offset;
        }
    }

    println!("{}", x * y);

    Ok(())
}
