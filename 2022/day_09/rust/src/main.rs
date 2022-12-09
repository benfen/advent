use std::{collections::HashSet, fs};

fn need_move(a: &(i32, i32), b: &(i32, i32)) -> Option<(i32, i32)> {
    let mut x_diff = a.0 - b.0;
    let mut y_diff = a.1 - b.1;
    let x_mag = x_diff.abs();
    let y_mag = y_diff.abs();

    if x_mag > 1 || y_mag > 1 {
        if x_diff > 1 {
            x_diff = 1;
        } else if x_diff < -1 {
            x_diff = -1;
        }
        if y_diff > 1 {
            y_diff = 1;
        } else if y_diff < -1 {
            y_diff = -1;
        }

        Some((x_diff, y_diff))
    } else {
        None
    }
}

fn main() {
    let input: String =
        fs::read_to_string("./input.txt").expect("Failed to retrieve input file from directory");

    let lines: Vec<&str> = input.split("\n").filter(|x| *x != "").collect();

    let mut visited = HashSet::new();
    let mut last_visited = HashSet::new();
    visited.insert((0, 0));
    last_visited.insert((0, 0));

    let mut tails = [(0, 0); 10];

    for line in lines {
        let mut pieces = line.split(" ");

        let dir = pieces.next().expect("No direction attached");
        let distance = pieces
            .next()
            .expect("No magnitude attached")
            .parse::<usize>()
            .expect("Failed to parse magnitude");

        let dir_value = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Invalid input"),
        };

        for _ in 0..distance {
            for i in 1..tails.len() {
                let (l, r) = tails.split_at_mut(i);
                let mut head = &mut l[i - 1];
                let mut tail = &mut r[0];
                if i == 1 {
                    head.0 += dir_value.0;
                    head.1 += dir_value.1;
                }

                match need_move(&head, &tail) {
                    Some((x, y)) => {
                        tail.0 += x;
                        tail.1 += y;

                        if i == 1 {
                            visited.insert(*tail);
                        } else if i == 9 {
                            last_visited.insert(*tail);
                        }
                    }
                    None => {}
                }
            }
        }
    }

    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", last_visited.len());
}
