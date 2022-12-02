use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Shape {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn parse(line: &str) -> Option<(Shape, Outcome)> {
    let mut i = line.chars();
    let theirs = i.next()?.try_into().ok()?;
    let ours = i.nth(1)?.try_into().ok()?;
    Some((theirs, ours))
}

fn main() -> io::Result<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines().flatten() {
        if let Some((theirs, outcome)) = parse(&line) {
            let ours = match theirs {
                Shape::Rock => match outcome {
                    Outcome::Lose => Shape::Scissors,
                    Outcome::Draw => Shape::Rock,
                    Outcome::Win => Shape::Paper,
                },
                Shape::Paper => match outcome {
                    Outcome::Lose => Shape::Rock,
                    Outcome::Draw => Shape::Paper,
                    Outcome::Win => Shape::Scissors,
                },
                Shape::Scissors => match outcome {
                    Outcome::Lose => Shape::Paper,
                    Outcome::Draw => Shape::Scissors,
                    Outcome::Win => Shape::Rock,
                },
            };
            score += ours.score() + outcome.score();
        }
    }
    dbg!(score);
    Ok(())
}
