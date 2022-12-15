use std::{collections::HashSet, fs};

use regex::Regex;

enum Overlap {
    Contains,
    Contained,
    LowerContained,
    UpperContained,
    Less,
    Greater,
}

fn check_overlap(a: &[i64; 2], b: &[i64; 2]) -> Overlap {
    if a[0] <= b[0] && a[1] >= b[1] {
        Overlap::Contains
    } else if a[0] >= b[0] && a[1] <= b[1] {
        Overlap::Contained
    } else if a[0] >= b[0] && a[1] >= b[1] {
        Overlap::LowerContained
    } else if a[0] <= b[0] && a[1] <= b[1] {
        Overlap::UpperContained
    } else if a[0] >= b[1] {
        Overlap::Greater
    } else {
        Overlap::Less
    }
}

fn within(a: i64, b: &[i64; 2]) -> bool {
    a >= b[0] && a <= b[1]
}

fn manhattan_distance(a: [i64; 2], b: [i64; 2]) -> i64 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

struct BeaconRow {
    beacons: HashSet<i64>,
    range: Vec<[i64; 2]>,
    target_row: i64,
}

impl BeaconRow {
    fn new(target_row: i64) -> Self {
        Self {
            beacons: HashSet::new(),
            range: Vec::new(),
            target_row,
        }
    }

    fn add_beacon_pair(&mut self, sensor: [i64; 2], beacon: [i64; 2]) {
        let radius = manhattan_distance(sensor, beacon);
        let distance = (sensor[1] - self.target_row).abs();

        if distance > radius {
            return;
        }

        let range = [
            sensor[0] - (radius - distance),
            sensor[0] + (radius - distance),
        ];
        self.merge_range(range);

        if beacon[1] == self.target_row {
            self.beacons.insert(beacon[0]);
        }
    }

    fn merge_range(&mut self, new_range: [i64; 2]) {
        if self.range.is_empty() {
            self.range.push(new_range);
            return;
        }

        let mut range_copy = new_range;
        let mut index = self.range.len();
        loop {
            if index == 0 {
                break;
            }
            index -= 1;

            match check_overlap(&range_copy, &self.range[index]) {
                Overlap::Greater => {
                    self.range.insert(index + 1, range_copy);
                    return;
                }
                Overlap::Less => {}
                Overlap::Contained => {
                    return;
                }
                Overlap::Contains => {
                    self.range.remove(index);
                }
                Overlap::LowerContained => {
                    range_copy = [self.range[index][0], range_copy[1]];
                    self.range.remove(index);
                }
                Overlap::UpperContained => {
                    range_copy = [range_copy[0], self.range[index][1]];
                    self.range.remove(index);
                }
            }
        }

        if index == 0 {
            self.range.insert(0, range_copy);
        }
    }

    fn count_invalid_positions(&self) -> i64 {
        let beacon_arr: Vec<&i64> = self.beacons.iter().collect();
        let mut count = 0;

        for range in self.range.iter() {
            let mut amount = range[1] - range[0] + 1;

            for b in beacon_arr.iter() {
                if within(**b, range) {
                    amount -= 1;
                }
            }

            count += amount;
        }

        count
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input string");

    let x_re = Regex::new(r"x=(-?\d+).*x=(-?\d+)").expect("Failed to build regex for x coordinate");
    let y_re = Regex::new(r"y=(-?\d+).*y=(-?\d+)").expect("Failed to build regex for y coordinate");

    let lines: Vec<[[i64; 2]; 2]> = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| {
            let x_coord = x_re.captures(x).expect("Failed to find x coordinate");
            let y_coord = y_re.captures(x).expect("Failed to find y coordinate");
            [
                [
                    x_coord[1]
                        .parse()
                        .expect(&format!("Failed to parse coordinate: {}", &x_coord[1])),
                    y_coord[1]
                        .parse()
                        .expect(&format!("Failed to parse coordinate: {}", &y_coord[1])),
                ],
                [
                    x_coord[2]
                        .parse()
                        .expect(&format!("Failed to parse coordinate: {}", &x_coord[2])),
                    y_coord[2]
                        .parse()
                        .expect(&format!("Failed to parse coordinate: {}", &y_coord[2])),
                ],
            ]
        })
        .collect();

    let mut beacon_row = BeaconRow::new(2000000);
    for line in lines {
        beacon_row.add_beacon_pair(line[0], line[1]);
    }

    println!("Part 1: {}", beacon_row.count_invalid_positions());
}
