use crate::data::Data;

#[derive(Debug)]
struct Barrel {
  monkeys: Vec<Monkey>,
  modulus: isize,
}

#[derive(Debug, Clone)]
struct Monkey {
  items: Vec<isize>,
  operation: Operation,
  test: isize,
  if_true: usize,
  if_false: usize,
  inspections: usize,
}

#[derive(Debug, Clone)]
enum Operation {
  Add(isize),
  Multiply(isize),
  Square,
}

impl Barrel {
  fn new(input: &str) -> Self {
    let monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();
    let modulus = monkeys.iter().map(|m| m.test).product();
    Self { monkeys, modulus }
  }

  fn run(&mut self, rounds: usize, part2: bool) {
    for _ in 0..rounds {
      for i in 0..self.monkeys.len() {
        let monkey = self.monkeys[i].clone();
        for item in &monkey.items {
          let new_item = match monkey.operation {
            Operation::Add(n) => item + n,
            Operation::Multiply(n) => item * n,
            Operation::Square => item * item,
          };
          let new_item = if part2 {
            new_item % self.modulus
          } else {
            new_item / 3
          };
          if new_item % monkey.test == 0 {
            self.monkeys[monkey.if_true].items.push(new_item);
          } else {
            self.monkeys[monkey.if_false].items.push(new_item);
          }
        }
        self.monkeys[i].items.clear();
        self.monkeys[i].inspections += monkey.items.len();
      }
    }
  }

  fn monkey_business(&self) -> usize {
    let mut monkeys = self.monkeys.clone();
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0..=1].iter().map(|m| m.inspections).product()
  }
}

impl Monkey {
  fn new(input: &str) -> Self {
    let mut lines = input.lines().skip(1);
    let items = lines
      .next()
      .unwrap()
      .split(": ")
      .nth(1)
      .unwrap()
      .split(", ")
      .map(|s| s.parse().unwrap())
      .collect();
    let operation = lines.next().unwrap().split(": ").nth(1).unwrap();
    let operation = if operation.starts_with("new = old * old") {
      Operation::Square
    } else if operation.starts_with("new = old") {
      let operation = operation.split("old ").nth(1).unwrap();
      if operation.starts_with("* ") {
        Operation::Multiply(operation.split(' ').nth(1).unwrap().parse().unwrap())
      } else if operation.starts_with("+ ") {
        Operation::Add(operation.split(' ').nth(1).unwrap().parse().unwrap())
      } else {
        panic!("Unknown operation: {}", operation);
      }
    } else {
      panic!("Unknown operation: {}", operation);
    };
    let test = lines
      .next()
      .unwrap()
      .split(": ")
      .nth(1)
      .unwrap()
      .split(' ')
      .nth(2)
      .unwrap()
      .parse()
      .unwrap();
    let if_true = lines
      .next()
      .unwrap()
      .split(": ")
      .nth(1)
      .unwrap()
      .split(' ')
      .nth(3)
      .unwrap()
      .parse()
      .unwrap();
    let if_false = lines
      .next()
      .unwrap()
      .split(": ")
      .nth(1)
      .unwrap()
      .split(' ')
      .nth(3)
      .unwrap()
      .parse()
      .unwrap();
    Self {
      items,
      operation,
      test,
      if_true,
      if_false,
      inspections: 0,
    }
  }
}

pub fn main() {
  let data = Data::get(11);
  let mut test1 = Barrel::new(&data.test);
  test1.run(20, false);
  assert_eq!(test1.monkey_business(), 10605);
  println!("Day 11, Test 1: {}", test1.monkey_business());

  let mut part1 = Barrel::new(&data.input);
  part1.run(20, false);
  println!("Day 11, Part 1: {}", part1.monkey_business());

  let mut test2 = Barrel::new(&data.test);
  test2.run(10000, true);
  assert_eq!(test2.monkey_business(), 2_713_310_158);
  println!("Day 11, Test 2: {}", test2.monkey_business());

  let mut part2 = Barrel::new(&data.input);
  part2.run(10000, true);
  println!("Day 11, Part 2: {}", part2.monkey_business());
}
