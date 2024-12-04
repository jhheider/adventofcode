use num::integer::lcm;
use std::collections::HashMap;
use std::fs;

const TEST1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
Zzz = (ZZZ, ZZZ)";

const TEST2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
Zzz = (ZZZ, ZZZ)";

const TEST3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[derive(Debug)]
enum Direction {
  Left,
  Right,
}

#[derive(Debug)]
struct Map {
  directions: Vec<Direction>,
  map: HashMap<String, (String, String)>,
}

#[derive(Debug)]
enum Ending {
  Zzz,
  Z,
}

impl Ending {
  fn test(&self, s: &str) -> bool {
    match self {
      Self::Zzz => s == "ZZZ",
      Self::Z => s.ends_with('Z'),
    }
  }
}

impl Map {
  fn new(input: &str) -> Self {
    let mut map = HashMap::new();
    let mut lines = input.lines();

    let directions = lines
      .next()
      .unwrap()
      .chars()
      .map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
      })
      .collect::<Vec<_>>();

    lines.next(); // discard blank line

    for line in lines {
      let mut parts = line.split(" = ");
      let key = parts.next().unwrap().to_string();
      let value = parts.next().unwrap();
      let mut parts = value.split(", ");
      let left = parts.next().unwrap().trim_matches('(').to_string();
      let right = parts.next().unwrap().trim_matches(')').to_string();
      map.insert(key, (left, right));
    }
    Self { directions, map }
  }

  fn run(&self, start: String, ending: Ending) -> usize {
    let mut steps = 0;
    let mut current = start;
    loop {
      for direction in &self.directions {
        let (left, right) = self.map.get(&current).unwrap();
        match direction {
          Direction::Left => current = left.to_string(),
          Direction::Right => current = right.to_string(),
        }
        steps += 1;
        if ending.test(&current) {
          return steps;
        }
      }
    }
  }

  fn single_run(&self) -> usize {
    self.run("AAA".to_string(), Ending::Zzz)
  }

  fn ghost_run(&self) -> usize {
    let steps = self
      .map
      .keys()
      .filter(|k| k.ends_with('A'))
      .map(|k| self.run(k.to_string(), Ending::Z))
      .collect::<Vec<_>>();
    steps.into_iter().fold(1, lcm)
  }
}

pub fn main() {
  let test1 = Map::new(TEST1).single_run();

  println!("Day 8, Test 1: {}", test1);

  let test2 = Map::new(TEST2).single_run();

  println!("Day 8, Test 2: {}", test2);

  let input = fs::read_to_string("data/day8.txt").unwrap();

  let part1 = Map::new(&input).single_run();

  println!("Day 8, Part 1: {}", part1);

  let test3 = Map::new(TEST3).ghost_run();

  println!("Day 8, Test 3: {}", test3);

  let part2 = Map::new(&input).ghost_run();

  println!("Day 8, Part 2: {}", part2);
}
