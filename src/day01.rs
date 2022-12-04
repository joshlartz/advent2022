pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut summed: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    summed.sort();
    summed.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 24_000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 45_000);
    }
}
