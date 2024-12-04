use std::{
  cmp::{max, min},
  collections::HashSet,
};

use crate::data::Data;
use regex::Regex;

pub fn main() {
  let data = Data::get(13);

  let test1 = fold(&data.test, 1);
  assert_eq!(test1.len(), 17);
  println!("Day 13: Test 1: {} dots after first fold", test1.len());

  let part1 = fold(&data.input, 1);
  println!("Day 13: Test 1: {} dots after first fold", part1.len());

  let test2 = fold(&data.test, usize::MAX);
  assert_eq!(test2.len(), 16);
  println!("Day 13: Test 2:");
  println!("{}", format_grid(test2));

  let part2 = fold(&data.input, usize::MAX);
  println!("Day 13: Part 2:");
  println!("{}", format_grid(part2));
}

fn fold(input: &str, steps: usize) -> HashSet<(usize, usize)> {
  let r1 = Regex::new(r"^(.*),(.*)$").unwrap();
  let points = input
    .lines()
    .map_while(|line| {
      let caps = r1.captures(line)?;
      Some((
        caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
      ))
    })
    .collect::<HashSet<(usize, usize)>>();

  let r2 = Regex::new(r"^fold along ([yx])=(.*)$").unwrap();
  let folds = input
    .lines()
    .filter_map(|line| {
      let caps = r2.captures(line)?;
      Some((
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
      ))
    })
    .collect::<Vec<(&str, usize)>>();

  folds
    .iter()
    .take(min(steps, folds.len()))
    .fold(points, |output, fold| {
      output
        .into_iter()
        .map(|(x, y)| match fold.0 {
          "y" if y > fold.1 => (x, 2 * fold.1 - y),
          "x" if x > fold.1 => (2 * fold.1 - x, y),
          _ => (x, y),
        })
        .collect()
    })
}

fn format_grid(grid: HashSet<(usize, usize)>) -> String {
  let size = grid.iter().fold((0, 0), |acc, v| {
    let x = max(acc.0, v.0);
    let y = max(acc.1, v.1);
    (x, y)
  });
  (0..=size.1).fold("".to_string(), |output, y| {
    output
      + &(0..=size.0)
        .map(|x| match grid.contains(&(x, y)) {
          true => '#',
          false => ' ',
        })
        .collect::<String>()
      + "\n"
  })
}
