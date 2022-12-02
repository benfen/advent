use std::fs;

fn main() {
    let l = fs::read_to_string("../input.txt")
        .expect("Failed to retrieve input file from super directory");

    let lines = l.split("\n").filter(|x| *x != "").collect::<Vec<&str>>();

    let mut sum = 0;
    let mut second_sum = 0;

    for line in lines {
        let v = match line {
            "A X" => [4, 3],
            "B X" => [1, 1],
            "C X" => [7, 2],
            "A Y" => [8, 4],
            "B Y" => [5, 5],
            "C Y" => [2, 6],
            "A Z" => [3, 8],
            "B Z" => [9, 9],
            "C Z" => [6, 7],
            _ => [0, 0],
        };

        sum += v[0];
        second_sum += v[1];
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", second_sum);
}
