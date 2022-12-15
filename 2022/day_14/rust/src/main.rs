use std::{cmp, collections::HashSet, fs};

struct NextMove<'a> {
    set: &'a mut HashSet<(usize, usize)>,
    current: (usize, usize),
}

impl<'a> NextMove<'a> {
    fn new(set: &'a mut HashSet<(usize, usize)>, current: (usize, usize)) -> Self {
        Self { set, current }
    }
    fn can_move_down(&self) -> bool {
        !self.set.contains(&(self.current.0, self.current.1 + 1))
    }

    fn can_move_left(&self) -> bool {
        !self.set.contains(&(self.current.0 - 1, self.current.1 + 1))
    }

    fn can_move_right(&self) -> bool {
        !self.set.contains(&(self.current.0 + 1, self.current.1 + 1))
    }

    fn update(&mut self) {
        self.set.insert(self.current);
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input string");

    let lines: Vec<Vec<(usize, usize)>> = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            x.split(" -> ")
                .map(|y| {
                    let mut pieces = y.split(",");
                    (
                        pieces
                            .next()
                            .expect("Failed to retrieve first element of pair")
                            .parse::<usize>()
                            .expect("Failed to parse first element of pair"),
                        pieces
                            .next()
                            .expect("Failed to retrieve second element of pair")
                            .parse::<usize>()
                            .expect("Failed to parse second element of pair"),
                    )
                })
                .collect()
        })
        .collect();

    let mut lowest_point = 0;
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for line in lines {
        for i in 0..line.len() - 1 {
            let start = line[i];
            let end = line[i + 1];

            if start.1 > lowest_point {
                lowest_point = start.1;
            }
            if end.1 > lowest_point {
                lowest_point = end.1;
            }

            let min_x = cmp::min(start.0, end.0);
            let min_y = cmp::min(start.1, end.1);
            let max_x = cmp::max(start.0, end.0);
            let max_y = cmp::max(start.1, end.1);

            for j in min_x..=max_x {
                for k in min_y..=max_y {
                    set.insert((j, k));
                }
            }
        }
    }

    let mut set_2 = set.clone();
    let mut sand_count = 0;
    'outer: loop {
        let mut start = (500, 0);

        loop {
            let mut next_move = NextMove::new(&mut set, start);

            if start.1 > lowest_point {
                break 'outer;
            } else if next_move.can_move_down() {
                start.1 += 1;
            } else if next_move.can_move_left() {
                start.0 -= 1;
                start.1 += 1;
            } else if next_move.can_move_right() {
                start.0 += 1;
                start.1 += 1;
            } else {
                next_move.update();
                sand_count += 1;
                break;
            }
        }
    }

    lowest_point += 1;
    let mut sand_count_2 = 0;
    'outer2: loop {
        let mut start = (500, 0);

        loop {
            let mut next_move = NextMove::new(&mut set_2, start);

            if start.1 == lowest_point {
                next_move.update();
                sand_count_2 += 1;
                break;
            } else if next_move.can_move_down() {
                start.1 += 1;
            } else if next_move.can_move_left() {
                start.0 -= 1;
                start.1 += 1;
            } else if next_move.can_move_right() {
                start.0 += 1;
                start.1 += 1;
            } else {
                if start == (500, 0) {
                    sand_count_2 += 1;
                    break 'outer2;
                }
                next_move.update();
                sand_count_2 += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", sand_count);
    println!("Part 2: {}", sand_count_2);
}
