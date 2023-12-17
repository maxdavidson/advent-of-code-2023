use std::{
  collections::{BinaryHeap, HashSet},
  convert::Infallible,
  fmt::Display,
  ops::Deref,
};

use crate::utils::Grid;

#[derive(Debug)]
struct Digit(u8);

impl Deref for Digit {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<char> for Digit {
  type Error = Infallible;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    Ok(Self(c.to_digit(10).unwrap().try_into().unwrap()))
  }
}

impl Display for Digit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Left,
  Down,
  Right,
}

impl Direction {
  const fn variants() -> [Self; 4] {
    [Self::Up, Self::Left, Self::Down, Self::Right]
  }

  fn step(self, [x, y]: [isize; 2]) -> [isize; 2] {
    match self {
      Direction::Up => [x, y - 1],
      Direction::Left => [x - 1, y],
      Direction::Down => [x, y + 1],
      Direction::Right => [x + 1, y],
    }
  }

  fn flip(self) -> Self {
    match self {
      Direction::Up => Direction::Down,
      Direction::Left => Direction::Right,
      Direction::Down => Direction::Up,
      Direction::Right => Direction::Left,
    }
  }
}

type Position = [isize; 2];

#[derive(Eq, PartialEq)]
struct QueueEntry {
  pos: Position,
  dir: Direction,
  cost: u32,
  steps: usize,
}

#[derive(Eq, PartialEq, Hash)]
struct VisitedKey {
  pos: Position,
  dir: Direction,
  steps: usize,
}

impl From<QueueEntry> for VisitedKey {
  fn from(
    QueueEntry {
      pos, dir, steps, ..
    }: QueueEntry,
  ) -> Self {
    Self { pos, dir, steps }
  }
}

impl std::cmp::Ord for QueueEntry {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.cost.cmp(&other.cost).reverse()
  }
}

impl std::cmp::PartialOrd for QueueEntry {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Grid<Digit> {
  pub fn least_heat_loss(
    &self,
    min_steps: usize,
    max_steps: usize,
  ) -> Option<u32> {
    let rows = self.rows();
    let cols = self.cols();

    let start_pos: Position = [0, 0];
    let end_pos: Position = [cols - 1, rows - 1];

    let mut visited = HashSet::<VisitedKey>::new();
    let mut queue = BinaryHeap::<QueueEntry>::new();

    queue.push(QueueEntry {
      pos: start_pos,
      dir: Direction::Down,
      cost: 0,
      steps: 0,
    });

    queue.push(QueueEntry {
      pos: start_pos,
      dir: Direction::Right,
      cost: 0,
      steps: 0,
    });

    while let Some(QueueEntry {
      pos,
      dir,
      cost,
      steps,
    }) = queue.pop()
    {
      if pos == end_pos && steps >= min_steps {
        return Some(cost);
      }

      for next_dir in Direction::variants() {
        if next_dir == dir.flip()
          || (next_dir == dir && steps >= max_steps)
          || (next_dir != dir && steps < min_steps)
        {
          continue;
        }

        let next_pos = next_dir.step(pos);

        let Some(&Digit(weight)) = self.get(next_pos) else {
          continue;
        };

        let next_cost = cost + (weight as u32);
        let next_steps = if next_dir == dir { steps + 1 } else { 1 };

        if visited.insert(VisitedKey {
          pos: next_pos,
          dir: next_dir,
          steps: next_steps,
        }) {
          queue.push(QueueEntry {
            pos: next_pos,
            dir: next_dir,
            cost: next_cost,
            steps: next_steps,
          });
        }
      }
    }

    None
  }
}

pub fn part1(input: &str) -> u32 {
  let grid: Grid<Digit> = input.parse().unwrap();

  grid.least_heat_loss(0, 3).unwrap()
}

pub fn part2(input: &str) -> u32 {
  let grid: Grid<Digit> = input.parse().unwrap();

  grid.least_heat_loss(4, 10).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("input.txt");
  const INPUT_TEST: &str = include_str!("input_test.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 102);
    assert_eq!(part1(INPUT), 1008);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 94);
    assert_eq!(part2(INPUT), 1210);
  }
}
