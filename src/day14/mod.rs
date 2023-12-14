use std::{
  collections::{
    hash_map::{self, DefaultHasher},
    HashMap,
  },
  fmt::Display,
  hash::{Hash, Hasher},
  str::FromStr,
};

#[derive(PartialEq, Eq, Hash)]
enum Entry {
  RoundRock,
  CubeRock,
  Empty,
}

#[derive(PartialEq, Eq, Hash)]
struct ParabolicReflectorDish {
  rows: usize,
  entries: Vec<Entry>,
}

#[derive(Debug)]
enum ParabolicReflectorDishParseError {
  InvalidChar(char),
}

impl FromStr for ParabolicReflectorDish {
  type Err = ParabolicReflectorDishParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut rows = 0;
    let mut entries = Vec::new();

    for line in s.lines() {
      rows += 1;
      for c in line.chars() {
        entries.push(match c {
          'O' => Entry::RoundRock,
          '#' => Entry::CubeRock,
          '.' => Entry::Empty,
          c => {
            return Err(ParabolicReflectorDishParseError::InvalidChar(c));
          }
        });
      }
    }

    Ok(Self { rows, entries })
  }
}

impl ParabolicReflectorDish {
  fn bounds(&self) -> [[usize; 2]; 2] {
    let y_min = 0;
    let y_max = self.rows - 1;
    let x_min = 0;
    let x_max = (self.entries.len() / self.rows) - 1;
    [[x_min, x_max], [y_min, y_max]]
  }
}

impl Display for ParabolicReflectorDish {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let [[x_min, x_max], [y_min, y_max]] = self.bounds();

    for y in y_min..=y_max {
      for x in x_min..=x_max {
        write!(
          f,
          "{}",
          match self.entries[y * self.rows + x] {
            Entry::CubeRock => '#',
            Entry::RoundRock => 'O',
            Entry::Empty => '.',
          }
        )?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl ParabolicReflectorDish {
  fn tilt_north(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = self.bounds();

    for x in x_min..=x_max {
      for y in y_min..=y_max {
        let index = y * self.rows + x;
        if let Entry::Empty = self.entries[index] {
          for y_next in y..=y_max {
            let index_next = y_next * self.rows + x;
            match self.entries[index_next] {
              Entry::Empty => {
                continue;
              }
              Entry::CubeRock => {
                break;
              }
              Entry::RoundRock => {
                self.entries.swap(index, index_next);
                break;
              }
            }
          }
        }
      }
    }
  }

  fn tilt_south(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = self.bounds();

    for x in x_min..=x_max {
      for y in (y_min..=y_max).rev() {
        let index = y * self.rows + x;
        if let Entry::Empty = self.entries[index] {
          for y_next in (y_min..=y).rev() {
            let index_next = y_next * self.rows + x;
            match self.entries[index_next] {
              Entry::Empty => {
                continue;
              }
              Entry::CubeRock => {
                break;
              }
              Entry::RoundRock => {
                self.entries.swap(index, index_next);
                break;
              }
            }
          }
        }
      }
    }
  }

  fn tilt_west(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = self.bounds();

    for y in y_min..=y_max {
      for x in x_min..=x_max {
        let index = y * self.rows + x;
        if let Entry::Empty = self.entries[index] {
          for x_next in x..=x_max {
            let index_next = y * self.rows + x_next;
            match self.entries[index_next] {
              Entry::Empty => {
                continue;
              }
              Entry::CubeRock => {
                break;
              }
              Entry::RoundRock => {
                self.entries.swap(index, index_next);
                break;
              }
            }
          }
        }
      }
    }
  }

  fn tilt_east(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = self.bounds();

    for y in y_min..=y_max {
      for x in (x_min..=x_max).rev() {
        let index = y * self.rows + x;
        if let Entry::Empty = self.entries[index] {
          for x_next in (x_min..=x).rev() {
            let index_next = y * self.rows + x_next;
            match self.entries[index_next] {
              Entry::Empty => {
                continue;
              }
              Entry::CubeRock => {
                break;
              }
              Entry::RoundRock => {
                self.entries.swap(index, index_next);
                break;
              }
            }
          }
        }
      }
    }
  }

  fn north_beams_load(&self) -> usize {
    let [_, [_, y_max]] = self.bounds();

    self
      .entries
      .iter()
      .enumerate()
      .filter_map(|(index, entry)| match entry {
        Entry::RoundRock => Some(y_max - index / self.rows + 1),
        _ => None,
      })
      .sum()
  }
}

pub fn part1(input: &str) -> usize {
  let mut dish: ParabolicReflectorDish = input.parse().unwrap();

  dish.tilt_north();

  dish.north_beams_load()
}

pub fn part2(input: &str) -> usize {
  let mut dish: ParabolicReflectorDish = input.parse().unwrap();

  let mut dish_hashes = HashMap::new();

  let mut results = Vec::new();

  for n in 0usize.. {
    let dish_hash = {
      let mut hasher = DefaultHasher::new();
      dish.hash(&mut hasher);
      hasher.finish()
    };

    match dish_hashes.entry(dish_hash) {
      hash_map::Entry::Occupied(entry) => {
        let prev_n = entry.get();
        let cycle_start = prev_n;
        let cycle_len = n - prev_n;
        return results
          [(1_000_000_000 - cycle_start) % cycle_len + cycle_start];
      }
      hash_map::Entry::Vacant(entry) => {
        entry.insert(n);
        results.push(dish.north_beams_load());
      }
    };

    dish.tilt_north();
    dish.tilt_west();
    dish.tilt_south();
    dish.tilt_east();
  }

  unreachable!();
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 136);
    assert_eq!(part1(INPUT), 112_773);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 64);
    assert_eq!(part2(INPUT), 98_894);
  }
}
