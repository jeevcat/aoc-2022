use std::{collections::HashMap, fs};

const WINDOW_SIZE: usize = 14;

fn main() {
    let signal = fs::read_to_string("input6.txt").unwrap();
    let start = find_start(&signal);
    dbg!(start);
}

fn find_start(signal: &str) -> usize {
    let mut prev: HashMap<u8, u32> = HashMap::new();
    let mut duplicates = 0;
    for c in signal.as_bytes().iter().take(WINDOW_SIZE) {
        let new = prev.entry(*c).or_default();
        if *new == 1 {
            duplicates += 1;
        }
        *new += 1;
    }
    dbg!(duplicates);

    signal
        .as_bytes()
        .windows(WINDOW_SIZE + 1)
        .position(|win| {
            dbg!(duplicates);
            let found = duplicates == 0;

            let new = prev.entry(*win.last().unwrap()).or_default();
            if *new == 1 {
                duplicates += 1;
            }
            *new += 1;

            let old = prev.entry(*win.first().unwrap()).or_default();
            *old -= 1;
            if *old == 1 {
                duplicates -= 1;
            }
            found
        })
        .unwrap()
        + WINDOW_SIZE
}
