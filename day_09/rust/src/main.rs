use std::fs;

fn visit(x: usize, y: usize, arr: &mut Vec<Vec<(u32, bool)>>) -> u32 {
    let mut value = &mut arr[x][y];
    if value.1 || value.0 == 9 {
        return 0;
    }
    value.1 = true;

    1 + if x > 0 { visit(x - 1, y, arr) } else { 0 }
        + if (x + 1) < arr.len() {
            visit(x + 1, y, arr)
        } else {
            0
        }
        + if y > 0 { visit(x, y - 1, arr) } else { 0 }
        + if (y + 1) < arr[0].len() {
            visit(x, y + 1, arr)
        } else {
            0
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
                .map(|y| (y.parse::<u32>().unwrap(), false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut best = (0, 0, 0);
    let mut hole_sum = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            let item = lines[i][j];

            if !(j > 0 && lines[i][j - 1].0 <= item.0)
                && !((j + 1) < lines[i].len() && lines[i][j + 1].0 <= item.0)
                && !(i > 0 && lines[i - 1][j].0 <= item.0)
                && !((i + 1) < lines.len() && lines[i + 1][j].0 <= item.0)
            {
                hole_sum += item.0 + 1;
            }

            let value = visit(i, j, &mut lines);

            if value > best.2 {
                best.0 = best.1;
                best.1 = best.2;
                best.2 = value;
            } else if value > best.1 {
                best.0 = best.1;
                best.1 = value;
            } else if value > best.0 {
                best.0 = value;
            }
        }
    }

    println!("Part 1: {}", hole_sum);
    println!("Part 2: {}", best.0 * best.1 * best.2);
}
