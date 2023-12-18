use std::sync::OnceLock;

use itertools::Itertools;
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, u8, u32)> + '_ {
  let pattern = {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
      RegexBuilder::new(r"^(U|D|L|R) (\d+) \(#([0-9a-z]+)\)$")
        .multi_line(true)
        .build()
        .unwrap()
    })
  };

  pattern.captures_iter(input).map(|c| {
    let dir = match c.get(1).unwrap().as_str() {
      "U" => Direction::Up,
      "D" => Direction::Down,
      "L" => Direction::Left,
      "R" => Direction::Right,
      _ => panic!(),
    };

    let len = c.get(2).unwrap().as_str().parse().unwrap();

    let color = u32::from_str_radix(c.get(3).unwrap().as_str(), 16).unwrap();

    (dir, len, color)
  })
}

fn solve(iter: impl Iterator<Item = (Direction, i64)>) -> i64 {
  let mut vertices = Vec::new();
  let mut circumference = 0;

  vertices.push([0, 0]);

  for (dir, len) in iter {
    let prev_pos = vertices.last().unwrap();
    let next_pos = match dir {
      Direction::Up => [prev_pos[0], prev_pos[1] - len],
      Direction::Down => [prev_pos[0], prev_pos[1] + len],
      Direction::Left => [prev_pos[0] - len, prev_pos[1]],
      Direction::Right => [prev_pos[0] + len, prev_pos[1]],
    };
    vertices.push(next_pos);
    circumference += len;
  }

  assert_eq!(vertices.first(), vertices.last());

  let twice_area = vertices
    .into_iter()
    .tuple_windows()
    .map(|([x0, y0], [x1, y1])| (x0 + x1) * (y0 - y1))
    .sum::<i64>()
    .abs();

  twice_area / 2 + (circumference / 2 + 1)
}

pub fn part1(input: &str) -> i64 {
  solve(parse_input(input).map(|(dir, len, _)| (dir, len as i64)))
}

pub fn part2(input: &str) -> i64 {
  solve(parse_input(input).map(|(_, _, color)| {
    let dir = match color & 0b111 {
      0 => Direction::Right,
      1 => Direction::Down,
      2 => Direction::Left,
      3 => Direction::Up,
      _ => panic!(),
    };
    let len = color >> 4;
    (dir, len as i64)
  }))
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 62);
    assert_eq!(part1(INPUT), 40_131);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 952_408_144_115);
    assert_eq!(part2(INPUT), 104_454_050_898_331);
  }
}
