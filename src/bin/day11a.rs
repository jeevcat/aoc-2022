use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    str::FromStr,
};

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: i32,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn receive(&mut self, value: i32) {
        self.items.push(value);
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn last_num<'a, T>(lines: &mut impl Iterator<Item = &'a str>) -> T
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .trim_matches(|c: char| !c.is_numeric())
                .parse()
                .unwrap()
        }

        let mut lines = s.split('\n').skip(1);
        let items = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(2)
            .map(|x| x.trim_end_matches(',').parse().unwrap())
            .collect();

        let mut words = lines.next().unwrap().split_whitespace().skip(4);
        let operator = words.next().unwrap();
        let operand = words.next().unwrap();
        let operation: Box<dyn Fn(i32) -> i32> = match operand {
            "old" => match operator {
                "*" => Box::new(|x| x * x),
                "+" => Box::new(|x| x + x),
                _ => panic!("bad op"),
            },
            num => {
                let num: i32 = num.parse().unwrap();
                match operator {
                    "*" => Box::new(move |x| x * num),
                    "/" => Box::new(move |x| x / num),
                    "+" => Box::new(move |x| x + num),
                    "-" => Box::new(move |x| x - num),
                    _ => panic!("bad op"),
                }
            }
        };
        let test = last_num(&mut lines);
        let true_target = last_num(&mut lines);
        let false_target = last_num(&mut lines);

        Ok(Self {
            items,
            operation,
            test,
            true_target,
            false_target,
        })
    }
}

fn main() {
    let contents = std::fs::read_to_string("input11.txt").unwrap();
    let mut monkeys: Vec<_> = contents.split("\n\n").flat_map(Monkey::from_str).collect();
    let mut inspections: HashMap<usize, u32> = HashMap::new();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            for mut item in items {
                // Monkey inspects the item
                item = (monkeys[i].operation)(item);
                *inspections.entry(i).or_default() += 1;

                // be relieved
                item /= 3;

                // test
                let target = if item % monkeys[i].test == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };

                // throw
                monkeys[target].receive(item);
            }
        }
    }
    let mut heap: BinaryHeap<_> = inspections.values().collect();
    dbg!(heap.pop().unwrap() * heap.pop().unwrap());
}
