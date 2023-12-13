use std::{array, collections::HashSet, ops::RangeInclusive};

fn ranges<'a, T: Copy + Ord + 'a, const N: usize>(
  items: impl Iterator<Item = &'a [T; N]>,
) -> Option<[RangeInclusive<T>; N]> {
  items.fold(None, |ranges, item| {
    Some(if let Some(ranges) = ranges {
      array::from_fn(|i| {
        item[i].min(*ranges[i].start())..=item[i].max(*ranges[i].end())
      })
    } else {
      array::from_fn(|i| item[i]..=item[i])
    })
  })
}

fn maps(input: &str) -> impl Iterator<Item = HashSet<[usize; 2]>> + '_ {
  input.split("\n\n").map(|pattern| {
    let mut map = HashSet::new();

    for (y, line) in pattern.lines().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          map.insert([x, y]);
        }
      }
    }

    map
  })
}

pub fn part1(input: &str) -> usize {
  maps(input)
    .map(|map| {
      let [x_range, y_range] = ranges(map.iter()).unwrap();

      if let Some(vertical) =
        (x_range.start() + 1..=*x_range.end()).find(|&x| {
          y_range.clone().all(|y| {
            (*x_range.start()..x)
              .rev()
              .zip(x..=*x_range.end())
              .all(|(x0, x1)| map.contains(&[x0, y]) == map.contains(&[x1, y]))
          })
        })
      {
        return vertical;
      }

      if let Some(horizontal) =
        (y_range.start() + 1..=*y_range.end()).find(|&y| {
          x_range.clone().all(|x| {
            (*y_range.start()..y)
              .rev()
              .zip(y..=*y_range.end())
              .all(|(y0, y1)| map.contains(&[x, y0]) == map.contains(&[x, y1]))
          })
        })
      {
        return horizontal * 100;
      }

      panic!("No line found!");
    })
    .sum()
}

pub fn part2(input: &str) -> usize {
  maps(input)
    .map(|map| {
      let [x_range, y_range] = ranges(map.iter()).unwrap();

      if let Some(vertical) =
        (x_range.start() + 1..=*x_range.end()).find(|&x| {
          y_range
            .clone()
            .map(|y| {
              (*x_range.start()..x)
                .rev()
                .zip(x..=*x_range.end())
                .filter(|&(x0, x1)| {
                  map.contains(&[x0, y]) != map.contains(&[x1, y])
                })
                .count()
            })
            .sum::<usize>()
            == 1
        })
      {
        return vertical;
      }

      if let Some(horizontal) =
        (y_range.start() + 1..=*y_range.end()).find(|&y| {
          x_range
            .clone()
            .map(|x| {
              (*y_range.start()..y)
                .rev()
                .zip(y..=*y_range.end())
                .filter(|&(y0, y1)| {
                  map.contains(&[x, y0]) != map.contains(&[x, y1])
                })
                .count()
            })
            .sum::<usize>()
            == 1
        })
      {
        return horizontal * 100;
      }

      panic!("No line found!");
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
    assert_eq!(part1(INPUT_TEST), 405);
    assert_eq!(part1(INPUT), 34_911);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 400);
    assert_eq!(part2(INPUT), 33_183);
  }
}
