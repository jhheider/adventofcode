use std::collections::HashSet;

use itertools::Itertools;

use crate::data::Data;

struct Team {
  first: HashSet<usize>,
  second: HashSet<usize>,
}

impl Team {
  fn one_is_super(&self) -> bool {
    self.first.is_superset(&self.second) || self.second.is_superset(&self.first)
  }

  fn overlap(&self) -> bool {
    self.first.intersection(&self.second).next().is_some()
  }
}

pub fn main() {
  let data = Data::get(4);
  let test = parse(&data.test);
  let input = parse(&data.input);

  let test1 = test.iter().filter(|t| t.one_is_super()).count();
  assert_eq!(test1, 2);
  println!("Day 4: Test 1: {test1} pairs are redundant");

  let part1 = input.iter().filter(|t| t.one_is_super()).count();
  println!("Day 4: Part 1: {part1} pairs are redundant");

  let test2 = test.iter().filter(|t| t.overlap()).count();
  assert_eq!(test2, 4);
  println!("Day 4: Test 2: {test2} pairs overlap");

  let part2 = input.iter().filter(|t| t.overlap()).count();
  println!("Day 4: Part 2: {part2} pairs overlap");
}

fn parse(input: &str) -> Vec<Team> {
  input
    .lines()
    .map(|l| {
      let mut split = l.split(',');
      let first = split.next().unwrap();
      let second = split.next().unwrap();
      Team {
        first: rangify(first),
        second: rangify(second),
      }
    })
    .collect_vec()
}

fn rangify(range: &str) -> HashSet<usize> {
  let mut split = range.split('-');
  let first = split.next().unwrap().parse().unwrap();
  let second = split.next().unwrap().parse().unwrap();
  HashSet::from_iter(first..=second)
}
