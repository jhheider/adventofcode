use regex::Regex;

use crate::data::Data;

pub fn main() {
  let data = Data::get(3);
  let test1 = pt1(&data.test);
  assert_eq!(test1, 161);
  println!("Day 3, Test 1: {}", test1);

  let part1 = pt1(&data.input);
  println!("Day 3, Part 1: {}", part1);

  let test2 = pt2(&data.test);
  assert_eq!(test2, 48);
  println!("Day 3, Test 2: {}", test2);

  let part2 = pt2(&data.input);
  println!("Day 3, Part 2: {}", part2);
}

fn pt1(input: &str) -> isize {
  let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut total = 0;
  for cap in regex.captures_iter(input) {
    let a = cap[1].parse::<isize>().unwrap();
    let b = cap[2].parse::<isize>().unwrap();
    total += a * b;
  }
  total
}

fn pt2(input: &str) -> isize {
  let regex = Regex::new(r"(mul)\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)").unwrap();
  let mut total = 0;
  let mut active = true;
  for cap in regex.captures_iter(input) {
    if cap.get(1).is_some() && active {
      let a = cap[2].parse::<isize>().unwrap();
      let b = cap[3].parse::<isize>().unwrap();
      total += a * b;
    } else if cap.get(4).is_some() {
      active = true;
    } else if cap.get(5).is_some() {
      active = false;
    }
  }
  total
}
