use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

// fn main() -> io::Result<()> {
//     let file = File::open("input.txt")?;
//     let reader = BufReader::new(file);

//     let mut sum = 0;
//     let mut max_sum = 0;
//     for line in reader.lines().flatten() {
//         if line.is_empty() {
//             if sum > max_sum {
//                 max_sum = sum;
//             }
//             sum = 0;
//         } else if let Ok(num) = line.parse::<u32>() {
//             sum += num;
//         }
//     }

//     dbg!(max_sum);

//     Ok(())
// }

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let mut sum: u32 = 0;
    for line in reader.lines().flatten() {
        if line.is_empty() {
            heap.push(Reverse(sum));
            if heap.len() > 3 {
                heap.pop();
            }
            sum = 0;
        } else if let Ok(num) = line.parse::<u32>() {
            sum += num;
        }
    }

    let total: u32 = heap.drain().map(|r| r.0).sum();
    dbg!(total);

    Ok(())
}
