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
            'X' => Ok(Self::Rock),
            'Y' => Ok(Self::Paper),
            'Z' => Ok(Self::Scissors),
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

fn parse(line: &str) -> Option<(Shape, Shape)> {
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
        if let Some((theirs, ours)) = parse(&line) {
            let outcome = match ours {
                Shape::Rock => match theirs {
                    Shape::Paper => 0,
                    Shape::Rock => 3,
                    Shape::Scissors => 6,
                },
                Shape::Paper => match theirs {
                    Shape::Scissors => 0,
                    Shape::Paper => 3,
                    Shape::Rock => 6,
                },
                Shape::Scissors => match theirs {
                    Shape::Rock => 0,
                    Shape::Scissors => 3,
                    Shape::Paper => 6,
                },
            };
            let shape_score = ours.score();
            let round_score = outcome + shape_score;
            score += round_score;
        }
    }
    dbg!(score);
    Ok(())
}
