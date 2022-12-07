use itertools::Itertools;
use regex::Regex;

type Stack = Vec<std::string::String>;
#[derive(Debug)]
pub struct Step {
    crates: usize,
    from: usize,
    to: usize,
}

type Input = (Vec<Stack>, Vec<Step>);

pub fn generator(input: &str) -> Input {
    let re = Regex::new(r"\[|\]").unwrap();
    let (crates, operations) = input.split_once("\n\n").unwrap();

    let crates = crates
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            re.replace_all(line, " ")
                .chars()
                .chunks(4)
                .into_iter()
                .map(|c| c.collect::<String>())
                .map(|s| s.trim().to_owned())
                .collect_vec()
        })
        .collect_vec();

    // make stacks of crates with an empty stack 0 to align with the inputs
    let mut stacks: Vec<Vec<String>> = vec![vec![]; crates[0].len() + 1];
    for each in crates {
        for (index, _crate) in each.iter().enumerate() {
            if _crate != "" {
                stacks[index + 1].push(_crate.clone());
            }
        }
    }

    let steps = operations
        .lines()
        .map(|op| {
            let split = op.split_whitespace().collect_vec();
            Step {
                crates: split[1].parse().unwrap(),
                from: split[3].parse().unwrap(),
                to: split[5].parse().unwrap(),
            }
        })
        .collect_vec();

    (stacks, steps)
}

pub fn part1(input: &Input) -> String {
    let (stacks, steps) = input;
    let mut stacks = stacks.clone();

    for step in steps {
        move_crates_9000(&mut stacks, &step);
    }

    top_crates(&stacks)
}

pub fn part2(input: &Input) -> String {
    let (stacks, steps) = input;
    let mut stacks = stacks.clone();

    for step in steps {
        move_crates_9001(&mut stacks, &step);
    }

    top_crates(&stacks)
}

fn top_crates(stacks: &Vec<Stack>) -> String {
    let mut top = vec![""];
    for stack in stacks {
        if stack.len() > 0 {
            top.push(stack.last().unwrap());
        }
    }
    top.concat()
}

fn move_crates_9000(stacks: &mut Vec<Stack>, step: &Step) -> () {
    for _ in 0..step.crates {
        let _crate = stacks[step.from].pop().unwrap();
        stacks[step.to].push(_crate);
    }
}

fn move_crates_9001(stacks: &mut Vec<Stack>, step: &Step) -> () {
    let len = stacks[step.from].len();
    let mut crates = stacks[step.from]
        .drain((len - step.crates)..)
        .map(String::from)
        .collect_vec();
    stacks[step.to].append(&mut crates);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), "MCD");
    }
}
