use itertools::Itertools;

fn layers(input: &str) -> impl Iterator<Item = Vec<Vec<i32>>> + '_ {
  input.lines().map(|line| {
    let nums: Vec<i32> = line
      .split_whitespace()
      .filter_map(|n| n.parse().ok())
      .collect();

    let mut layers = vec![nums];

    while let Some(last_layer) =
      layers.last().filter(|layer| layer.iter().any(|n| *n != 0))
    {
      layers.push(
        last_layer
          .iter()
          .tuple_windows()
          .map(|(a, b)| b - a)
          .collect(),
      );
    }

    layers
  })
}

pub fn part1(input: &str) -> i32 {
  layers(input)
    .map(|layers| {
      layers
        .iter()
        .rev()
        .fold(0, |prev, layer| layer.last().unwrap() + prev)
    })
    .sum()
}

pub fn part2(input: &str) -> i32 {
  layers(input)
    .map(|layers| {
      layers
        .iter()
        .rev()
        .fold(0, |prev, layer| layer.first().unwrap() - prev)
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 114);
    assert_eq!(part1(INPUT), 1_972_648_895);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 2);
    assert_eq!(part2(INPUT), 919);
  }
}
