use std::fs;

const TEST: &str = r"A Y
B X
C Z";

struct Turn {
  opponent: Move,
  player: Move,
}

#[derive(Clone, Copy)]
enum Move {
  Rock,
  Paper,
  Scissor,
}

enum Outcome {
  Win,
  Lose,
  Tie,
}

impl Move {
  fn from_str(c: &str) -> Self {
    match c {
      "A" | "X" => Move::Rock,
      "B" | "Y" => Move::Paper,
      "C" | "Z" => Move::Scissor,
      _ => panic!("impossible move"),
    }
  }

  fn points(&self) -> usize {
    match self {
      Move::Rock => 1,
      Move::Paper => 2,
      Move::Scissor => 3,
    }
  }

  fn outcome(&self, against: &Move) -> Outcome {
    match self {
      Move::Rock => match against {
        Move::Rock => Outcome::Tie,
        Move::Paper => Outcome::Lose,
        Move::Scissor => Outcome::Win,
      },
      Move::Paper => match against {
        Move::Rock => Outcome::Win,
        Move::Paper => Outcome::Tie,
        Move::Scissor => Outcome::Lose,
      },
      Move::Scissor => match against {
        Move::Rock => Outcome::Lose,
        Move::Paper => Outcome::Win,
        Move::Scissor => Outcome::Tie,
      },
    }
  }

  fn to_outcome(&self, player: &Move) -> Move {
    match self {
      Move::Rock => match player {
        Move::Rock => Move::Scissor,
        Move::Paper => Move::Rock,
        Move::Scissor => Move::Paper,
      },
      Move::Paper => *player,
      Move::Scissor => match player {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissor,
        Move::Scissor => Move::Rock,
      },
    }
  }

  fn outcome_points(&self) -> usize {
    match self {
      Move::Rock => 0,
      Move::Paper => 3,
      Move::Scissor => 6,
    }
  }
}

impl Outcome {
  fn points(&self) -> usize {
    match self {
      Outcome::Lose => 0,
      Outcome::Tie => 3,
      Outcome::Win => 6,
    }
  }
}

impl Turn {
  fn outcome(&self) -> Outcome {
    self.player.outcome(&self.opponent)
  }

  fn points(&self) -> usize {
    self.player.points() + self.outcome().points()
  }

  fn points_from_outcome(&self) -> usize {
    self.opponent.to_outcome(&self.player).points() + self.player.outcome_points()
  }
}

pub fn main() {
  let test = parse(TEST);
  let input = parse(&fs::read_to_string("data/day2.txt").unwrap());

  let test1: usize = test.iter().map(|t| t.points()).sum();
  assert_eq!(test1, 15);
  println!("Day 2: Test 1: total score is {test1}");

  let part1: usize = input.iter().map(|t| t.points()).sum();
  assert_eq!(part1, 13052);
  println!("Day 2: Part 1: total score is {part1}");

  let test2: usize = test.iter().map(|t| t.points_from_outcome()).sum();
  assert_eq!(test2, 12);
  println!("Day 2: Test 2: total score is {test2}");

  let part2: usize = input.iter().map(|t| t.points_from_outcome()).sum();
  assert_eq!(part2, 13693);
  println!("Day 2: Part 2: total score is {part2}");
}

fn parse(input: &str) -> Vec<Turn> {
  input
    .lines()
    .map(|l| {
      let mut split = l.split(' ');
      let i = split.next().unwrap();
      let o = split.next().unwrap();
      Turn {
        opponent: Move::from_str(i),
        player: Move::from_str(o),
      }
    })
    .collect()
}
