use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("./input.txt").expect("Failed to retrieve input file from directory");

    let lines: Vec<&str> = input.split("\n").filter(|x| *x != "").collect();

    let mut signal_strength: i64 = 0;
    let mut cycle: i64 = 0;
    let mut value: i64 = 1;
    let mut sprites = Vec::new();

    let mut increment_check_cycle = |v: Option<i64>| {
        cycle += 1;
        if cycle % 40 == 20 {
            signal_strength += cycle * value;
        }

        let c = (cycle - 1) % 40 + 1;
        if c >= value && c <= value + 2 {
            sprites.push('#');
        } else {
            sprites.push('.');
        }

        match v {
            Some(addx_value) => {
                value += addx_value;
            }
            None => {}
        }
    };

    for line in lines {
        if line == "noop" {
            increment_check_cycle(None);
        } else {
            let pieces = line.split(" ");
            let addx_value = pieces
                .skip(1)
                .next()
                .expect("Failed to retrieve addx value")
                .parse::<i64>()
                .expect("Failed to parse addx value");

            increment_check_cycle(None);
            increment_check_cycle(Some(addx_value));
        }
    }

    println!("Part 1: {}", signal_strength);

    println!("Part 2:");
    for (i, sprite) in sprites.iter().enumerate() {
        if i % 40 == 0 && i != 0 {
            println!();
        }
        print!("{}", sprite);
    }
    println!();
    // println!("Part 2: {}", last_visited.len());
}
