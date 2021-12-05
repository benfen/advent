use std::collections::HashMap;
use std::fs;

fn update_map(map: &mut HashMap<u64, u32>, key: u64) {
    if let Some(count) = map.get_mut(&key) {
        *count += 1;
    } else {
        map.insert(key, 1u32);
    }
}

fn main() {
    let lines = fs::read_to_string("../input.txt")
        .expect("Failed to read from input file")
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            let mut tokens = x.split(" -> ");
            let mut left = tokens.next().unwrap().split(",");
            let mut right = tokens.next().unwrap().split(",");
            return (
                (
                    left.next().unwrap().parse::<u32>().unwrap(),
                    left.next().unwrap().parse::<u32>().unwrap(),
                ),
                (
                    right.next().unwrap().parse::<u32>().unwrap(),
                    right.next().unwrap().parse::<u32>().unwrap(),
                ),
            );
        })
        .collect::<Vec<_>>();

    let mut point_count: HashMap<u64, u32> = HashMap::new();
    let mut full_point_count: HashMap<u64, u32> = HashMap::new();

    for line in lines {
        let ((x0, y0), (x1, y1)) = line;
        if x0 == x1 {
            let range = if y0 > y1 { y1..=y0 } else { y0..=y1 };

            for value in range {
                let key = (value as u64) << 32 | (x0 as u64);

                update_map(&mut point_count, key);
                update_map(&mut &mut full_point_count, key);
            }
        } else if y0 == y1 {
            let range = if x0 > x1 { x1..=x0 } else { x0..=x1 };

            for value in range {
                let key = (value as u64) | (y0 as u64) << 32;

                update_map(&mut point_count, key);
                update_map(&mut &mut full_point_count, key);
            }
        } else {
            let mut start = (x0, y0);
            loop {
                let key = (start.0 as u64) | (start.1 as u64) << 32;
                update_map(&mut full_point_count, key);

                if start.0 == x1 {
                    break;
                }

                if x0 > x1 {
                    start.0 -= 1;
                } else {
                    start.0 += 1;
                }

                if y0 > y1 {
                    start.1 -= 1;
                } else {
                    start.1 += 1;
                }
            }
        }
    }

    let intersection_count = point_count.values().filter(|x| **x > 1).count();
    println!("Part 1: {}", intersection_count);

    let full_intersection_count = full_point_count.values().filter(|x| **x > 1).count();
    println!("Part 2: {}", full_intersection_count);
}
