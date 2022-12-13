use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Move {
    count: u32,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(char::is_whitespace);
        let count = i.nth(1).unwrap().parse().unwrap();
        let from = i.nth(1).unwrap().parse().unwrap();
        let to = i.nth(1).unwrap().parse().unwrap();
        Ok(Self { count, from, to })
    }
}

struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn new(lines: &[String]) -> Self {
        let mut i = lines.iter().rev();
        let size = i.next().unwrap().split_whitespace().count();
        let mut stacks = vec![vec![]; size];
        for line in i {
            let mut j = line.chars().skip(1);
            let mut index = 0;
            while let Some(c) = j.next() {
                if c.is_alphabetic() {
                    stacks[index].push(c);
                }
                index += 1;
                j.nth(2); //skip to next letter
            }
        }

        Self(stacks)
    }

    fn apply(&mut self, m: &Move) {
        for _ in 0..m.count {
            let popped = self.0[m.from - 1].pop().unwrap();
            self.0[m.to - 1].push(popped);
        }
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stack in self.0.iter() {
            write!(f, "{}", stack.last().unwrap_or(&'_'))?;
        }
        Ok(())
    }
}

fn main() {
    let file = File::open("input5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().flatten();
    let mut stacks = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        stacks.push(line);
    }

    let mut stacks = Stacks::new(&stacks);

    for mv in lines.map(|line| line.parse().unwrap()) {
        stacks.apply(&mv);
    }
    println!("{stacks}");
}
