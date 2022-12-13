use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

struct Pair(RangeInclusive<i32>, RangeInclusive<i32>);

impl Pair {
    fn overlapping(&self) -> bool {
        self.0.start() <= self.1.end() && self.0.end() >= self.1.start()
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s.split(',').map(|s| {
            let mut range = s.split('-').map(|s| s.parse::<i32>().unwrap());
            RangeInclusive::new(range.next().unwrap(), range.next().unwrap())
        });
        Ok(Pair(ranges.next().unwrap(), ranges.next().unwrap()))
    }
}

fn main() {
    let file = File::open("input4.txt").unwrap();
    let reader = BufReader::new(file);

    let total = reader
        .lines()
        .flatten()
        .map(|line| line.parse::<Pair>().unwrap())
        .filter(Pair::overlapping)
        .count();
    dbg!(total);
}
