// use std::collections::HashMap;

type Input = Vec<Vec<char>>;

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().filter(|char| char != &' ').collect())
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().fold(0, |acc: u32, round: &Vec<char>| {
        acc + <u8 as Into<u32>>::into(outcome(&round[0], &round[1]) + shape_score(&round[1]))
    })
}

pub fn part2(input: &Input) -> u32 {
    input.iter().fold(0, |acc: u32, round: &Vec<char>| {
        acc + <u8 as Into<u32>>::into(
            outcome_score(&round[1]) + shape_score(&shape(&round[0], &round[1])),
        )
    })
}

// figure out const hash map
fn shape_score(shape: &char) -> u8 {
    match shape {
        'A' | 'X' => 1,              // rock
        'B' | 'Y' => 2,              // paper
        'C' | 'Z' => 3,              // scissors
        _ => panic!("fall through"), // lookups wouldn't have this problem
    }
}

fn outcome_score(shape: &char) -> u8 {
    match shape {
        'X' => 0, // lose
        'Y' => 3, // draw
        'Z' => 6, // win
        _ => panic!("fall through"),
    }
}

fn outcome(opponent: &char, you: &char) -> u8 {
    match opponent {
        // rock
        'A' => match you {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            _ => panic!("fall through"),
        },
        // paper
        'B' => match you {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("fall through"),
        },
        'C' => match you {
            // scissors
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            _ => panic!("fall through"),
        },
        _ => panic!("fall through"),
    }
}

fn shape(opponent: &char, outcome: &char) -> char {
    match outcome {
        // lose
        'X' => match opponent {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => panic!("fall through"),
        },
        // draw
        'Y' => *opponent,
        // win
        'Z' => match opponent {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => panic!("fall through"),
        },
        _ => panic!("fall through"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 12);
    }
}
