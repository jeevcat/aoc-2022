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
    distance: i32,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let position = Pos {
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
        };
        let closest = Pos {
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
        };
        Ok(Sensor {
            position,
            closest,
            distance: dist(position, closest),
        })
    }
}

impl Sensor {
    fn within_range(&self, other: Pos) -> bool {
        dist(self.position, other) <= self.distance
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

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for sensor in &sensors {
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
    dbg!(min_x, max_x, min_y, max_y);

    let mut count = 0;
    let y = 2000000;
    for x in (min_x - 10000000)..=(max_x + 10000000) {
        let c = sensors.iter().any(|sensor| {
            !(sensor.closest.x == x && sensor.closest.y == y) && sensor.within_range(Pos { x, y })
        });
        if c {
            count += 1;
        }
        //print!("{}", c.unwrap_or('.'));
    }
    println!();
    dbg!(count);
}
