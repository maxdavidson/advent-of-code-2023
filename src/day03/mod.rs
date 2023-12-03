use std::collections::{BTreeMap, BTreeSet};

pub fn part1(input: &str) -> u32 {
  let mut chars = BTreeMap::new();
  let mut numbers = Vec::new();

  for (y, line) in input.lines().enumerate() {
    let mut char_iter = line.chars().enumerate().peekable();
    while let Some((x, char)) = char_iter.next() {
      chars.insert((x, y), char);

      if let Some(digit) = char.to_digit(10) {
        let mut number = digit;
        let mut last_x = x;
        while let Some((next_x, next_digit)) =
          char_iter.peek().and_then(|(next_x, next_char)| {
            next_char.to_digit(10).map(|digit| (*next_x, digit))
          })
        {
          number = 10 * number + next_digit;
          last_x = next_x;
          char_iter.next().unwrap();
        }

        numbers.push((number, x, last_x, y))
      }
    }
  }

  numbers
    .into_iter()
    .filter(|(_number, min_x, max_x, y)| {
      for x in (if *min_x == 0 { 0 } else { min_x - 1 })..=max_x + 1 {
        for y in (if *y == 0 { 0 } else { y - 1 })..=y + 1 {
          if chars
            .get(&(x, y))
            .map(|c| c.to_digit(10).is_none() && *c != '.')
            .unwrap_or(false)
          {
            return true;
          }
        }
      }

      false
    })
    .map(|(number, _, _, _)| number)
    .sum()
}

pub fn part2(input: &str) -> u32 {
  enum Entry {
    Gear,
    Number(u32),
  }

  let mut entries = BTreeMap::new();

  for (y, line) in input.lines().enumerate() {
    let mut char_iter = line.chars().enumerate().peekable();
    while let Some((x, char)) = char_iter.next() {
      if char == '*' {
        entries.insert((x, y), Entry::Gear);
      } else if let Some(digit) = char.to_digit(10) {
        let mut number = digit;
        let mut last_x = x;
        while let Some((next_x, next_digit)) =
          char_iter.peek().and_then(|(next_x, next_char)| {
            next_char.to_digit(10).map(|digit| (*next_x, digit))
          })
        {
          number = 10 * number + next_digit;
          last_x = next_x;
          char_iter.next().unwrap();
        }

        for x in x..=last_x {
          entries.insert((x, y), Entry::Number(number));
        }
      }
    }
  }

  entries
    .iter()
    .filter_map(|((x, y), entry)| {
      if let Entry::Gear = *entry {
        let mut adjacent_numbers = BTreeSet::new();

        for x in (if *x == 0 { 0 } else { x - 1 })..=x + 1 {
          for y in (if *y == 0 { 0 } else { y - 1 })..=y + 1 {
            if let Some(Entry::Number(number)) = entries.get(&(x, y)) {
              adjacent_numbers.insert(*number);
            }
          }
        }

        if adjacent_numbers.len() == 2 {
          return Some(adjacent_numbers.into_iter().product::<u32>());
        }
      }

      return None;
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
    assert_eq!(part1(INPUT_TEST), 4361);
    assert_eq!(part1(INPUT), 546_563);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 467_835);
    assert_eq!(part2(INPUT), 91_031_374);
  }
}
