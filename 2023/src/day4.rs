use std::collections::HashSet;
use std::fs;

const TEST: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[derive(Debug)]
struct Card {
  _id: usize,
  winning: Vec<usize>,
  have: Vec<usize>,
}

impl Card {
  fn new(input: &str) -> Self {
    let _id = input
      .split(':')
      .next()
      .unwrap()
      .split(' ')
      .filter(|s| !s.is_empty())
      .last()
      .unwrap()
      .parse::<usize>()
      .unwrap();

    let winning = input
      .split('|')
      .next()
      .unwrap()
      .split(' ')
      .filter(|s| !s.is_empty())
      .skip(2)
      .map(|n| n.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();

    let have = input
      .split('|')
      .last()
      .unwrap()
      .split(' ')
      .filter(|s| !s.is_empty())
      .map(|n| n.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();
    Self { _id, winning, have }
  }

  fn score(&self) -> usize {
    let score = self.score2();
    if score == 0 {
      return 0;
    }
    2usize.pow(score as u32 - 1)
  }

  fn score2(&self) -> usize {
    let w: HashSet<usize> = HashSet::from_iter(self.winning.clone());
    let count = self.have.iter().filter(|n| w.contains(n)).count();
    count
  }
}

#[derive(Debug)]
struct Collection {
  cards: Vec<Card>,
  totals: Vec<usize>,
}

impl Collection {
  fn new(input: &str) -> Self {
    let cards = input.lines().map(Card::new).collect::<Vec<Card>>();
    let totals = vec![1; cards.len()];
    Self { cards, totals }
  }

  fn score(&mut self) -> usize {
    for i in 0..self.cards.len() {
      let score = self.cards[i].score2();
      if score == 0 {
        continue;
      }
      for j in 1..=score {
        self.totals[i + j] += self.totals[i];
      }
    }
    self.totals.iter().sum()
  }
}

pub fn main() {
  let test: usize = TEST.lines().map(Card::new).map(|c| c.score()).sum();
  println!("Day 4, Test 1: {}", test);

  let input = fs::read_to_string("data/day4.txt").unwrap();

  let part1 = input
    .lines()
    .map(Card::new)
    .map(|c| c.score())
    .sum::<usize>();
  println!("Day 4, Part 1: {}", part1);

  let test2 = Collection::new(TEST).score();
  println!("Day 4, Test 2: {}", test2);

  let part2 = Collection::new(&input).score();
  println!("Day 4, Part 2: {}", part2);
}
