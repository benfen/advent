use std::fs;

fn visit(x: usize, y: usize, arr: &mut Vec<Vec<u32>>) {
    if x >= arr.len() || y >= arr[x].len() {
        return;
    }

    let item = &mut arr[x][y];
    if *item >= 10 {
        return;
    }

    *item += 1;

    if *item >= 10 {
        let x_min = x > 0;
        let x_max = x < arr.len() - 1;
        let y_min = y > 0;
        let y_max = y < arr[x].len() - 1;

        if x_min {
            visit(x - 1, y, arr);

            if y_min {
                visit(x - 1, y - 1, arr);
            }

            if y_max {
                visit(x - 1, y + 1, arr);
            }
        }

        if x_max {
            visit(x + 1, y, arr);

            if y_min {
                visit(x + 1, y - 1, arr);
            }

            if y_max {
                visit(x + 1, y + 1, arr);
            }
        }

        if y_min {
            visit(x, y - 1, arr);
        }

        if y_max {
            visit(x, y + 1, arr);
        }
    }
}

fn main() {
    let mut lines = fs::read_to_string("../input.txt")
        .expect("Failed to read from input file")
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            x.split("")
                .filter(|y| *y != "")
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_flash_count = 0;
    let octopus_count = lines.len() * lines[0].len();
    let mut i = 0;

    loop {
        let mut flashes = 0;
        for x in 0..lines.len() {
            for y in 0..lines[x].len() {
                visit(x, y, &mut lines);
            }
        }

        for x in 0..lines.len() {
            for y in 0..lines[x].len() {
                let item = &mut lines[x][y];
                if *item >= 10 {
                    *item = 0;
                    flashes += 1;
                }
            }
        }

        if i < 100 {
            total_flash_count += flashes;
        }

        if flashes == octopus_count {
            break;
        }

        i += 1;
    }

    println!("Part 1: {}", total_flash_count);
    println!("Part 2: {}", i + 1);
}
