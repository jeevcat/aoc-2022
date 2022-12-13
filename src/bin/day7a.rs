use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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
    for line in buffer.lines().flatten() {
        let line: Line = line.parse().unwrap();
        let mut directory_stack = vec![];
        match line {
            Line::Command(Command::ChangeDirectory { path }) => match path.as_str() {
                ".." => {
                    directory_stack.pop();
                }
                "/" => {}
                _ => {
                    directory_stack.push(path);
                }
            },
            Line::Command(Command::ListDirectory) => {
                println!("ls");
            }
            Line::Listing(Listing::File { name, size }) => {
                println!("file {} {}", name, size);
            }
            Line::Listing(Listing::Directory { name }) => {
                println!("dir {}", name);
            }
        }
    }
}
