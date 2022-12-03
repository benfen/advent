use std::collections::HashSet;
use std::fs;

fn main() {
    let l =
        fs::read_to_string("./input.txt").expect("Failed to retrieve input file from directory");

    let lines = l.split("\n").collect::<Vec<&str>>();
    let mut half_set = HashSet::new();
    let mut set = HashSet::new();
    let mut previous_sets = [HashSet::new(), HashSet::new()];
    let mut sum: u32 = 0;
    let mut group_sum: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        // All input is valid ASCII
        let half_line_length = line.len() / 2;
        let mut copied_char = '0';
        let mut group_char = '0';

        for (j, c) in line.chars().enumerate() {
            if i % 3 == 2 {
                if previous_sets[0].contains(&c) && previous_sets[1].contains(&c) {
                    group_char = c;
                }
            }

            set.insert(c);

            if j < half_line_length {
                half_set.insert(c);
            } else {
                if half_set.contains(&c) {
                    copied_char = c;
                }
            }
        }

        // Quick and dirty ASCII translations
        if copied_char != '0' {
            let val = (copied_char as u8) as u32;

            sum += if val >= 97 { val - 96 } else { val - 38 };
        }

        if i % 3 == 2 && group_char != '0' {
            let group_val = (group_char as u8) as u32;
            group_sum += if group_val >= 97 {
                group_val - 96
            } else {
                group_val - 38
            };
        }

        if i % 3 == 0 {
            previous_sets[0] = set;
        } else if i % 3 == 1 {
            previous_sets[1] = set;
        }

        set = HashSet::new();
        half_set.clear();
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", group_sum);
}
