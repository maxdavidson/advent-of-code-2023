use std::collections::HashMap;

fn count_arrangements<'a, 'b>(
  cache: &mut HashMap<(&'a str, &'b [usize]), usize>,
  springs: &'a str,
  groups: &'b [usize],
) -> usize {
  if let Some(&sum) = cache.get(&(springs, groups)) {
    return sum;
  }

  if groups.is_empty() && !springs.contains('#') {
    return 1;
  }

  let Some((&first_group, rem_groups)) = groups.split_first() else {
    return 0;
  };

  let mut chars = springs.chars();

  let mut sum = 0;

  if springs.starts_with(['.', '?']) {
    let mut chars = chars.clone();
    chars.next().unwrap();
    sum += count_arrangements(cache, chars.as_str(), groups);
  }

  for _ in 0..first_group {
    let Some('#' | '?') = chars.next() else {
      return sum;
    };
  }

  let (Some('.' | '?') | None) = chars.next() else {
    return sum;
  };

  sum += count_arrangements(cache, chars.as_str(), rem_groups);

  cache.insert((springs, groups), sum);

  sum
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, Box<[usize]>)> {
  input.lines().map(|line| {
    let (springs, groups) = line.split_once(' ').unwrap();
    let groups: Vec<usize> =
      groups.split(',').map(|g| g.parse().unwrap()).collect();
    (springs, groups.into())
  })
}

pub fn part1(input: &str) -> usize {
  parse_input(input)
    .map(|(springs, groups)| {
      count_arrangements(&mut HashMap::new(), springs, &groups)
    })
    .sum()
}

pub fn part2(input: &str) -> usize {
  parse_input(input)
    .map(|(springs, groups)| {
      count_arrangements(
        &mut HashMap::new(),
        &[springs; 5].join("?"),
        &groups.repeat(5),
      )
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
    assert_eq!(part1(INPUT_TEST), 21);
    assert_eq!(part1(INPUT), 7771);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 525_152);
    assert_eq!(part2(INPUT), 10_861_030_975_833);
  }
}
