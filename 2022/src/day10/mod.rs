use std::fmt::{Display, Formatter};

use crate::data::Data;

struct Cpu {
  x: isize,
  tick: usize,
  states: Vec<isize>,
  crt: Vec<bool>,
}

enum Instruction {
  Noop,
  Addx(isize),
}

impl Cpu {
  fn new() -> Self {
    Self {
      x: 1,
      tick: 1,
      states: vec![],
      crt: vec![false; 240],
    }
  }

  fn tick(&mut self, instruction: &Instruction) {
    for _ in 0..instruction.ticks() {
      if (self.x - (self.tick as isize - 1) % 40).abs() < 2 {
        self.crt[self.tick - 1] = true;
      }
      self.states.push(self.x);
      self.tick += 1;
    }
    self.x += instruction.dx();
  }

  fn run(&mut self, instructions: &[Instruction]) {
    for instruction in instructions {
      self.tick(instruction);
    }
  }

  fn signal_strengths(&self) -> Vec<isize> {
    let mut strengths = vec![];
    for tick in (20..=220).step_by(40) {
      strengths.push(self.states[tick - 1] * tick as isize);
    }
    strengths
  }
}

impl Display for Cpu {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for (i, crt) in self.crt.iter().enumerate() {
      if i % 40 == 0 {
        writeln!(f)?;
      }
      write!(f, "{}", if *crt { '█' } else { ' ' })?;
    }
    writeln!(f)
  }
}

impl Instruction {
  fn new(line: &str) -> Self {
    let mut parts = line.split_whitespace();
    let op = parts.next().unwrap();
    match op {
      "noop" => Self::Noop,
      "addx" => {
        let arg = parts.next().unwrap().parse::<isize>().unwrap();
        Self::Addx(arg)
      }
      _ => panic!("Unknown instruction: {}", op),
    }
  }

  fn ticks(&self) -> usize {
    match self {
      Instruction::Noop => 1,
      Instruction::Addx(_) => 2,
    }
  }

  fn dx(&self) -> isize {
    match self {
      Instruction::Noop => 0,
      Instruction::Addx(x) => *x,
    }
  }
}

pub fn main() {
  let data = Data::get(10);
  let test = data.test.lines().map(Instruction::new).collect::<Vec<_>>();
  let mut test1 = Cpu::new();
  test1.run(&test);
  assert_eq!(test1.signal_strengths().iter().sum::<isize>(), 13140);
  println!(
    "Day 10: Test 1: {}",
    test1.signal_strengths().iter().sum::<isize>()
  );

  let input = data.input.lines().map(Instruction::new).collect::<Vec<_>>();
  let mut part1 = Cpu::new();
  part1.run(&input);
  println!(
    "Day 10: Part 1: {}",
    part1.signal_strengths().iter().sum::<isize>()
  );

  let mut test2 = Cpu::new();
  test2.run(&test);
  println!("Day 10: Test 2: {}", test2);

  let mut part2 = Cpu::new();
  part2.run(&input);
  println!("Day 10: Part 2: {}", part2);
}
