use std::collections::HashMap;
use std::fs;

enum Fold {
    X(u32),
    Y(u32),
}

fn display_map(map: &HashMap<u64, ()>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for key in map.keys() {
        let x = key >> 32;
        let y = key - (x << 32);

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }
    }

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            let key = (x as u64) << 32 | (y as u64);

            if map.contains_key(&key) {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!("");
    }
}

fn main() {
    let lines = fs::read_to_string("../input.txt")
        .expect("Failed to read from input file")
        .split("\n")
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let mut parsing_points = true;
    let mut points = Vec::new();
    let mut folds = Vec::new();

    for line in lines {
        if line == "" {
            parsing_points = false;
            continue;
        }

        if parsing_points {
            let mut tokens = line.split(",");
            points.push((
                tokens.next().unwrap().parse::<u32>().unwrap(),
                tokens.next().unwrap().parse::<u32>().unwrap(),
            ));
        } else {
            let mut tokens = line.split("=").skip(1);
            if line.contains("x") {
                folds.push(Fold::X(tokens.next().unwrap().parse::<u32>().unwrap()));
            } else {
                folds.push(Fold::Y(tokens.next().unwrap().parse::<u32>().unwrap()));
            }
        }
    }

    let transform = |coord: &(u32, u32), only_first_fold: bool| {
        let max = if only_first_fold { 1 } else { folds.len() };

        let mut end_x = coord.0;
        let mut end_y = coord.1;
        for i in 0..max {
            match folds[i] {
                Fold::X(val) => {
                    if end_x == val {
                        return None;
                    } else if end_x > val {
                        end_x = val - (end_x - val);
                    }
                }
                Fold::Y(val) => {
                    if end_y == val {
                        return None;
                    } else if end_y > val {
                        end_y = val - (end_y - val);
                    }
                }
            }
        }

        Some((end_x, end_y))
    };

    let mut first_map = HashMap::new();
    let mut second_map = HashMap::new();
    for point in points {
        if let Some(actual_coord) = transform(&point, true) {
            let key = (actual_coord.0 as u64) << 32 | (actual_coord.1 as u64);

            first_map.insert(key, ());
        }

        if let Some(actual_coord) = transform(&point, false) {
            let key = (actual_coord.0 as u64) << 32 | (actual_coord.1 as u64);

            second_map.insert(key, ());
        }
    }

    println!("Part 1: {}", first_map.len());

    println!("Part 2:");
    display_map(&second_map);
}
