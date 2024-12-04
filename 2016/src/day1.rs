use std::collections::HashSet;

use crate::data::Data;

#[derive(Debug, Clone)]
enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Debug)]
enum Turn {
  Left,
  Right,
}

#[derive(Debug, Clone)]
struct Position {
  x: i32,
  y: i32,
  dir: Direction,
}

impl Direction {
  fn turn(&self, turn: Turn) -> Direction {
    match (self, turn) {
      (Direction::North, Turn::Left) => Direction::West,
      (Direction::North, Turn::Right) => Direction::East,
      (Direction::East, Turn::Left) => Direction::North,
      (Direction::East, Turn::Right) => Direction::South,
      (Direction::South, Turn::Left) => Direction::East,
      (Direction::South, Turn::Right) => Direction::West,
      (Direction::West, Turn::Left) => Direction::South,
      (Direction::West, Turn::Right) => Direction::North,
    }
  }
}

impl Position {
  fn turn(&mut self, turn: Turn) -> &mut Position {
    self.dir = self.dir.turn(turn);
    self
  }

  fn walk(&mut self, dist: i32) -> &mut Position {
    match self.dir {
      Direction::North => self.y += dist,
      Direction::East => self.x += dist,
      Direction::South => self.y -= dist,
      Direction::West => self.x -= dist,
    }
    self
  }
}

impl From<&str> for Turn {
  fn from(s: &str) -> Turn {
    match s {
      "L" => Turn::Left,
      "R" => Turn::Right,
      _ => panic!("Invalid turn: {}", s),
    }
  }
}

pub fn main() {
  let input = Data::get(1).input;

  let final_pos = input.split(", ").fold(
    Position {
      x: 0,
      y: 0,
      dir: Direction::North,
    },
    |p, i| {
      let (dir, dist) = i.split_at(1);
      let dist = dist.parse::<i32>().unwrap();
      p.clone().turn(dir.into()).walk(dist).clone()
    },
  );

  println!("Day 1 Part 1: {}", final_pos.x.abs() + final_pos.y.abs());

  let p2 = part2(input);
  println!("Day 1 Part 2: {}", p2.x.abs() + p2.y.abs());
}

fn part2(input: String) -> Position {
  let mut pos = Position {
    x: 0,
    y: 0,
    dir: Direction::North,
  };
  let mut visited: HashSet<(i32, i32)> = HashSet::from_iter(vec![(0, 0)]);
  for i in input.split(", ") {
    let (dir, dist) = i.split_at(1);
    let dist = dist.parse::<i32>().unwrap();
    pos.turn(dir.into());
    for _ in 0..dist {
      pos.walk(1);
      if visited.contains(&(pos.x, pos.y)) {
        return pos;
      }
      visited.insert((pos.x, pos.y));
    }
  }
  pos
}
