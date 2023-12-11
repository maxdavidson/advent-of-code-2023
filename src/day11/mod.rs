pub fn solve(input: &str, multiplier: i64) -> i64 {
  let mut galaxies = Vec::new();

  let mut y_max = 0;
  let mut x_max = 0;

  for (y, line) in input.lines().enumerate() {
    y_max = y_max.max(y);
    for (x, c) in line.chars().enumerate() {
      x_max = x_max.max(x);
      if c == '#' {
        galaxies.push([x as i64, y as i64]);
      }
    }
  }

  let mut empty_cols = vec![true; x_max + 1];
  let mut empty_rows = vec![true; y_max + 1];

  for &[x, y] in &galaxies {
    empty_cols[x as usize] = false;
    empty_rows[y as usize] = false;
  }

  let mut sum = 0;

  for (n, &[x0, y0]) in galaxies.iter().enumerate() {
    for &[x1, y1] in galaxies.iter().skip(n + 1) {
      let mut empty_spaces = 0;

      let x_diff = (x1 - x0).abs();
      let x_min = x0.min(x1);

      let y_diff = (y1 - y0).abs();
      let y_min = y0.min(y1);

      for x in x_min..=x_min + x_diff {
        if empty_cols[x as usize] {
          empty_spaces += 1
        };
      }

      for y in y_min..=y_min + y_diff {
        if empty_rows[y as usize] {
          empty_spaces += 1
        };
      }

      sum += x_diff + y_diff + (multiplier - 1) * empty_spaces;
    }
  }

  sum
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(solve(INPUT_TEST, 2), 374);
    assert_eq!(solve(INPUT, 2), 9_742_154);
  }

  #[test]
  fn part2_works() {
    assert_eq!(solve(INPUT_TEST, 10), 1030);
    assert_eq!(solve(INPUT_TEST, 100), 8410);
    assert_eq!(solve(INPUT, 1_000_000), 411_142_919_886);
  }
}
