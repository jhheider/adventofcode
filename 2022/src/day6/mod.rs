use std::{collections::HashSet, fs};

use itertools::Itertools;

const TESTS: [(&str, usize, usize); 5] = [
  ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
  ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
  ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
  ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
  ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

pub fn main() {
  for (input, answer1, answer2) in TESTS.iter() {
    let (part1, part2) = solve(input);
    assert_eq!(part1, *answer1);
    assert_eq!(part2, *answer2);
    println!("Day 6, test, {} -> {}, {}", input, part1, part2);
  }

  let input = &fs::read_to_string("data/day6.txt").unwrap();
  let (part1, part2) = solve(input);
  assert_eq!(part1, 1892);
  println!("Day 6, part 1: {}", part1);
  assert_eq!(part2, 0);
  println!("Day 6, part 2: {}", part2);
}

fn solve(input: &str) -> (usize, usize) {
  let mut part1 = 0;
  let chars = input.chars().collect_vec();
  for p in 3..chars.len() {
    if chars[p - 3..=p]
      .iter()
      .copied()
      .collect::<HashSet<char>>()
      .len()
      == 4
    {
      part1 = p + 1;
      break;
    }
  }
  for p in 13..chars.len() {
    if chars[p - 13..=p]
      .iter()
      .copied()
      .collect::<HashSet<char>>()
      .len()
      == 14
    {
      return (part1, p + 1);
    }
  }
  (part1, 0)
}
