use std::ops::ControlFlow;

use itertools::{EitherOrBoth, Itertools};
use regex::Regex;
use serde_json::Value;

type Input<'a> = Vec<(&'a str, &'a str)>;

pub fn generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|chunk| chunk.split_once('\n').unwrap())
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let json: Vec<(Vec<Value>, Vec<Value>)> = input
        .iter()
        .map(|pair| {
            (
                serde_json::from_str(pair.0).unwrap(),
                serde_json::from_str(pair.1).unwrap(),
            )
        })
        .collect();
    let mut right_pairs: usize = 0;

    for (i, pair) in json.iter().enumerate() {
        if recursive_parse(&pair.0, &pair.1) == ControlFlow::Break(true) {
            right_pairs += i + 1;
        }
    }

    right_pairs
}

pub fn part2(input: &Input) -> usize {
    let only_brackets = Regex::new(r"^(\[|\])*$").unwrap();

    let mut empty_pairs: Vec<&str> = Vec::new();
    let mut pairs: Vec<Vec<u8>> = input
        .iter()
        .fold(Vec::new(), |mut acc, pair| {
            if only_brackets.is_match(pair.0) {
                empty_pairs.push(pair.0);
            } else {
                acc.push(pair.0);
            }
            if only_brackets.is_match(pair.1) {
                empty_pairs.push(pair.1);
            } else {
                acc.push(pair.1);
            }
            acc
        })
        .iter()
        .map(|packet| {
            packet
                .replace(['[', ']'], "")
                .split(',')
                .map(|num| num.parse().unwrap_or_default())
                .collect()
        })
        .collect();

    let two = vec![2];
    let six = vec![6];
    pairs.append(&mut vec![two.clone(), six.clone()]);
    pairs.sort();

    (pairs.binary_search(&two).unwrap() + empty_pairs.len() + 1)
        * (pairs.binary_search(&six).unwrap() + empty_pairs.len() + 1)
}

fn recursive_parse(left: &[Value], right: &[Value]) -> ControlFlow<bool> {
    left.iter()
        .zip_longest(right.iter())
        .try_for_each(|pair| match pair {
            EitherOrBoth::Left(_) => ControlFlow::Break(false),
            EitherOrBoth::Right(_) => ControlFlow::Break(true),
            EitherOrBoth::Both(l, r) => parse(l, r),
        })
}

fn parse(left: &Value, right: &Value) -> ControlFlow<bool> {
    if left.is_number() && right.is_number() {
        if left.as_u64() < right.as_u64() {
            return ControlFlow::Break(true);
        }
        if left.as_u64() > right.as_u64() {
            return ControlFlow::Break(false);
        }
    }
    if left.is_array() && right.is_number() {
        return recursive_parse(left.as_array().unwrap(), &[right.clone()]);
    }
    if left.is_number() && right.is_array() {
        return recursive_parse(&[left.clone()], right.as_array().unwrap());
    }
    if left.is_array() && right.is_array() {
        return recursive_parse(left.as_array().unwrap(), right.as_array().unwrap());
    }
    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 140);
    }
}
