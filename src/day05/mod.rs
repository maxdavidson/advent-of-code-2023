use std::{array, convert::Infallible, mem, str::FromStr};

struct SeedsAndTransforms {
  seeds: Vec<u64>,
  transforms: Vec<Vec<[u64; 3]>>,
}

impl FromStr for SeedsAndTransforms {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut groups_iter = s.trim_end().split("\n\n");

    let seeds = groups_iter
      .next()
      .unwrap()
      .strip_prefix("seeds: ")
      .unwrap()
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();

    let transforms = groups_iter
      .map(|group| {
        group
          .lines()
          .skip(1)
          .map(|line| {
            let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());
            array::from_fn(|_| iter.next().unwrap())
          })
          .collect()
      })
      .collect();

    Ok(Self { seeds, transforms })
  }
}

pub fn part1(input: &str) -> u64 {
  let SeedsAndTransforms { seeds, transforms } = input.parse().unwrap();

  let mut numbers = seeds;

  for number in &mut numbers {
    for transform_ranges in &transforms {
      for &[dst_start, src_start, len] in transform_ranges {
        if (src_start..src_start + len).contains(number) {
          *number += dst_start;
          *number -= src_start;
          break;
        }
      }
    }
  }

  numbers.into_iter().min().unwrap()
}

pub fn part2(input: &str) -> u64 {
  let SeedsAndTransforms { seeds, transforms } = input.parse().unwrap();

  let seed_ranges = seeds
    .chunks(2)
    .map(|c| c[0]..c[0] + c[1])
    .collect::<Vec<_>>();

  let mut ranges = seed_ranges;
  let mut next_ranges = Vec::with_capacity(ranges.capacity());

  for transform_ranges in transforms {
    'next_range: while let Some(range) = ranges.pop() {
      for &[dst_start, src_start, len] in &transform_ranges {
        let transform_range = src_start..src_start + len;

        let max_start = range.start.max(transform_range.start);
        let min_end = range.end.min(transform_range.end);

        if max_start < min_end {
          if range.start < max_start {
            ranges.push(range.start..max_start);
          }

          if min_end < range.end {
            ranges.push(min_end..range.end);
          }

          next_ranges.push(
            max_start + dst_start - transform_range.start
              ..min_end + dst_start - transform_range.start,
          );

          continue 'next_range;
        }
      }

      next_ranges.push(range);
    }

    mem::swap(&mut ranges, &mut next_ranges);
  }

  ranges.into_iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 35);
    assert_eq!(part1(INPUT), 324_724_204);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 46);
    assert_eq!(part2(INPUT), 104_070_862);
  }
}
