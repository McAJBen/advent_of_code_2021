use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileType {
    Dir,
    File,
}

#[derive(Debug)]
struct FileDescriptor {
    size: u64,
    file_type: FileType,
}

impl FileDescriptor {
    pub fn from_terminal(input: &str) -> HashMap<PathBuf, Self> {
        let mut fds = HashMap::new();
        fds.insert(
            PathBuf::new(),
            FileDescriptor {
                size: 0,
                file_type: FileType::Dir,
            },
        );

        let mut current_path = PathBuf::new();

        for line in input.lines() {
            if line == "$ cd /" {
                current_path = PathBuf::new();
            } else if line == "$ cd .." {
                current_path.pop();
            } else if line == "$ ls" {
            } else if let Some(name) = line.strip_prefix("$ cd ") {
                current_path.push(name);
            } else if let Some(name) = line.strip_prefix("dir ") {
                fds.insert(
                    current_path.join(name),
                    FileDescriptor {
                        size: 0,
                        file_type: FileType::Dir,
                    },
                );
            } else if let Some((size, name)) = line.split_once(' ') {
                fds.insert(
                    current_path.join(name),
                    FileDescriptor {
                        size: size.parse().unwrap(),
                        file_type: FileType::File,
                    },
                );
            }
        }

        let mut dirs: HashMap<PathBuf, u64> = HashMap::new();

        for (path, fd) in fds.iter() {
            if fd.file_type == FileType::File {
                let mut path = path.as_path();
                while let Some(parent) = path.parent() {
                    let size = dirs.entry(parent.to_path_buf()).or_default();
                    *size += fd.size;

                    path = parent;
                }
            }
        }

        for (path, size) in dirs {
            fds.entry(path).and_modify(|f| f.size = size);
        }

        fds
    }
}

pub fn part1(input: &str) -> u64 {
    let fds = FileDescriptor::from_terminal(input);

    fds.values()
        .map(|f| {
            if f.file_type == FileType::Dir && f.size <= 100000 {
                f.size
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let fds = FileDescriptor::from_terminal(input);

    let used_size = fds.get(&PathBuf::new()).unwrap().size;

    let space_needed = used_size - 40000000;

    fds.values()
        .map(|f| f.size)
        .filter(|s| *s > space_needed)
        .min()
        .unwrap()
}
