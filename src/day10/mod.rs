use std::{
  collections::HashMap,
  iter,
  ops::{AddAssign, SubAssign},
  str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point<T, const N: usize>(pub [T; N]);

impl<T> AddAssign<Direction> for Point<T, 2>
where
  T: AddAssign + SubAssign + num::One,
{
  fn add_assign(&mut self, dir: Direction) {
    let Self([x, y]) = self;
    match dir {
      Direction::North => *y -= num::one(),
      Direction::South => *y += num::one(),
      Direction::East => *x += num::one(),
      Direction::West => *x -= num::one(),
    }
  }
}

type Position = Point<i32, 2>;

#[derive(Debug, Clone, Copy)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  fn variants() -> impl Iterator<Item = Self> {
    [Self::North, Self::South, Self::East, Self::West].into_iter()
  }

  fn rotate(&mut self, rot: isize) {
    for _ in 0..rot.rem_euclid(4) {
      *self = match self {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
  Start,
  Vertical,
  Horizontal,
  NorthEast,
  NorthWest,
  SouthWest,
  SouthEast,
}

#[derive(Debug)]
struct Tiles(HashMap<Position, Tile>);

impl FromStr for Tiles {
  type Err = std::convert::Infallible;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut tiles = HashMap::new();

    for (y, line) in input.lines().enumerate() {
      for (x, c) in line.chars().enumerate() {
        tiles.insert(
          Point([x as i32, y as i32]),
          match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::Start,
            _ => continue,
          },
        );
      }
    }

    Ok(Self(tiles))
  }
}

impl Tiles {
  fn start_pos(&self) -> Option<Position> {
    self
      .0
      .iter()
      .find_map(|(p, t)| if let Tile::Start = t { Some(*p) } else { None })
  }

  fn steps(
    &self,
    mut pos: Position,
    mut dir: Direction,
  ) -> impl Iterator<Item = Position> + '_ {
    iter::from_fn(move || {
      pos += dir;
      dir.rotate(match (dir, *self.0.get(&pos)?) {
        (_, Tile::Start) => 0,
        (Direction::North, Tile::Vertical) => 0,
        (Direction::North, Tile::SouthWest) => -1,
        (Direction::North, Tile::SouthEast) => 1,
        (Direction::South, Tile::Vertical) => 0,
        (Direction::South, Tile::NorthEast) => -1,
        (Direction::South, Tile::NorthWest) => 1,
        (Direction::East, Tile::Horizontal) => 0,
        (Direction::East, Tile::NorthWest) => -1,
        (Direction::East, Tile::SouthWest) => 1,
        (Direction::West, Tile::Horizontal) => 0,
        (Direction::West, Tile::SouthEast) => -1,
        (Direction::West, Tile::NorthEast) => 1,
        _ => return None,
      });
      Some(pos)
    })
  }
}

pub fn part1(input: &str) -> usize {
  let tiles: Tiles = input.parse().unwrap();

  let start_pos = tiles.start_pos().unwrap();

  Direction::variants()
    .filter_map(|start_dir| {
      let (steps, _) = tiles.steps(start_pos, start_dir).enumerate().last()?;
      Some(steps / 2 + 1)
    })
    .max()
    .unwrap()
}

pub fn part2(input: &str) -> i32 {
  let tiles: Tiles = input.parse().unwrap();

  let start_pos = tiles.start_pos().unwrap();

  let (_, start_dir) = Direction::variants()
    .filter_map(|start_dir| {
      let (steps, _) = tiles.steps(start_pos, start_dir).enumerate().last()?;
      Some((steps, start_dir))
    })
    .max_by_key(|&(steps, _)| steps)
    .unwrap();

  let path = iter::once(start_pos)
    .chain(tiles.steps(start_pos, start_dir))
    .collect::<Vec<_>>();

  let twice_area = path
    .iter()
    .tuple_windows()
    .map(|(Point([x0, y0]), Point([x1, y1]))| (x1 + x0) * (y1 - y0))
    .sum::<i32>()
    .abs();

  twice_area / 2 - (path.len() as i32 / 2 - 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("input.txt");
  const INPUT_TEST_0: &str = include_str!("input_test_0.txt");
  const INPUT_TEST_1: &str = include_str!("input_test_1.txt");
  const INPUT_TEST_2: &str = include_str!("input_test_2.txt");
  const INPUT_TEST_3: &str = include_str!("input_test_3.txt");
  const INPUT_TEST_4: &str = include_str!("input_test_4.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST_0), 4);
    assert_eq!(part1(INPUT_TEST_1), 8);
    assert_eq!(part1(INPUT), 6690);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST_2), 4);
    assert_eq!(part2(INPUT_TEST_3), 8);
    assert_eq!(part2(INPUT_TEST_4), 10);
    assert_eq!(part2(INPUT), 525);
  }
}
