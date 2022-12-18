use std::{
    collections::HashSet,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Read, Write},
    str::FromStr,
};

#[derive(Debug)]
enum Move {
    Up { dist: i32 },
    Down { dist: i32 },
    Left { dist: i32 },
    Right { dist: i32 },
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dir = split.next().ok_or(())?;
        let dist = split.next().ok_or(())?.parse().map_err(|_| ())?;
        match dir {
            "U" => Ok(Move::Up { dist }),
            "D" => Ok(Move::Down { dist }),
            "L" => Ok(Move::Left { dist }),
            "R" => Ok(Move::Right { dist }),
            _ => Err(()),
        }
    }
}

impl Move {
    fn dist(&self) -> i32 {
        match self {
            Move::Up { dist } => *dist,
            Move::Down { dist } => *dist,
            Move::Left { dist } => *dist,
            Move::Right { dist } => *dist,
        }
    }
}

struct Head {
    x: i32,
    y: i32,
}

impl Head {
    fn apply(&mut self, m: &Move) {
        match m {
            Move::Up { .. } => self.y -= 1,
            Move::Down { .. } => self.y += 1,
            Move::Left { .. } => self.x -= 1,
            Move::Right { .. } => self.x += 1,
        }
    }
}

struct Tail {
    x: i32,
    y: i32,
}

impl Tail {
    fn follow(&mut self, head: &Head) {
        // position of head relative to tail
        let (x, y) = (head.x - self.x, head.y - self.y);

        let touching = x * x + y * y <= 2;
        if touching {
            return;
        }

        if x == 0 {
            if y > 0 {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if y == 0 {
            if x > 0 {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else if x > 0 {
            if y > 0 {
                self.x += 1;
                self.y += 1;
            } else {
                self.x += 1;
                self.y -= 1;
            }
        } else if y > 0 {
            self.x -= 1;
            self.y += 1;
        } else {
            self.x -= 1;
            self.y -= 1;
        }
    }
}

fn main() {
    // read the input file
    let file = File::open("input9.txt").unwrap();
    let reader = BufReader::new(file);

    let mut head = Head { x: 0, y: 0 };
    let mut tail = Tail { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert((tail.x, tail.y));

    for line in reader.lines().flatten() {
        let m: Move = line.parse().unwrap();
        for _ in 0..m.dist() {
            head.apply(&m);
            tail.follow(&head);
            visited.insert((tail.x, tail.y));
        }
    }

    dbg!(visited.len());
}
