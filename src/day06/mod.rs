const fn get_count(record_time: u64, record_distance: u64) -> u64 {
  let mut min_time = 0;
  let mut max_time = record_time;

  while min_time <= max_time {
    let mid_time = (min_time + max_time) / 2;
    let distance = mid_time * (record_time - mid_time);
    if record_distance < distance {
      max_time = mid_time - 1;
    } else {
      min_time = mid_time + 1;
    }
  }

  record_time - min_time * 2 + 1
}

pub fn part1(input: &str) -> u64 {
  let mut iter = input.lines().map(|line| {
    line
      .split_whitespace()
      .skip(1)
      .map(|s| s.parse::<u64>().unwrap())
  });

  let times = iter.next().unwrap();
  let distances = iter.next().unwrap();

  let records = times.zip(distances);

  records
    .map(|(record_time, record_distance)| {
      get_count(record_time, record_distance)
    })
    .product()
}

pub fn part2(input: &str) -> u64 {
  let mut iter = input.lines().map(|line| {
    line
      .split_whitespace()
      .skip(1)
      .collect::<String>()
      .parse::<u64>()
      .unwrap()
  });

  let record_time = iter.next().unwrap();
  let record_distance = iter.next().unwrap();

  get_count(record_time, record_distance)
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 288);
    assert_eq!(part1(INPUT), 3_317_888);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 71_503);
    assert_eq!(part2(INPUT), 24_655_068);
  }
}
