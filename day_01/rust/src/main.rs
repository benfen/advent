use std::fs;

fn main() {
    let depths = fs::read_to_string("../input.txt")
        .expect("Failed to retrieve input file from super directory")
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            x.parse::<u64>()
                .expect("Failed to parse input from the file into a number")
        })
        .collect::<Vec<u64>>();

    // Store a reference to the last depth to compare current depth against
    // Update on every iteration
    let mut last = depths[0];
    let mut count = 0;
    for &depth in &depths {
        if depth > last {
            count += 1;
        }
        last = depth;
    }

    println!("Part 1: {}", count);

    // The only changes to the sliding window are the first and new elements
    // If the new element is larger, depth increased and vice versa
    // Therefore, we compare elements three apart
    let max = depths.len() - 3;
    let mut count2 = 0;
    for i in 0..max {
        if depths[i + 3] > depths[i] {
            count2 += 1;
        }
    }

    println!("Part 2: {}", count2);
}
