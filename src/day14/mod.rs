use std::{
  array,
  collections::{
    hash_map::{self, DefaultHasher},
    BTreeMap, HashMap,
  },
  convert::Infallible,
  fmt::Display,
  hash::{Hash, Hasher},
  str::FromStr,
};

fn bounds<'a, T: Copy + Ord + 'a, const N: usize>(
  items: impl Iterator<Item = &'a [T; N]>,
) -> Option<[[T; 2]; N]> {
  items.fold(None, |ranges, item| {
    Some(if let Some(ranges) = ranges {
      array::from_fn(|i| {
        let [min, max] = ranges[i];
        [item[i].min(min), item[i].max(max)]
      })
    } else {
      array::from_fn(|i| [item[i], item[i]])
    })
  })
}

#[derive(PartialEq, Eq, Hash)]
enum Rock {
  Round,
  Cube,
}

#[derive(PartialEq, Eq, Hash)]
struct ParabolicReflectorDish {
  map: BTreeMap<[usize; 2], Rock>,
}

impl FromStr for ParabolicReflectorDish {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut map = BTreeMap::new();

    for (y, line) in s.lines().enumerate() {
      for (x, c) in line.chars().enumerate() {
        map.insert(
          [x, y],
          match c {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            _ => continue,
          },
        );
      }
    }

    Ok(Self { map })
  }
}

impl Display for ParabolicReflectorDish {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let [[x_min, x_max], [y_min, y_max]] = bounds(self.map.keys()).unwrap();

    for y in y_min..=y_max {
      for x in x_min..=x_max {
        write!(
          f,
          "{}",
          match self.map.get(&[x, y]) {
            Some(Rock::Cube) => '#',
            Some(Rock::Round) => 'O',
            None => '.',
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
    let [[x_min, x_max], [y_min, y_max]] = bounds(self.map.keys()).unwrap();

    for x in x_min..=x_max {
      for y in y_min..=y_max {
        if self.map.get(&[x, y]).is_none() {
          if let Some((next_y, &Rock::Round)) =
            (y..=y_max).find_map(|y| Some((y, self.map.get(&[x, y])?)))
          {
            self.map.remove(&[x, next_y]).unwrap();
            self.map.insert([x, y], Rock::Round);
          }
        }
      }
    }
  }

  fn tilt_south(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = bounds(self.map.keys()).unwrap();

    for x in x_min..=x_max {
      for y in (y_min..=y_max).rev() {
        if self.map.get(&[x, y]).is_none() {
          if let Some((next_y, &Rock::Round)) = (y_min..=y)
            .rev()
            .find_map(|y| Some((y, self.map.get(&[x, y])?)))
          {
            self.map.remove(&[x, next_y]).unwrap();
            self.map.insert([x, y], Rock::Round);
          }
        }
      }
    }
  }

  fn tilt_west(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = bounds(self.map.keys()).unwrap();

    for y in (y_min..=y_max).rev() {
      for x in x_min..=x_max {
        if self.map.get(&[x, y]).is_none() {
          if let Some((next_x, &Rock::Round)) =
            (x..=x_max).find_map(|x| Some((x, self.map.get(&[x, y])?)))
          {
            self.map.remove(&[next_x, y]).unwrap();
            self.map.insert([x, y], Rock::Round);
          }
        }
      }
    }
  }

  fn tilt_east(&mut self) {
    let [[x_min, x_max], [y_min, y_max]] = bounds(self.map.keys()).unwrap();

    for y in y_min..=y_max {
      for x in (x_min..=x_max).rev() {
        if self.map.get(&[x, y]).is_none() {
          if let Some((next_x, &Rock::Round)) = (x_min..=x)
            .rev()
            .find_map(|x| Some((x, self.map.get(&[x, y])?)))
          {
            self.map.remove(&[next_x, y]).unwrap();
            self.map.insert([x, y], Rock::Round);
          }
        }
      }
    }
  }

  fn north_beams_load(&self) -> usize {
    let [_, [_, y_max]] = bounds(self.map.keys()).unwrap();

    self
      .map
      .iter()
      .filter_map(|([_, y], rock)| match rock {
        Rock::Round => Some(y_max - y + 1),
        Rock::Cube => None,
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
