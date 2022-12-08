use std::fs;

#[derive(Debug)]
struct Tree {
    value: i32,
    north: (i32, i32),
    east: (i32, i32),
    south: (i32, i32),
    west: (i32, i32),
}

impl Tree {
    fn new(value: i32) -> Self {
        Self {
            value,
            north: (0, 0),
            east: (0, 0),
            south: (0, 0),
            west: (0, 0),
        }
    }
}

#[derive(Debug)]
struct Forest(Vec<Vec<Tree>>);

impl Forest {
    fn process_dir<T, U>(
        &mut self,
        outer_iter: T,
        inner_iter: U,
        reverse_flow: bool,
        mutate: fn(t: &mut Tree, value: (i32, i32)),
    ) where
        T: Iterator<Item = usize>,
        U: Iterator<Item = usize> + Clone,
    {
        for i in outer_iter {
            let mut highest = -1;
            let mut higher_than_stack: Vec<(i32, i32)> = Vec::new();

            for j in inner_iter.clone() {
                let item = if reverse_flow {
                    &mut self.0[j][i]
                } else {
                    &mut self.0[i][j]
                };
                let last_highest = highest;
                if item.value > highest {
                    highest = item.value
                }

                let mut higher_than = 0;
                while item.value > higher_than_stack.last().unwrap_or(&(10, 0)).0 {
                    higher_than += higher_than_stack.pop().unwrap().1 + 1;
                }

                // This is super ugly, but it serves to increment the amount of trees someone can see
                // iff there is still a tree left in their line of sight (read the stack is not empty)
                mutate(
                    item,
                    (
                        last_highest,
                        if higher_than_stack.is_empty() {
                            higher_than
                        } else {
                            higher_than + 1
                        },
                    ),
                );
                higher_than_stack.push((item.value, higher_than));
            }
        }
    }

    fn process(&mut self) {
        self.process_dir(
            0..self.0.len(),
            0..self.0[0].len(),
            false,
            |t: &mut Tree, value: (i32, i32)| {
                t.west = value;
            },
        );
        self.process_dir(
            0..self.0.len(),
            (0..self.0[0].len()).rev(),
            false,
            |t: &mut Tree, value: (i32, i32)| {
                t.east = value;
            },
        );
        self.process_dir(
            0..self.0[0].len(),
            0..self.0.len(),
            true,
            |t: &mut Tree, value: (i32, i32)| {
                t.south = value;
            },
        );
        self.process_dir(
            0..self.0[0].len(),
            (0..self.0.len()).rev(),
            true,
            |t: &mut Tree, value: (i32, i32)| {
                t.north = value;
            },
        );
    }

    fn get_visible(&self) -> i32 {
        let mut count = 0;
        for row in self.0.iter() {
            for item in row {
                if item.value > item.north.0
                    || item.value > item.east.0
                    || item.value > item.south.0
                    || item.value > item.west.0
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn best_scenic_score(&self) -> i32 {
        let mut highest = 0;
        for row in self.0.iter().skip(1).rev().skip(1) {
            for item in row.iter().skip(1).rev().skip(1) {
                let score = item.north.1 * item.east.1 * item.south.1 * item.west.1;
                if score > highest {
                    highest = score;
                }
            }
        }
        highest
    }
}

fn main() {
    let input: String =
        fs::read_to_string("./input.txt").expect("Failed to retrieve input file from directory");

    let lines: Vec<&str> = input.split("\n").filter(|x| *x != "").collect();

    let mut forest = Forest(Vec::new());
    for line in lines {
        let mut tree_line = Vec::new();
        for ch in line.chars() {
            tree_line.push(Tree::new(
                ch.to_digit(10)
                    .expect("Failed to parse number from character") as i32,
            ));
        }
        forest.0.push(tree_line);
    }

    forest.process();

    println!("Part 1: {}", forest.get_visible());
    println!("Part 2: {}", forest.best_scenic_score());
}
