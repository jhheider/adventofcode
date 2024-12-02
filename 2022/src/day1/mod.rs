use itertools::Itertools;

use crate::data::Data;

#[derive(Debug)]
struct Elf {
  food: Vec<usize>,
}

pub fn main() {
  let data = Data::get(1);
  let test_elves = parse(&data.test);
  let test1 = top(&test_elves).unwrap();
  assert_eq!(test1, 24000);
  println!("Day 1: Part 1: test max is {test1}");

  let input = parse(&data.input);
  let part1 = top(&input).unwrap();
  println!("Day 1: Part 1: max is {part1}");

  let test2 = top_three(&test_elves);
  assert_eq!(test2, 45000);
  println!("Day 1: Part 2: test top 3 is {test2}");

  let part2 = top_three(&input);
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
