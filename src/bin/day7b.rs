use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const TOTAL_SPACE: u64 = 70_000_000;
const NEEDED_SPACE: u64 = 30_000_000;

enum Line {
    Command(Command),
    Listing(Listing),
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('$') {
            Self::Command(s.parse::<Command>()?)
        } else {
            Self::Listing(s.parse::<Listing>()?)
        })
    }
}

enum Command {
    ChangeDirectory { path: String },
    ListDirectory,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace().skip(1);
        match iter.next() {
            Some("cd") => Ok(Self::ChangeDirectory {
                path: iter.next().unwrap().to_owned(),
            }),
            Some("ls") => Ok(Self::ListDirectory),
            _ => Err(()),
        }
    }
}

enum Listing {
    File { name: String, size: u64 },
    Directory { name: String },
}

impl FromStr for Listing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next() {
            Some("dir") => Ok(Self::Directory {
                name: iter.next().unwrap().to_owned(),
            }),
            Some(size) => Ok(Self::File {
                name: iter.next().unwrap().to_owned(),
                size: size.parse().unwrap(),
            }),
            _ => Err(()),
        }
    }
}

fn main() {
    let buffer = BufReader::new(File::open("input7.txt").unwrap());
    let mut directories: HashMap<String, u64> = HashMap::new();
    let mut directory_stack = vec![];
    for line in buffer.lines().flatten() {
        let line: Line = line.parse().unwrap();
        match line {
            Line::Command(Command::ChangeDirectory { path }) => match path.as_str() {
                ".." => {
                    directory_stack.pop();
                }
                "/" => directory_stack.clear(),
                _ => {
                    directory_stack.push(path);
                }
            },
            Line::Command(Command::ListDirectory) => {
                // do nothing
            }
            Line::Listing(Listing::File { name: _, size }) => {
                for i in 0..=directory_stack.len() {
                    let path = directory_stack[..i].join("/");
                    *directories.entry(path).or_default() += size;
                }
            }
            Line::Listing(Listing::Directory { .. }) => {
                // do nothing
            }
        }
    }
    let used_space = directories[""];
    let free_space = TOTAL_SPACE - used_space;
    let delete_at_least = NEEDED_SPACE - free_space;

    dbg!(used_space, free_space, delete_at_least);

    let deleted: u64 = directories
        .into_iter()
        .filter_map(|(_, size)| {
            if size >= delete_at_least {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    dbg!(deleted);
}
