use std::{collections::HashMap, sync::OnceLock};

use regex::{Regex, RegexBuilder};

struct Roll<'a> {
  count: usize,
  color: &'a str,
}

struct Game<'a> {
  index: usize,
  subsets: &'a str,
}

impl Game<'_> {
  fn rolls(&self) -> impl Iterator<Item = Roll> {
    self.subsets.split("; ").flat_map(|subset| {
      subset.split(", ").map(|group| {
        let (count, color) = group.split_once(' ').unwrap();
        let count = count.parse().unwrap();
        Roll { count, color }
      })
    })
  }
}

fn games(input: &str) -> impl Iterator<Item = Game> {
  let game_pattern = {
    static GAME_PATTERN: OnceLock<Regex> = OnceLock::new();
    GAME_PATTERN.get_or_init(|| {
      RegexBuilder::new(r"^Game (\d+): (.*)$")
        .multi_line(true)
        .build()
        .unwrap()
    })
  };

  game_pattern.captures_iter(input).map(|m| Game {
    index: m.get(1).unwrap().as_str().parse().unwrap(),
    subsets: m.get(2).unwrap().as_str(),
  })
}

pub fn part1(input: &str) -> usize {
  games(input)
    .filter(|game| {
      let mut counts = HashMap::new();

      for roll in game.rolls() {
        let prev_count = counts.entry(roll.color).or_default();
        *prev_count = roll.count.max(*prev_count);
      }

      counts["red"] <= 12 && counts["green"] <= 13 && counts["blue"] <= 14
    })
    .map(|game| game.index)
    .sum()
}

pub fn part2(input: &str) -> usize {
  games(input)
    .map(|game| {
      let mut counts = HashMap::new();

      for roll in game.rolls() {
        let prev_count = counts.entry(roll.color).or_default();
        *prev_count = roll.count.max(*prev_count);
      }

      counts.into_values().product::<usize>()
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
    assert_eq!(part1(INPUT_TEST), 8);
    assert_eq!(part1(INPUT), 3035);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 2286);
    assert_eq!(part2(INPUT), 66027);
  }
}
