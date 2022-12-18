use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    dir: Direction,
    dist: i32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dir = split.next().ok_or(())?;
        let dist = split.next().ok_or(())?.parse().map_err(|_| ())?;
        match dir {
            "U" => Ok(Move {
                dir: Direction::Up,
                dist,
            }),
            "D" => Ok(Move {
                dir: Direction::Down,
                dist,
            }),
            "L" => Ok(Move {
                dir: Direction::Left,
                dist,
            }),
            "R" => Ok(Move {
                dir: Direction::Right,
                dist,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn apply(&mut self, m: &Move) {
        match m.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn follow(&mut self, in_front: &Knot) {
        // position of head relative to tail
        let (x, y) = (in_front.x - self.x, in_front.y - self.y);

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

#[derive(Default)]
struct Rope([Knot; 10]);

impl Rope {
    fn last(&self) -> &Knot {
        &self.0[self.0.len() - 1]
    }

    fn apply(&mut self, m: &Move) {
        self.0[0].apply(m);

        for i in 1..self.0.len() {
            let (in_front, knot) = self.0.split_at_mut(i);
            knot[0].follow(&in_front[in_front.len() - 1]);
        }
    }
}

fn main() {
    // read the input file
    let file = File::open("input9.txt").unwrap();
    let reader = BufReader::new(file);

    // Rope of ten knots
    let mut rope = Rope::default();

    let mut visited = HashSet::new();
    visited.insert((rope.last().x, rope.last().y));

    for line in reader.lines().flatten() {
        let m: Move = line.parse().unwrap();
        for _ in 0..m.dist {
            rope.apply(&m);
            visited.insert((rope.last().x, rope.last().y));
        }
    }

    dbg!(visited.len());
}
