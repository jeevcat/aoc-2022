use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Trees = Vec<Vec<i8>>;
type Grid = Vec<Vec<bool>>;

fn main() {
    let file = File::open("input8.txt").unwrap();
    let reader = BufReader::new(file);

    let heights: Trees = reader
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .flat_map(|c| c.to_digit(10).map(|d| d as i8))
                .collect()
        })
        .collect();

    let h = heights.len();
    let w = heights[0].len();

    let mut visible: Grid = vec![vec![false; w]; h];

    for row in 0..h {
        // looking east
        let mut max = -1;
        for col in 0..w {
            if check(&heights, &mut visible, row, col, &mut max) {
                break;
            }
        }
        // looking west
        let mut max = -1;
        for col in (0..w).rev() {
            if check(&heights, &mut visible, row, col, &mut max) {
                break;
            }
        }
    }
    for col in 0..w {
        // looking south
        let mut max = -1;
        for row in 0..h {
            if check(&heights, &mut visible, row, col, &mut max) {
                break;
            }
        }
        // looking north
        let mut max = -1;
        for row in (0..h).rev() {
            if check(&heights, &mut visible, row, col, &mut max) {
                break;
            }
        }
    }

    for row in 0..h {
        for col in 0..w {
            if !visible[row][col] {
                print!(" ");
            } else {
                print!("{}", heights[row][col]);
            }
        }
        println!();
    }
    let count = visible
        .iter()
        .map(|row| row.iter().map(|b| if *b { 1 } else { 0 }).sum::<i32>())
        .sum::<i32>();
    dbg!(count);
}

fn check(heights: &Trees, visible: &mut Grid, row: usize, col: usize, max: &mut i8) -> bool {
    let h = heights[row][col];
    if h > *max {
        visible[row][col] = true;
    }
    if h > *max {
        *max = h;
    }

    // no need to continue if we can't see anything
    heights[row][col] == 9
}
