use std::collections::HashMap;

use crate::data::Data;

struct Computer {
    registers: HashMap<char, isize>,
    program: Vec<Instruction>,
    pc: usize,
}

enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

impl Computer {
    fn new(program: Vec<Instruction>, a: isize) -> Computer {
        Computer {
            registers: HashMap::from_iter([('a', a), ('b', 0)].iter().cloned()),
            program,
            pc: 0,
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let instruction = &self.program[self.pc];
        match *instruction {
            Instruction::Hlf(r) => {
                *self.registers.get_mut(&r).unwrap() /= 2;
                self.pc += 1;
            }
            Instruction::Tpl(r) => {
                *self.registers.get_mut(&r).unwrap() *= 3;
                self.pc += 1;
            }
            Instruction::Inc(r) => {
                *self.registers.get_mut(&r).unwrap() += 1;
                self.pc += 1;
            }
            Instruction::Jmp(offset) => {
                self.pc = (self.pc as isize + offset) as usize;
            }
            Instruction::Jie(r, offset) => {
                if *self.registers.get(&r).unwrap() % 2 == 0 {
                    self.pc = (self.pc as isize + offset) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jio(r, offset) => {
                if *self.registers.get(&r).unwrap() == 1 {
                    self.pc = (self.pc as isize + offset) as usize;
                } else {
                    self.pc += 1;
                }
            }
        }
    }
}

pub fn main() {
    let data = Data::get(23);
    let mut test = Computer::new(parse(&data.test), 0);
    test.run();
    let test1 = test.registers.get(&'a').unwrap();
    assert_eq!(*test1, 2);
    println!("Day 23: Test 1: {}", test1);

    let mut input = Computer::new(parse(&data.input), 0);
    input.run();
    let part1 = input.registers.get(&'b').unwrap();
    println!("Day 23: Part 1: {}", part1);

    let mut input2 = Computer::new(parse(&data.input), 1);
    input2.run();
    let part2 = input2.registers.get(&'b').unwrap();
    println!("Day 23: Part 2: {}", part2);
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            let arg = parts.next().unwrap();
            match instruction {
                "hlf" => Instruction::Hlf(arg.chars().next().unwrap()),
                "tpl" => Instruction::Tpl(arg.chars().next().unwrap()),
                "inc" => Instruction::Inc(arg.chars().next().unwrap()),
                "jmp" => Instruction::Jmp(arg.parse().unwrap()),
                "jie" => Instruction::Jie(
                    arg.chars().next().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                ),
                "jio" => Instruction::Jio(
                    arg.chars().next().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                ),
                _ => panic!("Unknown instruction: {}", instruction),
            }
        })
        .collect()
}
