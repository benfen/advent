use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
struct Item {
    score: u32,
    elevation: u8,
}

impl Item {
    fn new(ch: char) -> Self {
        Self {
            score: if ch == 'S' { 0 } else { u32::MAX },
            elevation: if ch == 'S' {
                'a' as u8
            } else if ch == 'E' {
                'z' as u8
            } else {
                ch as u8
            },
        }
    }

    fn check(&self, other: &Self) -> bool {
        self.elevation + 1 >= other.elevation && other.score > self.score
    }
}

#[derive(Clone, Debug)]
struct Grid(Vec<Vec<Item>>);

impl Grid {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn next_moves(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut v = Vec::new();

        if x > 0 {
            v.push((x - 1, y));
        }
        if y > 0 {
            v.push((x, y - 1));
        }
        if x < self.0[0].len() - 1 {
            v.push((x + 1, y));
        }
        if y < self.0.len() - 1 {
            v.push((x, y + 1));
        }

        v
    }

    fn find_path(&mut self, start: (usize, usize), end: (usize, usize), a_is_start: bool) {
        let mut move_stack = Vec::new();
        move_stack.push(start);

        while !move_stack.is_empty() {
            let mut next_moves = HashSet::new();

            for m in move_stack.drain(..) {
                for n in self.next_moves(m) {
                    if self.0[m.1][m.0].check(&self.0[n.1][n.0]) {
                        let s = if a_is_start && self.0[n.1][n.0].elevation == 'a' as u8 {
                            0
                        } else {
                            self.0[m.1][m.0].score + 1
                        };

                        if s >= self.0[end.1][end.0].score {
                            continue;
                        }

                        let item = &mut self.0[n.1][n.0];
                        item.score = s;

                        next_moves.insert(n);
                    }
                }
            }

            for nm in next_moves {
                move_stack.push(nm);
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in
        io::BufReader::new(File::open("./input.txt").expect("Failed to open input.txt"))
            .lines()
            .enumerate()
    {
        let mut item_line = Vec::new();
        let l = line.expect("Failed to read line");
        for (j, ch) in l.chars().enumerate() {
            item_line.push(Item::new(ch));

            if ch == 'S' {
                start = (j, i);
            } else if ch == 'E' {
                end = (j, i);
            }
        }

        grid.0.push(item_line);
    }

    let mut second_grid = grid.clone();

    grid.find_path(start, end, false);
    second_grid.find_path(start, end, true);

    println!("Part 1: {}", grid.0[end.1][end.0].score);
    println!("Part 2: {}", second_grid.0[end.1][end.0].score);
}
