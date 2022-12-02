use std::fs;

fn main() {
    let l = fs::read_to_string("../input.txt")
        .expect("Failed to retrieve input file from super directory");

    let lines = l.split("\n").collect::<Vec<&str>>();

    let mut next_sum = 0;
    let mut highest_sum = [0, 0, 0];

    let mut update_highest = |next: u64| {
        if next > highest_sum[0] {
            highest_sum[2] = highest_sum[1];
            highest_sum[1] = highest_sum[0];
            highest_sum[0] = next;
        } else if next > highest_sum[1] {
            highest_sum[2] = highest_sum[1];
            highest_sum[1] = next;
        } else if next > highest_sum[2] {
            highest_sum[2] = next;
        }
    };

    for line in lines {
        if line == "" {
            update_highest(next_sum);
            next_sum = 0
        } else {
            let value = line
                .parse::<u64>()
                .expect("Failed to parse input from the file into a number");
            next_sum += value;
        }
    }

    update_highest(next_sum);

    println!("Part 1: {}", highest_sum[0]);
    println!(
        "Part 2: {}",
        highest_sum[0] + highest_sum[1] + highest_sum[2]
    );
}
