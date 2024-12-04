use num::abs;
use regex::Regex;
use std::{
  cmp::{max, min},
  collections::HashMap,
  hash::{Hash, Hasher},
};

use crate::data::Data;

// This is safe to ignore at this time.
#[derive(Debug, Eq, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

impl PartialEq for Point {
  fn eq(&self, rhs: &Point) -> bool {
    (self.x == rhs.x) && (self.y == rhs.y)
  }
}

impl Hash for Point {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.hash(state);
    self.y.hash(state);
  }
}

#[derive(Debug)]
struct Line {
  start: Point,
  end: Point,
}

pub fn main() {
  let data = Data::get(5);
  let test = lines(&data.test);
  let input = lines(&data.input);

  let test1 = overlaps(&test, false);
  assert_eq!(test1, 5);
  println!("Day 5: Test 1: {} points on 2 or more lines", test1);

  let part1 = overlaps(&input, false);
  println!("Day 5: Part 1: {} points on 2 or more lines", part1);

  let test2 = overlaps(&test, true);
  assert_eq!(test2, 12);
  println!("Day 5: Test 2: {} points on 2 or more lines", test2);

  let part2 = overlaps(&input, true);
  println!("Day 5: Part 2: {} points on 2 or more lines", part2);
}

fn lines(input: &str) -> Vec<Line> {
  let r = Regex::new(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$").unwrap();
  input
    .lines()
    .map(|line| {
      let caps = r.captures(line).unwrap();
      Line {
        start: Point {
          x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
          y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        },
        end: Point {
          x: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
          y: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        },
      }
    })
    .collect::<Vec<Line>>()
}

fn overlaps(lines: &[Line], with_diagonals: bool) -> usize {
  let mut points = HashMap::new();

  for line in lines.iter() {
    if line.start.x == line.end.x {
      for y in min(line.start.y, line.end.y)..=max(line.start.y, line.end.y) {
        let new_point = Point { x: line.start.x, y };
        points.insert(new_point, points.get(&new_point).unwrap_or(&0u32) + 1);
      }
    } else if line.start.y == line.end.y {
      for x in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
        let new_point = Point { x, y: line.start.y };
        points.insert(new_point, points.get(&new_point).unwrap_or(&0u32) + 1);
      }
    } else if with_diagonals {
      let steps = (line.end.x - line.start.x, line.end.y - line.start.y);
      for increment in 0..=abs(steps.0) {
        let new_point = Point {
          x: line.start.x
            + match steps.0 > 0 {
              true => increment,
              false => -increment,
            },
          y: line.start.y
            + match steps.1 > 0 {
              true => increment,
              false => -increment,
            },
        };
        points.insert(new_point, points.get(&new_point).unwrap_or(&0u32) + 1);
      }
    }
  }

  points.into_values().filter(|v| *v >= 2).count()
}
