use std::cmp::max;
use std::fs;

const TEST: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

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
  let test_games = TEST.lines().map(Game::new).collect::<Vec<_>>();
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

  let input = fs::read_to_string("data/day2.txt").unwrap();
  let games = input.lines().map(Game::new).collect::<Vec<_>>();
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
