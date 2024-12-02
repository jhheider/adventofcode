use std::collections::HashSet;

use crate::data::Data;

const TEST2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

struct State {
  knots: Vec<(isize, isize)>,
  visited: HashSet<(isize, isize)>,
}

enum Move {
  Left(isize),
  Right(isize),
  Up(isize),
  Down(isize),
}

impl State {
  fn new(length: usize) -> Self {
    Self {
      knots: vec![(0, 0); length],
      visited: HashSet::new(),
    }
  }

  fn r#move(&mut self, m: Move) {
    let (dx, dy, steps) = match m {
      Move::Left(i) => (-1, 0, i),
      Move::Right(i) => (1, 0, i),
      Move::Up(i) => (0, 1, i),
      Move::Down(i) => (0, -1, i),
    };
    for _ in 0..steps {
      for i in 0..self.knots.len() - 1 {
        if i == 0 {
          self.knots[i] = (self.knots[i].0 + dx, self.knots[i].1 + dy);
        }
        match (
          self.knots[i].0 - self.knots[i + 1].0,
          self.knots[i].1 - self.knots[i + 1].1,
        ) {
          (dx, 0) if dx.abs() > 1 => {
            self.knots[i + 1] = (self.knots[i + 1].0 + (dx / dx.abs()), self.knots[i + 1].1)
          }
          (0, dy) if dy.abs() > 1 => {
            self.knots[i + 1] = (self.knots[i + 1].0, self.knots[i + 1].1 + (dy / dy.abs()))
          }
          (dx, dy) if dx.abs() > 1 || dy.abs() > 1 => {
            self.knots[i + 1] = (
              self.knots[i + 1].0 + (dx / dx.abs()),
              self.knots[i + 1].1 + (dy / dy.abs()),
            )
          }
          (_, _) => (),
        }
      }
      self.visited.insert(self.knots[self.knots.len() - 1]);
    }
  }

  fn run(&mut self, input: &str) {
    for line in input.lines() {
      let (dir, steps) = line.split_at(1);
      let steps = steps.trim_start().parse::<isize>().unwrap();
      match dir {
        "L" => self.r#move(Move::Left(steps)),
        "R" => self.r#move(Move::Right(steps)),
        "U" => self.r#move(Move::Up(steps)),
        "D" => self.r#move(Move::Down(steps)),
        _ => panic!("Invalid direction"),
      }
    }
  }
}

pub fn main() {
  let data = Data::get(9);
  let mut test1 = State::new(2);
  test1.run(&data.test);
  assert_eq!(test1.visited.len(), 13);
  println!("Day 9: Test 1: {}", test1.visited.len());

  let mut part1 = State::new(2);
  part1.run(&data.input);
  println!("Day 9: Part 1: {}", part1.visited.len());

  let mut test2 = State::new(10);
  test2.run(TEST2);
  assert_eq!(test2.visited.len(), 36);
  println!("Day 9: Test 2: {}", test2.visited.len());

  let mut part2 = State::new(10);
  part2.run(&data.input);
  println!("Day 9: Part 2: {}", part2.visited.len());
}
