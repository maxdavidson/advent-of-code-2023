pub fn part1(_input: &str) -> u32 {
  0
}

pub fn part2(_input: &str) -> u32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 0);
    assert_eq!(part1(INPUT), 0);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 0);
    assert_eq!(part2(INPUT), 0);
  }
}
