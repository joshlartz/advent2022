use std::collections::HashSet;

type Input = Vec<Vec<HashSet<u32>>>;

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|pair| {
                    let mut sections = pair.split('-');
                    (sections.next().unwrap().parse::<u32>().unwrap()
                        ..sections.next().unwrap().parse::<u32>().unwrap() + 1)
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|pair| {
            let overlap = &pair[0].intersection(&pair[1]).count();
            overlap >= &pair[0].len() || overlap >= &pair[1].len()
        })
        .filter(|overlapping| *overlapping)
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|pair| pair[0].intersection(&pair[1]).count() > 0)
        .filter(|overlapping| *overlapping)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 4);
    }
}
