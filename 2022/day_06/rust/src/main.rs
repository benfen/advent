use std::fs;

#[derive(Debug)]
struct Window<const N: usize> {
    arr: [char; N],
    checkable: bool,
}

impl<const N: usize> Window<{ N }> {
    fn add_to_window(&mut self, c: char) {
        for i in (1..self.arr.len()).rev() {
            self.arr[i] = self.arr[i - 1];
        }
        self.arr[0] = c;

        if self.arr[self.arr.len() - 1] != ' ' {
            self.checkable = true;
        }
    }

    fn check_window(&self) -> bool {
        if !self.checkable {
            return false;
        }

        for i in 0..N {
            for j in (i + 1)..N {
                if self.arr[i] == self.arr[j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn new() -> Self {
        return Self {
            arr: [' '; N],
            checkable: false,
        };
    }
}

fn main() {
    let chars: Vec<char> = fs::read_to_string("./input.txt")
        .expect("Failed to retrieve input file from directory")
        .chars()
        .collect();

    let mut small_window: Window<4> = Window::new();
    let mut large_window: Window<14> = Window::new();
    let mut small_after_index = 0;
    let mut large_after_index = 0;

    for (i, char) in chars.iter().enumerate() {
        small_window.add_to_window(*char);

        if small_window.check_window() {
            small_after_index = i;
            break;
        }
    }

    for (i, char) in chars.iter().enumerate() {
        large_window.add_to_window(*char);

        if large_window.check_window() {
            large_after_index = i;
            break;
        }
    }

    println!("Part 1: {}", small_after_index + 1);
    println!("Part 2: {}", large_after_index + 1);
}
