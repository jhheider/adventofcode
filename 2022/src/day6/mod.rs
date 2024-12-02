use std::collections::HashSet;

use itertools::Itertools;

use crate::data::Data;

const TESTS: [(&str, usize, usize); 5] = [
  ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
  ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
  ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
  ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
  ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

pub fn main() {
  let data = Data::get(6);
  for (n, (input, answer1, answer2)) in TESTS.iter().enumerate() {
    let (part1, part2) = solve(input);
    assert_eq!(part1, *answer1);
    assert_eq!(part2, *answer2);
    println!("Day 6: Test {}, {} -> {}, {}", n + 1, input, part1, part2);
  }

  let (part1, part2) = solve(&data.input);
  println!("Day 6: Part 1: {}", part1);
  println!("Day 6: Part 2: {}", part2);
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
