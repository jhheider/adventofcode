use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct Elf {
  food: Vec<usize>,
}

pub fn main() {
  let test = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

  let test_elves = parse(test);
  let test1 = top(&test_elves).unwrap();
  assert_eq!(test1, 24000);
  println!("Day 1: Part 1: test max is {test1}");

  let input = parse(&fs::read_to_string("data/day1.txt").unwrap());
  let part1 = top(&input).unwrap();
  assert_eq!(part1, 71506);
  println!("Day 1: Part 1: max is {part1}");

  let test2 = top_three(&test_elves);
  assert_eq!(test2, 45000);
  println!("Day 1: Part 2: test top 3 is {test2}");

  let part2 = top_three(&input);
  assert_eq!(part2, 209603);
  println!("Day 1: Part 2: top 3 is {part2}");
}

fn parse(input: &str) -> Vec<Elf> {
  input
    .split("\n\n")
    .map(|e| Elf {
      food: e.split('\n').map(|f| f.parse::<usize>().unwrap()).collect(),
    })
    .collect()
}

fn top(elves: &[Elf]) -> Option<usize> {
  elves.iter().map(|e| e.food.iter().sum::<usize>()).max()
}

fn top_three(elves: &[Elf]) -> usize {
  elves
    .iter()
    .map(|e| e.food.iter().sum::<usize>())
    .sorted()
    .rev()
    .take(3)
    .sum()
}
