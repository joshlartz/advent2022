// fn generator(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {

// }

// pub fn part1(input: &str) -> usize {
 
// }

// pub fn part2(input: &str) -> usize {
  
// }

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "A Y
B X
C Z";

  #[test]
  fn test_part1() {
      assert_eq!(part1(SAMPLE), 15);
  }

  #[test]
  fn test_part2() {
      assert_eq!(part2(SAMPLE), 12);
  }
}
