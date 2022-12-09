use std::{
    boxed::Box,
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Rucksack {
    compartments: [Compartment; 2],
}

struct Compartment {
    items: HashSet<Item>,
}

#[derive(Hash, Eq, PartialEq)]
struct Item {
    priority: i32,
}

impl TryFrom<char> for Item {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_lowercase() {
            return Ok(Item {
                priority: (value as i32) - ('a' as i32) + 1,
            });
        }
        if value.is_ascii_uppercase() {
            return Ok(Item {
                priority: (value as i32) - ('A' as i32) + 27,
            });
        }
        Err(())
    }
}

impl FromStr for Compartment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Compartment {
            items: s
                .chars()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len % 2 != 0 {
            return Err(());
        }
        let half = len / 2;

        Ok(Rucksack {
            compartments: [s[..half].parse()?, s[half..].parse()?],
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // load the input file and loop through its lines
    let file = File::open("input3.txt")?;
    let reader = BufReader::new(file);

    let total: i32 = reader
        .lines()
        .flatten()
        .map(|line| {
            let rucksack: Rucksack = line.parse().unwrap();
            let common = rucksack.compartments[0]
                .items
                .intersection(&rucksack.compartments[1].items)
                .next()
                .unwrap()
                .priority;
            common
        })
        .sum();
    dbg!(total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item() {
        let item = Item::try_from('a').unwrap();
        assert_eq!(item.priority, 1);

        let item = Item::try_from('z').unwrap();
        assert_eq!(item.priority, 26);

        let item = Item::try_from('A').unwrap();
        assert_eq!(item.priority, 27);

        let item = Item::try_from('Z').unwrap();
        assert_eq!(item.priority, 52);
    }

    #[test]
    fn test_parse_compartment() {
        let compartment: Compartment = "aA".parse().unwrap();
        assert_eq!(compartment.items.len(), 2);
        assert_eq!(compartment.items[0].priority, 1);
        assert_eq!(compartment.items[1].priority, 27);

        let compartment: Compartment = "abcdef".parse().unwrap();
        assert_eq!(compartment.items.len(), 6);
        assert_eq!(compartment.items[0].priority, 1);
        assert_eq!(compartment.items[1].priority, 2);
        assert_eq!(compartment.items[2].priority, 3);
        assert_eq!(compartment.items[3].priority, 4);
        assert_eq!(compartment.items[4].priority, 5);
    }

    #[test]
    fn test_parse_rucksack() {
        let rucksack: Rucksack = "abcdefABCDEF".parse().unwrap();
        assert_eq!(rucksack.compartments[0].items.len(), 6);
        assert_eq!(rucksack.compartments[0].items[0].priority, 1);
        assert_eq!(rucksack.compartments[0].items[1].priority, 2);
        assert_eq!(rucksack.compartments[0].items[2].priority, 3);
        assert_eq!(rucksack.compartments[0].items[3].priority, 4);
        assert_eq!(rucksack.compartments[0].items[4].priority, 5);
        assert_eq!(rucksack.compartments[1].items.len(), 6);
        assert_eq!(rucksack.compartments[1].items[0].priority, 27);
        assert_eq!(rucksack.compartments[1].items[1].priority, 28);
        assert_eq!(rucksack.compartments[1].items[2].priority, 29);
        assert_eq!(rucksack.compartments[1].items[3].priority, 30);
        assert_eq!(rucksack.compartments[1].items[4].priority, 31);
    }
}
