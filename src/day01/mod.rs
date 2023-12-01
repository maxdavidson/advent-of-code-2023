use regex::Regex;

fn lines(input: &str) -> impl Iterator<Item = &str> {
  input.trim().lines().map(|line| line.trim())
}

pub fn part1(input: &str) -> u32 {
  lines(input)
    .map(|line| {
      let first_digit = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .unwrap();

      let last_digit = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .last()
        .and_then(|c| c.to_digit(10))
        .unwrap();

      first_digit * 10 + last_digit
    })
    .sum()
}

pub fn part2(input: &str) -> u32 {
  let first_digit_pattern =
    Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*$")
      .unwrap();

  let last_digit_pattern =
    Regex::new(r"^.*(\d|one|two|three|four|five|six|seven|eight|nine).*?$")
      .unwrap();

  let get_digit = |m: &str| match m {
    "1" | "one" => 1,
    "2" | "two" => 2,
    "3" | "three" => 3,
    "4" | "four" => 4,
    "5" | "five" => 5,
    "6" | "six" => 6,
    "7" | "seven" => 7,
    "8" | "eight" => 8,
    "9" | "nine" => 9,
    _ => unreachable!(),
  };

  let mut first_digit_locations = first_digit_pattern.capture_locations();
  let mut last_digit_locations = last_digit_pattern.capture_locations();

  lines(input)
    .map(|line| {
      first_digit_pattern
        .captures_read(&mut first_digit_locations, line)
        .unwrap();

      last_digit_pattern
        .captures_read(&mut last_digit_locations, line)
        .unwrap();

      let first_digit = first_digit_locations
        .get(1)
        .map(|(a, b)| &line[a..b])
        .map(get_digit)
        .unwrap();

      let last_digit = last_digit_locations
        .get(1)
        .map(|(a, b)| &line[a..b])
        .map(get_digit)
        .unwrap();

      first_digit * 10 + last_digit
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(
      part1(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
      ),
      142
    );

    assert_eq!(part1(INPUT), 54990);
  }

  #[test]
  fn part2_works() {
    assert_eq!(
      part2(
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
      ),
      281
    );
    assert_eq!(part2(INPUT), 54473);
  }
}
