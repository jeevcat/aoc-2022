use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Op {
    Noop,
    Addx(i32),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        match split.next().unwrap() {
            "noop" => Ok(Op::Noop),
            "addx" => Ok(Op::Addx(split.next().unwrap().parse().unwrap())),
            op => panic!("bad op '{op}'"),
        }
    }
}

impl Op {
    fn cycles(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

struct Vm {
    // Single register
    x: i32,
    // Program counter
    pc: usize,
}

impl Vm {
    fn new() -> Self {
        Self { x: 1, pc: 0 }
    }

    fn apply(&mut self, op: &Op) {
        match op {
            Op::Noop => {}
            Op::Addx(val) => {
                self.x += val;
            }
        }
        self.pc += op.cycles();
    }
}

fn main() {
    let file = File::open("input10.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vm = Vm::new();
    let mut pixel = 0;

    for line in reader.lines().flatten() {
        let op: Op = line.parse().unwrap();

        while pixel < vm.pc + op.cycles() {
            let col = (pixel % 40) as i32;
            if col == 0 {
                println!();
            }
            if vm.x >= col - 1 && vm.x <= col + 1 {
                print!("#");
            } else {
                print!(" ");
            }
            pixel += 1;
        }

        vm.apply(&op);
    }
}
