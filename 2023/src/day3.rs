use std::collections::HashSet;

use crate::data::Data;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Direction {
  NW,
  N,
  NE,
  E,
  SE,
  S,
  SW,
  W,
}

impl Direction {
  fn point(&self, x: usize, y: usize) -> (usize, usize) {
    match self {
      Direction::NW => (x - 1, y - 1),
      Direction::N => (x, y - 1),
      Direction::NE => (x + 1, y - 1),
      Direction::E => (x + 1, y),
      Direction::SE => (x + 1, y + 1),
      Direction::S => (x, y + 1),
      Direction::SW => (x - 1, y + 1),
      Direction::W => (x - 1, y),
    }
  }
}

pub fn main() {
  let data = Data::get(3);
  let test = parse(&data.test);
  let test1 = part1(&test);
  println!("Day 3, Test 1: {}", test1);

  let input = parse(&data.input);
  let part1 = part1(&input);
  println!("Day 3, Part 1: {}", part1);

  let test2 = part2(&test);
  println!("Day 3, Test 2: {}", test2);

  let part2 = part2(&input);
  println!("Day 3, Part 2: {}", part2);
}

fn parse(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &[Vec<char>]) -> usize {
  let mut sum = 0;
  for (y, line) in input.iter().enumerate() {
    let mut x = 0;
    while x < line.len() {
      if line[x].is_numeric() {
        let mut number = line[x].to_digit(10).unwrap();
        let mut len = 1;
        for i in line.clone().iter().skip(x + 1) {
          if i.is_numeric() {
            number = number * 10 + i.to_digit(10).unwrap();
            len += 1;
          } else {
            break;
          }
        }
        for i in (x as i32) - 1..=(x as i32) + len {
          for j in (y as i32) - 1..=(y as i32) + 1 {
            if i < 0
              || j < 0
              || i >= line.len().try_into().unwrap()
              || j >= input.len().try_into().unwrap()
            {
              continue;
            }
            let v = input[j as usize][i as usize];
            if v != '.' && !v.is_numeric() {
              sum += number as usize;
              break;
            }
          }
        }
        x += len as usize;
        continue;
      }
      x += 1;
    }
  }
  sum
}

fn part2(input: &[Vec<char>]) -> usize {
  let mut sum = 0;
  for (y, line) in input.iter().enumerate() {
    for (x, c) in line.iter().enumerate() {
      if *c == '*' {
        let s = sum_neighbors(input, x, y);
        sum += s;
      }
    }
  }
  sum
}

fn neighbors(x: usize, y: usize, input: &[Vec<char>]) -> HashSet<Direction> {
  let mut d = HashSet::new();
  if x > 0 && input[y][x - 1].is_numeric() {
    d.insert(Direction::W);
  }
  if x < input[y].len() - 1 && input[y][x + 1].is_numeric() {
    d.insert(Direction::E);
  }
  if y > 0 {
    if input[y - 1][x].is_numeric() {
      d.insert(Direction::N);
    } else {
      if x > 0 && input[y - 1][x - 1].is_numeric() {
        d.insert(Direction::NW);
      }
      if x < input[y].len() - 1 && input[y - 1][x + 1].is_numeric() {
        d.insert(Direction::NE);
      }
    }
  }
  if y < input.len() - 1 {
    if input[y + 1][x].is_numeric() {
      d.insert(Direction::S);
    } else {
      if x > 0 && input[y + 1][x - 1].is_numeric() {
        d.insert(Direction::SW);
      }
      if x < input[y].len() - 1 && input[y + 1][x + 1].is_numeric() {
        d.insert(Direction::SE);
      }
    }
  }
  d
}

fn sum_neighbors(input: &[Vec<char>], x: usize, y: usize) -> usize {
  let neighbors = neighbors(x, y, input);
  if neighbors.len() != 2 {
    return 0;
  }
  neighbors
    .into_iter()
    .map(|d| number_from_point(input, d.point(x, y)))
    .product()
}

fn number_from_point(input: &[Vec<char>], (x, y): (usize, usize)) -> usize {
  let mut start = x;
  for x0 in (0..=x).rev() {
    if x0 == 0 || !input[y][x0 - 1].is_numeric() {
      start = x0;
      break;
    }
  }
  let mut v = 0;
  for x0 in start..input[y].len() {
    if !input[y][x0].is_numeric() {
      break;
    }
    v = v * 10 + input[y][x0].to_digit(10).unwrap() as usize;
  }
  v
}
