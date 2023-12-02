use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let fs = parse_fs(&input, vis);
    let mut total = 0;
    for dir in fs.dirs() {
        let s = dir.total_size();
        if s <= 100000 {
            total += s;
        }
    }
    Box::new(total)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    const TOT_SPACE: usize = 70000000;
    const NEED: usize = 30000000;
    let fs = parse_fs(&input, vis);
    let free = TOT_SPACE - fs.total_size();
    let mut will_free = fs.total_size();
    for dir in fs.dirs() {
        let s = dir.total_size();
        if s < will_free && (free + s > NEED) {
            will_free = s;
        }
    }
    Box::new(will_free)
}

fn parse_fs(input: &str, vis: bool) -> FS {
    fn collect(mut stack: Vec<(&str, Vec<(String, FS)>)>) -> FS {
        let mut res = None;
        while let Some((name, mut entries)) = stack.pop() {
            match res {
                None => (),
                Some((name, fs)) => entries.push((name, fs)),
            };
            res = Some((name.to_string(), FS::Dir(entries)));
        }
        res.unwrap().1
    }

    let mut stack = vec![("/", Vec::new())];
    for line in input.lines() {
        match &line[0..4] {
            "$ cd" => match &line[5..] {
                "/" => {
                    match collect(stack) {
                        FS::Dir(entries) => stack = vec![("/", entries)],
                        _ => unreachable!(),
                    };
                }
                ".." => {
                    let (name, dir) = stack.pop().unwrap();
                    stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push((name.to_string(), FS::Dir(dir)));
                }
                d => {
                    stack.push((d, Vec::new()));
                }
            },
            "$ ls" => (),
            _ => {
                let (size, name) = line.split_once(' ').unwrap();
                if vis {
                    println!("{} => ({}, {})", line, name, size);
                }
                if size != "dir" {
                    stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push((name.to_string(), FS::File(size.parse().unwrap())));
                }
            }
        };
    }
    collect(stack)
}

enum FS {
    Dir(Vec<(String, FS)>),
    File(usize),
}

impl FS {
    fn total_size(&self) -> usize {
        match self {
            Self::Dir(children) => children.iter().fold(0, |s, (_, e)| s + e.total_size()),
            Self::File(size) => *size,
        }
    }

    fn dirs(&self) -> DirIter {
        DirIter {
            remaining: vec![self],
        }
    }
}

struct DirIter<'a> {
    remaining: Vec<&'a FS>,
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a FS;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.remaining.pop() {
                None => return None,
                Some(fs) => {
                    if let FS::Dir(ref entries) = fs {
                        for (_, e) in entries {
                            self.remaining.push(e);
                        }
                        return Some(fs);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_tests!(example, r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        part1 => 95437,
        part2 => 24933642);
}
