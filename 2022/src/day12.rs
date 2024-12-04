use std::{
  collections::{HashMap, HashSet},
  fmt::{Display, Error, Formatter},
};

use crate::data::Data;

struct Map {
  heights: Vec<Vec<usize>>,
  start: (usize, usize),
  target: (usize, usize),
}

impl Map {
  fn new(input: &str) -> Self {
    let mut heights = Vec::new();
    let mut start = (0, 0);
    let mut target = (0, 0);
    for (y, row) in input.lines().enumerate() {
      let mut row_nodes = Vec::new();
      for (x, c) in row.chars().enumerate() {
        let height = match c {
          'a'..='z' => c as usize - 'a' as usize + 1,
          'S' => 1,
          'E' => 26,
          _ => panic!("Invalid character"),
        };
        if c == 'S' {
          start = (x, y);
        }
        if c == 'E' {
          target = (x, y);
        }
        row_nodes.push(height);
      }
      heights.push(row_nodes);
    }
    Self {
      heights,
      start,
      target,
    }
  }

  fn find_shortest_path(&self, reverse: bool) -> usize {
    let mut unvisited = HashSet::<(usize, usize)>::new();
    let mut distances = HashMap::<(usize, usize), usize>::new();
    let (start, target) = match reverse {
      true => (self.target, self.start),
      false => (self.start, self.target),
    };
    for y in 0..self.heights.len() {
      for x in 0..self.heights[0].len() {
        let distance = if (x, y) == start { 0 } else { usize::MAX };
        unvisited.insert((x, y));
        distances.insert((x, y), distance);
      }
    }
    while unvisited
      .iter()
      .any(|(x, y)| distances[&(*x, *y)] != usize::MAX)
    {
      let u = unvisited.clone();
      let (x, y) = u
        .iter()
        .map(|(x, y)| ((x, y), *distances.get(&(*x, *y)).unwrap()))
        .min_by_key(|((_, _), d)| *d)
        .unwrap()
        .0;
      unvisited.remove(&(*x, *y));
      let distance = distances.get(&(*x, *y)).copied().unwrap();
      if ((*x, *y) == target) || (reverse && self.heights[*y][*x] == 1) {
        return distance;
      }
      let neighbors: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
      for (dx, dy) in neighbors {
        let nx = (*x as isize + dx) as usize;
        let ny = (*y as isize + dy) as usize;
        if let Some(neighbor_distance) = distances.get(&(nx, ny)) {
          match reverse {
            false => {
              if (self.heights[ny][nx] > self.heights[*y][*x])
                && (self.heights[ny][nx] - self.heights[*y][*x] > 1)
              {
                continue;
              }
            }
            true => {
              if (self.heights[*y][*x] > self.heights[ny][nx])
                && (self.heights[*y][*x] - self.heights[ny][nx] > 1)
              {
                continue;
              }
            }
          }
          let from_distance = distance + 1;
          let new_distance = neighbor_distance.min(&from_distance);
          distances.insert((nx, ny), *new_distance);
        }
      }
    }
    self.print_distances(&distances);
    panic!("No path found");
  }

  fn print_distances(&self, distances: &HashMap<(usize, usize), usize>) {
    for y in 0..self.heights.len() {
      for x in 0..self.heights[y].len() {
        let distance = distances.get(&(x, y)).unwrap();
        if distance == &usize::MAX {
          print!("inf");
        } else {
          print!("{:3}", distance);
        }
      }
      println!();
    }
  }
}

impl Display for Map {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    for row in &self.heights {
      for node in row {
        write!(f, "{:3}", node)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

pub fn main() {
  let data = Data::get(12);
  let test = Map::new(&data.test);
  let test1 = test.find_shortest_path(false);
  assert_eq!(test1, 31);
  println!("Day 12: Test 1: {}", test1);

  let input = Map::new(&data.input);
  let part1 = input.find_shortest_path(false);
  println!("Day 12: Part 1: {}", part1);

  let test2 = test.find_shortest_path(true);
  assert_eq!(test2, 29);
  println!("Day 12: Test 2: {}", test2);

  let part2 = input.find_shortest_path(true);
  println!("Day 12: Part 2: {}", part2);
}
