use std::sync::OnceLock;

use bit_set::BitSet;
use regex::{Regex, RegexBuilder};

#[derive(Debug)]
struct Card {
  index: usize,
  winning: BitSet<usize>,
  owned: BitSet<usize>,
}

fn cards(input: &str) -> impl Iterator<Item = Card> + '_ {
  let card_pattern = {
    static CARD_PATTERN: OnceLock<Regex> = OnceLock::new();
    CARD_PATTERN.get_or_init(|| {
      RegexBuilder::new(r"^Card +(\d+): +(.+) \| +(.*)$")
        .multi_line(true)
        .build()
        .unwrap()
    })
  };

  card_pattern.captures_iter(input).map(|c| Card {
    index: c.get(1).unwrap().as_str().parse().unwrap(),
    winning: c
      .get(2)
      .unwrap()
      .as_str()
      .split_whitespace()
      .map(|c| c.parse().unwrap())
      .collect(),
    owned: c
      .get(3)
      .unwrap()
      .as_str()
      .split_whitespace()
      .map(|c| c.parse().unwrap())
      .collect(),
  })
}

pub fn part1(input: &str) -> usize {
  cards(input)
    .map(|card| {
      let matches = card.owned.intersection(&card.winning).count();
      if matches == 0 {
        0
      } else {
        1 << (matches - 1)
      }
    })
    .sum()
}

pub fn part2(input: &str) -> usize {
  let cards = cards(input).collect::<Vec<_>>();
  let mut card_counts = vec![1; cards.len()];

  for card in cards.into_iter() {
    let matches = card.owned.intersection(&card.winning).count();
    for i in 0..matches {
      card_counts[card.index + i] += card_counts[card.index - 1];
    }
  }

  card_counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 13);
    assert_eq!(part1(INPUT), 27_059);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 30);
    assert_eq!(part2(INPUT), 5_744_979);
  }
}
