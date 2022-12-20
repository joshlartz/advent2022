use std::{cmp::Ordering, ops::ControlFlow};

use itertools::{EitherOrBoth, Itertools};
use serde_json::Value;

type Input = Vec<(Vec<Value>, Vec<Value>)>;

pub fn generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|chunk| {
            let pair = chunk.split_once('\n').unwrap();
            (
                serde_json::from_str(pair.0).unwrap(),
                serde_json::from_str(pair.1).unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut right_pairs: usize = 0;

    for (i, pair) in input.iter().enumerate() {
        if recursive_parse(&pair.0, &pair.1) == ControlFlow::Break(true) {
            right_pairs += i + 1;
        }
    }

    right_pairs
}

pub fn part2(input: &Input) -> usize {
    let mut flattened: Vec<Vec<Value>> = input.iter().fold(Vec::new(), |mut acc, pair| {
        acc.push(pair.0.clone());
        acc.push(pair.1.clone());
        acc
    });

    let two: Vec<Value> = serde_json::from_str("[[2]]").unwrap();
    let six: Vec<Value> = serde_json::from_str("[[6]]").unwrap();
    flattened.push(two.clone());
    flattened.push(six.clone());

    flattened.sort_by(|left, right| match recursive_parse(left, right) {
        ControlFlow::Break(true) => Ordering::Less,
        ControlFlow::Break(false) => Ordering::Greater,
        ControlFlow::Continue(_) => Ordering::Equal,
    });

    let mut iter = flattened.iter();
    let two_pos = iter.position(|packet| packet == &two).unwrap() + 1;
    let six_pos = iter.position(|packet| packet == &six).unwrap() + 1 + two_pos;

    two_pos * six_pos
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
