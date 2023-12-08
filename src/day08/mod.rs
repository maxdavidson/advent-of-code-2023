use std::collections::HashMap;

#[derive(Debug)]
enum Step {
  Left,
  Right,
}

struct Network<'a> {
  steps: Vec<Step>,
  nodes: HashMap<&'a str, [&'a str; 2]>,
}

impl<'a> From<&'a str> for Network<'a> {
  fn from(input: &'a str) -> Self {
    let mut lines = input.lines();

    let steps = lines
      .next()
      .unwrap()
      .chars()
      .filter_map(|c| match c {
        'L' => Some(Step::Left),
        'R' => Some(Step::Right),
        _ => None,
      })
      .collect();

    lines.next().unwrap();

    let nodes = lines
      .filter_map(|line| {
        let (from, rest) = line.split_once(" = ")?;
        let &(left, right) = &rest[1..rest.len() - 1].split_once(", ")?;
        Some((from, [left, right]))
      })
      .collect();

    Network { steps, nodes }
  }
}

impl Network<'_> {
  fn nodes(&self) -> impl Iterator<Item = &str> {
    self.nodes.keys().copied()
  }

  fn steps<'a>(&'a self, node: &'a str) -> impl Iterator<Item = &'a str> {
    self.steps.iter().cycle().scan(node, |node, step| {
      let &[left_node, right_node] = self.nodes.get(node)?;
      *node = match *step {
        Step::Left => left_node,
        Step::Right => right_node,
      };
      Some(*node)
    })
  }

  fn search(
    &self,
    from_node: &str,
    to_node: impl FnMut(&str) -> bool,
  ) -> usize {
    self.steps(from_node).position(to_node).unwrap() + 1
  }
}

pub fn part1(input: &str) -> usize {
  let network = Network::from(input);

  network.search("AAA", |node| node == "ZZZ")
}

pub fn part2(input: &str) -> usize {
  let network = Network::from(input);

  network
    .nodes()
    .filter(|node| node.ends_with('A'))
    .map(|node| network.search(node, |node| node.ends_with('Z')))
    .fold(1, num::integer::lcm)
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("input.txt");
  const INPUT_TEST_0: &str = include_str!("input_test_0.txt");
  const INPUT_TEST_1: &str = include_str!("input_test_1.txt");
  const INPUT_TEST_2: &str = include_str!("input_test_2.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST_0), 2);
    assert_eq!(part1(INPUT_TEST_1), 6);
    assert_eq!(part1(INPUT), 23_147);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST_2), 6);
    assert_eq!(part2(INPUT), 22_289_513_667_691);
  }
}
