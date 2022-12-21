use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Pos,
    closest: Pos,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        Ok(Sensor {
            position: Pos {
                x: words
                    .nth(2)
                    .unwrap()
                    .trim_start_matches("x=")
                    .trim_end_matches(',')
                    .parse()
                    .unwrap(),
                y: words
                    .next()
                    .unwrap()
                    .trim_start_matches("y=")
                    .trim_end_matches(':')
                    .parse()
                    .unwrap(),
            },
            closest: Pos {
                x: words
                    .nth(4)
                    .unwrap()
                    .trim_start_matches("x=")
                    .trim_end_matches(',')
                    .parse()
                    .unwrap(),
                y: words
                    .next()
                    .unwrap()
                    .trim_start_matches("y=")
                    .parse()
                    .unwrap(),
            },
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Sensor,
    Beacon,
    NoBeacon,
    Empty,
}

struct Grid {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(sensors: &[Sensor]) -> Self {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for sensor in sensors {
            if sensor.position.x < min_x {
                min_x = sensor.position.x;
            }
            if sensor.position.x > max_x {
                max_x = sensor.position.x;
            }
            if sensor.position.y < min_y {
                min_y = sensor.position.y;
            }
            if sensor.position.y > max_y {
                max_y = sensor.position.y;
            }
            if sensor.closest.x < min_x {
                min_x = sensor.closest.x;
            }
            if sensor.closest.x > max_x {
                max_x = sensor.closest.x;
            }
            if sensor.closest.y < min_y {
                min_y = sensor.closest.y;
            }
            if sensor.closest.y > max_y {
                max_y = sensor.closest.y;
            }
        }

        min_y = 2000000;
        max_y = 2000000;

        let grid =
            vec![vec![Tile::Empty; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        let mut grid = Grid {
            min_x,
            max_x,
            min_y,
            max_y,
            grid,
        };

        for sensor in sensors {
            println!("Sensor: {sensor:?}");
            grid.add_sensor(sensor);
        }

        grid
    }

    fn add_sensor(&mut self, sensor: &Sensor) {
        let dist = dist(sensor.position, sensor.closest);
        for split in 0..=dist {
            let rest = dist - split;
            // draw horizontal lines outward from beacon
            for x in 1..=split {
                self.set(
                    Pos {
                        x: sensor.position.x + x,
                        y: sensor.position.y,
                    },
                    Tile::NoBeacon,
                );
                self.set(
                    Pos {
                        x: sensor.position.x - x,
                        y: sensor.position.y,
                    },
                    Tile::NoBeacon,
                );
            }
            // draw vertical lines outward from ends of horizontal lines
            for y in 1..=rest {
                self.set(
                    Pos {
                        x: sensor.position.x + split,
                        y: sensor.position.y + y,
                    },
                    Tile::NoBeacon,
                );
                self.set(
                    Pos {
                        x: sensor.position.x + split,
                        y: sensor.position.y - y,
                    },
                    Tile::NoBeacon,
                );
                self.set(
                    Pos {
                        x: sensor.position.x - split,
                        y: sensor.position.y + y,
                    },
                    Tile::NoBeacon,
                );
                self.set(
                    Pos {
                        x: sensor.position.x - split,
                        y: sensor.position.y - y,
                    },
                    Tile::NoBeacon,
                );
            }
        }
        self.set(sensor.position, Tile::Sensor);
        self.set(sensor.closest, Tile::Beacon);
    }

    fn set(&mut self, pos: Pos, tile: Tile) {
        if pos.x < self.min_x || pos.x > self.max_x || pos.y < self.min_y || pos.y > self.max_y {
            return;
        }
        self.grid[(pos.y - self.min_y) as usize][(pos.x - self.min_x) as usize] = tile;
    }

    fn row(&self, y: i32) -> &[Tile] {
        &self.grid[(y - self.min_y) as usize]
    }
}

// get the manhattan distance between two points
fn dist(a: Pos, b: Pos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn main() {
    let sensors: Vec<Sensor> = BufReader::new(File::open("input15.txt").unwrap())
        .lines()
        .flatten()
        .flat_map(|l| l.parse::<Sensor>())
        .collect();

    let grid = Grid::new(&sensors);

    let count = grid
        .row(2000000)
        .iter()
        .filter(|t| matches!(t, Tile::NoBeacon))
        .count();
    dbg!(count);

    for row in grid.grid {
        for tile in row {
            match tile {
                Tile::Sensor => print!("S"),
                Tile::Beacon => print!("B"),
                Tile::NoBeacon => print!("#"),
                Tile::Empty => print!("."),
            }
        }
        println!();
    }
}
