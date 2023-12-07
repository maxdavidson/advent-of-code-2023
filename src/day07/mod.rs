use std::{
  array,
  fmt::{Display, Write},
  marker::PhantomData,
  str::FromStr,
};

trait Rules: Eq + Sized {
  fn rank(card: &Card<Self>) -> usize;
  fn kind(hand: &Hand<Self>) -> HandKind;
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Card<R> {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  JackOrJoker,
  Queen,
  King,
  Ace(PhantomData<R>),
}

impl<R: Rules> PartialOrd for Card<R> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl<R: Rules> Ord for Card<R> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.rank().cmp(&other.rank())
  }
}

impl<R: Rules> Card<R> {
  fn rank(&self) -> usize {
    R::rank(self)
  }
}

#[derive(Debug)]
enum CardError {
  InvalidChar(char),
}

impl<R> TryFrom<char> for Card<R> {
  type Error = CardError;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      '2' => Ok(Self::Two),
      '3' => Ok(Self::Three),
      '4' => Ok(Self::Four),
      '5' => Ok(Self::Five),
      '6' => Ok(Self::Six),
      '7' => Ok(Self::Seven),
      '8' => Ok(Self::Eight),
      '9' => Ok(Self::Nine),
      'T' => Ok(Self::Ten),
      'J' => Ok(Self::JackOrJoker),
      'Q' => Ok(Self::Queen),
      'K' => Ok(Self::King),
      'A' => Ok(Self::Ace(PhantomData)),
      _ => Err(CardError::InvalidChar(c)),
    }
  }
}

impl<R> From<&Card<R>> for char {
  fn from(card: &Card<R>) -> Self {
    match *card {
      Card::Two => '2',
      Card::Three => '3',
      Card::Four => '4',
      Card::Five => '5',
      Card::Six => '6',
      Card::Seven => '7',
      Card::Eight => '8',
      Card::Nine => '9',
      Card::Ten => 'T',
      Card::JackOrJoker => 'J',
      Card::Queen => 'Q',
      Card::King => 'K',
      Card::Ace(_) => 'A',
    }
  }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandKind {
  HighCard,
  OnePair,
  TwoPairs,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

impl HandKind {
  fn from_counts(counts: &[u8]) -> Self {
    if counts.contains(&5) {
      return HandKind::FiveOfAKind;
    }

    if counts.contains(&4) {
      return HandKind::FourOfAKind;
    }

    if counts.contains(&3) {
      if counts.contains(&2) {
        return HandKind::FullHouse;
      }

      return HandKind::ThreeOfAKind;
    }

    let pair_count = counts.iter().filter(|count| **count == 2).count();

    if pair_count == 2 {
      return HandKind::TwoPairs;
    }

    if pair_count == 1 {
      return HandKind::OnePair;
    }

    HandKind::HighCard
  }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Hand<R> {
  cards: [Card<R>; 5],
  _rules: PhantomData<R>,
}

impl<R: Rules> Hand<R> {
  fn kind(&self) -> HandKind {
    Rules::kind(self)
  }
}

impl<R> FromStr for Hand<R> {
  type Err = CardError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let cards = array::from_fn(|_| chars.next().unwrap().try_into().unwrap());
    Ok(Self {
      cards,
      _rules: PhantomData,
    })
  }
}

impl<R> Display for Hand<R> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for card in &self.cards {
      f.write_char(char::from(card))?;
    }
    Ok(())
  }
}

impl<R: Rules> PartialOrd for Hand<R> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cards.cmp(&other.cards))
  }
}

impl<R: Rules> Ord for Hand<R> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self
      .kind()
      .cmp(&other.kind())
      .then_with(|| self.cards.cmp(&other.cards))
  }
}

fn play_game<R: Rules>(input: &str) -> u32 {
  let mut data = input
    .lines()
    .map(|line| {
      let (hand, bid) = line.split_once(' ').unwrap();
      (
        hand.parse::<Hand<R>>().unwrap(),
        bid.parse::<u32>().unwrap(),
      )
    })
    .collect::<Vec<_>>();

  data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

  data
    .into_iter()
    .enumerate()
    .map(|(i, (_hand, bid))| (i as u32 + 1) * bid)
    .sum()
}

pub fn part1(input: &str) -> u32 {
  #[derive(Eq, PartialEq)]
  struct Part1;

  impl Rules for Part1 {
    fn rank(card: &Card<Self>) -> usize {
      match *card {
        Card::Two => 2,
        Card::Three => 3,
        Card::Four => 4,
        Card::Five => 5,
        Card::Six => 6,
        Card::Seven => 7,
        Card::Eight => 8,
        Card::Nine => 9,
        Card::Ten => 10,
        Card::JackOrJoker => 11,
        Card::Queen => 12,
        Card::King => 13,
        Card::Ace(_) => 14,
      }
    }

    fn kind(hand: &Hand<Self>) -> HandKind {
      let mut counts = [0u8; 0x14];

      for card in &hand.cards {
        counts[card.rank()] += 1;
      }

      HandKind::from_counts(&counts)
    }
  }

  play_game::<Part1>(input)
}

pub fn part2(input: &str) -> u32 {
  #[derive(Eq, PartialEq)]
  struct Part2;

  impl Rules for Part2 {
    fn rank(card: &Card<Self>) -> usize {
      match *card {
        Card::JackOrJoker => 1,
        Card::Two => 2,
        Card::Three => 3,
        Card::Four => 4,
        Card::Five => 5,
        Card::Six => 6,
        Card::Seven => 7,
        Card::Eight => 8,
        Card::Nine => 9,
        Card::Ten => 10,
        Card::Queen => 12,
        Card::King => 13,
        Card::Ace(_) => 14,
      }
    }

    fn kind(hand: &Hand<Self>) -> HandKind {
      let mut counts = [0u8; 0x14];

      for card in &hand.cards {
        counts[card.rank()] += 1;
      }

      let mut joker_count = counts[Card::<Self>::JackOrJoker.rank()];
      counts[Card::<Self>::JackOrJoker.rank()] = 0;

      while 0 < joker_count {
        let max_count = counts.iter_mut().max().unwrap();
        let assigned = (5 - *max_count).min(joker_count);
        *max_count += assigned;
        joker_count -= assigned;
      }

      HandKind::from_counts(&counts)
    }
  }

  play_game::<Part2>(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 6440);
    assert_eq!(part1(INPUT), 247_823_654);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 5905);
    assert_eq!(part2(INPUT), 245_461_700);
  }
}
