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
        if s.is_empty() {
            return Err(());
        }

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

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut packets: Vec<Packet> = BufReader::new(File::open("input13.txt").unwrap())
        .lines()
        .flatten()
        .flat_map(|line| line.parse())
        .collect();

    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    packets.push(divider1.clone());
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(divider2.clone());

    packets.sort();

    let index1 = packets.iter().position(|p| p == &divider1).unwrap();
    let index2 = packets.iter().position(|p| p == &divider2).unwrap();
    dbg!((index1 + 1) * (index2 + 1));
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
