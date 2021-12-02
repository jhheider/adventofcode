use std::fs;

enum Direction {
  Down,
  Forward,
  Up,
}

pub fn main() {
  let test = parse(r"forward 5
down 5
forward 8
up 3
down 8
forward 2".to_string());
  let input = parse(fs::read_to_string("data/day2.txt").unwrap());

  let test1 = run(&test);
  assert_eq!(test1.0 * test1.1, 150);
  println!("Day 2: Test 1: final position is ({}, {}); product: {}", test1.0, test1.1, test1.0 * test1.1);

  let part1 = run(&input);
  assert_eq!(part1.0 * part1.1, 2039256);
  println!("Day 2: Part 1: final position is ({}, {}); product: {}", part1.0, part1.1, part1.0 * part1.1);

  let test2 = run2(&test);
  assert_eq!(test2.0 * test2.1, 900);
  println!("Day 2: Test 2: final position is ({}, {}); product: {}", test2.0, test2.1, test2.0 * test2.1);

  let part2 = run2(&input);
  assert_eq!(part2.0 * part2.1, 1856459736);
  println!("Day 2: Part 2: final position is ({}, {}); product: {}", part2.0, part2.1, part2.0 * part2.1);
}

fn parse(input: String) -> Vec<(Direction, i32)> {
  input
    .lines()
    .map(|line| {
      let fields: Vec<&str> = line.split(' ').collect();

      let direction = match fields[0] {
        "down" => Direction::Down,
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        _ => panic!("Unexpected direction in input {}", fields[0]),
      };
      (direction, fields[1].parse::<i32>().unwrap())
    })
    .collect::<Vec<(Direction, i32)>>()
}

fn run(steps: &[(Direction, i32)]) -> (i32, i32) {
  let mut position = (0, 0);
  for step in steps {
    match step.0 {
      Direction::Down => position = (position.0, position.1 + step.1),
      Direction::Forward => position = (position.0 + step.1, position.1),
      Direction::Up => position = (position.0, position.1 - step.1),
    };
  }
  position
}

fn run2(steps: &[(Direction, i32)]) -> (i32, i32) {
  let mut position = (0, 0, 0);
  for step in steps {
    match step.0 {
      Direction::Down => position = (position.0, position.1, position.2 + step.1),
      Direction::Up => position = (position.0, position.1, position.2 - step.1),
      Direction::Forward => position = (position.0 + step.1, position.1 + step.1 * position.2, position.2),
    };
  }
  (position.0, position.1)
}
