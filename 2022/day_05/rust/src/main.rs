use std::fs;

fn main() {
    let input =
        fs::read_to_string("./input.txt").expect("Failed to retrieve input file from directory");

    let mut lines = input.split("\n").collect::<Vec<&str>>();
    lines.reverse();

    let mut handling_moves = true;
    let mut moves: Vec<[u32; 3]> = Vec::new();
    let mut columns: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if line == "" {
            handling_moves = false;
            continue;
        } else if handling_moves {
            let move_pieces: Vec<&str> = line.split(" ").collect();

            moves.push([
                move_pieces[1]
                    .parse::<u32>()
                    .expect("Failed to parse first segment of move"),
                move_pieces[3]
                    .parse::<u32>()
                    .expect("Failed to parse second segment of move"),
                move_pieces[5]
                    .parse::<u32>()
                    .expect("Failed to parse third segment of move"),
            ])
        } else {
            if columns.len() == 0 {
                for c in line.chars() {
                    if c != ' ' {
                        columns.push(Vec::new());
                    }
                }
            } else {
                let chars: Vec<char> = line.chars().collect();

                for i in 0..columns.len() {
                    let index = 4 * i + 1;

                    if index >= chars.len() {
                        break;
                    }

                    if chars[index] != ' ' {
                        columns[i].push(chars[index]);
                    }
                }
            }
        }
    }

    let mut second_columns = Vec::new();
    for column in &columns {
        second_columns.push(column.clone());
    }

    moves.reverse();

    for next_move in &moves {
        let start_index = (next_move[1] - 1) as usize;
        let end_index = (next_move[2] - 1) as usize;

        for _ in 0..next_move[0] {
            let elem = columns[start_index].pop().unwrap();
            columns[end_index].push(elem);
        }

        let mut temp_arr = Vec::new();
        for _ in 0..next_move[0] {
            let elem = second_columns[start_index].pop().unwrap();
            temp_arr.push(elem);
        }

        temp_arr.reverse();
        second_columns[end_index].append(&mut temp_arr);
    }

    let tops: String = columns.iter_mut().filter_map(|c| c.pop()).collect();
    let second_tops: String = second_columns.iter_mut().filter_map(|c| c.pop()).collect();

    println!("Part 1: {}", tops);
    println!("Part 2: {}", second_tops);
}
