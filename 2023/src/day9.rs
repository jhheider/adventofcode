use std::fs;

const TEST: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

pub fn main() {
  let test1 = TEST.lines().map(parse).map(find_next).sum::<isize>();

  println!("Day 9 Part 1: {}", test1);

  let input = fs::read_to_string("data/day9.txt").unwrap();
  let part1 = input.lines().map(parse).map(find_next).sum::<isize>();

  println!("Day 9 Part 1: {}", part1);

  let test2 = TEST.lines().map(parse).map(find_prior).sum::<isize>();

  println!("Day 9 Part 2: {}", test2);

  let part2 = input.lines().map(parse).map(find_prior).sum::<isize>();

  println!("Day 9 Part 2: {}", part2);
}

fn parse(input: &str) -> Vec<isize> {
  input
    .split_whitespace()
    .map(|n| n.parse::<isize>().unwrap())
    .collect::<Vec<_>>()
}

fn find_next(input: Vec<isize>) -> isize {
  let di = derive(&input);
  di.last().cloned().unwrap()
}

fn find_prior(input: Vec<isize>) -> isize {
  let i = input.into_iter().rev().collect::<Vec<_>>();
  let di = derive(&i);
  di.last().cloned().unwrap()
}

fn derive(input: &[isize]) -> Vec<isize> {
  let mut next = vec![];
  let mut out = input.to_vec();

  for i in 0..input.len() - 1 {
    let a = input[i];
    let b = input[i + 1];
    next.push(b - a);
  }
  if next.iter().all(|n| *n == 0) {
    out.push(*input.last().unwrap());
    return out;
  }
  next = derive(&next);

  out.push(input.last().unwrap() + next.last().unwrap());
  out
}
