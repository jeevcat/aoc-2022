use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

type Cell = (usize, usize);

struct Rock(Vec<Cell>);

impl FromStr for Rock {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rock(
            s.split(" -> ")
                .flat_map(|s| {
                    let p = s.split_once(',')?;
                    Some((p.0.parse().ok()?, p.1.parse().ok()?))
                })
                .collect(),
        ))
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    left: usize,
    right: usize,
    bottom: usize,
    rock_bottom: usize,
    top: usize,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: vec![],
            left: usize::MAX,
            right: usize::MIN,
            bottom: usize::MAX,
            rock_bottom: usize::MIN,
            top: usize::MIN,
        }
    }

    fn fill_rocks(&mut self, filename: &str) {
        for rock in BufReader::new(File::open(filename).unwrap())
            .lines()
            .flatten()
            .flat_map(|line| line.parse::<Rock>())
        {
            self.set((500, 0), '+');
            for point in rock.0.windows(2) {
                let (mut current, end) = (point[0], point[1]);
                let (dx, dy) = (
                    (end.0 as isize - current.0 as isize).signum(),
                    (end.1 as isize - current.1 as isize).signum(),
                );
                while current != end {
                    self.set(current, '#');
                    self.rock_bottom = self.rock_bottom.max(current.1);
                    let (x, y) = (current.0 as isize + dx, current.1 as isize + dy);
                    current = (x as usize, y as usize);
                }
                self.set(end, '#');
            }
        }
    }

    fn get(&self, cell: Cell) -> char {
        self.data[cell.1][cell.0]
    }

    fn set(&mut self, cell: Cell, value: char) {
        self.left = self.left.min(cell.0);
        self.right = self.right.max(cell.0);
        self.bottom = self.bottom.min(cell.1);
        self.top = self.top.max(cell.1);

        self.data.resize(self.top + 2, vec![' '; self.right + 2]);
        self.data[cell.1].resize(self.right + 2, ' ');

        self.data[cell.1][cell.0] = value;
    }
}

fn simulate_sand(grid: &Grid) -> Cell {
    let mut sand = (500, 0);
    while let Some(next) = [
        (sand.0, sand.1 + 1),
        (sand.0 - 1, sand.1 + 1),
        (sand.0 + 1, sand.1 + 1),
    ]
    .into_iter()
    .find(|&candidate| grid.get(candidate) == ' ')
    {
        sand = next;
        if sand.1 > grid.rock_bottom {
            return sand;
        }
    }

    sand
}

fn main() {
    let mut grid = Grid::new();
    grid.fill_rocks("input14.txt");

    // Simulate sand
    let mut iterations = 0;
    while grid.get((500, 0)) != 'o' {
        let sand = simulate_sand(&grid);
        grid.set(sand, 'o');
        iterations += 1;
    }

    dbg!(iterations);

    for row in grid.data[grid.bottom..=grid.top].iter() {
        for cell in row[grid.left..=grid.right].iter() {
            print!("{cell}");
        }
        println!();
    }
}
