use itertools::Itertools;

use crate::data::Data;

#[derive(Debug, Clone)]
struct Input {
  stacks: Vec<Vec<char>>,
  moves: Vec<Move>,
}

#[derive(Debug, Clone)]
struct Move {
  count: usize,
  from: usize,
  to: usize,
}

impl Input {
  fn apply(&self, move_: &Move) -> Self {
    let mut stacks = self.stacks.clone();
    let mut from = stacks[move_.from - 1].clone();
    let mut to = stacks[move_.to - 1].clone();
    let mut count = move_.count;
    while count > 0 {
      to.push(from.pop().unwrap());
      count -= 1;
    }
    stacks[move_.from - 1] = from;
    stacks[move_.to - 1] = to;
    Input {
      stacks,
      moves: self.moves.clone(),
    }
  }

  fn apply_reversed(&self, move_: &Move) -> Self {
    let mut stacks = self.stacks.clone();
    let mut from = stacks[move_.from - 1].clone();
    let mut to = stacks[move_.to - 1].clone();
    let mut count = move_.count;
    let mut temp = Vec::new();
    while count > 0 {
      temp.push(from.pop().unwrap());
      count -= 1;
    }
    to.extend(temp.iter().rev().cloned());
    stacks[move_.from - 1] = from;
    stacks[move_.to - 1] = to;
    Input {
      stacks,
      moves: self.moves.clone(),
    }
  }

  fn apply_all(&self) -> Self {
    let mut input = self.clone();
    for m in &self.moves {
      input = input.apply(m);
    }
    input
  }

  fn apply_all_reversed(&self) -> Self {
    let mut input = self.clone();
    for m in self.moves.iter() {
      input = input.apply_reversed(m);
    }
    input
  }

  fn get_tops(&self) -> Vec<char> {
    self.stacks.iter().map(|s| *s.last().unwrap()).collect_vec()
  }
}

pub fn main() {
  let data = Data::get(5);
  let test = parse(&data.test);
  let input = parse(&data.input);

  let test1 = test.apply_all().get_tops().iter().join("");
  assert_eq!(test1, "CMZ");
  println!("Day 5: Test 1: {}", test1);

  let part1 = input.apply_all().get_tops().iter().join("");
  println!("Day 5: Part 1: {}", part1);

  let test2 = test.apply_all_reversed().get_tops().iter().join("");
  assert_eq!(test2, "MCD");
  println!("Day 5: Test 2: {}", test2);

  let part2 = input.apply_all_reversed().get_tops().iter().join("");
  println!("Day 5: Part 2: {}", part2);
}

fn parse(input: &str) -> Input {
  let split = input.split("\n\n").collect_vec();
  let stack_lines = split[0];
  let move_lines = split[1];
  Input {
    stacks: parse_stacks(stack_lines),
    moves: parse_moves(move_lines),
  }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
  let mut lines = input.lines().rev();
  let num_stacks = (lines.next().unwrap().len() + 1) / 4;
  let mut stacks = (0..num_stacks).map(|_| Vec::new()).collect_vec();
  for line in lines {
    for (i, c) in line.chars().enumerate() {
      if c == '[' {
        stacks[i / 4].push(line.chars().nth(i + 1).unwrap());
      }
    }
  }
  stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
  input
    .lines()
    .map(|line| {
      let mut words = line.split_whitespace();
      Move {
        count: words.nth(1).unwrap().parse().unwrap(),
        from: words.nth(1).unwrap().parse().unwrap(),
        to: words.nth(1).unwrap().parse().unwrap(),
      }
    })
    .collect_vec()
}
