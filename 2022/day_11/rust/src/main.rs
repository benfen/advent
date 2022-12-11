use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
struct Monkey {
    count: u64,
    div: u64,
    items: Vec<u64>,
    op_value: (String, Option<u64>),
    throw: (usize, usize),
}

impl Monkey {
    fn new() -> Self {
        Self {
            count: 0,
            div: 0,
            items: Vec::new(),
            op_value: ("".to_owned(), None),
            throw: (0, 0),
        }
    }

    fn op(&self, old: u64) -> u64 {
        match (self.op_value.0.as_str(), self.op_value.1) {
            ("+", None) => old + old,
            ("*", None) => old * old,
            ("+", Some(v)) => old + v,
            ("*", Some(v)) => old * v,
            _ => panic!("Invalid operation"),
        }
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut overall_mod = 1;

    for (i, line) in
        io::BufReader::new(File::open("./input.txt").expect("Failed to open input.txt"))
            .lines()
            .enumerate()
    {
        let i_mod = i % 7;
        let l = line.expect("Failed to read line");

        match i_mod {
            1 => {
                let pieces = l.split(" items: ");
                let items = pieces.skip(1).next().expect("Failed to get items");

                let mut monkey = Monkey::new();
                for item in items.split(", ") {
                    monkey.items.push(
                        item.parse::<u64>()
                            .expect("Failed to parse item into number"),
                    );
                }
                monkeys.push(monkey);
            }
            2 => {
                let monkey = monkeys.last_mut().expect("Failed to retrieve last monkey");
                let pieces = l.split("= old ");
                let mut items = pieces
                    .skip(1)
                    .next()
                    .expect("Failed to get operation")
                    .split(" ");

                let operation = items.next().expect("Failed to get kind for operation");
                let value = items
                    .next()
                    .expect("Failed to get value for operation")
                    .parse::<u64>();

                monkey.op_value = (operation.to_owned(), value.ok());
            }
            3 => {
                let monkey = monkeys.last_mut().expect("Failed to retrieve last monkey");
                let pieces = l.split(" by ");

                monkey.div = pieces
                    .skip(1)
                    .next()
                    .expect("Failed to retrieve divisor")
                    .parse::<u64>()
                    .expect("Failed to parse divisor");
                overall_mod *= monkey.div;
            }
            4 | 5 => {
                let monkey = monkeys.last_mut().expect("Failed to retrieve last monkey");
                let pieces = l.split(" monkey ");

                let value = pieces
                    .skip(1)
                    .next()
                    .expect("Failed to retrieve target for throw")
                    .parse::<usize>()
                    .expect("Failed to parse target for throw");

                if i_mod == 4 {
                    monkey.throw.0 = value;
                } else {
                    monkey.throw.1 = value;
                }
            }
            _ => {
                continue;
            }
        }
    }

    let mut second_monkeys = monkeys.clone();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut t_target = Vec::new();
            let mut f_target = Vec::new();

            // Some silly scoping to deal with borrow rules
            let (t_target_index, f_target_index) = {
                let monkey = &mut monkeys[i];
                monkey.items.reverse();

                while !monkey.items.is_empty() {
                    monkey.count += 1;
                    let item = monkey.items.pop().unwrap();
                    let worry_level = monkey.op(item) / 3;

                    if worry_level % monkey.div == 0 {
                        t_target.push(worry_level);
                    } else {
                        f_target.push(worry_level);
                    }
                }
                (monkey.throw.0, monkey.throw.1)
            };

            monkeys[t_target_index].items.append(&mut t_target);
            monkeys[f_target_index].items.append(&mut f_target);
        }
    }

    for _ in 0..10000 {
        for i in 0..second_monkeys.len() {
            let mut t_target = Vec::new();
            let mut f_target = Vec::new();

            // Some silly scoping to deal with borrow rules
            let (t_target_index, f_target_index) = {
                let monkey = &mut second_monkeys[i];
                monkey.items.reverse();

                while !monkey.items.is_empty() {
                    monkey.count += 1;
                    let item = monkey.items.pop().unwrap();
                    let worry_level = monkey.op(item) % overall_mod;

                    if worry_level % monkey.div == 0 {
                        t_target.push(worry_level);
                    } else {
                        f_target.push(worry_level);
                    }
                }
                (monkey.throw.0, monkey.throw.1)
            };

            second_monkeys[t_target_index].items.append(&mut t_target);
            second_monkeys[f_target_index].items.append(&mut f_target);
        }
    }

    let mut max = (0, 0);
    for monkey in monkeys {
        if monkey.count > max.0 {
            max.1 = max.0;
            max.0 = monkey.count;
        } else if monkey.count > max.1 {
            max.1 = monkey.count;
        }
    }

    println!("Part 1: {}", max.0 * max.1);

    for monkey in second_monkeys {
        if monkey.count > max.0 {
            max.1 = max.0;
            max.0 = monkey.count;
        } else if monkey.count > max.1 {
            max.1 = monkey.count;
        }
    }

    println!("Part 2: {}", max.0 * max.1);
}
