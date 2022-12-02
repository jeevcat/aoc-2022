use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut max_sum = 0;
    for line in reader.lines().flatten() {
        if line.is_empty() {
            if sum > max_sum {
                max_sum = sum;
            }
            sum = 0;
        } else if let Ok(num) = line.parse::<u32>() {
            sum += num;
        }
    }

    dbg!(max_sum);

    Ok(())
}
