use std::array;

fn hash_label(s: &str) -> u32 {
  s.chars().fold(0, |v, c| ((v + (c as u32)) * 17) & 0xFF)
}

pub fn part1(input: &str) -> u32 {
  input.trim_end().split(',').map(hash_label).sum()
}

pub fn part2(input: &str) -> usize {
  #[derive(Debug)]
  enum Op {
    Rm,
    Eq(usize),
  }

  let instr = input.trim_end().split(',').map(|s| {
    let mut c = s.char_indices();
    let (op_pos, op) = c.find(|(_, c)| *c == '=' || *c == '-').unwrap();
    let label = &s[..op_pos];
    let op = match op {
      '=' => Op::Eq(c.as_str().parse().unwrap()),
      '-' => Op::Rm,
      _ => panic!("invalid char"),
    };
    (label, op)
  });

  let mut buckets: [_; 0x100] = array::from_fn(|_| Vec::new());

  for (label, op) in instr {
    let lenses = &mut buckets[hash_label(label) as usize];
    let index = lenses.iter().position(|&(l, _)| l == label);
    match op {
      Op::Rm => {
        if let Some(index) = index {
          lenses.remove(index);
        }
      }
      Op::Eq(focal_len) => {
        if let Some(index) = index {
          lenses[index] = (label, focal_len);
        } else {
          lenses.push((label, focal_len))
        }
      }
    }
  }

  buckets
    .into_iter()
    .enumerate()
    .map(|(bucket_num, lenses)| {
      lenses
        .into_iter()
        .enumerate()
        .map(|(lens_num, (_, focal_len))| {
          (bucket_num + 1) * (lens_num + 1) * (focal_len)
        })
        .sum::<usize>()
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 1320);
    assert_eq!(part1(INPUT), 497_373);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 145);
    assert_eq!(part2(INPUT), 259_356);
  }
}
