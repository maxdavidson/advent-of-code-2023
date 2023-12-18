#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, u8, u32)> + '_ {
  input.lines().map(|line| {
    let mut words = line.split_whitespace();
    let dir = match words.next().unwrap() {
      "U" => Direction::Up,
      "D" => Direction::Down,
      "L" => Direction::Left,
      "R" => Direction::Right,
      _ => panic!(),
    };

    let len = words.next().unwrap().parse().unwrap();

    let color = u32::from_str_radix(
      words
        .next()
        .unwrap()
        .trim_start_matches("(#")
        .trim_end_matches(')'),
      16,
    )
    .unwrap();

    (dir, len, color)
  })
}

fn solve(iter: impl Iterator<Item = (Direction, i64)>) -> i64 {
  let mut twice_area = 0;
  let mut circumference = 0;
  let mut pos = [0, 0];

  for (dir, len) in iter {
    let next_pos = match dir {
      Direction::Up => [pos[0], pos[1] - len],
      Direction::Down => [pos[0], pos[1] + len],
      Direction::Left => [pos[0] - len, pos[1]],
      Direction::Right => [pos[0] + len, pos[1]],
    };
    twice_area += (pos[0] + next_pos[0]) * (pos[1] - next_pos[1]);
    circumference += len;
    pos = next_pos;
  }

  twice_area.abs() / 2 + (circumference / 2 + 1)
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
