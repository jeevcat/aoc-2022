use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Trees = Vec<Vec<i8>>;

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

    let best_spot = heights
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, _)| (r, c)))
        .map(|(row, col)| score(&heights, row, col))
        .max();
    dbg!(best_spot);
}

fn score(heights: &Trees, row: usize, col: usize) -> i32 {
    let h = heights[row][col];

    let mut right = 0;
    for col in (col + 1)..(heights[col].len()) {
        right += 1;
        if heights[row][col] >= h {
            break;
        }
    }

    let mut left = 0;
    for col in (0..col).rev() {
        left += 1;
        if heights[row][col] >= h {
            break;
        }
    }

    let mut down = 0;
    for row in heights.iter().skip(row + 1) {
        down += 1;
        if row[col] >= h {
            break;
        }
    }

    let mut up = 0;
    for row in (0..row).rev() {
        up += 1;
        if heights[row][col] >= h {
            break;
        }
    }

    right * left * up * down
}
