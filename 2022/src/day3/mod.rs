use std::{collections::HashSet, fs};

use itertools::Itertools;

const TEST: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

struct Bag {
  first: Vec<char>,
  second: Vec<char>,
}

impl Bag {
  fn in_both(&self) -> char {
    let first = HashSet::<char>::from_iter(self.first.clone());
    let second = HashSet::from_iter(self.second.clone());
    **first.intersection(&second).collect_vec().first().unwrap()
  }

  fn all(&self) -> Vec<char> {
    let mut all = self.first.clone();
    all.extend(self.second.clone().iter());
    all
  }

  fn in_three(&self, second_bag: &Bag, third_bag: &Bag) -> char {
    let first = HashSet::<char>::from_iter(self.all());
    let second = HashSet::<char>::from_iter(second_bag.all());
    let third = HashSet::<char>::from_iter(third_bag.all());
    **first
      .intersection(&second)
      .copied()
      .collect::<HashSet<char>>()
      .intersection(&third)
      .collect_vec()
      .first()
      .unwrap()
  }
}

pub fn main() {
  let test = parse(TEST);
  let input = parse(&fs::read_to_string("data/day3.txt").unwrap());

  let test1 = test.iter().map(|b| char_value(b.in_both())).sum::<usize>();
  assert_eq!(test1, 157);
  println!("Day 3: Test 1: {test1}");

  let part1 = input.iter().map(|b| char_value(b.in_both())).sum::<usize>();
  assert_eq!(part1, 7997);
  println!("Day 3: Part 1: {part1}");

  let test2 = test
    .chunks(3)
    .map(|c| {
      let mut i = c.iter();
      let a = i.next().unwrap();
      let b = i.next().unwrap();
      let c = i.next().unwrap();
      char_value(a.in_three(b, c))
    })
    .sum::<usize>();
  assert_eq!(test2, 70);
  println!("Day 3: Test 2: {test2}");

  let part2 = input
    .chunks(3)
    .map(|c| {
      let mut i = c.iter();
      let a = i.next().unwrap();
      let b = i.next().unwrap();
      let c = i.next().unwrap();
      char_value(a.in_three(b, c))
    })
    .sum::<usize>();
  assert_eq!(part2, 2545);
  println!("Day 3: Part 2: {part2}");
}

fn parse(unwrap: &str) -> Vec<Bag> {
  unwrap
    .lines()
    .map(|l| {
      let contents = l.chars().collect_vec();
      let (first, second) = contents.split_at(contents.len() / 2);
      Bag {
        first: first.to_vec(),
        second: second.to_vec(),
      }
    })
    .collect_vec()
}

const LOWER_OFFSET: usize = 'a' as usize;
const UPPER_OFFSET: usize = 'A' as usize;

fn char_value(c: char) -> usize {
  let ascii = c as usize;
  match c.is_ascii_lowercase() {
    true => ascii - LOWER_OFFSET + 1,
    false => ascii - UPPER_OFFSET + 27,
  }
}
