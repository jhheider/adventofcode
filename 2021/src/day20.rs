use std::collections::HashSet;

use crate::data::Data;

struct Algorithm {
  algorithm: Vec<bool>,
}

struct Image {
  top: isize,
  left: isize,
  right: isize,
  bottom: isize,
  image: HashSet<(isize, isize)>,
}

impl Algorithm {
  fn new(algorithm: Vec<&str>) -> Self {
    let algorithm = algorithm
      .iter()
      .flat_map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
      .collect();
    Self { algorithm }
  }

  fn lookup(&self, index: usize) -> bool {
    self.algorithm[index]
  }
}

impl Image {
  fn new(input: &str) -> Self {
    let mut image = HashSet::new();
    let top = 0;
    let left = 0;
    let mut right = 0;
    let mut bottom = 0;
    for (y, line) in input.lines().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          image.insert((x as isize, y as isize));
          if x as isize > right {
            right = x as isize;
          }
          if y as isize > bottom {
            bottom = y as isize;
          }
        }
      }
    }
    Self {
      top,
      left,
      right,
      bottom,
      image,
    }
  }

  fn enhance(&mut self, algorithm: &Algorithm) {
    let mut new_image = HashSet::new();
    for y in self.top - 1..=self.bottom + 1 {
      for x in self.left - 1..=self.right + 1 {
        let mut index = 0;
        for (dx, dy) in &[
          (x - 1, y - 1),
          (x, y - 1),
          (x + 1, y - 1),
          (x - 1, y),
          (x, y),
          (x + 1, y),
          (x - 1, y + 1),
          (x, y + 1),
          (x + 1, y + 1),
        ] {
          index <<= 1;
          if self.image.contains(&(*dx, *dy)) {
            index += 1;
          }
        }
        if algorithm.lookup(index) {
          new_image.insert((x, y));
        }
      }
    }
    // Special case for "toggle" rules
    if algorithm.lookup(0) && !algorithm.lookup(511) {
      if !self.image.contains(&(self.left - 1, self.top - 1)) {
        for i in self.left - 3..=self.right + 3 {
          new_image.insert((i, self.top - 3));
          new_image.insert((i, self.top - 2));
          new_image.insert((i, self.bottom + 2));
          new_image.insert((i, self.bottom + 3));
          new_image.insert((self.left - 3, i));
          new_image.insert((self.left - 2, i));
          new_image.insert((self.right + 2, i));
          new_image.insert((self.right + 3, i));
        }
      } else {
        for i in self.left - 2..=self.right + 2 {
          new_image.remove(&(i, self.top - 2));
          new_image.remove(&(i, self.bottom + 2));
          new_image.remove(&(self.left - 2, i));
          new_image.remove(&(self.right + 2, i));
        }
      }
    }
    self.image = new_image;
    self.top -= 1;
    self.left -= 1;
    self.right += 1;
    self.bottom += 1;
  }

  fn lit(&self) -> usize {
    self.image.len()
  }

  fn run(&mut self, algorithm: &Algorithm, iterations: usize) {
    for _ in 0..iterations {
      self.enhance(algorithm);
    }
  }
}

pub fn main() {
  let data = Data::get(20);
  let mut test1 = parse(&data.test);
  test1.1.run(&test1.0, 2);
  assert_eq!(test1.1.lit(), 35);
  println!("Day 20: Test 1: {}", test1.1.lit());

  let mut input1 = parse(&data.input);
  input1.1.run(&input1.0, 2);
  println!("Day 20: Part 1: {}", input1.1.lit());

  let mut test2 = parse(&data.test);
  test2.1.run(&test2.0, 50);
  assert_eq!(test2.1.lit(), 3351);
  println!("Day 20: Test 2: {}", test2.1.lit());

  let mut input2 = parse(&data.input);
  input2.1.run(&input2.0, 50);
  println!("Day 20: Part 2: {}", input2.1.lit());
}

fn parse(input: &str) -> (Algorithm, Image) {
  let (algorithm, image) = input.split_once("\n\n").unwrap();
  (
    Algorithm::new(algorithm.lines().collect()),
    Image::new(image),
  )
}
