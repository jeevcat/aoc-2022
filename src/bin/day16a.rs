use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Valve {
    name: String,
    neighbors: Vec<String>,
    flow_rate: i32,
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

    let valves: HashMap<&str, &Valve> = parsed.iter().map(|v| (v.name.as_str(), v)).collect();
    let distances = distances(&valves);

    // Traverse the graph and find the maximum total pressure released
    let mut total = 0;

    const AVAILABLE_TIME: i32 = 20;
    let mut queue = VecDeque::new();
    for neighbour in &valves["AA"].neighbors {
        if valves[neighbour.as_str()].flow_rate > 0 {
            queue.push_back((neighbour.as_str(), AVAILABLE_TIME, 0, HashSet::new()));
        }
    }

    while let Some((name, remaining_time, pressure, opened)) = queue.pop_front() {
        if remaining_time == 0 {
            total = total.max(pressure);
            continue;
        }

        let valve = valves[name];

        // try opening the valve
        if !opened.contains(name) {
            let mut new_opened = opened.clone();
            new_opened.insert(name);
            queue.push_back((
                name,
                remaining_time - 1,
                pressure + valve.flow_rate * remaining_time,
                new_opened,
            ));
        }

        for (neighbour, distance) in &distances[name] {
            //  try moving to neighbor
            let remaining_time = remaining_time - distance;
            if remaining_time >= 0 {
                queue.push_back((neighbour, remaining_time, pressure, opened.clone()));
            }
        }
    }

    dbg!(total);
}

fn distances<'a>(valves: &'a HashMap<&'a str, &Valve>) -> HashMap<&'a str, Vec<(&'a str, i32)>> {
    let mut distances = HashMap::new();
    for start in valves.keys().filter(|v| valves[*v].flow_rate > 0) {
        let mut queue = VecDeque::new();
        queue.push_back((*start, 0));
        let mut visited = HashSet::new();

        while let Some((current, distance)) = queue.pop_front() {
            for neighbor in valves[current].neighbors.iter() {
                if !visited.contains(neighbor.as_str()) {
                    visited.insert(neighbor.as_str());
                    queue.push_back((neighbor.as_str(), distance + 1));
                }
            }
            if current != *start && valves[current].flow_rate > 0 {
                distances
                    .entry(*start)
                    .or_insert_with(Vec::new)
                    .push((current, distance));
            }
        }
    }
    distances
}
