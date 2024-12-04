use std::{
  collections::HashSet,
  fmt::{Display, Formatter, Result},
};

use crate::data::Data;

struct Cave {
  floor: usize,
  state: HashSet<Point>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
  x: usize,
  y: usize,
}

impl Cave {
  fn new(input: &str) -> Self {
    let mut state = HashSet::new();
    for line in input.lines() {
      let points = line.split(" -> ").map(Point::new).collect::<Vec<_>>();
      for p in 1..points.len() {
        let p1 = points[p - 1];
        let p2 = points[p];
        let mut p1 = p1;
        while p1 != p2 {
          state.insert(p1);
          if p1.x < p2.x {
            p1.x += 1;
          } else if p1.x > p2.x {
            p1.x -= 1;
          } else if p1.y < p2.y {
            p1.y += 1;
          } else if p1.y > p2.y {
            p1.y -= 1;
          }
        }
        state.insert(p2);
      }
    }
    let floor = state.iter().max_by_key(|p| p.y).unwrap().y;
    Cave { floor, state }
  }

  fn add_sand(&mut self, with_floor: bool) -> bool {
    if self.state.contains(&Point { x: 500, y: 0 }) {
      return false;
    }
    let mut sand = Point { x: 500, y: 0 };
    while sand.y <= self.floor {
      if !self.state.contains(&Point {
        x: sand.x,
        y: sand.y + 1,
      }) {
        sand.y += 1;
      } else if !self.state.contains(&Point {
        x: sand.x - 1,
        y: sand.y + 1,
      }) {
        sand.x -= 1;
        sand.y += 1;
      } else if !self.state.contains(&Point {
        x: sand.x + 1,
        y: sand.y + 1,
      }) {
        sand.x += 1;
        sand.y += 1;
      } else {
        break;
      }
    }
    if !with_floor && sand.y > self.floor {
      false
    } else {
      self.state.insert(sand);
      true
    }
  }

  fn fill(&mut self, with_floor: bool) -> usize {
    let mut sand = 0;
    while self.add_sand(with_floor) {
      sand += 1;
    }
    sand
  }
}

impl Display for Cave {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let min = self.state.iter().min_by_key(|p| p.x).unwrap().x;
    let max = self.state.iter().max_by_key(|p| p.x).unwrap().x;
    for y in 0..=self.floor + 1 {
      for x in min - 1..=max + 1 {
        if self.state.contains(&Point { x, y }) {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

impl Point {
  fn new(input: &str) -> Self {
    let mut parts = input.split(',');
    Point {
      x: parts.next().unwrap().parse().unwrap(),
      y: parts.next().unwrap().parse().unwrap(),
    }
  }
}

pub fn main() {
  let data = Data::get(14);
  let mut test = Cave::new(&data.test);
  let test1 = test.fill(false);
  assert_eq!(test1, 24);
  println!("Day 14: Test 1: {}", test1);

  let mut input = Cave::new(&data.input);
  let part1 = input.fill(false);
  println!("Day 14: Part 1: {}", part1);

  let mut test = Cave::new(&data.test);
  let test2 = test.fill(true);
  assert_eq!(test2, 93);
  println!("Day 14: Test 2: {}", test2);

  let mut input = Cave::new(&data.input);
  let part2 = input.fill(true);
  println!("Day 14: Part 2: {}", part2);
}
