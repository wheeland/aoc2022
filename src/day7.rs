struct Directory {
    name: String,
    files: Vec<(String, usize)>,
    dirs: Vec<Directory>,
    size: usize,
}

impl Directory {
    fn create(name: &str, input: &[&str], mut index: usize) -> (Self, usize) {
        let name = name.to_string();
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let mut size = 0;

        while index < input.len() {
            let line = input[index];

            if line.starts_with("$ cd ..") {
                index += 1;
                break;
            }

            if line.starts_with("$ cd ") {
                let name = &line[5..];
                let dir = Self::create(name, input, index + 1);
                size += dir.0.size;
                dirs.push(dir.0);
                index = dir.1;
                continue;
            }

            if line.starts_with("$ ls") {
                index += 1;
                while index < input.len() && !input[index].starts_with("$ ") {
                    if !input[index].starts_with("dir") {
                        let mut parts = input[index].split(" ");
                        let file_size = parts.next().unwrap().parse().unwrap();
                        let name = parts.next().unwrap().to_string();
                        files.push((name, file_size));
                        size += file_size;
                    }
                    index += 1;
                }
            }
        }

        (
            Self {
                name,
                files,
                dirs,
                size,
            },
            index,
        )
    }

    fn task1(&self) -> usize {
        let mut result = 0;
        if self.size <= 100000 {
            result += self.size;
        }
        for dir in &self.dirs {
            result += dir.task1();
        }
        result
    }

    fn find_smallest_over(&self, min_size: usize) -> usize {
        let mut smallest = usize::MAX;
        if self.size >= min_size {
            smallest = smallest.min(self.size);
        }
        for dir in &self.dirs {
            smallest = smallest.min(dir.find_smallest_over(min_size));
        }
        smallest
    }
}

pub fn solve() {
    let mut lines: Vec<&str> = include_str!("inputs/7.txt").split('\n').collect();
    lines.retain(|line| !line.trim().is_empty());
    let root = Directory::create("/", &lines, 1).0;

    let used_space = 70000000 - root.size;
    let required_space = 30000000 - used_space;

    println!("[day 07] task 1: {}", root.task1());
    println!(
        "[day 07] task 2: {}",
        root.find_smallest_over(required_space)
    );
}
