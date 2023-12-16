use std::{
  collections::HashSet,
  fmt::{Display, Write},
};

use crate::utils::Grid;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
  EmptySpace,
  LeftMirror,
  RightMirror,
  VerticalSplitter,
  HorizontalSplitter,
}

#[derive(Debug)]
enum ParseItemError {
  InvalidChar(char),
}

impl TryFrom<char> for Tile {
  type Error = ParseItemError;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      '.' => Ok(Self::EmptySpace),
      '|' => Ok(Self::VerticalSplitter),
      '-' => Ok(Self::HorizontalSplitter),
      '\\' => Ok(Self::LeftMirror),
      '/' => Ok(Self::RightMirror),
      c => Err(ParseItemError::InvalidChar(c)),
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::EmptySpace => f.write_char('.'),
      Self::VerticalSplitter => f.write_char('|'),
      Self::HorizontalSplitter => f.write_char('-'),
      Self::LeftMirror => f.write_char('\\'),
      Self::RightMirror => f.write_char('/'),
    }
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
  fn step(self, [x, y]: [isize; 2]) -> [isize; 2] {
    match self {
      Direction::Up => [x, y - 1],
      Direction::Left => [x - 1, y],
      Direction::Down => [x, y + 1],
      Direction::Right => [x + 1, y],
    }
  }

  fn turn_left(self) -> Direction {
    match self {
      Direction::Up => Direction::Left,
      Direction::Left => Direction::Down,
      Direction::Down => Direction::Right,
      Direction::Right => Direction::Up,
    }
  }

  fn turn_right(self) -> Direction {
    match self {
      Direction::Up => Direction::Right,
      Direction::Left => Direction::Up,
      Direction::Down => Direction::Left,
      Direction::Right => Direction::Down,
    }
  }
}

impl Grid<Tile> {
  fn energized_len(
    &self,
    start_pos: [isize; 2],
    start_dir: Direction,
  ) -> usize {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();

    queue.push((start_pos, start_dir));

    while let Some((pos, dir)) = queue.pop() {
      if !visited.contains(&(pos, dir)) {
        if let Some(tile) = self.get(pos) {
          visited.insert((pos, dir));
          match (tile, dir) {
            (Tile::EmptySpace, _)
            | (Tile::HorizontalSplitter, Direction::Left | Direction::Right)
            | (Tile::VerticalSplitter, Direction::Up | Direction::Down) => {
              queue.push((dir.step(pos), dir));
            }

            (Tile::LeftMirror, Direction::Left | Direction::Right)
            | (Tile::RightMirror, Direction::Up | Direction::Down) => {
              let dir = dir.turn_right();
              queue.push((dir.step(pos), dir));
            }

            (Tile::LeftMirror, Direction::Up | Direction::Down)
            | (Tile::RightMirror, Direction::Left | Direction::Right) => {
              let dir = dir.turn_left();
              queue.push((dir.step(pos), dir));
            }

            (Tile::HorizontalSplitter, Direction::Up | Direction::Down)
            | (Tile::VerticalSplitter, Direction::Left | Direction::Right) => {
              {
                let dir = dir.turn_right();
                queue.push((dir.step(pos), dir));
              }

              {
                let dir = dir.turn_left();
                queue.push((dir.step(pos), dir));
              }
            }
          }
        }
      }
    }

    visited
      .into_iter()
      .map(|(pos, _)| pos)
      .collect::<HashSet<_>>()
      .len()
  }
}

pub fn part1(input: &str) -> usize {
  let grid: Grid<Tile> = input.parse().unwrap();

  grid.energized_len([0isize, 0], Direction::Right)
}

pub fn part2(input: &str) -> usize {
  let grid: Grid<Tile> = input.parse().unwrap();

  let rows = grid.rows();
  let cols = grid.cols();

  (0..rows)
    .flat_map(|y| {
      [([0, y], Direction::Right), ([cols - 1, y], Direction::Left)]
    })
    .chain((0..cols).flat_map(|x| {
      [([x, 0], Direction::Down), ([x, rows - 1], Direction::Up)]
    }))
    .map(|(pos, dir)| grid.energized_len(pos, dir))
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 46);
    assert_eq!(part1(INPUT), 7798);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 51);
    // Too slow, take ≈5s
    // assert_eq!(part2(INPUT), 8026);
  }
}
