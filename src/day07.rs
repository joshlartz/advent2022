use std::collections::HashMap;

enum Operation<'a> {
    MoveUp,
    MoveDown(String),
    Dir(&'a str),
    File(u32),
}

type Input = HashMap<String, u32>;

pub fn generator(input: &str) -> Input {
    let mut tree: Input = HashMap::new();
    tree.insert(String::from("/"), 0);
    let mut pointer: Vec<String> = Vec::new();

    for line in input.lines() {
        match parse(line) {
            Some(op) => match op {
                Operation::MoveUp => {
                    pointer.pop();
                }
                Operation::MoveDown(dir) => {
                    pointer.push(dir.to_string());
                }
                Operation::Dir(dir) => {
                    tree.entry(pointer.concat() + dir).or_insert(0);
                }
                Operation::File(file) => {
                    // roll up sizes to parent directories
                    for index in 0..=pointer.len() {
                        tree.entry(pointer[0..index].concat())
                            .and_modify(|dir| *dir += file);
                    }
                }
            },
            None => (),
        }
    }

    tree
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|(_path, dir)| if dir < &100_000 { Some(dir) } else { None })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    const TOTAL: u32 = 70_000_000;
    const FREE: u32 = 30_000_000;
    let unused: u32 = TOTAL - input.get("/").unwrap();
    let goal: u32 = FREE - unused;

    *input
        .iter()
        .filter_map(|(_path, dir)| if dir >= &goal { Some(dir) } else { None })
        .min()
        .unwrap()
}

fn parse(line: &str) -> Option<Operation> {
    if line.starts_with("$ ls") {
        None
    } else if line == "$ cd .." {
        Some(Operation::MoveUp)
    } else if line.starts_with("$ cd") {
        Some(Operation::MoveDown(line[5..].to_string()))
    } else if line.starts_with("dir ") {
        Some(Operation::Dir(&line[4..]))
    } else {
        Some(Operation::File(
            line.split_whitespace().next().unwrap().parse().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$ cd /
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
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 24933642);
    }
}
