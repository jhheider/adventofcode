use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const TEST: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
  Number(usize),
  Jack,
  Queen,
  King,
  Ace,
}

impl Card {
  fn new(card: char) -> Self {
    match card {
      'T' => Self::Number(10),
      'J' => Self::Jack,
      'Q' => Self::Queen,
      'K' => Self::King,
      'A' => Self::Ace,
      _ => Self::Number(card.to_digit(10).unwrap() as usize),
    }
  }

  fn value(&self) -> usize {
    match self {
      Self::Number(n) => *n,
      Self::Jack => 11,
      Self::Queen => 12,
      Self::King => 13,
      Self::Ace => 14,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
  cards: [Card; 5],
  bid: usize,
  part2: bool,
}

impl Hand {
  fn new(input: &str, part2: bool) -> Self {
    let mut cards = [Card::Number(0); 5];
    let bid = input.split_whitespace().last().unwrap().parse().unwrap();
    for (i, card) in input.split_whitespace().next().unwrap().chars().enumerate() {
      cards[i] = Card::new(card);
    }
    Self { cards, bid, part2 }
  }

  fn score(&self) -> usize {
    if self.part2 {
      return self.score2();
    }
    self.score1()
  }

  fn score1(&self) -> usize {
    let mut values: HashMap<usize, usize> = HashMap::new();
    for card in self.cards.into_iter() {
      values.insert(card.value(), values.get(&card.value()).unwrap_or(&0) + 1);
    }
    if values.values().any(|&v| v == 5) {
      return 50; // 5 of a kind
    } else if values.values().any(|&v| v == 4) {
      return 40; // 4 of a kind
    } else if values.values().any(|&v| v == 3) && values.values().any(|&v| v == 2) {
      return 35; // Full house
    } else if values.values().any(|&v| v == 3) {
      return 30; // 3 of a kind
    } else if values.values().filter(|&v| *v == 2).count() == 2 {
      return 20; // 2 pairs
    } else if values.values().any(|&v| v == 2) {
      return 15; // 1 pair
    }
    0 // High card
  }

  fn score2(&self) -> usize {
    let mut values: HashMap<usize, usize> = HashMap::new();
    for card in self.cards.into_iter() {
      values.insert(card.value(), values.get(&card.value()).unwrap_or(&0) + 1);
    }

    let jokers = values.remove(&11).unwrap_or(0);
    if jokers == 0 {
      return self.score1();
    }
    let count = values.values().count();
    match (jokers, count) {
      (5, _) | (_, 1) => 50, // 5 of a kind
      (_, 5) => 0,           // High card
      (_, 4) => 15,          // 1 pair
      (_, 3) => 30,          // 3 of a kind
      (1, 2) => {
        if values.values().any(|&v| v == 3) {
          return 40; // 4 of a kind
        }
        35 // Full house
      }
      (_, 2) => 40, // 4 of a kind
      _ => 0,       // High card
    }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    let a = self.score();
    let b = other.score();
    let cmp = a.cmp(&b);
    if cmp != Ordering::Equal {
      return cmp;
    }
    for (a, b) in self.cards.iter().zip(other.cards.iter()) {
      let av = if a.value() == 11 { 1 } else { a.value() };
      let bv = if b.value() == 11 { 1 } else { b.value() };
      let cmp = av.cmp(&bv);
      if cmp != Ordering::Equal {
        return cmp;
      }
    }
    Ordering::Equal
  }
}

#[derive(Debug)]
struct Game {
  hands: Vec<Hand>,
}

impl Game {
  fn new(input: &str, part2: bool) -> Self {
    let hands = input.lines().map(|line| Hand::new(line, part2)).collect();
    Self { hands }
  }

  fn score(&self) -> usize {
    let mut played = self.hands.clone();
    played.sort();
    played
      .iter()
      .enumerate()
      .map(|(i, hand)| hand.bid * (i + 1))
      .sum()
  }
}

pub fn main() {
  let test = Game::new(TEST, false).score();

  println!("Day 7, Test 1: {}", test);

  let input = fs::read_to_string("data/day7.txt").unwrap();
  let part1 = Game::new(&input, false).score();

  println!("Day 7, Part 1: {}", part1);

  let test = Game::new(TEST, true).score();

  println!("Day 7, Test 2: {}", test);

  let part2 = Game::new(&input, true).score();

  println!("Day 7, Part 2: {}", part2);
}
