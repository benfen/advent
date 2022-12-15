use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut cals = 0;
    let mut heap = BinaryHeap::new();
    for line in lines {
        if line == "" {
            heap.push(cals);
            cals = 0;
        } else {
            let val = line.parse::<u32>().expect("Unable to parse val from file");
            cals += val;
        }
    }

    let mut total_cals = 0;

    for i in 0..3 {
        if let Some(sum) = heap.pop() {
            total_cals += sum;
            if i == 0 {
                println!("Part 1: {total_cals}");
            }
        }
    }

    println!("Part 2: {total_cals}");
}
