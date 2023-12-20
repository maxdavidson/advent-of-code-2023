use std::{
  collections::{HashMap, HashSet},
  ops::Range,
};

#[derive(Debug)]
enum RuleOperator {
  LessThan,
  GreaterThan,
}

#[derive(Debug)]
struct RuleTest<'a> {
  category: &'a str,
  operator: RuleOperator,
  value: u64,
}

type Part<'a> = HashMap<&'a str, u64>;
type Workflow<'a> = Vec<Rule<'a>>;
type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;

impl RuleTest<'_> {
  fn call(&self, part: &Part) -> bool {
    let part_value = part[self.category];
    match self.operator {
      RuleOperator::LessThan => part_value < self.value,
      RuleOperator::GreaterThan => part_value > self.value,
    }
  }
}

#[derive(Debug)]
enum RuleOutcome<'a> {
  Final(bool),
  Workflow(&'a str),
}

#[derive(Debug)]
enum Rule<'a> {
  Condition(RuleTest<'a>, RuleOutcome<'a>),
  Fallback(RuleOutcome<'a>),
}

fn parse_input(input: &str) -> (Workflows, impl Iterator<Item = Part>) {
  let (workflows, inputs) = input.split_once("\n\n").unwrap();

  let workflows = workflows
    .lines()
    .map(|line| {
      let (workflow, rest) = line.split_once('{').unwrap();
      let rules = rest
        .trim_end_matches('}')
        .split(',')
        .map(|rule| {
          if let Some((test, target)) = rule.split_once(':') {
            let test = if let Some((category, value)) = test.split_once('>') {
              RuleTest {
                category,
                value: value.parse().unwrap(),
                operator: RuleOperator::GreaterThan,
              }
            } else if let Some((category, value)) = test.split_once('<') {
              RuleTest {
                category,
                value: value.parse().unwrap(),
                operator: RuleOperator::LessThan,
              }
            } else {
              panic!()
            };

            let outcome = match target {
              "A" => RuleOutcome::Final(true),
              "R" => RuleOutcome::Final(false),
              workflow => RuleOutcome::Workflow(workflow),
            };

            Rule::Condition(test, outcome)
          } else {
            let outcome = match rule {
              "A" => RuleOutcome::Final(true),
              "R" => RuleOutcome::Final(false),
              workflow => RuleOutcome::Workflow(workflow),
            };

            Rule::Fallback(outcome)
          }
        })
        .collect();

      (workflow, rules)
    })
    .collect();

  let parts = inputs.lines().map(|line| {
    line
      .trim_start_matches('{')
      .trim_end_matches('}')
      .split(',')
      .map(|group| {
        let (category, value) = group.split_once('=').unwrap();
        (category, value.parse().unwrap())
      })
      .collect()
  });

  (workflows, parts)
}

pub fn part1(input: &str) -> u64 {
  let (workflows, parts) = parse_input(input);

  parts
    .filter(|part| {
      let mut workflow = "in";

      loop {
        for rule in workflows.get(workflow).unwrap() {
          let outcome = match rule {
            Rule::Condition(test, outcome) if test.call(part) => outcome,
            Rule::Fallback(outcome) => outcome,
            _ => continue,
          };

          match outcome {
            RuleOutcome::Final(result) => {
              return *result;
            }
            RuleOutcome::Workflow(next_workflow) => {
              workflow = next_workflow;
              break;
            }
          }
        }
      }
    })
    .map(|part| part.values().sum::<u64>())
    .sum()
}

pub fn part2(input: &str) -> u64 {
  let (workflows, parts) = parse_input(input);

  let mut categories = HashSet::new();
  for part in parts {
    for &key in part.keys() {
      categories.insert(key);
    }
  }

  type Ranges<'a> = HashMap<&'a str, Range<u64>>;

  let ranges = categories
    .into_iter()
    .map(|category| (category, 1..4001))
    .collect();

  fn process_outcome(
    ranges: Ranges,
    workflows: &Workflows,
    outcome: &RuleOutcome,
  ) -> u64 {
    match outcome {
      RuleOutcome::Final(true) => ranges
        .into_values()
        .map(|range| range.end - range.start)
        .product(),
      RuleOutcome::Final(false) => 0,
      RuleOutcome::Workflow(workflow_name) => accepted_combinations(
        ranges,
        workflows,
        workflows[workflow_name].iter(),
      ),
    }
  }

  fn accepted_combinations<'a>(
    ranges: Ranges<'a>,
    workflows: &Workflows,
    mut rule_iter: impl Iterator<Item = &'a Rule<'a>>,
  ) -> u64 {
    match rule_iter.next().unwrap() {
      Rule::Condition(test, outcome) => {
        let range = ranges[test.category].clone();

        let (range_match, range_miss) = match test.operator {
          RuleOperator::LessThan => {
            if range.end < test.value {
              (Some(range), None)
            } else if range.start >= test.value {
              (None, Some(range))
            } else {
              (Some(range.start..test.value), Some(test.value..range.end))
            }
          }
          RuleOperator::GreaterThan => {
            if range.start > test.value {
              (Some(range), None)
            } else if range.end <= test.value {
              (None, Some(range))
            } else {
              (
                Some(test.value + 1..range.end),
                Some(range.start..test.value + 1),
              )
            }
          }
        };

        let mut sum = 0;

        if let Some(range) = range_match {
          let mut ranges = ranges.clone();
          ranges.insert(test.category, range);
          sum += process_outcome(ranges, workflows, outcome)
        }

        if let Some(range) = range_miss {
          let mut ranges = ranges;
          ranges.insert(test.category, range);
          sum += accepted_combinations(ranges, workflows, rule_iter)
        }

        sum
      }

      Rule::Fallback(outcome) => process_outcome(ranges, workflows, outcome),
    }
  }

  accepted_combinations(ranges, &workflows, workflows["in"].iter())
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT_TEST: &str = include_str!("input_test.txt");
  const INPUT: &str = include_str!("input.txt");

  #[test]
  fn part1_works() {
    assert_eq!(part1(INPUT_TEST), 19_114);
    assert_eq!(part1(INPUT), 373_302);
  }

  #[test]
  fn part2_works() {
    assert_eq!(part2(INPUT_TEST), 167_409_079_868_000);
    assert_eq!(part2(INPUT), 130_262_715_574_114);
  }
}
