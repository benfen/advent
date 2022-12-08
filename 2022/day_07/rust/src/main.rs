use std::{collections::HashMap, fs, ptr};

#[derive(Debug)]
struct Directory {
    dirs: Vec<Box<Self>>,
    files: Vec<(String, u64)>,
    name: String,
    parent: *mut Self,
}

impl Directory {
    fn new(name: String, parent: *mut Self) -> Self {
        Self {
            dirs: Vec::new(),
            files: Vec::new(),
            name,
            parent,
        }
    }

    fn add_directory(&mut self, directory: Self) -> *mut Directory {
        let mut b = Box::new(directory);
        let r = b.as_mut() as *mut Directory;
        self.dirs.push(b);
        r
    }

    fn add_file(&mut self, name: String, size: u64) {
        self.files.push((name, size));
    }

    fn get_child_dir_path(&self, path: &str) -> String {
        if self.name == "/" {
            format!("/{}", path)
        } else {
            format!("{}/{}", self.name, path)
        }
    }

    fn sum_dirs(&self) -> (u64, u64) {
        let mut total_size = 0;
        let mut size = 0;

        for file in &self.files {
            size += file.1;
        }

        for dir in &self.dirs {
            let (dir_size, total_dir_size) = dir.sum_dirs();

            size += dir_size;
            total_size += total_dir_size;
        }

        if size < 100_000 {
            total_size += size;
        }

        (size, total_size)
    }

    fn smallest_dir(&self, req_size: u64) -> (u64, u64) {
        let mut smallest_size = u64::MAX;
        let mut size = 0;

        for file in &self.files {
            size += file.1;
        }

        for dir in &self.dirs {
            let (dir_size, smallest_dir_size) = dir.smallest_dir(req_size);

            size += dir_size;

            if smallest_dir_size < smallest_size && smallest_dir_size > req_size {
                smallest_size = smallest_dir_size;
            }
        }

        if size < smallest_size && size > req_size {
            smallest_size = size;
        }

        (size, smallest_size)
    }
}

fn main() {
    let input: String =
        fs::read_to_string("./input.txt").expect("Failed to retrieve input file from directory");

    let lines: Vec<&str> = input.split("\n").filter(|x| *x != "").collect();

    let mut dir_map = HashMap::new();
    let mut cd = Directory::new("/".to_owned(), ptr::null_mut());

    let mut cd_ref = &mut cd as *mut Directory;
    dir_map.insert("/".to_owned(), cd_ref);

    for line in lines.iter().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        if chars[0] == '$' {
            if chars[2] == 'c' {
                let filename: String = chars[5..].iter().collect();

                if &filename == ".." {
                    cd_ref = unsafe {
                        cd_ref
                            .as_mut()
                            .expect("Failed to get reference to parent")
                            .parent
                    };
                } else {
                    let filepath = unsafe {
                        cd_ref
                            .as_ref()
                            .expect("Failed to get reference to current directory")
                            .get_child_dir_path(&filename)
                    };
                    cd_ref = *dir_map
                        .get(&filepath)
                        .expect(&format!("Could not find directory {}", filename));
                }
            }

            continue;
        }

        let mut ls_info = line.split(" ");

        let size_or_type = ls_info
            .next()
            .expect("Failed to retrieve size or type from file/dir");
        let name = ls_info.next().expect("Failed to get name of file/dir");

        if size_or_type == "dir" {
            let parent_dir = unsafe {
                cd_ref
                    .as_mut()
                    .expect("Failed to get reference to add directory")
            };

            let filepath = parent_dir.get_child_dir_path(name);
            let directory = Directory::new(filepath.clone(), cd_ref);

            let d = parent_dir.add_directory(directory);
            dir_map.insert(filepath, d as *mut Directory);
        } else {
            unsafe {
                cd_ref
                    .as_mut()
                    .expect("Failed to get reference to add director")
                    .add_file(
                        name.to_owned(),
                        size_or_type
                            .parse::<u64>()
                            .expect("Failed to parse file size"),
                    );
            }
        }
    }

    let size = cd.sum_dirs();
    println!("Part 1: {}", size.1);
    println!("Part 2: {}", cd.smallest_dir(size.0 - 40_000_000).1)
}
