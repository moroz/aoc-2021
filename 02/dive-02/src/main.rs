use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(());
        }
        let mut iter = value.split_ascii_whitespace();
        if let (Some(keyword), Some(digits)) = (iter.next(), iter.next()) {
            if let Ok(number) = digits.parse::<isize>() {
                match keyword {
                    "forward" => return Ok(Self::Forward(number)),
                    "down" => return Ok(Self::Down(number)),
                    "up" => return Ok(Self::Up(number)),
                    _ => (),
                }
            }
        }
        return Err(());
    }
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let file = BufReader::new(file);

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim: isize = 0;

    for line in file.lines() {
        if let Ok(command) = Command::try_from(line.unwrap().as_str()) {
            match command {
                Command::Forward(val) => {
                    x += val;
                    y += val * aim;
                }
                Command::Down(val) => {
                    aim += val;
                }
                Command::Up(val) => {
                    aim -= val;
                }
            }
        }
    }

    println!("{}", x * y);

    Ok(())
}
