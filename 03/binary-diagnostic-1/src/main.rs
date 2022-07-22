use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};

fn main() -> io::Result<()> {
    // First, peek the first line in file to know how many bits to track
    let mut file = File::open("../input.txt")?;
    let lines = &mut BufReader::new(&file).lines();
    let first_line = lines.next().unwrap()?;
    let bits = first_line.chars().count();

    // Rewind file pointer so that the new BufReader can read the whole file
    file.seek(SeekFrom::Start(0)).unwrap();
    // Create a buffer of `bits` filled with 0s
    let mut counts = vec![0; bits];
    let lines = &mut BufReader::new(&file).lines();
    let mut line_count = 0;

    for line in lines {
        let line = line?;
        line_count += 1;
        let digits = line.chars().enumerate();
        for (i, digit) in digits {
            if digit == '1' {
                counts[i] += 1;
            }
        }
    }

    let threshold = line_count / 2;
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for (i, count) in counts.iter().enumerate() {
        if *count > threshold {
            // set bit at position i to 1
            gamma_rate = (1 << bits - i - 1) | gamma_rate;
        } else {
            epsilon_rate = (1 << bits - i - 1) | epsilon_rate;
        }
    }
    println!("{}", gamma_rate * epsilon_rate);

    Ok(())
}
