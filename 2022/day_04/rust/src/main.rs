use std::fs;

fn main() {
    let assignments = fs::read_to_string("./input.txt")
        .expect("Failed to retrieve input file from directory")
        .split("\n")
        .filter(|x| *x != "")
        .map(|line| {
            let mut pairs = line.split(",");

            let create_pair = |pair: &str| {
                let mut times = pair.split("-");

                [
                    times
                        .next()
                        .expect("First element of pair missing")
                        .parse::<u32>()
                        .expect("Failed to parse first element of pair"),
                    times
                        .next()
                        .expect("Second element of pair missing")
                        .parse::<u32>()
                        .expect("Failed to parse second element of pair"),
                ]
            };

            [
                create_pair(pairs.next().expect("First pair missing")),
                create_pair(pairs.next().expect("Second pair missing")),
            ]
        })
        .collect::<Vec<[[u32; 2]; 2]>>();

    let mut fully_overlapping_pair_count = 0;
    let mut partially_overlapping_pair_count = 0;

    for assignment in assignments {
        if (assignment[0][0] <= assignment[1][0] && assignment[0][1] >= assignment[1][1])
            || (assignment[1][0] <= assignment[0][0] && assignment[1][1] >= assignment[0][1])
        {
            fully_overlapping_pair_count += 1;
            partially_overlapping_pair_count += 1;
        } else if (assignment[0][1] >= assignment[1][0] && assignment[0][1] <= assignment[1][1])
            || (assignment[0][0] >= assignment[1][0] && assignment[0][0] <= assignment[1][1])
            || (assignment[1][0] >= assignment[0][1] && assignment[1][0] <= assignment[0][0])
            || (assignment[1][1] >= assignment[0][1] && assignment[1][1] <= assignment[0][0])
        {
            partially_overlapping_pair_count += 1;
        }
    }

    println!("Part 1: {}", fully_overlapping_pair_count);
    println!("Part 2: {}", partially_overlapping_pair_count);
}
