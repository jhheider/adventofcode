use crate::data::Data;
use std::collections::HashSet;

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
  let data = Data::get(4);
  let test: usize = data.test.lines().map(Card::new).map(|c| c.score()).sum();
  println!("Day 4, Test 1: {}", test);

  let part1 = data
    .input
    .lines()
    .map(Card::new)
    .map(|c| c.score())
    .sum::<usize>();
  println!("Day 4, Part 1: {}", part1);

  let test2 = Collection::new(&data.test).score();
  println!("Day 4, Test 2: {}", test2);

  let part2 = Collection::new(&data.input).score();
  println!("Day 4, Part 2: {}", part2);
}
