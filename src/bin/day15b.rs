use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    position: Pos,
    distance: i64,
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
            distance: dist(position, closest),
        })
    }
}

struct Region {
    top_left: Pos,
    bottom_right: Pos,
}
impl Sensor {
    // Is given position in range of this sensor?
    fn within_range(&self, other: Pos) -> bool {
        dist(self.position, other) <= self.distance
    }

    /// Is there any point in the given region that is outside the range of the sensor?
    fn outside_range(&self, region: &Region) -> bool {
        [
            region.top_left,
            Pos {
                x: region.top_left.x,
                y: region.bottom_right.y,
            },
            region.bottom_right,
            Pos {
                x: region.bottom_right.x,
                y: region.top_left.y,
            },
        ]
        .iter()
        .map(|p| dist(self.position, *p))
        .max()
        .unwrap()
            > self.distance
    }
}

// get the manhattan distance between two points
fn dist(a: Pos, b: Pos) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn main() {
    let sensors: Vec<Sensor> = BufReader::new(File::open("input15.txt").unwrap())
        .lines()
        .flatten()
        .flat_map(|l| l.parse::<Sensor>())
        .collect();

    const MAX: i64 = 4000000;
    let mut stack: Vec<Region> = vec![Region {
        top_left: Pos { x: 0, y: 0 },
        bottom_right: Pos { x: MAX, y: MAX },
    }];
    while let Some(region) = stack.pop() {
        if region.top_left.x == region.bottom_right.x
            && region.top_left.y == region.bottom_right.y
            && sensors.iter().all(|s| !s.within_range(region.top_left))
        {
            dbg!(region.top_left);
            let tuning_freq = region.top_left.x * MAX + region.top_left.y;
            dbg!(tuning_freq);
            exit(0);
        }

        let mid_x = (region.top_left.x + region.bottom_right.x) / 2;
        let mid_y = (region.top_left.y + region.bottom_right.y) / 2;

        stack.extend(
            [
                // Top left
                Region {
                    top_left: region.top_left,
                    bottom_right: Pos { x: mid_x, y: mid_y },
                },
                // Top right
                Region {
                    top_left: Pos {
                        x: mid_x + 1,
                        y: region.top_left.y,
                    },
                    bottom_right: Pos {
                        x: region.bottom_right.x,
                        y: mid_y,
                    },
                },
                // Bottom left
                Region {
                    top_left: Pos {
                        x: region.top_left.x,
                        y: mid_y + 1,
                    },
                    bottom_right: Pos {
                        x: mid_x,
                        y: region.bottom_right.y,
                    },
                },
                // Bottom right
                Region {
                    top_left: Pos {
                        x: mid_x + 1,
                        y: mid_y + 1,
                    },
                    bottom_right: region.bottom_right,
                },
            ]
            .into_iter()
            .filter(|region| sensors.iter().all(|s| s.outside_range(region))),
        );
    }
}
