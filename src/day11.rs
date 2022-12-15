use std::fmt::Debug;

use itertools::Itertools;

pub struct Input<'a> {
    holding: Vec<u32>,
    operation: (&'a str, &'a str),
    test: (&'a str, &'a str),
    if_true: usize,
    if_false: usize,
}

pub struct Monkey {
    holding: Vec<u32>,
    operation: Box<dyn Fn(&u32) -> u32>,
    test: Box<dyn Fn(&u32) -> bool>,
    if_true: usize,
    if_false: usize,
    inspections: u32,
}
impl Monkey {
    fn new(input: &Input) -> Self {
        Self {
            holding: input.holding.clone(),
            operation: {
                let num: u32 = input.operation.1.parse().unwrap_or_else(|_| 0);
                match input.operation.0 {
                    "*" => match num {
                        0 => Box::new(move |item| item.pow(2)),
                        _ => Box::new(move |item| item * num),
                    },
                    "+" => Box::new(move |item| item + num),
                    _ => panic!("unknown operation"),
                }
            },
            test: {
                let num: u32 = input
                    .test
                    .1
                    .parse()
                    .unwrap_or_else(|_| panic!("{}", input.test.1));
                match input.test.0 {
                    "divisible" => Box::new(move |item| item % num == 0),
                    _ => panic!("unknown operation"),
                }
            },
            if_true: input.if_true,
            if_false: input.if_false,
            inspections: 0,
        }
    }

    fn inspect(&mut self) {
        for item in self.holding.as_mut_slice() {
            *item = (self.operation)(item) / 3;
            self.inspections += 1;
        }
    }

    fn test(&self) -> Vec<usize> {
        self.holding
            .iter()
            .map(|item| match (self.test)(&item) {
                true => self.if_true,
                false => self.if_false,
            })
            .collect()
    }
}
impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.holding))
    }
}

pub fn generator(input: &str) -> Vec<Input> {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();
            lines.next(); // don't need the monkey number as it aligns with the index
            Input {
                holding: lines
                    .next()
                    .unwrap()
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|item| item.parse().unwrap_or_else(|item| panic!("{}", item)))
                    .collect(),
                operation: lines
                    .next()
                    .unwrap()
                    .split_once("= old ")
                    .unwrap()
                    .1
                    .split_once(" ")
                    .unwrap(),
                test: lines
                    .next()
                    .unwrap()
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_once(" by ")
                    .unwrap(),
                if_true: lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|item| panic!("{}", item)),
                if_false: lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap_or_else(|item| panic!("{}", item)),
            }
        })
        .collect()
}

pub fn part1(input: &Vec<Input>) -> u32 {
    let mut monkies: Vec<Monkey> = input.iter().map(|monkey| Monkey::new(monkey)).collect();

    for _round in 0..20 {
        for m in 0..monkies.len() {
            monkies[m].inspect();

            let throws = monkies[m]
                .test()
                .iter()
                .enumerate()
                .map(|(i, throw_to)| (*throw_to, monkies[m].holding[i]))
                .collect_vec();

            for (throw_to, item) in throws {
                monkies[throw_to].holding.push(item);
            }
            monkies[m].holding.clear();
        }
    }

    monkies
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

// pub fn part2(input: &Vec<Input>) -> String {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 10605);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(
    //         part2(&generator(SAMPLE)), 2);
    // }
}
