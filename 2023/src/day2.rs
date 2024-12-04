use crate::data::Data;
use std::cmp::max;

struct Game {
  id: u32,
  pulls: Vec<Pull>,
}

struct Pull {
  blue: u32,
  green: u32,
  red: u32,
}

struct Bag {
  blue: u32,
  green: u32,
  red: u32,
}

impl Game {
  fn new(line: &str) -> Self {
    let mut parts = line.split(": ");
    let id = parts
      .next()
      .unwrap()
      .split(' ')
      .nth(1)
      .unwrap()
      .parse()
      .unwrap();
    let pulls = parts.next().unwrap().split("; ").map(Pull::new).collect();
    Self { id, pulls }
  }

  fn valid(&self, bag: &Bag) -> bool {
    self.pulls.iter().all(|pull| pull.valid(bag))
  }
}

impl Pull {
  fn new(line: &str) -> Self {
    let parts = line.split(", ");
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;
    for p in parts {
      let mut parts = p.split(' ');
      let count = parts.next().unwrap().parse().unwrap();
      let color = parts.next().unwrap();
      match color {
        "blue" => blue = count,
        "green" => green = count,
        "red" => red = count,
        _ => panic!("Invalid color"),
      }
    }
    Self { blue, green, red }
  }

  fn valid(&self, bag: &Bag) -> bool {
    self.blue <= bag.blue && self.green <= bag.green && self.red <= bag.red
  }
}

pub fn main() {
  let data = Data::get(2);
  let test_games = data.test.lines().map(Game::new).collect::<Vec<_>>();
  let bag = Bag {
    blue: 14,
    green: 13,
    red: 12,
  };

  let test = test_games
    .iter()
    .filter(|game| game.valid(&bag))
    .map(|game| game.id)
    .sum::<u32>();
  println!("Day 2, Test: {}", test);

  let games = data.input.lines().map(Game::new).collect::<Vec<_>>();
  let part1 = games
    .iter()
    .filter(|game| game.valid(&bag))
    .map(|game| game.id)
    .sum::<u32>();
  println!("Day 2, Part 1: {}", part1);

  println!("Day 2, Test 2: {}", power(&test_games));
  println!("Day 2, Part 2: {}", power(&games));
}

fn power(games: &[Game]) -> u32 {
  let mut power = 0;
  for game in games {
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;
    for pull in &game.pulls {
      blue = max(blue, pull.blue);
      green = max(green, pull.green);
      red = max(red, pull.red);
    }
    power += blue * green * red;
  }
  power
}
