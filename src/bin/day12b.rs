use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn add(self, rhs: (i32, i32)) -> Option<Self> {
        Some(Pos(
            TryInto::<i32>::try_into(self.0)
                .ok()?
                .checked_add(rhs.0)?
                .try_into()
                .ok()?,
            TryInto::<i32>::try_into(self.1)
                .ok()?
                .checked_add(rhs.1)?
                .try_into()
                .ok()?,
        ))
    }
}

struct Grid {
    heights: Vec<Vec<u8>>,
    prev: HashSet<Pos>,
}

impl Grid {
    fn new(file: File) -> (Grid, Vec<Pos>, Pos) {
        let mut starts = Vec::new();
        let mut end = Default::default();

        let reader = BufReader::new(file);
        let grid = reader
            .lines()
            .flatten()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        'S' | 'a' => {
                            starts.push(Pos(row, col));
                            0
                        }
                        'E' => {
                            end = Pos(row, col);
                            b'z' - b'a'
                        }
                        c if c.is_ascii_lowercase() => c as u8 - b'a',
                        _ => panic!("invalid input"),
                    })
                    .collect()
            })
            .collect();

        (
            Grid {
                heights: grid,
                prev: starts.iter().copied().collect(),
            },
            starts,
            end,
        )
    }

    fn get(&self, pos: Pos) -> u8 {
        self.heights[pos.0][pos.1]
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        pos.0 < self.heights.len() && pos.1 < self.heights[pos.0].len()
    }

    fn valid_move(&self, from: Pos, to: Pos) -> bool {
        if !self.in_bounds(to) || self.visited(to) {
            return false;
        }
        self.get(to) <= self.get(from) + 1
    }

    fn visited(&self, pos: Pos) -> bool {
        self.prev.contains(&pos)
    }

    fn neighbors(&mut self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .flat_map(move |d| {
                let to = pos.add(d)?;
                if self.valid_move(pos, to) {
                    self.prev.insert(to);
                    Some(to)
                } else {
                    None
                }
            })
    }
}

fn bfs(grid: &mut Grid, start: Vec<Pos>, end: Pos) -> Option<usize> {
    let mut queue: VecDeque<_> = start.into_iter().map(|p| (p, 0)).collect();

    while let Some((pos, step)) = queue.pop_front() {
        if pos == end {
            return Some(step);
        }

        // Add neighbors
        for n in grid.neighbors(pos) {
            queue.push_back((n, step + 1));
        }
    }
    None
}

fn main() {
    let file = File::open("input12.txt").unwrap();
    let (mut grid, starts, end) = Grid::new(file);

    let step = bfs(&mut grid, starts, end);

    dbg!(step);
}
