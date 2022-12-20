use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

fn parse_packet(s: &str) -> (Packet, &str) {
    if let Some(rest) = s.strip_prefix('[') {
        parse_list(rest)
    } else {
        parse_num(s)
    }
}

fn parse_num(s: &str) -> (Packet, &str) {
    let index = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    (
        Packet::Integer(s[..index].parse::<i32>().unwrap()),
        &s[index..],
    )
}

fn parse_list(s: &str) -> (Packet, &str) {
    let mut list = Vec::new();
    if let Some(rest) = s.strip_prefix(']') {
        return (Packet::List(list), rest);
    }

    let mut s = s;
    loop {
        s = {
            let (packet, rest) = parse_packet(s);
            list.push(packet);
            rest
        };

        if let Some(rest) = s.strip_prefix(']') {
            return (Packet::List(list), rest);
        } else if let Some(rest) = s.strip_prefix(',') {
            s = rest;
        } else {
            panic!("invalid input");
        }
    }
}

impl FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (packet, rest) = parse_packet(s);
        assert!(rest.is_empty());
        Ok(packet)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.partial_cmp(right)?,
            (Packet::List(left), Packet::List(right)) => {
                for (left, right) in left.iter().zip(right.iter()) {
                    let cmp = left.partial_cmp(right)?;
                    if cmp != Ordering::Equal {
                        return Some(cmp);
                    }
                }
                left.len().cmp(&right.len())
            }
            (Packet::Integer(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)?
            }
            (Packet::List(_), Packet::Integer(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))?
            }
        })
    }
}

fn main() {
    let mut lines = BufReader::new(File::open("input13.txt").unwrap())
        .lines()
        .flatten();

    let (mut index, mut total) = (1, 0);
    while let (Some(left), Some(right), _) = (lines.next(), lines.next(), lines.next()) {
        let (left, right): (Packet, Packet) = (left.parse().unwrap(), right.parse().unwrap());

        if left < right {
            total += index;
        }

        index += 1;
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "[[1],[2,3,4]]";
        let packet: Packet = line.parse().unwrap();
        dbg!(packet);
    }
}
