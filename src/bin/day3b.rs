use std::{
    boxed::Box,
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Group {
    rucksacks: [Rucksack; 3],
}

struct Rucksack {
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

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            items: s
                .chars()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // load the input file and loop through its lines
    let file = File::open("input3.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut lines = reader.lines().flatten();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let group = Group {
            rucksacks: [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()],
        };
        let common = group.rucksacks[0]
            .items
            .iter()
            .filter(|i| group.rucksacks[1].items.contains(i))
            .find(|i| group.rucksacks[2].items.contains(i))
            .unwrap();
        total += common.priority;
    }
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
    fn test_parse_rucksack() {
        let rucksack: Rucksack = "abcdefABCDEF".parse().unwrap();
        assert_eq!(rucksack.items.len(), 12);
    }
}
