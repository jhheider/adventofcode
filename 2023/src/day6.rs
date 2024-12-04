use crate::data::Data;

pub fn main() {
  let data = Data::get(6);
  let test = parse(&data.test);
  let test_wins = test.iter().map(|i| i.wins()).product::<usize>();

  println!("Day 6, Test 1: {}", test_wins);

  let input = parse(&data.input);
  let part1 = input.iter().map(|i| i.wins()).product::<usize>();

  println!("Day 6, Part 1: {}", part1);

  let test2 = parse2(&data.test);
  let test2_wins = test2.wins();

  println!("Day 6, Test 2: {}", test2_wins);

  let input2 = parse2(&data.input);
  let part2 = input2.wins();

  println!("Day 6, Part 2: {}", part2);
}

#[derive(Debug)]
struct Race {
  time: usize,
  distance: usize,
}

impl Race {
  fn new((time, distance): (&str, &str)) -> Self {
    Self {
      time: time.parse().unwrap(),
      distance: distance.parse().unwrap(),
    }
  }

  fn wins(&self) -> usize {
    (1..self.time)
      .filter(|&t| t * (self.time - t) > self.distance)
      .count()
  }
}

fn parse(input: &str) -> Vec<Race> {
  let times = input.lines().next().unwrap().split_whitespace().skip(1);
  let distances = input.lines().last().unwrap().split_whitespace().skip(1);

  times.zip(distances).map(Race::new).collect()
}

fn parse2(input: &str) -> Race {
  let time = input
    .lines()
    .next()
    .unwrap()
    .split(':')
    .nth(1)
    .unwrap()
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>()
    .parse()
    .unwrap();
  let distance = input
    .lines()
    .last()
    .unwrap()
    .split(':')
    .nth(1)
    .unwrap()
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>()
    .parse()
    .unwrap();
  Race { time, distance }
}
