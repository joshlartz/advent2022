use itertools::Itertools;

type Input = Vec<char>;

pub fn generator(input: &str) -> Input {
    input.chars().collect_vec()
}

pub fn part1(input: &Input) -> usize {
    find_start_marker(&input, 4)
}

pub fn part2(input: &Input) -> usize {
    find_start_marker(&input, 14)
}

fn find_start_marker(input: &Input, size: usize) -> usize {
    for (index, chars) in input.windows(size).enumerate() {
        if chars.iter().unique().count() == size {
            return index + size;
        }
    }
    panic!("start marker not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE1)), 7);
        assert_eq!(part1(&generator(SAMPLE2)), 5);
        assert_eq!(part1(&generator(SAMPLE3)), 6);
        assert_eq!(part1(&generator(SAMPLE4)), 10);
        assert_eq!(part1(&generator(SAMPLE5)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE1)), 19);
        assert_eq!(part2(&generator(SAMPLE2)), 23);
        assert_eq!(part2(&generator(SAMPLE3)), 23);
        assert_eq!(part2(&generator(SAMPLE4)), 29);
        assert_eq!(part2(&generator(SAMPLE5)), 26);
    }
}
