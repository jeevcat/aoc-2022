use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Valve {
    name: String,
    neighbors: Vec<String>,
    flow_rate: u32,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let name = words.nth(1).unwrap().to_string();
        let flow_rate = words
            .nth(2)
            .unwrap()
            .trim_start_matches("rate=")
            .trim_end_matches(';')
            .parse()
            .unwrap();
        let neighbors = words
            .skip(4)
            .map(|n| n.trim_end_matches(',').to_owned())
            .collect();
        Ok(Valve {
            name,
            neighbors,
            flow_rate,
        })
    }
}

fn main() {
    let mut parsed: Vec<Valve> = BufReader::new(File::open("test.txt").unwrap())
        .lines()
        .flatten()
        .flat_map(|l| l.parse::<Valve>())
        .collect();

    let start = parsed
        .iter()
        .find(|v| v.name == "AA")
        .unwrap()
        .neighbors
        .clone();

    let ignore = parsed
        .iter()
        .filter(|v| v.flow_rate == 0)
        .map(|v| v.name.clone())
        .collect::<HashSet<_>>();

    // Remove all valves with flow rate 0
    parsed.retain(|v| !ignore.contains(&v.name));
    // Remove all neighbors which with flow rate 0
    for v in parsed.iter_mut() {
        v.neighbors.retain(|n| !ignore.contains(n));
    }

    let valves: HashMap<&str, &Valve> = parsed.iter().map(|v| (v.name.as_str(), v)).collect();

    // Do breadth first search
    let mut queue = VecDeque::new();
    const AVAILABLE_TIME: u32 = 30;
    for n in &start {
        if let Some(n) = valves.get(n.as_str()) {
            if n.flow_rate > 0 {
                queue.push_back((n.name.as_str(), AVAILABLE_TIME - 1, 0));
            }
        }
    }

    let mut opened = HashSet::new();

    let mut max_score = 0;

    while let Some((name, time, score)) = queue.pop_front() {
        if time == 0 {
            if score > max_score {
                max_score = score;
                dbg!(name, score);
            }
            continue;
        }

        let valve = valves[name];
        // Stay here and open the valve
        if !opened.contains(name) {
            opened.insert(name);
            queue.push_back((name, time - 1, score + valve.flow_rate * time));
        }
        // stay here and do nothing
        // queue.push_back((name, time - 1, score));

        // Move to a neighbor
        for n in &valve.neighbors {
            queue.push_back((n.as_str(), time - 1, score));
        }
    }

    dbg!(max_score);
}
