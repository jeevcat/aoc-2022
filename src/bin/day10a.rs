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
    let mut inspect_cycle = 20;
    let mut total_signal_strength = 0;

    for line in reader.lines().flatten() {
        let op: Op = line.parse().unwrap();

        // if applying this op would take us past the cycle we want to inspect, output the current
        // value of the register
        if vm.pc + op.cycles() >= inspect_cycle {
            let signal_strength = vm.x * inspect_cycle as i32;
            total_signal_strength += signal_strength;
            inspect_cycle += 40;
        }

        vm.apply(&op);
    }

    dbg!(total_signal_strength);
}
