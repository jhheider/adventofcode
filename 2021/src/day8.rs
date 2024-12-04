use crate::data::Data;
use std::collections::{HashMap, HashSet};

pub fn main() {
  let data = Data::get(8);
  let test = parse_input(&data.test);
  let input = parse_input(&data.input);

  let test1 = count_1478(&test);
  assert_eq!(test1, 26);
  println!(
    "Day 8: Test 1: digits 1, 4, 7, and 8 appear {} times in the output",
    test1
  );

  let part1 = count_1478(&input);
  println!(
    "Day 8: Part 1: digits 1, 4, 7, and 8 appear {} times in the output",
    part1
  );

  let test2 = figure_outputs(&test);
  assert_eq!(test2, 61229);
  println!("Day 8: Test 2: Sum of all outputs is {}", test2);

  let part2 = figure_outputs(&input);
  println!("Day 8: Part 2: Sum of all outputs is {}", part2);
}

fn parse_input(input: &str) -> Vec<Vec<Vec<HashSet<char>>>> {
  input
    .lines()
    .map(|line| {
      line
        .split(" | ")
        .map(|split| {
          split
            .split_whitespace()
            .map(|display| display.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>()
        })
        .collect::<Vec<Vec<HashSet<char>>>>()
    })
    .collect::<Vec<Vec<Vec<HashSet<char>>>>>()
}

fn count_1478(input: &[Vec<Vec<HashSet<char>>>]) -> i32 {
  input
    .iter()
    .map(|line| {
      line[1].iter().fold(0, |sum, value| {
        if [2, 3, 4, 7].contains(&value.len()) {
          return sum + 1;
        }
        sum
      })
    })
    .sum()
}

fn figure_outputs(input: &[Vec<Vec<HashSet<char>>>]) -> i32 {
  input
    .iter()
    .map(|row| {
      let mut solutions = HashMap::<i32, &HashSet<char>>::new();
      solutions.insert(1, row[0].iter().find(|v| v.len() == 2).unwrap());
      solutions.insert(4, row[0].iter().find(|v| v.len() == 4).unwrap());
      solutions.insert(7, row[0].iter().find(|v| v.len() == 3).unwrap());
      solutions.insert(8, row[0].iter().find(|v| v.len() == 7).unwrap());

      let fourdiff = solutions
        .get(&4)
        .unwrap()
        .difference(solutions.get(&1).unwrap())
        .copied()
        .collect::<HashSet<char>>();

      for i in &row[0] {
        match i.len() {
          2 | 3 | 4 | 7 => continue,
          5 => {
            if solutions.get(&1).unwrap().difference(i).count() == 0 {
              solutions.insert(3, i);
            } else if fourdiff.difference(i).count() == 0 {
              solutions.insert(5, i);
            } else {
              solutions.insert(2, i);
            }
          }
          6 => {
            if solutions.get(&4).unwrap().difference(i).count() == 0 {
              solutions.insert(9, i);
            } else if fourdiff.difference(i).count() == 0 {
              solutions.insert(6, i);
            } else {
              solutions.insert(0, i);
            }
          }
          _ => panic!("Bad input! {:?}", i),
        }
      }

      row[1].iter().fold(0, |sum, value| {
        10 * sum
          + solutions
            .iter()
            .find(|solution| {
              (solution.1.difference(value).count() == 0)
                && (value.difference(solution.1).count() == 0)
            })
            .unwrap()
            .0
      })
    })
    .sum()
}
/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

 0 => 6
 1 => 2
 2 => 5
 3 => 5
 4 => 4
 5 => 5
 6 => 6
 7 => 3
 8 => 7
 9 => 6

 2 => 1
 3 => 7
 4 => 4
 5 => 2, 3, 5
 6 => 0, 6, 9
 7 => 8

 a => 8 || 2,
 b => 6 || 4,
 c => 8 || 2,
 d => 7 || 3,
 e => 4 || 6,
 f => 9 || 1,
 g => 6 || 4,
 */
