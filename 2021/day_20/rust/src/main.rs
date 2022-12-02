use std::collections::HashMap;
use std::fs;

fn main() {
    let lines = fs::read_to_string("../input.txt")
        .expect("Failed to read from input file")
        .split("\n")
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let enhancement_arr = lines[0].as_bytes();
    let mut image = HashMap::new();

    for i in 2..lines.len() {
        let bytes = lines[i].as_bytes();
        for j in 0..bytes.len() {
            if bytes[j] == b'#' {
                let key = (j as u64) << 32 | (i as u64) << 16;
                image.insert(key, ());
            }
        }
    }

    for i in 0..2 {
        let keys = image.keys().map(|x| *x).collect::<Vec<_>>();

        for key in keys {
            let x = key >> 32;
            let y = key - (x << 32);

            let mut power = 8;
            let mut result = 0;
            for offset_x in 0..2 {
                for offset_y in 0..2 {
                    if x == 0 && offset_x == 0 || y == 0 && offset_y == 0 {
                        power -= 1;
                        continue;
                    }

                    let new_key = (x + offset_x - 1) << 32 | (y + offset_y - 1);
                    if let Some(val) = image.get(&new_key) {
                        result += 2 ^ power;
                    }

                    power -= 1;
                }
            }
        }
    }

    // for line in lines {
    //     if line == "" {
    //         parsing_points = false;
    //         continue;
    //     }

    //     if parsing_points {
    //         let mut tokens = line.split(",");
    //         points.push((
    //             tokens.next().unwrap().parse::<u32>().unwrap(),
    //             tokens.next().unwrap().parse::<u32>().unwrap(),
    //         ));
    //     } else {
    //         let mut tokens = line.split("=").skip(1);
    //         if line.contains("x") {
    //             folds.push(Fold::X(tokens.next().unwrap().parse::<u32>().unwrap()));
    //         } else {
    //             folds.push(Fold::Y(tokens.next().unwrap().parse::<u32>().unwrap()));
    //         }
    //     }
    // }

    // let transform = |coord: &(u32, u32), only_first_fold: bool| {
    //     let max = if only_first_fold { 1 } else { folds.len() };

    //     let mut end_x = coord.0;
    //     let mut end_y = coord.1;
    //     for i in 0..max {
    //         match folds[i] {
    //             Fold::X(val) => {
    //                 if end_x == val {
    //                     return None;
    //                 } else if end_x > val {
    //                     end_x = val - (end_x - val);
    //                 }
    //             }
    //             Fold::Y(val) => {
    //                 if end_y == val {
    //                     return None;
    //                 } else if end_y > val {
    //                     end_y = val - (end_y - val);
    //                 }
    //             }
    //         }
    //     }

    //     Some((end_x, end_y))
    // };

    // let mut first_map = HashMap::new();
    // let mut second_map = HashMap::new();
    // for point in points {
    //     if let Some(actual_coord) = transform(&point, true) {
    //         let key = (actual_coord.0 as u64) << 32 | (actual_coord.1 as u64);

    //         first_map.insert(key, ());
    //     }

    //     if let Some(actual_coord) = transform(&point, false) {
    //         let key = (actual_coord.0 as u64) << 32 | (actual_coord.1 as u64);

    //         second_map.insert(key, ());
    //     }
    // }

    // println!("Part 1: {}", first_map.len());

    // println!("Part 2:");
    // display_map(&second_map);
}
