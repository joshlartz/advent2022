use std::collections::HashSet;

// figure out lifetimes
type Input<'a> = Vec<&'a str>;

pub fn generator(input: &str) -> Input {
    input.lines().collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().fold(0, |acc, line| {
        let rucksack = line.split_at(line.len() / 2);
        let left = rucksack.0.chars().collect::<HashSet<_>>();
        let right = rucksack.1.chars().collect::<HashSet<_>>();
        let item = &mut left.intersection(&right);
        acc + priority(item.next().unwrap()) as u32
    })
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
        .collect::<Vec<HashSet<char>>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .fold(0, |acc, group| {
            let badge = group[0]
                .iter()
                .filter(|item| group[1..].iter().all(|rucksack| rucksack.contains(item)))
                .collect::<Vec<&char>>()[0];
            acc + priority(badge) as u32
        })
}

fn priority(item: &char) -> u8 {
    let offset = if item.is_lowercase() { 96 } else { 38 };

    item.to_string().as_bytes()[0] - offset
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 70);
    }
}
